use std::path::{Path, PathBuf};
use std::fs;
use tauri_plugin_shell::ShellExt;

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

pub fn js_runtime_setup_message() -> String {
    "YouTube 取得には JavaScript ランタイム（Deno 推奨）が必要です。\n\n\
    以下のいずれかで Deno をインストールしてください:\n\
    1. winget install DenoLand.Deno\n\
    2. https://docs.deno.com/runtime/getting_started/installation/\n\n\
    インストール後、アプリを再起動してください。\n\
    参考: https://github.com/yt-dlp/yt-dlp/wiki/EJS"
        .to_string()
}

pub fn is_js_challenge_error(message: &str) -> bool {
    let lower = message.to_lowercase();
    lower.contains("n challenge solving failed")
        || lower.contains("javascript runtime")
        || lower.contains("challenge solver")
        || lower.contains("/wiki/ejs")
}

fn find_js_runtime(app: &tauri::AppHandle) -> Option<(String, PathBuf)> {
    if let Some(path) = find_sidecar_executable(app, "deno") {
        return Some(("deno".to_string(), path));
    }

    if let Some(path) = find_sidecar_sibling(app, "yt-dlp", "deno.exe") {
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

fn find_sidecar_executable(app: &tauri::AppHandle, name: &str) -> Option<PathBuf> {
    if app.shell().sidecar(name).is_err() {
        return None;
    }
    find_runtime_candidate_next_to_app(name)
}

fn find_runtime_candidate_next_to_app(name: &str) -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?;

    for candidate in sidecar_filename_candidates(name) {
        let path = dir.join(candidate);
        if path.exists() {
            return Some(path);
        }
    }

    None
}

fn sidecar_filename_candidates(name: &str) -> Vec<String> {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        return vec![
            format!("{name}-x86_64-pc-windows-msvc.exe"),
            format!("{name}.exe"),
            name.to_string(),
        ];
    }

    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    {
        return vec![
            format!("{name}-aarch64-pc-windows-msvc.exe"),
            format!("{name}.exe"),
            name.to_string(),
        ];
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        return vec![
            format!("{name}-aarch64-apple-darwin"),
            name.to_string(),
        ];
    }

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    {
        return vec![
            format!("{name}-x86_64-apple-darwin"),
            name.to_string(),
        ];
    }

    #[cfg(not(any(windows, target_os = "macos")))]
    {
        vec![name.to_string()]
    }
}

fn find_sidecar_sibling(
    app: &tauri::AppHandle,
    sidecar_name: &str,
    sibling_name: &str,
) -> Option<PathBuf> {
    let sidecar_path = find_sidecar_executable(app, sidecar_name)?;
    let sibling = sidecar_path.parent()?.join(sibling_name);
    if sibling.exists() {
        Some(sibling)
    } else {
        None
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

/// Spawns the yt-dlp command.
/// Attempts to use the bundled sidecar first, and falls back to the system 'yt-dlp'.
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
    let mut full_args = js_runtime_args(app);
    full_args.extend(args);

    // 1. Try sidecar
    if let Ok(sidecar_cmd) = app.shell().sidecar("yt-dlp") {
        let sidecar_cmd = sidecar_cmd.args(&full_args);
        match sidecar_cmd.spawn() {
            Ok(res) => return Ok(res),
            Err(e) => {
                println!("Failed to spawn yt-dlp sidecar: {}. Trying system fallback...", e);
            }
        }
    }

    // 2. Try system fallback
    let system_cmd = app.shell().command("yt-dlp").args(full_args);
    system_cmd
        .spawn()
        .map_err(|e| {
            format!(
                "Failed to spawn yt-dlp. Make sure yt-dlp is installed and in your system PATH.\nError: {}",
                e
            )
        })
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
