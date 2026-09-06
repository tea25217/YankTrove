# ADR 0006: yt-dlp と Deno を同梱。FFmpeg は同梱しない

- Status: Accepted
- Date: 2026-08-14
- Updated: 2026-09-05

## Context

yt-dlp は YouTube 取得に必須。近年は JS ランタイム（Deno）も必須。FFmpeg は動画・音声の mux / 変換にだけ必要で、サイズが大きい。GitHub のファイルサイズ制限のため、sidecar は Git に置かず CI で取得する。

## Decision

- インストーラーに **yt-dlp** と **Deno** を同梱する。配置はアプリ本体直下ではなく **`bin/`**（`bundle.resources`）。第三者ライセンスは **`licenses/`**。Windows / macOS / Linux（`.deb`）で同じ方針。
- **FFmpeg は同梱しない**（`.deb` の Depends にも入れない）。動画・音声保存時のみ、利用者が PATH または別途配置した FFmpeg を使う。
- 開発時の配置手順は `src-tauri/binaries/README.md`。CI は `.github/scripts/download-sidecars.sh`（`resources/bin/` へもコピー。Linux x86_64 含む）。

## Consequences

- 字幕・メタデータ・チャット・サムネイルのみなら FFmpeg なしで動く。
- 動画/音声保存で「FFmpeg: 未検出」になるのは仕様。
- 同梱ランタイムの更新はリリースパイプライン側の作業になる。
- 実行時は `bin/` を優先し、無い場合はシステム PATH の `yt-dlp` / `deno` にフォールバックする。
