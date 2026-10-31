title: fly launch・fly.toml・ポート
---checklist
- ☐ fly.toml の各項目（app・primary_region・[build]・[env]・[http_service]・ヘルスチェック・[[vm]]）と、RustNote の設定の対応を説明できる
- ☐ ブラウザから RustNote までの道筋（443 → 入口 → internal_port 8080 → 0.0.0.0:8080）を言える
- ☐ fly launch がすること（調べる・名前と場所・fly.toml・公開）と --no-deploy を、公式のドキュメントにそって説明できる
- ☐ fly.toml と Dockerfile の食い違い（PORT・force_https・[env] の秘密）を、手元の道具とコンテナで見つけられる
---glossary
| fly.toml | Fly.io でアプリを動かすときの設定ファイル（TOML）。名前・地域・ポート・ヘルスチェック・マシンの大きさを書く |
| リージョン | Fly.io のマシンを置く場所。3文字の名前（nrt は東京） |
| `internal_port` | Fly.io の入口が、マシンの中のどのポートへリクエストを届けるか |
| `force_https` | http:// のリクエストを https:// へ移す設定 |
| `auto_stop_machines` | 使われていない間、マシンを止めるか（"off"・"stop"・"suspend"） |
| `tomllib` | Python の標準の、TOML を読むための部品 |
