use std::path::{Path, PathBuf};
use std::fs;
use tauri::Manager;
use tauri_plugin_shell::ShellExt;

const WINDOWS_RESERVED_NAMES: &[&str] = &[
    "CON", "PRN", "AUX", "NUL",
    "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
    "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

/// Removes characters that are illegal in Windows / macOS folder names.
pub fn sanitize_folder_name(name: &str) -> String {
    let without_illegal: String = name
        .chars()
        .map(|ch| match ch {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => ' ',
            c if c.is_control() => ' ',
            c => c,
        })
        .collect();
    without_illegal
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_matches(|c: char| c == '.' || c.is_whitespace())
        .to_string()
}

/// Folder name for one video: `{YYYYMMDD-hhmm}_{sanitized title}` (UTC).
/// Title is truncated so the folder stays within common path limits.
/// Missing datetimes become `unknown-date`. Unix timestamps become UTC date+time.
pub fn video_folder_name(title: &str, uploaded_at: Option<&str>) -> String {
    let date = folder_datetime_prefix(uploaded_at);

    let mut name = sanitize_folder_name(title);
    if name.is_empty() {
        name = "untitled".to_string();
    }

    const MAX_TITLE_CHARS: usize = 80;
    if name.chars().count() > MAX_TITLE_CHARS {
        name = name.chars().take(MAX_TITLE_CHARS).collect::<String>();
        name = name.trim_matches(|c: char| c == '.' || c.is_whitespace()).to_string();
        if name.is_empty() {
            name = "untitled".to_string();
        }
    }

    let folder = format!("{date}_{name}");
    if WINDOWS_RESERVED_NAMES.iter().any(|reserved| folder.eq_ignore_ascii_case(reserved)) {
        format!("_{folder}")
    } else {
        folder
    }
}

fn folder_datetime_prefix(uploaded_at: Option<&str>) -> String {
    let Some(raw) = uploaded_at.map(str::trim).filter(|value| !value.is_empty()) else {
        return "unknown-date".to_string();
    };

    if looks_like_compact_ymd_hm(raw) {
        return raw[..13].to_string();
    }

    if looks_like_dashed_ymd_hm(raw) {
        return compact_from_dashed_ymd_hm(raw);
    }

    if looks_like_ymd(raw) {
        return format!("{}{}{}-0000", &raw[0..4], &raw[5..7], &raw[8..10]);
    }

    if raw.len() == 8 && raw.bytes().all(|b| b.is_ascii_digit()) {
        return format!("{raw}-0000");
    }

    if let Ok(timestamp) = raw.parse::<i64>() {
        return unix_utc_ymd_hm(timestamp);
    }

    let sanitized = sanitize_folder_name(raw).replace(' ', "-");
    if sanitized.is_empty() {
        "unknown-date".to_string()
    } else {
        sanitized
    }
}

fn looks_like_ymd(raw: &str) -> bool {
    raw.len() >= 10
        && raw.as_bytes().get(4) == Some(&b'-')
        && raw.as_bytes().get(7) == Some(&b'-')
        && raw.as_bytes()[..10]
            .iter()
            .all(|b| b.is_ascii_digit() || *b == b'-')
}

fn looks_like_dashed_ymd_hm(raw: &str) -> bool {
    raw.len() >= 16
        && looks_like_ymd(raw)
        && raw.as_bytes().get(10) == Some(&b'-')
        && raw.as_bytes().get(13) == Some(&b'-')
        && raw.as_bytes()[11..13].iter().all(u8::is_ascii_digit)
        && raw.as_bytes()[14..16].iter().all(u8::is_ascii_digit)
}

fn looks_like_compact_ymd_hm(raw: &str) -> bool {
    raw.len() >= 13
        && raw.as_bytes()[..8].iter().all(u8::is_ascii_digit)
        && raw.as_bytes().get(8) == Some(&b'-')
        && raw.as_bytes()[9..13].iter().all(u8::is_ascii_digit)
}

fn compact_from_dashed_ymd_hm(raw: &str) -> String {
    format!(
        "{}{}{}-{}{}",
        &raw[0..4],
        &raw[5..7],
        &raw[8..10],
        &raw[11..13],
        &raw[14..16]
    )
}

fn unix_utc_ymd_hm(timestamp: i64) -> String {
    let days = timestamp.div_euclid(86_400);
    let tod = timestamp.rem_euclid(86_400) as u32;
    let hh = tod / 3600;
    let mm = (tod % 3600) / 60;
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 }.div_euclid(146_097);
    let doe = (z - era * 146_097) as u32;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}{m:02}{d:02}-{hh:02}{mm:02}")
}

