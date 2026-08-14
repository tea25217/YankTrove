# ADR 0003: 配布 ID は `com.yanktrove.desktop`

- Status: Accepted
- Date: 2026-08-14

## Context

Tauri / OS はアプリを bundle identifier で識別する。初期は `com.yanktrove.app` だったが、公開用として `desktop` に揃えた。

## Decision

identifier は `com.yanktrove.desktop` とする（`src-tauri/tauri.conf.json`）。

## Consequences

- OS 上は `com.yanktrove.app` とは別アプリ。既存インストールの上書きも設定引き継ぎもしない。
- identifier を再度変えるとまた別アプリになる。安易に戻さない。
