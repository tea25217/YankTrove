# ADR 0008: チャットログは yt-dlp の `live_chat` 字幕として取る

- Status: Accepted
- Date: 2026-08-14

## Context

UI に「チャットログ (JSON形式)」がある。yt-dlp に `--write-chat` は存在せず、渡すと `no such option: --write-chat` で失敗する。ライブチャットは字幕トラック `live_chat` として扱われる。

## Decision

チャット ON のときは `--write-subs --sub-langs live_chat` を使う。字幕と併用するときは `live_chat` を `--sub-langs` に含め、`--sub-format vtt/json3/best` にして JSON も落ちるようにする。

## Consequences

- チャットは `*.live_chat.json` として保存される想定。
- 進行中ライブのチャット取得は yt-dlp 側の制約（配信終了まで待つ等）を引き継ぐ。
- `--write-chat` を復活させない。