#[cfg(test)]
mod folder_name_tests {
    use super::{folder_datetime_prefix, unix_utc_ymd_hm, video_folder_name};

    #[test]
    fn formats_unix_timestamp_with_utc_time() {
        // 2024-01-02 03:04:05 UTC
        assert_eq!(unix_utc_ymd_hm(1_704_164_645), "20240102-0304");
    }

    #[test]
    fn folder_prefix_from_unix_and_date_only() {
        assert_eq!(folder_datetime_prefix(Some("1704164645")), "20240102-0304");
        assert_eq!(folder_datetime_prefix(Some("2024-01-02")), "20240102-0000");
        assert_eq!(folder_datetime_prefix(Some("20240102")), "20240102-0000");
        assert_eq!(
            folder_datetime_prefix(Some("2024-01-02-15-30")),
            "20240102-1530"
        );
        assert_eq!(
            folder_datetime_prefix(Some("20240102-1530")),
            "20240102-1530"
        );
    }

    #[test]
    fn folder_name_includes_datetime() {
        let name = video_folder_name("Hello World", Some("1704164645"));
        assert_eq!(name, "20240102-0304_Hello World");
    }
}

#[derive(Clone, Debug)]
pub struct JsRuntimeInfo {
    pub installed: bool,
    pub runtime: Option<String>,
}

/// Checks if ffmpeg is available on the system.
/// It first checks for a bundled sidecar "ffmpeg" and then falls back to the system path.
pub fn is_ffmpeg_installed(app: &tauri::AppHandle) -> bool {
    // 1. Try sidecar "ffmpeg"
    if let Ok(sidecar_cmd) = app.shell().sidecar("ffmpeg") {
        let sidecar_cmd = sidecar_cmd.arg("-version");
        if sidecar_cmd.spawn().is_ok() {
            return true;
        }
    }

    // 2. Try system "ffmpeg"
    if let Ok(_system_cmd) = app.shell().command("ffmpeg").arg("-version").spawn() {
        // Just spawning is enough to know if it exists in PATH
        return true;
    }

    false
}

pub fn detect_js_runtime(app: &tauri::AppHandle) -> JsRuntimeInfo {
    if let Some((runtime, _path)) = find_js_runtime(app) {
        JsRuntimeInfo {
            installed: true,
            runtime: Some(runtime),
        }
    } else {
        JsRuntimeInfo {
            installed: false,
            runtime: None,
        }
    }
}

pub fn is_js_challenge_error(message: &str) -> bool {
    let lower = message.to_lowercase();
    lower.contains("n challenge solving failed")
        || lower.contains("javascript runtime")
        || lower.contains("challenge solver")
        || lower.contains("/wiki/ejs")
}

fn find_js_runtime(app: &tauri::AppHandle) -> Option<(String, PathBuf)> {
    if let Some(path) = find_bundled_tool(app, "deno") {
        return Some(("deno".to_string(), path));
    }

    if let Some(path) = find_executable_in_path("deno") {
        return Some(("deno".to_string(), path));
    }

    if let Some(path) = find_executable_in_path("node") {
        return Some(("node".to_string(), path));
    }

    None
}

