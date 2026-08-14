use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::LazyLock;
use tokio::sync::Mutex;
use tauri::Emitter;
use tauri_plugin_shell::process::{CommandChild, CommandEvent};

use crate::utils::{spawn_yt_dlp, is_js_challenge_error};
use crate::text_decode::StreamDecoder;
use crate::i18n::{
    UiLocale, channel_not_found_error, cookie_extraction_error_message, js_runtime_setup_message,
    video_list_unavailable, ytdlp_fetch_error,
};

static PROGRESS_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"\[download\]\s+(\d+\.\d+)%").unwrap()
});

static SPEED_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"at\s+([^\s]+(?:B|b)/s)").unwrap()
});

static ETA_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"ETA\s+([^\s]+)").unwrap()
});

/// Shared application state to manage active downloads and cancellation
pub struct AppState {
    pub active_process: Mutex<Option<CommandChild>>,
    pub is_cancelled: AtomicBool,
    pub current_video_id: Mutex<Option<String>>,
    pub current_download_dir: Mutex<Option<PathBuf>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            active_process: Mutex::new(None),
            is_cancelled: AtomicBool::new(false),
            current_video_id: Mutex::new(None),
            current_download_dir: Mutex::new(None),
        }
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct VideoDownloadTarget {
    pub id: String,
    pub url: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub duration: Option<f64>,
    #[serde(default)]
    pub uploaded_at: Option<String>,
    #[serde(default)]
    pub availability: Option<String>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct DownloadOptions {
    pub chat: bool,
    pub metadata: bool,
    pub description: bool,
    pub subtitles: bool,
    pub thumbnail: bool,
    pub video: bool,
    pub audio: bool,
    pub audio_format: String, // "mp3" or "m4a"
    pub cookies_browser: String, // "chrome", "firefox", "edge", "safari", "none"
    #[serde(default)]
    pub csv: bool,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct VideoInfo {
    pub id: String,
    pub title: String,
    pub url: String,
    pub duration: Option<f64>,
    pub uploaded_at: Option<String>,
    pub availability: Option<String>,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct ChannelInfo {
    pub channel_title: String,
    pub videos: Vec<VideoInfo>,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct ProgressPayload {
    pub video_id: String,
    pub percentage: f32,
    pub speed: Option<String>,
    pub eta: Option<String>,
    pub status: String, // "Downloading", "Processing", "Finished", "Cancelled", "Error"
    pub log: Option<String>,
}

/// Helper to parse progress percentage, speed, and ETA from yt-dlp output
fn parse_progress(line: &str) -> Option<(f32, Option<String>, Option<String>)> {
    if !line.contains("[download]") {
        return None;
    }
    
    let percentage = PROGRESS_REGEX.captures(line)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<f32>().ok())?;

    let speed = SPEED_REGEX.captures(line)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string());

    let eta = ETA_REGEX.captures(line)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string());

    Some((percentage, speed, eta))
}

fn is_cookie_extraction_error(err_summary: &str) -> bool {
    let lower = err_summary.to_lowercase();
    lower.contains("cookie")
        || lower.contains("dpapi")
        || lower.contains("permission denied")
        || lower.contains("could not copy")
}

fn entry_page_url(entry: &serde_json::Value) -> Option<String> {
    entry["url"]
        .as_str()
        .filter(|url| !url.is_empty())
        .or_else(|| entry["webpage_url"].as_str())
        .filter(|url| !url.is_empty())
        .map(str::to_string)
}

fn is_youtube_video_id(id: &str) -> bool {
    id.len() == 11 && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

fn is_channel_tab_entry(entry: &serde_json::Value) -> bool {
    if entry["_type"].as_str() == Some("playlist") {
        return true;
    }

    let id = entry["id"].as_str().unwrap_or("");
    if id.starts_with("UC") && id.len() > 11 && entry["duration"].is_null() {
        return true;
    }

    if let Some(page_url) = entry_page_url(entry) {
        return page_url.contains("/videos")
            || page_url.contains("/streams")
            || page_url.contains("/shorts");
    }

    false
}

fn tab_label_from_entry(entry: &serde_json::Value) -> Option<&'static str> {
    if let Some(page_url) = entry_page_url(entry) {
        if page_url.contains("/streams") {
            return Some("Live");
        }
        if page_url.contains("/shorts") {
            return Some("Shorts");
        }
        if page_url.contains("/videos") {
            return Some("Videos");
        }
    }

    match entry["title"].as_str() {
        Some(title) if title.ends_with(" - Live") => Some("Live"),
        Some(title) if title.ends_with(" - Shorts") => Some("Shorts"),
        Some(title) if title.ends_with(" - Videos") => Some("Videos"),
        _ => None,
    }
}

fn parse_uploaded_at(entry: &serde_json::Value) -> Option<String> {
    // Prefer full timestamps so folder names can include hour/minute.
    if let Some(timestamp) = entry["timestamp"]
        .as_f64()
        .or_else(|| entry["release_timestamp"].as_f64())
        .map(|value| value as i64)
    {
        return Some(timestamp.to_string());
    }

    if let Some(date) = entry["upload_date"].as_str() {
        if date.len() == 8 && date.chars().all(|c| c.is_ascii_digit()) {
            return Some(format!("{}{}{}-0000", &date[0..4], &date[4..6], &date[6..8]));
        }
        if !date.is_empty() {
            return Some(date.to_string());
        }
    }

    None
}

fn parse_video_language(entry: &serde_json::Value) -> Option<String> {
    entry["language"]
        .as_str()
        .or_else(|| entry["lang"].as_str())
        .map(str::trim)
        .filter(|lang| !lang.is_empty())
        .map(str::to_string)
}

/// Per-video metadata from yt-dlp `-J --skip-download` (not flat-playlist).
#[derive(Clone, Debug, Default)]
pub struct VideoProbe {
    pub uploaded_at: Option<String>,
    pub language: Option<String>,
}

/// Fetch full video metadata for folder dates / subtitle language.
/// Independent of the UI "metadata" checkbox (that only controls writing info.json).
pub async fn probe_video(
    app: &tauri::AppHandle,
    video_url: &str,
    cookies_browser: &str,
) -> Option<VideoProbe> {
    let mut args = vec![
        "-J".to_string(),
        "--skip-download".to_string(),
        video_url.to_string(),
    ];

    if cookies_browser != "none" {
        args.push("--cookies-from-browser".to_string());
        args.push(cookies_browser.to_string());
    }

    let (mut rx, _child) = spawn_yt_dlp(app, args).ok()?;
    let mut json_buffer = Vec::new();

    while let Some(event) = rx.recv().await {
        if let CommandEvent::Stdout(bytes) = event {
            json_buffer.extend_from_slice(&bytes);
        }
    }

    let json_str = String::from_utf8(json_buffer).ok()?;
    let parsed: serde_json::Value = serde_json::from_str(&json_str).ok()?;

    Some(VideoProbe {
        uploaded_at: parse_uploaded_at(&parsed),
        language: parse_video_language(&parsed),
    })
}

fn parse_availability(entry: &serde_json::Value) -> Option<String> {
    let availability = entry["availability"]
        .as_str()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    let live_status = entry["live_status"]
        .as_str()
        .map(str::trim)
        .filter(|value| !value.is_empty() && *value != "not_live")
        .map(str::to_string);

    match (availability, live_status) {
        (Some(availability), Some(live_status)) => Some(format!("{availability}/{live_status}")),
        (Some(availability), None) => Some(availability),
        (None, Some(live_status)) => Some(live_status),
        (None, None) => None,
    }
}

fn parse_video_entries(entries: &[serde_json::Value], title_prefix: Option<&str>) -> Vec<VideoInfo> {
    let mut videos = Vec::new();

    for entry in entries {
        if is_channel_tab_entry(entry) {
            continue;
        }

        let id = entry["id"].as_str().unwrap_or("").to_string();
        let Some(page_url) = entry_page_url(entry) else {
            continue;
        };

        if !is_youtube_video_id(&id) {
            continue;
        }

        let raw_title = entry["title"].as_str().unwrap_or("").to_string();
        let title = match title_prefix {
            Some(prefix) if !raw_title.is_empty() => format!("[{prefix}] {raw_title}"),
            _ => raw_title,
        };
        let duration = entry["duration"].as_f64();
        let uploaded_at = parse_uploaded_at(entry);
        let availability = parse_availability(entry);

        videos.push(VideoInfo {
            id,
            title,
            url: page_url,
            duration,
            uploaded_at,
            availability,
        });
    }

    videos
}

async fn run_flat_playlist_json(
    app: &tauri::AppHandle,
    url: &str,
    cookies_browser: &str,
    locale: UiLocale,
) -> Result<serde_json::Value, String> {
    let mut args = vec![
        "--ignore-errors".to_string(),
        "--flat-playlist".to_string(),
        "-J".to_string(),
        url.to_string(),
    ];

    if cookies_browser != "none" {
        args.push("--cookies-from-browser".to_string());
        args.push(cookies_browser.to_string());
    }

    let (mut rx, _child) = spawn_yt_dlp(app, args)?;
    let mut json_buffer = Vec::new();
    let mut stderr_messages = Vec::new();
    let mut stderr_decoder = StreamDecoder::new();

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(bytes) => {
                json_buffer.extend_from_slice(&bytes);
            }
            CommandEvent::Stderr(bytes) => {
                let err_msg = stderr_decoder.push(&bytes);
                if !err_msg.is_empty() {
                    eprintln!("yt-dlp flat-playlist stderr: {}", err_msg);
                    stderr_messages.push(err_msg.trim().to_string());
                }
            }
            CommandEvent::Terminated(status) => {
                let terminated_with_error = !status.code.map(|c| c == 0).unwrap_or(false);
                if terminated_with_error {
                    let err_summary = stderr_messages.join("\n");
                    if cookies_browser != "none" && is_cookie_extraction_error(&err_summary) {
                        return Err(cookie_extraction_error_message(cookies_browser, locale, &err_summary));
                    }
                    if err_summary.contains("404")
                        || err_summary.contains("Not Found")
                        || err_summary.contains("Requested entity was not found")
                    {
                        return Err(channel_not_found_error(url, &err_summary, locale));
                    }
                    if json_buffer.is_empty() || json_buffer.trim_ascii() == b"null" {
                        return Err(ytdlp_fetch_error(&err_summary, locale));
                    }
                }
            }
            _ => {}
        }
    }

    let leftover = stderr_decoder.finish();
    if !leftover.trim().is_empty() {
        stderr_messages.push(leftover.trim().to_string());
    }

    let json_str_trimmed = String::from_utf8_lossy(&json_buffer);
    let trimmed = json_str_trimmed.trim();
    if trimmed.is_empty() || trimmed == "null" {
        let err_summary = stderr_messages.join("\n");
        if cookies_browser != "none" && is_cookie_extraction_error(&err_summary) {
            return Err(cookie_extraction_error_message(cookies_browser, locale, &err_summary));
        }
        if err_summary.contains("404")
            || err_summary.contains("Not Found")
            || err_summary.contains("Requested entity was not found")
        {
            return Err(channel_not_found_error(url, &err_summary, locale));
        }
        return Err(video_list_unavailable(&err_summary, locale));
    }

    let json_str = String::from_utf8(json_buffer)
        .map_err(|e| format!("Failed to parse yt-dlp output as UTF-8: {}", e))?;

    serde_json::from_str(&json_str).map_err(|e| format!("Failed to parse metadata JSON: {}", e))
}

