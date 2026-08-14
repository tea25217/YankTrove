# ADR 0007: 動画フォルダは UTC 日時プレフィックス。日付は個別メタから取る

- Status: Accepted
- Date: 2026-08-14

## Context

初期は `{title} [{id}]`。一覧性と日付順のため `{date}_{title}` に変えた。リスト取得は yt-dlp `--flat-playlist` で、ここには `upload_date` が付かないことが多い。日付だけだと同日複数投稿が並ばない。

UI の「メタデータ」チェックは info.json をディスクに書くかどうかであり、フォルダ名用の日時取得とは別である。

## Decision

- フォルダ名は `{YYYYMMDD-hhmm}_{title}`（**UTC**）。取れないときは `unknown-date_{title}`。
- ダウンロード直前に各動画へ `yt-dlp -J --skip-download` でメタを取る（probe）。UI のメタデータチェックが OFF でも行う。
- probe では `timestamp` / `release_timestamp` を優先。日付だけなら時刻は `0000`。
- 字幕言語用のメタ取得も、可能な範囲でこの probe と共有する。

## Follow-up

パターンを設定から選べるようにする件は未着手（#19）。

## Consequences

- 保存レイアウトは `{title} [{id}]` および日付のみプレフィックスと互換がない。
- 動画あたり probe が 1 回増える（レート制限・待ち時間の対象）。
- フォルダ内のファイル名は従来どおり yt-dlp の `{title} [{id}].ext`。
