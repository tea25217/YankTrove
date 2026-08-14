# ADR 0001: 取得経路は YouTube 公式 API ではなく yt-dlp

- Status: Accepted
- Date: 2026-08-14

## Context

チャンネル / プレイリストの一覧、メンバー限定を含むアーカイブ、字幕・チャット・メタデータをまとめて保存したい。YouTube Data API はクォータ、OAuth、取得できる項目の制約がある。

## Decision

公式 API は使わない。公開ページとストリーム情報は [yt-dlp](https://github.com/yt-dlp/yt-dlp) 経由で取る。ログインが必要なものはブラウザクッキー（ADR 0004）で、ログイン中アカウントとしてアクセスする。

利用は自己責任とする（規約・法令・レート制限・アカウント停止を含む）。取得してよいかの判断も利用者側。

## Consequences

- YouTube 側の変更で yt-dlp が壊れることがある。配布物に yt-dlp を同梱し、更新経路を確保する必要がある（ADR 0006）。
- JS チャレンジ対策として Deno が必要になる。
- 公式 API 向けのクライアント ID やクォータ管理は持たない。
