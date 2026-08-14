# ADR 0010: 秘密情報スキャンは Betterleaks

- Status: Accepted
- Date: 2026-08-14

## Context

公開リポジトリのため、トークンや個人情報の push を止めたい。Gitleaks は開発停止と判断し、採用しない。

## Decision

- CI（`.github/workflows/secret-scan.yml`）は [Betterleaks](https://github.com/betterleaks/betterleaks) 固定バージョンで `git` スキャンする。設定は `.betterleaks.toml`。
- ローカルは任意。`scripts/install-git-hooks.ps1` で pre-commit / pre-push を入れる。Betterleaks 未導入時は hook 側の正規表現フォールバックで高確度パターンのみ拒否する。
- GitHub の Secret scanning / Push protection も有効にする。

## Consequences

- Gitleaks 設定（`.gitleaks.toml`）には戻さない。
- Betterleaks のバージョン上げは CI のピンを明示的に更新する。
