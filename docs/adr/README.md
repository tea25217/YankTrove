# Architecture Decision Records

Yank Trove で後から覆すとコストが大きい判断を、番号付きの ADR として残す。

- 形式は [Nygard の ADR](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions) に近い。
- 1 判断 1 ファイル。本文は日本語。
- `Status` は `Accepted` / `Superseded by ADR-NNNN` / `Deprecated`。
- 新しい判断は次の空き番号で追加し、この一覧を更新する。
- アプリの挙動そのものは README を正とする。ADR は「なぜそうなっているか」を残す。

| 番号 | タイトル | 状態 |
| --- | --- | --- |
| [0001](./0001-yt-dlp-not-official-api.md) | 取得経路は YouTube 公式 API ではなく yt-dlp | Accepted |
| [0002](./0002-zero-based-versioning.md) | 当面 0.x。破壊的変更もマイナー | Accepted |
| [0003](./0003-bundle-identifier.md) | 配布 ID は `com.yanktrove.desktop` | Accepted |
| [0004](./0004-cookie-source.md) | クッキーは Firefox 推奨。Chrome は非推奨 | Accepted |
| [0005](./0005-supported-platforms.md) | Windows を本線。macOS は未確認。Linux は対象外 | Accepted |
| [0006](./0006-bundled-runtimes.md) | yt-dlp と Deno を同梱。FFmpeg は同梱しない | Accepted |
| [0007](./0007-video-folder-naming.md) | 動画フォルダは UTC `{YYYYMMDD-hhmm}_{title}`。日付は個別メタから取る | Accepted |
| [0008](./0008-live-chat-as-subtitles.md) | チャットログは yt-dlp の `live_chat` 字幕として取る | Accepted |
| [0009](./0009-os-locale-i18n.md) | 表示言語は OS ロケール。アプリ内切替はしない | Superseded by 0012 |
| [0010](./0010-secret-scanning.md) | 秘密情報スキャンは Betterleaks | Accepted |
| [0011](./0011-generated-release-notes.md) | GitHub Release 本文はコミットから自動生成 | Accepted |
| [0012](./0012-in-app-language.md) | 表示言語は OS 初期値。以降はアプリ内で選び永続化する | Accepted |
