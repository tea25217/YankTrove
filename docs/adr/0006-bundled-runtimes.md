# ADR 0006: yt-dlp と Deno を同梱。FFmpeg は同梱しない

- Status: Accepted
- Date: 2026-08-14

## Context

yt-dlp は YouTube 取得に必須。近年は JS ランタイム（Deno）も必須。FFmpeg は動画・音声の mux / 変換にだけ必要で、サイズが大きい。GitHub のファイルサイズ制限のため、sidecar は Git に置かず CI で取得する。

## Decision

- インストーラーに **yt-dlp** と **Deno** を sidecar として同梱する。
- **FFmpeg は同梱しない**。動画・音声保存時のみ、利用者が PATH または別途配置した FFmpeg を使う。
- 開発時の配置手順は `src-tauri/binaries/README.md`。CI は `.github/scripts/download-sidecars.sh`。

## Consequences

- 字幕・メタデータ・チャット・サムネイルのみなら FFmpeg なしで動く。
- 動画/音声保存で「FFmpeg: 未検出」になるのは仕様。
- sidecar の更新はリリースパイプライン側の作業になる。
