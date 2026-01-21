# 環境変数

開発時は `.envrc` で環境変数を設定する。

## 使用する変数

- `DISCORD_BOT_TOKEN`: Discord Bot のトークン（必須）。
- `LOG_LEVEL`: ログレベル（例: `info`, `debug`）。設定時は `RUST_LOG` より優先する。
- `DEV_MODE`: `true/1/yes/on` の場合に有効。ログの詳細表示を有効化し、`LOG_LEVEL` と `RUST_LOG` が未指定のときは既定を `debug` にする。
- `RUST_LOG`: `LOG_LEVEL` が未指定のときに利用する。

## 優先順位

1. `LOG_LEVEL`
2. `RUST_LOG`
3. `DEV_MODE` による既定値（`debug` / `info`）
