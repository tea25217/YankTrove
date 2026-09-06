fn main() {
    tauri_build::build();
    sync_resources_bin_from_binaries();
}

/// Copies platform Deno/yt-dlp from `binaries/` into `resources/bin/` so local
/// builds match the install layout (`bin/` next to the app) without requiring a
/// full CI download when triple-named files are already present.
fn sync_resources_bin_from_binaries() {
    let Some((yt_src, deno_src, yt_dest_name, deno_dest_name)) = platform_sidecar_names() else {
        return;
    };

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let binaries = std::path::Path::new(&manifest_dir).join("binaries");
    let res_bin = std::path::Path::new(&manifest_dir).join("resources").join("bin");
    if std::fs::create_dir_all(&res_bin).is_err() {
        return;
    }

    let yt_src_path = binaries.join(yt_src);
    let deno_src_path = binaries.join(deno_src);
    if yt_src_path.exists() {
        let dest = res_bin.join(yt_dest_name);
        if let Err(error) = std::fs::copy(&yt_src_path, &dest) {
            println!("cargo:warning=Failed to copy yt-dlp into resources/bin: {error}");
        } else {
            #[cfg(unix)]
            set_executable(&dest);
        }
    }
    if deno_src_path.exists() {
        let dest = res_bin.join(deno_dest_name);
        if let Err(error) = std::fs::copy(&deno_src_path, &dest) {
            println!("cargo:warning=Failed to copy Deno into resources/bin: {error}");
        } else {
            #[cfg(unix)]
            set_executable(&dest);
        }
    }
}

#[cfg(unix)]
fn set_executable(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;
    if let Ok(meta) = std::fs::metadata(path) {
        let mut perms = meta.permissions();
        perms.set_mode(perms.mode() | 0o111);
        let _ = std::fs::set_permissions(path, perms);
    }
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
fn platform_sidecar_names() -> Option<(&'static str, &'static str, &'static str, &'static str)> {
    Some((
        "yt-dlp-x86_64-pc-windows-msvc.exe",
        "deno-x86_64-pc-windows-msvc.exe",
        "yt-dlp.exe",
        "deno.exe",
    ))
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn platform_sidecar_names() -> Option<(&'static str, &'static str, &'static str, &'static str)> {
    Some((
        "yt-dlp-aarch64-apple-darwin",
        "deno-aarch64-apple-darwin",
        "yt-dlp",
        "deno",
    ))
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
fn platform_sidecar_names() -> Option<(&'static str, &'static str, &'static str, &'static str)> {
    Some((
        "yt-dlp-x86_64-apple-darwin",
        "deno-x86_64-apple-darwin",
        "yt-dlp",
        "deno",
    ))
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn platform_sidecar_names() -> Option<(&'static str, &'static str, &'static str, &'static str)> {
    Some((
        "yt-dlp-x86_64-unknown-linux-gnu",
        "deno-x86_64-unknown-linux-gnu",
        "yt-dlp",
        "deno",
    ))
}

#[cfg(not(any(
    all(target_os = "windows", target_arch = "x86_64"),
    all(target_os = "macos", target_arch = "aarch64"),
    all(target_os = "macos", target_arch = "x86_64"),
    all(target_os = "linux", target_arch = "x86_64"),
)))]
fn platform_sidecar_names() -> Option<(&'static str, &'static str, &'static str, &'static str)> {
    None
}
