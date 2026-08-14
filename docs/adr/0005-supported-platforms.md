# ADR 0005: Windows を本線。macOS は未確認。Linux は対象外

- Status: Accepted
- Date: 2026-08-14

## Context

開発環境は Windows。macOS 用の Tauri ビルドは CI で出せるが、実機確認できない。Linux 向けパッケージ（`.deb` 等）は Issue として残っているが、容易ではない。

## Decision

- **Windows**: 動作確認済みの本線。NSIS / MSI を配布する。
- **macOS**: Apple Silicon / Intel の `.dmg` を CI で出す。動作未確認と明記する。
- **Linux**: 当面対応しない。

## Consequences

- リリース workflow は Windows + macOS のみ。
- Linux Issue は Pending のまま進めない。再開するときはこの ADR を更新する。
