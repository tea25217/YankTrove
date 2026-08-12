fn main() {
    tauri_build::build();
    copy_deno_runtime_alias();
}

/// Copies the platform Deno sidecar binary to `deno.exe` beside the build output
/// so yt-dlp can auto-detect it when `--js-runtimes` path resolution is unavailable.
fn copy_deno_runtime_alias() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let target_dir = std::path::Path::new(&manifest_dir)
        .join("target")
        .join(&profile);

    let Some(src_name) = deno_sidecar_filename() else {
        return;
    };

    let src = std::path::Path::new(&manifest_dir)
        .join("binaries")
        .join(src_name);
    if !src.exists() {
        return;
    }

    if std::fs::create_dir_all(&target_dir).is_err() {
        return;
    }

    let dest = target_dir.join("deno.exe");
    if let Err(error) = std::fs::copy(&src, &dest) {
        println!("cargo:warning=Failed to copy Deno runtime alias to target dir: {error}");
    }
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
fn deno_sidecar_filename() -> Option<&'static str> {
    Some("deno-x86_64-pc-windows-msvc.exe")
}

#[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]
fn deno_sidecar_filename() -> Option<&'static str> {
    None
}