/// Search dirs for bundled yt-dlp / Deno.
///
/// Linux `.deb` installs the binary to `/usr/bin/<bin>` and resources to
/// `/usr/lib/<productName>/` (note the product name, e.g. `Yank Trove`).
/// Tauri's `resource_dir()` often resolves `/usr/lib/<package_name>` instead,
/// so we also probe `../lib/<productName>` next to the executable.
fn tool_search_dirs(app: &tauri::AppHandle) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    let mut lib_names: Vec<String> = Vec::new();
    if let Some(product_name) = app.config().product_name.clone() {
        lib_names.push(product_name);
    }
    lib_names.push(app.package_info().name.clone());
    // Hard-coded fallback matching tauri.conf.json productName / Linux .deb layout.
    if !lib_names.iter().any(|n| n == "Yank Trove") {
        lib_names.push("Yank Trove".to_string());
    }
    if !lib_names.iter().any(|n| n == "yank-trove") {
        lib_names.push("yank-trove".to_string());
    }

    let mut push_dir = |path: PathBuf| {
        if !dirs.iter().any(|existing| existing == &path) {
            dirs.push(path);
        }
    };

    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            push_dir(parent.join("bin"));
            push_dir(parent.to_path_buf());

            for lib_name in &lib_names {
                let lib_root = parent.join("..").join("lib").join(lib_name);
                push_dir(lib_root.join("bin"));
                push_dir(lib_root.clone());
                if let Ok(canonical) = lib_root.canonicalize() {
                    push_dir(canonical.join("bin"));
                    push_dir(canonical);
                }
            }
        }
    }

    if let Ok(resource_dir) = app.path().resource_dir() {
        push_dir(resource_dir.join("bin"));
        push_dir(resource_dir.clone());
        if let Some(parent) = resource_dir.parent() {
            for lib_name in &lib_names {
                push_dir(parent.join(lib_name).join("bin"));
                push_dir(parent.join(lib_name));
            }
        }
    }

    dirs
}

fn find_bundled_tool(app: &tauri::AppHandle, name: &str) -> Option<PathBuf> {
    for dir in tool_search_dirs(app) {
        for candidate in sidecar_filename_candidates(name) {
            let path = dir.join(&candidate);
            if path.is_file() {
                return Some(path);
            }
        }
    }
    None
}

fn sidecar_filename_candidates(name: &str) -> Vec<String> {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        return vec![
            format!("{name}.exe"),
            format!("{name}-x86_64-pc-windows-msvc.exe"),
            name.to_string(),
        ];
    }

    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    {
        return vec![
            format!("{name}.exe"),
            format!("{name}-aarch64-pc-windows-msvc.exe"),
            name.to_string(),
        ];
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        return vec![
            name.to_string(),
            format!("{name}-aarch64-apple-darwin"),
        ];
    }

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    {
        return vec![
            name.to_string(),
            format!("{name}-x86_64-apple-darwin"),
        ];
    }

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        return vec![
            name.to_string(),
            format!("{name}-x86_64-unknown-linux-gnu"),
        ];
    }

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        return vec![
            name.to_string(),
            format!("{name}-aarch64-unknown-linux-gnu"),
        ];
    }

    #[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
    {
        vec![name.to_string()]
    }
}