async fn collect_videos_from_playlist_json(
    app: &tauri::AppHandle,
    parsed: &serde_json::Value,
    cookies_browser: &str,
    title_prefix: Option<&str>,
    locale: UiLocale,
) -> Result<Vec<VideoInfo>, String> {
    let entries = parsed["entries"]
        .as_array()
        .cloned()
        .unwrap_or_default();

    if entries.is_empty() {
        return Ok(Vec::new());
    }

    if entries.iter().all(is_channel_tab_entry) {
        let mut videos = Vec::new();
        for tab_entry in entries {
            let Some(tab_url) = entry_page_url(&tab_entry) else {
                continue;
            };
            let tab_label = tab_label_from_entry(&tab_entry);
            let tab_json = run_flat_playlist_json(app, &tab_url, cookies_browser, locale).await?;
            let tab_entries = tab_json["entries"]
                .as_array()
                .cloned()
                .unwrap_or_default();
            videos.extend(parse_video_entries(&tab_entries, tab_label));
        }
        return Ok(videos);
    }

    Ok(parse_video_entries(&entries, title_prefix))
}

fn dedupe_videos(videos: Vec<VideoInfo>) -> Vec<VideoInfo> {
    let mut seen = std::collections::HashSet::new();
    videos
        .into_iter()
        .filter(|video| seen.insert(video.id.clone()))
        .collect()
}

