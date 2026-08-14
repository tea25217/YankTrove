# ADR 0011: GitHub Release 本文はコミットから自動生成

- Status: Accepted
- Date: 2026-08-14

## Context

Release workflow の `releaseBody` がインストール手順の固定文だけだと、そのタグで何が変わったか分からない。手書き CHANGELOG は運用コストが高い。

## Decision

Release 作成は workflow の `create-release` ジョブで一度だけ行う。前タグからのコミット件名（`release:` は除く）を「変更」としてインストール手順の後に付ける。各 OS の `tauri-action` は既存 Release へ成果物を載せるだけにする。

このリポジトリは PR をほぼ使わないため、GitHub の PR ベース自動生成より `git log` の方が中身が残る。`tauri-action` の `generateReleaseNotes` を matrix 各ジョブで使うと、後続ジョブが本文をインストール手順だけに戻す。

## Consequences

- コミット先頭（`feat:` / `fix:` / `release:` 等）が Release にそのまま出る。メッセージは利用者にも読める粒度にする。
- より細かい分類が必要になったら git-cliff 等を検討し、この ADR を更新する。
