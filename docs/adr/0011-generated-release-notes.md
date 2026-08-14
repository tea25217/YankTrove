# ADR 0011: GitHub Release 本文はコミットから自動生成

- Status: Accepted
- Date: 2026-08-14

## Context

Release workflow の `releaseBody` がインストール手順の固定文だけだと、そのタグで何が変わったか分からない。手書き CHANGELOG は運用コストが高い。

## Decision

Release 作成は workflow の `create-release` ジョブで一度だけ行う。GitHub の Release Notes API で前タグからの変更一覧を作り、インストール手順の後に付ける。各 OS の `tauri-action` は既存 Release へ成果物を載せるだけにする。

v0.7.0 以前の Release は遡って書き換えない。v0.8.0 から有効。

`tauri-action` の `generateReleaseNotes` を matrix の各ジョブで使うと、後続ジョブの `updateRelease` が本文をインストール手順だけに戻してしまうため使わない。

## Consequences

- コミット先頭（`feat:` / `fix:` / `release:` 等）が Release にそのまま出る。メッセージは利用者にも読める粒度にする。
- より細かい分類が必要になったら git-cliff 等を検討し、この ADR を更新する。
