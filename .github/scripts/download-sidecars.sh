#!/usr/bin/env bash
# Download yt-dlp and Deno into src-tauri/binaries/ for the given Rust target triple.
set -euo pipefail

TARGET="${1:?Usage: download-sidecars.sh <rust-target-triple>}"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BIN_DIR="$ROOT/src-tauri/binaries"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

mkdir -p "$BIN_DIR"

extract_zip() {
  python - "$1" "$2" <<'PY'
import sys, zipfile
zipfile.ZipFile(sys.argv[1]).extractall(sys.argv[2])
PY
}

case "$TARGET" in
  x86_64-pc-windows-msvc)
    curl -fsSL -o "$BIN_DIR/yt-dlp-${TARGET}.exe" \
      "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe"
    curl -fsSL -o "$TMP_DIR/deno.zip" \
      "https://github.com/denoland/deno/releases/latest/download/deno-x86_64-pc-windows-msvc.zip"
    extract_zip "$TMP_DIR/deno.zip" "$TMP_DIR/deno"
    mv "$TMP_DIR/deno/deno.exe" "$BIN_DIR/deno-${TARGET}.exe"
    ;;
  aarch64-apple-darwin|x86_64-apple-darwin)
    # yt-dlp_macos is a universal binary; copy under the target-triple name Tauri expects.
    curl -fsSL -o "$BIN_DIR/yt-dlp-${TARGET}" \
      "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos"
    chmod +x "$BIN_DIR/yt-dlp-${TARGET}"
    curl -fsSL -o "$TMP_DIR/deno.zip" \
      "https://github.com/denoland/deno/releases/latest/download/deno-${TARGET}.zip"
    extract_zip "$TMP_DIR/deno.zip" "$TMP_DIR/deno"
    mv "$TMP_DIR/deno/deno" "$BIN_DIR/deno-${TARGET}"
    chmod +x "$BIN_DIR/deno-${TARGET}"
    ;;
  *)
    echo "Unsupported target for sidecars: $TARGET" >&2
    exit 1
    ;;
esac

echo "Sidecars ready for $TARGET:"
ls -la "$BIN_DIR"
