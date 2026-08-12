# 同梱バイナリ（Git 管理外）

`yt-dlp` と Deno の実行ファイルはサイズが大きいため、リポジトリには含めません。  
開発・ビルド時は、このディレクトリに配置してください。

CI（`.github/workflows/release.yml`）では `.github/scripts/download-sidecars.sh` が自動取得します。

## 必要なファイル

Tauri の `externalBin` は、ターゲット三重項付きのファイル名を参照します。

| ファイル名 | 用途 |
|-----------|------|
| `yt-dlp-x86_64-pc-windows-msvc.exe` | Windows x64 用 yt-dlp |
| `deno-x86_64-pc-windows-msvc.exe` | Windows x64 用 Deno |
| `yt-dlp-x86_64-apple-darwin` | macOS Intel 用 yt-dlp |
| `deno-x86_64-apple-darwin` | macOS Intel 用 Deno |
| `yt-dlp-aarch64-apple-darwin` | macOS Apple Silicon 用 yt-dlp |
| `deno-aarch64-apple-darwin` | macOS Apple Silicon 用 Deno |

## 入手先

- yt-dlp: https://github.com/yt-dlp/yt-dlp/releases
- Deno: https://github.com/denoland/deno/releases

配置後、ファイル名を上表どおりにリネームしてください。

ローカルで CI と同じ取得をする場合:

```bash
bash .github/scripts/download-sidecars.sh x86_64-pc-windows-msvc
# macOS Apple Silicon:
# bash .github/scripts/download-sidecars.sh aarch64-apple-darwin
```