fn find_executable_in_path(name: &str) -> Option<PathBuf> {
    #[cfg(windows)]
    {
        let output = std::process::Command::new("where.exe")
            .arg(name)
            .output()
            .ok()?;
        if !output.status.success() {
            return None;
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout
            .lines()
            .map(str::trim)
            .find(|line| !line.is_empty())
            .map(PathBuf::from)
    }

    #[cfg(not(windows))]
    {
        let output = std::process::Command::new("which")
            .arg(name)
            .output()
            .ok()?;
        if !output.status.success() {
            return None;
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout
            .lines()
            .map(str::trim)
            .find(|line| !line.is_empty())
            .map(PathBuf::from)
    }
}

fn js_runtime_args(app: &tauri::AppHandle) -> Vec<String> {
    find_js_runtime(app)
        .map(|(runtime, path)| {
            vec![
                "--js-runtimes".to_string(),
                format!("{}:{}", runtime, path.to_string_lossy()),
            ]
        })
        .unwrap_or_default()
}

fn yt_dlp_supports_js_runtimes(program: &str) -> bool {
    let output = std::process::Command::new(program)
        .arg("--help")
        .output();
    match output {
        Ok(out) => {
            let text = String::from_utf8_lossy(&out.stdout).to_ascii_lowercase()
                + &String::from_utf8_lossy(&out.stderr).to_ascii_lowercase();
            text.contains("--js-runtimes") || text.contains("js-runtimes")
        }
        Err(_) => false,
    }
}

/// Spawns the yt-dlp command.
/// Prefers the bundled tool under `bin/`, then falls back to system `yt-dlp`.
pub fn spawn_yt_dlp(
    app: &tauri::AppHandle,
    args: Vec<String>,
) -> Result<
    (
        tokio::sync::mpsc::Receiver<tauri_plugin_shell::process::CommandEvent>,
        tauri_plugin_shell::process::CommandChild,
    ),
    String,
> {
    if let Some(path) = find_bundled_tool(app, "yt-dlp") {
        let path_str = path.to_string_lossy().to_string();
        let mut full_args = js_runtime_args(app);
        full_args.extend(args.clone());
        match app.shell().command(&path_str).args(&full_args).spawn() {
            Ok(res) => return Ok(res),
            Err(e) => {
                println!(
                    "Failed to spawn bundled yt-dlp at {}: {}. Trying system fallback...",
                    path_str, e
                );
            }
        }
    }

    // System yt-dlp on older distros (e.g. apt) may not support --js-runtimes.
    let mut full_args = Vec::new();
    if yt_dlp_supports_js_runtimes("yt-dlp") {
        full_args.extend(js_runtime_args(app));
    }
    full_args.extend(args);

    let system_cmd = app.shell().command("yt-dlp").args(full_args);
    system_cmd.spawn().map_err(|e| {
        format!(
            "Failed to spawn yt-dlp. Make sure yt-dlp is installed and in your system PATH.\nError: {}",
            e
        )
    })
}

/// True when the video folder already has finished outputs (not .part / .ytdl / .temp).
pub fn video_dir_has_existing_outputs(dir: &Path) -> bool {
    let Ok(entries) = fs::read_dir(dir) else {
        return false;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let lower = name.to_ascii_lowercase();
        if lower.ends_with(".part")
            || lower.ends_with(".ytdl")
            || lower.ends_with(".temp")
            || lower.ends_with(".tmp")
        {
            continue;
        }
        return true;
    }
    false
}

/// Cleans up any incomplete download files matching the video_id in the target directory.
/// Searches for files ending with .part, .temp, or containing the video_id that are incomplete.
pub fn cleanup_incomplete_files(dir: &Path, video_id: &str) -> std::io::Result<()> {
    if !dir.exists() || !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                // If it contains the video_id and is a temporary/part file, delete it
                let is_part = filename.ends_with(".part")
                    || filename.ends_with(".temp")
                    || filename.ends_with(".ytdl");
                let contains_id = filename.contains(video_id);
                if contains_id && is_part {
                    if let Err(e) = fs::remove_file(&path) {
                        eprintln!("Failed to clean up incomplete file {:?}: {}", path, e);
                    }
                }
            }
        }
    }
    Ok(())
}

pub struct CsvVideoRow {
    pub id: String,
    pub title: String,
    pub url: String,
    pub uploaded_at: String,
    pub duration: String,
    pub availability: String,
    pub save_path: String,
}

pub fn format_duration_seconds(duration: Option<f64>) -> String {
    let Some(total) = duration.filter(|value| *value >= 0.0) else {
        return String::new();
    };
    let total = total.round() as u64;
    let hours = total / 3600;
    let minutes = (total % 3600) / 60;
    let seconds = total % 60;
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

fn csv_field(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

/// Writes a UTF-8 (BOM) CSV summary for Excel-friendly reading.
pub fn write_channel_summary_csv(
    path: &Path,
    rows: &[CsvVideoRow],
    locale: crate::i18n::UiLocale,
) -> std::io::Result<()> {
    let header = crate::i18n::csv_header(locale);
    let mut body = String::from("\u{FEFF}");
    body.push_str(&header.map(csv_field).join(","));
    body.push_str("\r\n");
    for row in rows {
        let line = [
            csv_field(&row.id),
            csv_field(&row.title),
            csv_field(&row.url),
            csv_field(&row.uploaded_at),
            csv_field(&row.duration),
            csv_field(&row.availability),
            csv_field(&row.save_path),
        ]
        .join(",");
        body.push_str(&line);
        body.push_str("\r\n");
    }
    fs::write(path, body)
}
