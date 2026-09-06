# ADR 0005: Windows を本線。macOS / Linux は CI 配布（実機確認は限定的）

- Status: Accepted
- Date: 2026-08-14
- Updated: 2026-09-06

## Context

開発環境は Windows。macOS 用の Tauri ビルドは CI で出せるが、実機確認できない。Linux（`.deb`）は Epic #12 で x86_64 配布を始めた。

## Decision

- **Windows**: 動作確認済みの本線。NSIS / MSI を配布する。
- **macOS**: Apple Silicon / Intel の `.dmg` を CI で出す。動作未確認と明記する。
- **Linux**: x86_64 の `.deb` を CI で出す（Ubuntu 22.04 ビルド）。AppImage / aarch64 は別途。開発者が Linux 実機を常時持てるわけではないため、継続的な動作確認は限定的と明記する。

## Consequences

- リリース workflow は Windows + macOS + Linux（`.deb`）を含む。
- README / Release 本文に macOS・Linux の確認範囲を書く。
- FFmpeg は同梱せず Depends にも入れない（ADR 0006）。Linux 利用者向けは README で `apt install ffmpeg` 等を案内する。