/// Fetches the list of all video archives in the channel/playlist
pub async fn fetch_channel_videos(
    app: tauri::AppHandle,
    url: &str,
    cookies_browser: &str,
    locale: UiLocale,
) -> Result<ChannelInfo, String> {
    let parsed = run_flat_playlist_json(&app, url, cookies_browser, locale).await?;

    if parsed.is_null() {
        return Err(channel_not_found_error(url, "null response", locale));
    }

    let channel_title = parsed["title"]
        .as_str()
        .or_else(|| parsed["uploader"].as_str())
        .unwrap_or("Unknown Channel")
        .to_string();

    let videos = dedupe_videos(
        collect_videos_from_playlist_json(&app, &parsed, cookies_browser, None, locale).await?,
    );

    Ok(ChannelInfo {
        channel_title,
        videos,
    })
}

async fn fetch_video_language(
    app: &tauri::AppHandle,
    video_url: &str,
    cookies_browser: &str,
) -> Option<String> {
    probe_video(app, video_url, cookies_browser)
        .await
        .and_then(|probe| probe.language)
}

/// Build a yt-dlp --sub-langs value that prefers the video's original language.
fn build_original_sub_lang_filter(language: Option<&str>) -> String {
    match language.filter(|lang| !lang.is_empty()) {
        // English auto-captions often use plain "en" without an "-orig" suffix.
        Some("en") => ".+-orig,en".to_string(),
        Some(lang) => format!(".+-orig,{},-{}", lang, lang),
        None => ".+-orig,en".to_string(),
    }
}

