# ADR 0004: クッキーは Firefox 推奨。Chrome は非推奨

- Status: Accepted
- Date: 2026-08-14

## Context

メンバー限定コンテンツは、ログイン済みブラウザのクッキーを yt-dlp の `--cookies-from-browser` で読む。Windows の Chrome / Edge は DPAPI および App-Bound Encryption のため、外部プロセスが Cookie DB を読めないことが多い（[yt-dlp#10927](https://github.com/yt-dlp/yt-dlp/issues/10927)）。ブラウザを終了しても直らない。

Netscape 形式の `cookies.txt` 手動投入は、取り扱いリスクが大きく、当面スコープ外とした。

## Decision

- 推奨ソースは **Firefox**。
- UI 上 Chrome は「非推奨」。失敗時の案内も Firefox へ誘導する。
- Edge / Safari は選択肢として残すが、Windows では Edge も Chrome と同様に失敗しやすい。
- `cookies.txt` は実装しない。

## Consequences

- 「Chrome を閉じれば直る」は誤り。ドキュメントとエラー文面で DPAPI / 暗号化を明示する。
- メン限取得の実働確認は Firefox ログインが前提になる。
