# 同梱バイナリ（Git 管理外）

`yt-dlp` と Deno の実行ファイルはサイズが大きいため、リポジトリには含めません。  
開発・ビルド時は、このディレクトリに配置してください。

CI（`.github/workflows/release.yml`）では `.github/scripts/download-sidecars.sh` が自動取得します。

## インストール時の配置

インストーラーでは次のレイアウトになります（アプリ本体と同じ階層にユーザー向け文書、ツールは `bin/`）。

```
Yank Trove(.exe)
README.txt
LICENSE
bin/
  yt-dlp(.exe)
  deno(.exe)
licenses/
  THIRD_PARTY_LICENSES.md
```

ビルド前に `download-sidecars.sh`（または `build.rs`）が `src-tauri/resources/bin/` へ短い名前でコピーし、`tauri.conf.json` の `bundle.resources` が `bin/` にマップします。

## 開発用ファイル（三重項付き）

ローカル参照・`build.rs` のコピー元として、三重項付きの名前も `binaries/` に置きます。

| ファイル名 | 用途 |
|-----------|------|
| `yt-dlp-x86_64-pc-windows-msvc.exe` | Windows x64 用 yt-dlp |
| `deno-x86_64-pc-windows-msvc.exe` | Windows x64 用 Deno |
| `yt-dlp-x86_64-apple-darwin` | macOS Intel 用 yt-dlp |
| `deno-x86_64-apple-darwin` | macOS Intel 用 Deno |
| `yt-dlp-aarch64-apple-darwin` | macOS Apple Silicon 用 yt-dlp |
| `deno-aarch64-apple-darwin` | macOS Apple Silicon 用 Deno |
| `yt-dlp-x86_64-unknown-linux-gnu` | Linux x86_64 用 yt-dlp |
| `deno-x86_64-unknown-linux-gnu` | Linux x86_64 用 Deno |

## 入手先

- yt-dlp: https://github.com/yt-dlp/yt-dlp/releases
- Deno: https://github.com/denoland/deno/releases

ローカルで CI と同じ取得をする場合:

```bash
bash .github/scripts/download-sidecars.sh x86_64-pc-windows-msvc
# macOS Apple Silicon:
# bash .github/scripts/download-sidecars.sh aarch64-apple-darwin
# Linux x86_64:
# bash .github/scripts/download-sidecars.sh x86_64-unknown-linux-gnu
```