/// Core function to download files for a specific video
pub async fn download_single_video(
    app: &tauri::AppHandle,
    state: &AppState,
    video_id: &str,
    video_url: &str,
    options: &DownloadOptions,
    output_dir: &Path,
    locale: UiLocale,
    prefetched_language: Option<String>,
) -> Result<(), String> {
    // Save current video ID and target folder in AppState for cancellation cleanup
    {
        *state.current_video_id.lock().await = Some(video_id.to_string());
        *state.current_download_dir.lock().await = Some(output_dir.to_path_buf());
    }

    let mut args = vec![video_url.to_string()];

    // Cookies
    if options.cookies_browser != "none" {
        args.push("--cookies-from-browser".to_string());
        args.push(options.cookies_browser.clone());
    }

    // Duplicate avoidance (No overwrites)
    args.push("--no-overwrites".to_string());

    // Configure downloads
    if options.metadata {
        args.push("--write-info-json".to_string());
    }
    if options.description {
        args.push("--write-description".to_string());
    }
    // Live chat is a subtitle track named "live_chat" (there is no --write-chat).
    if options.chat || options.subtitles {
        let mut sub_langs: Vec<String> = Vec::new();

        if options.chat {
            sub_langs.push("live_chat".to_string());
        }

        if options.subtitles {
            let video_language = match prefetched_language {
                Some(lang) => Some(lang),
                None => fetch_video_language(app, video_url, &options.cookies_browser).await,
            };
            sub_langs.push(build_original_sub_lang_filter(video_language.as_deref()));

            args.push("--extractor-args".to_string());
            args.push("youtube:skip=translated_subs".to_string());
            args.push("--write-auto-subs".to_string());
        }

        args.push("--write-subs".to_string());
        args.push("--sub-langs".to_string());
        args.push(sub_langs.join(","));

        // Prefer VTT for captions; allow json3 so live_chat can still download.
        if options.subtitles {
            args.push("--sub-format".to_string());
            args.push(if options.chat {
                "vtt/json3/best".to_string()
            } else {
                "vtt".to_string()
            });
        }
    }
    if options.thumbnail {
        args.push("--write-thumbnail".to_string());
        args.push("--convert-thumbnails".to_string());
        args.push("jpg".to_string());
    }

    // Video / Audio formatting
    if options.video {
        // High quality mp4
        args.push("-f".to_string());
        args.push("bv*[ext=mp4]+ba[ext=m4a]/bv*+ba/b".to_string());
        args.push("--merge-output-format".to_string());
        args.push("mp4".to_string());
    } else if options.audio {
        // Extract audio only
        args.push("-x".to_string());
        args.push("--audio-format".to_string());
        args.push(options.audio_format.clone());
    }

    // Skip downloading actual streams if neither video nor audio is selected
    if !options.video && !options.audio {
        args.push("--skip-download".to_string());
    }

    // Output template inside the per-video subdirectory
    // Example: {date}_{title}/{title} [{id}].ext
    args.push("--windows-filenames".to_string());
    let output_template = output_dir.join("%(title)s [%(id)s].%(ext)s");
    let output_str = output_template.to_string_lossy().to_string();
    args.push("-o".to_string());
    args.push(output_str);

    // Spawning the download process
    let (mut rx, child) = spawn_yt_dlp(app, args)?;
    
    // Store process handle in state
    {
        let mut active = state.active_process.lock().await;
        if state.is_cancelled.load(Ordering::SeqCst) {
            let _ = child.kill();
            return Err("Cancelled".to_string());
        }
        *active = Some(child);
    }

    let mut error_occurred = false;
    let mut js_challenge_failed = false;
    let mut last_err_msg = String::new();
    let mut stderr_messages = Vec::new();
    let mut stdout_decoder = StreamDecoder::new();
    let mut stderr_decoder = StreamDecoder::new();

    while let Some(event) = rx.recv().await {
        // Check cancellation
        if state.is_cancelled.load(Ordering::SeqCst) {
            return Err("Cancelled".to_string());
        }

        match event {
            CommandEvent::Stdout(bytes) => {
                let line = stdout_decoder.push(&bytes);
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                // Emit log line to frontend
                let _ = app.emit("download-log", ProgressPayload {
                    video_id: video_id.to_string(),
                    percentage: 0.0,
                    speed: None,
                    eta: None,
                    status: "Downloading".to_string(),
                    log: Some(trimmed.to_string()),
                });

                // Parse progress values
                if let Some((percentage, speed, eta)) = parse_progress(trimmed) {
                    let _ = app.emit("download-progress", ProgressPayload {
                        video_id: video_id.to_string(),
                        percentage,
                        speed,
                        eta,
                        status: "Downloading".to_string(),
                        log: None,
                    });
                }
            }
            CommandEvent::Stderr(bytes) => {
                let line = stderr_decoder.push(&bytes);
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    last_err_msg = trimmed.to_string();
                    stderr_messages.push(trimmed.to_string());
                    if is_js_challenge_error(trimmed) {
                        js_challenge_failed = true;
                    }
                    if trimmed.to_lowercase().contains("error") {
                        error_occurred = true;
                    }
                    let _ = app.emit("download-log", ProgressPayload {
                        video_id: video_id.to_string(),
                        percentage: 0.0,
                        speed: None,
                        eta: None,
                        status: "Downloading".to_string(),
                        log: Some(format!("[ERROR] {}", trimmed)),
                    });
                }
            }
            CommandEvent::Terminated(status) => {
                let leftover_out = stdout_decoder.finish();
                if !leftover_out.trim().is_empty() {
                    let _ = app.emit("download-log", ProgressPayload {
                        video_id: video_id.to_string(),
                        percentage: 0.0,
                        speed: None,
                        eta: None,
                        status: "Downloading".to_string(),
                        log: Some(leftover_out.trim().to_string()),
                    });
                }
                let leftover_err = stderr_decoder.finish();
                if !leftover_err.trim().is_empty() {
                    last_err_msg = leftover_err.trim().to_string();
                    stderr_messages.push(leftover_err.trim().to_string());
                }

                // Clear active child process
                {
                    *state.active_process.lock().await = None;
                }

                if state.is_cancelled.load(Ordering::SeqCst) {
                    return Err("Cancelled".to_string());
                }

                if !status.code.map(|c| c == 0).unwrap_or(false) {
                    let err_summary = if stderr_messages.is_empty() {
                        last_err_msg.clone()
                    } else {
                        stderr_messages.join("\n")
                    };
                    if options.cookies_browser != "none"
                        && is_cookie_extraction_error(&err_summary)
                    {
                        return Err(cookie_extraction_error_message(
                            &options.cookies_browser,
                            locale,
                            &err_summary,
                        ));
                    }
                    if js_challenge_failed || is_js_challenge_error(&last_err_msg) {
                        return Err(js_runtime_setup_message(locale));
                    }
                    if error_occurred {
                        return Err(ytdlp_fetch_error(&last_err_msg, locale));
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}
