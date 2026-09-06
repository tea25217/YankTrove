#!/usr/bin/env bash
# Download yt-dlp and Deno into src-tauri/binaries/ (triple names for reference)
# and src-tauri/resources/bin/ (install layout: bin/ next to the app).
set -euo pipefail

TARGET="${1:?Usage: download-sidecars.sh <rust-target-triple>}"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BIN_DIR="$ROOT/src-tauri/binaries"
RES_BIN="$ROOT/src-tauri/resources/bin"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

mkdir -p "$BIN_DIR" "$RES_BIN"

extract_zip() {
  python - "$1" "$2" <<'PY'
import sys, zipfile
zipfile.ZipFile(sys.argv[1]).extractall(sys.argv[2])
PY
}

clear_cross_platform_res_bin() {
  # Avoid shipping the wrong OS binaries when the workspace is reused across jobs.
  rm -f "$RES_BIN/yt-dlp.exe" "$RES_BIN/deno.exe" "$RES_BIN/yt-dlp" "$RES_BIN/deno"
}

case "$TARGET" in
  x86_64-pc-windows-msvc)
    clear_cross_platform_res_bin
    curl -fsSL -o "$BIN_DIR/yt-dlp-${TARGET}.exe" \
      "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe"
    curl -fsSL -o "$TMP_DIR/deno.zip" \
      "https://github.com/denoland/deno/releases/latest/download/deno-x86_64-pc-windows-msvc.zip"
    extract_zip "$TMP_DIR/deno.zip" "$TMP_DIR/deno"
    mv "$TMP_DIR/deno/deno.exe" "$BIN_DIR/deno-${TARGET}.exe"
    cp "$BIN_DIR/yt-dlp-${TARGET}.exe" "$RES_BIN/yt-dlp.exe"
    cp "$BIN_DIR/deno-${TARGET}.exe" "$RES_BIN/deno.exe"
    ;;
  aarch64-apple-darwin|x86_64-apple-darwin)
    clear_cross_platform_res_bin
    # yt-dlp_macos is a universal binary; copy under the target-triple name for local reference.
    curl -fsSL -o "$BIN_DIR/yt-dlp-${TARGET}" \
      "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos"
    chmod +x "$BIN_DIR/yt-dlp-${TARGET}"
    curl -fsSL -o "$TMP_DIR/deno.zip" \
      "https://github.com/denoland/deno/releases/latest/download/deno-${TARGET}.zip"
    extract_zip "$TMP_DIR/deno.zip" "$TMP_DIR/deno"
    mv "$TMP_DIR/deno/deno" "$BIN_DIR/deno-${TARGET}"
    chmod +x "$BIN_DIR/deno-${TARGET}"
    cp "$BIN_DIR/yt-dlp-${TARGET}" "$RES_BIN/yt-dlp"
    cp "$BIN_DIR/deno-${TARGET}" "$RES_BIN/deno"
    chmod +x "$RES_BIN/yt-dlp" "$RES_BIN/deno"
    ;;
  x86_64-unknown-linux-gnu)
    clear_cross_platform_res_bin
    curl -fsSL -o "$BIN_DIR/yt-dlp-${TARGET}" \
      "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux"
    chmod +x "$BIN_DIR/yt-dlp-${TARGET}"
    curl -fsSL -o "$TMP_DIR/deno.zip" \
      "https://github.com/denoland/deno/releases/latest/download/deno-x86_64-unknown-linux-gnu.zip"
    extract_zip "$TMP_DIR/deno.zip" "$TMP_DIR/deno"
    mv "$TMP_DIR/deno/deno" "$BIN_DIR/deno-${TARGET}"
    chmod +x "$BIN_DIR/deno-${TARGET}"
    cp "$BIN_DIR/yt-dlp-${TARGET}" "$RES_BIN/yt-dlp"
    cp "$BIN_DIR/deno-${TARGET}" "$RES_BIN/deno"
    chmod +x "$RES_BIN/yt-dlp" "$RES_BIN/deno"
    ;;
  *)
    echo "Unsupported target for sidecars: $TARGET" >&2
    exit 1
    ;;
esac

echo "Sidecars ready for $TARGET:"
ls -la "$BIN_DIR"
echo "Install layout copies:"
ls -la "$RES_BIN"
