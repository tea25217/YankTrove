use std::path::PathBuf;
use std::sync::atomic::Ordering;
use tauri::{Emitter, Manager};

mod utils;
mod command_runner;
mod i18n;
mod text_decode;

use crate::utils::{
    is_ffmpeg_installed, detect_js_runtime, cleanup_incomplete_files, sanitize_folder_name,
    video_folder_name, video_dir_has_existing_outputs, CsvVideoRow, format_duration_seconds,
    write_channel_summary_csv,
};
use crate::i18n::UiLocale;
use crate::command_runner::{
    AppState, DownloadOptions, OverwriteAction, VideoDownloadTarget, ChannelInfo, ProgressPayload,
    fetch_channel_videos, download_single_video, probe_video,
};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

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
    let channel_dir = if options.create_yanktrove_folder {
        base_dir.join("YankTrove").join(&sanitized_title)
    } else {
        base_dir.join(&sanitized_title)
    };

    // Ensure download folder exists
    std::fs::create_dir_all(&channel_dir)
        .map_err(|e| format!("Failed to create directory {:?}: {}", channel_dir, e))?;

    let total_count = videos.len();
    let mut csv_rows: Vec<CsvVideoRow> = Vec::new();

    for (index, video) in videos.iter().enumerate() {
        if state.is_cancelled.load(Ordering::SeqCst) {
            return Err("Cancelled".to_string());
        }

        // Flat-playlist list often lacks upload_date; always probe full metadata for the folder
        // name (even when the UI "metadata" checkbox is off — that only writes info.json).
        let list_date = video
            .uploaded_at
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string);
        let probe = probe_video(&app, &video.url, &options.cookies_browser).await;
        let uploaded_at = probe
            .as_ref()
            .and_then(|p| p.uploaded_at.clone())
            .or(list_date);
        let prefetched_language = probe.and_then(|p| p.language);

        let video_dir = channel_dir.join(video_folder_name(
            &video.title,
            uploaded_at.as_deref(),
        ));
        std::fs::create_dir_all(&video_dir)
            .map_err(|e| format!("Failed to create directory {:?}: {}", video_dir, e))?;

        if options.csv {
            csv_rows.push(CsvVideoRow {
                id: video.id.clone(),
                title: video.title.clone(),
                url: video.url.clone(),
                uploaded_at: uploaded_at.clone().unwrap_or_default(),
                duration: format_duration_seconds(video.duration),
                availability: video
                    .availability
                    .as_deref()
                    .map(|value| crate::i18n::csv_availability_label(value, ui_locale))
                    .unwrap_or_default(),
                save_path: video_dir.to_string_lossy().into_owned(),
            });
        }

        let has_existing = video_dir_has_existing_outputs(&video_dir);
        let action = if !has_existing {
            OverwriteAction::Overwrite
        } else {
            match options.overwrite_mode.as_str() {
                "overwrite" => OverwriteAction::Overwrite,
                "ask" => {
                    let app_for_dialog = app.clone();
                    let title_for_dialog = video.title.clone();
                    let decided = tokio::task::spawn_blocking(move || {
                        app_for_dialog
                            .dialog()
                            .message(crate::i18n::overwrite_prompt_message(
                                &title_for_dialog,
                                ui_locale,
                            ))
                            .title(crate::i18n::overwrite_prompt_title(ui_locale))
                            .buttons(MessageDialogButtons::YesNo)
                            .blocking_show()
                    })
                    .await
                    .map_err(|e| format!("Overwrite prompt failed: {e}"))?;

                    if decided {
                        OverwriteAction::Overwrite
                    } else {
                        OverwriteAction::Skip
                    }
                }
                _ => OverwriteAction::Skip, // "skip" and unknown
            }
        };

        // Notify UI that video processing has started
        let _ = app.emit("video-started", video.id.clone());

        if action == OverwriteAction::Skip {
            let _ = app.emit(
                "download-log",
                ProgressPayload {
                    video_id: video.id.clone(),
                    percentage: 0.0,
                    speed: None,
                    eta: None,
                    status: "Skipped".to_string(),
                    log: Some(crate::i18n::existing_files_skip_log(&video.title, ui_locale)),
                },
            );
            let _ = app.emit("video-finished", video.id.clone());
        } else {
            let force_overwrite =
                has_existing || options.overwrite_mode.as_str() == "overwrite";
            match download_single_video(
                &app,
                &state,
                &video.id,
                &video.url,
                &options,
                &video_dir,
                ui_locale,
                prefetched_language,
                force_overwrite,
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

#[tauri::command]
fn open_save_folder(
    app: tauri::AppHandle,
    custom_dir: Option<String>,
    create_yanktrove_folder: Option<bool>,
) -> Result<(), String> {
    let create_yanktrove = create_yanktrove_folder.unwrap_or(true);
    let dir = match custom_dir.filter(|path| !path.trim().is_empty()) {
        Some(path) => PathBuf::from(path),
        None => {
            let downloads = app.path().download_dir().map_err(|e| e.to_string())?;
            if create_yanktrove {
                downloads.join("YankTrove")
            } else {
                downloads
            }
        }
    };

    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Failed to create directory {:?}: {}", dir, e))?;

    open_path_in_file_manager(&dir)
}

fn open_path_in_file_manager(dir: &std::path::Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    let mut command = {
        let mut command = std::process::Command::new("explorer");
        command.arg(dir);
        command
    };
    #[cfg(target_os = "macos")]
    let mut command = {
        let mut command = std::process::Command::new("open");
        command.arg(dir);
        command
    };
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let mut command = {
        let mut command = std::process::Command::new("xdg-open");
        command.arg(dir);
        command
    };

    command.spawn().map(|_| ()).map_err(|e| e.to_string())
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
            cancel_downloads,
            open_save_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
