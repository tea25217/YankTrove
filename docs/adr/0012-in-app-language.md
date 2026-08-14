# ADR 0012: 表示言語は OS 初期値。以降はアプリ内で選び永続化する

- Status: Accepted
- Date: 2026-08-14
- Supersedes: [ADR 0009](./0009-os-locale-i18n.md)

## Context

#6 で UI の日英対応は入れたが、言語は OS ロケールのみだった。英語 OS で日本語にしたい（逆も）ため、アプリ内切替が追加要望になった。

## Decision

- **未設定時の初期値**は従来どおり OS 依存（`ja*` なら日本語、それ以外は英語）。
- ヘッダーで日本語 / English を選べる。
- 選択は `localStorage` キー `yank-trove.locale`（`ja` | `en`）に保存し、次回起動でも使う。
- 保存値があるときは OS ロケールを上書きする。未保存に戻す UI は持たない。
- バックエンドへ渡す `locale` もこの選択に従う（エラー文・CSV 見出し）。

## Consequences

- README の「アプリ内切替なし」は古い。
- 処理ログの既存行は書き込んだ時点の言語のまま。切替後の新規ログだけ新言語。
- WebView の localStorage が消えると OS ロケールに戻る。
