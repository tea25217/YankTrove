# ADR 0011: GitHub Release 本文はコミットから自動生成

- Status: Accepted
- Date: 2026-08-14

## Context

Release workflow の `releaseBody` がインストール手順の固定文だけだと、そのタグで何が変わったか分からない。手書き CHANGELOG は運用コストが高い。

## Decision

`tauri-apps/tauri-action` の `generateReleaseNotes: true` を使う。GitHub の Release Notes API が前タグからのコミット / PR を本文にする。既存のインストール手順は `releaseBody` に残し、自動生成の前に付く。

v0.7.0 以前の Release は遡って書き換えない。次の `v*` タグから有効。

## Consequences

- コミット先頭（`feat:` / `fix:` / `release:` 等）が Release にそのまま出る。メッセージは利用者にも読める粒度にする。
- より細かい分類が必要になったら git-cliff 等を検討し、この ADR を更新する。
