use std::path::PathBuf;
use std::sync::atomic::Ordering;
use tauri::{Emitter, Manager};

mod utils;
mod command_runner;
mod i18n;

use crate::utils::{
    is_ffmpeg_installed, detect_js_runtime, cleanup_incomplete_files, sanitize_folder_name,
    video_folder_name, CsvVideoRow, format_duration_seconds, write_channel_summary_csv,
};
use crate::i18n::UiLocale;
use crate::command_runner::{
    AppState, DownloadOptions, VideoDownloadTarget, ChannelInfo, ProgressPayload,
    fetch_channel_videos, download_single_video,
};

#[derive(serde::Serialize)]
struct EnvStatus {
    ffmpeg_installed: bool,
    js_runtime_installed: bool,
    js_runtime_name: Option<String>,
}

#[tauri::command]
async fn check_environment(app: tauri::AppHandle) -> EnvStatus {
    let js_runtime = detect_js_runtime(&app);
    EnvStatus {
        ffmpeg_installed: is_ffmpeg_installed(&app),
        js_runtime_installed: js_runtime.installed,
        js_runtime_name: js_runtime.runtime,
    }
}

#[tauri::command]
async fn get_channel_videos(
    app: tauri::AppHandle,
    url: String,
    cookies_browser: String,
    locale: String,
) -> Result<ChannelInfo, String> {
    fetch_channel_videos(app, &url, &cookies_browser, UiLocale::parse(&locale)).await
}

#[tauri::command]
async fn start_download_archive(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    options: DownloadOptions,
    videos: Vec<VideoDownloadTarget>,
    channel_title: String,
    delay_seconds: u64,
    custom_dir: Option<String>,
    locale: String,
) -> Result<(), String> {
    let ui_locale = UiLocale::parse(&locale);
    // Reset cancellation flag
    state.is_cancelled.store(false, Ordering::SeqCst);

    // Resolve base download directory
    let base_dir = if let Some(path_str) = custom_dir {
        PathBuf::from(path_str)
    } else {
        app.path().download_dir().map_err(|e| e.to_string())?
    };

    let sanitized_title = sanitize_folder_name(&channel_title);
    let channel_dir = base_dir.join("YankTrove").join(sanitized_title);

    // Ensure download folder exists
    std::fs::create_dir_all(&channel_dir)
        .map_err(|e| format!("Failed to create directory {:?}: {}", channel_dir, e))?;

    let total_count = videos.len();
    let mut csv_rows: Vec<CsvVideoRow> = Vec::new();

    for (index, video) in videos.iter().enumerate() {
        if state.is_cancelled.load(Ordering::SeqCst) {
            return Err("Cancelled".to_string());
        }

        let video_dir = channel_dir.join(video_folder_name(&video.title, &video.id));
        std::fs::create_dir_all(&video_dir)
            .map_err(|e| format!("Failed to create directory {:?}: {}", video_dir, e))?;

        if options.csv {
            csv_rows.push(CsvVideoRow {
                id: video.id.clone(),
                title: video.title.clone(),
                url: video.url.clone(),
                uploaded_at: video.uploaded_at.clone().unwrap_or_default(),
                duration: format_duration_seconds(video.duration),
                availability: video
                    .availability
                    .as_deref()
                    .map(|value| crate::i18n::csv_availability_label(value, ui_locale))
                    .unwrap_or_default(),
                save_path: video_dir.to_string_lossy().into_owned(),
            });
        }

        // Notify UI that video processing has started
        let _ = app.emit("video-started", video.id.clone());

        match download_single_video(
            &app,
            &state,
            &video.id,
            &video.url,
            &options,
            &video_dir,
            ui_locale,
        )
        .await
        {
            Ok(_) => {
                let _ = app.emit("video-finished", video.id.clone());
            }
            Err(e) => {
                if e == "Cancelled" {
                    // Clean up incomplete files for the active download
                    let _ = cleanup_incomplete_files(&video_dir, &video.id);
                    let _ = app.emit("download-log", ProgressPayload {
                        video_id: video.id.clone(),
                        percentage: 0.0,
                        speed: None,
                        eta: None,
                        status: "Cancelled".to_string(),
                        log: Some(crate::i18n::download_cancelled_log(&video.id, ui_locale)),
                    });
                    return Err("Cancelled".to_string());
                } else {
                    let _ = app.emit("download-log", ProgressPayload {
                        video_id: video.id.clone(),
                        percentage: 0.0,
                        speed: None,
                        eta: None,
                        status: "Error".to_string(),
                        log: Some(crate::i18n::video_failed_log(&video.id, &e, ui_locale)),
                    });
                }
            }
        }

        // Sleep delay (IP ban rate limit protection)
        if index + 1 < total_count && !state.is_cancelled.load(Ordering::SeqCst) {
            let _ = app.emit("download-log", ProgressPayload {
                video_id: "".to_string(),
                percentage: 0.0,
                speed: None,
                eta: None,
                status: "Waiting".to_string(),
                log: Some(crate::i18n::waiting_before_next(delay_seconds, ui_locale)),
            });
            tokio::time::sleep(tokio::time::Duration::from_secs(delay_seconds)).await;
        }
    }

    if options.csv && !state.is_cancelled.load(Ordering::SeqCst) {
        let csv_path = channel_dir.join("summary.csv");
        match write_channel_summary_csv(&csv_path, &csv_rows, ui_locale) {
            Ok(()) => {
                let _ = app.emit("download-log", ProgressPayload {
                    video_id: "".to_string(),
                    percentage: 0.0,
                    speed: None,
                    eta: None,
                    status: "Finished".to_string(),
                    log: Some(crate::i18n::csv_written_log(
                        &csv_path.to_string_lossy(),
                        ui_locale,
                    )),
                });
            }
            Err(error) => {
                let _ = app.emit("download-log", ProgressPayload {
                    video_id: "".to_string(),
                    percentage: 0.0,
                    speed: None,
                    eta: None,
                    status: "Error".to_string(),
                    log: Some(crate::i18n::csv_write_failed_log(&error.to_string(), ui_locale)),
                });
            }
        }
    }

    Ok(())
}

#[tauri::command]
async fn cancel_downloads(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.is_cancelled.store(true, Ordering::SeqCst);

    // Terminate currently running child process if any
    if let Some(child) = state.active_process.lock().await.take() {
        let _ = child.kill();
    }

    // Clean up current active download directory
    let current_id = state.current_video_id.lock().await.take();
    let current_dir = state.current_download_dir.lock().await.take();

    if let (Some(id), Some(dir)) = (current_id, current_dir) {
        let _ = cleanup_incomplete_files(&dir, &id);
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            check_environment,
            get_channel_videos,
            start_download_archive,
            cancel_downloads
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
