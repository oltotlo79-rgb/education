title: secrets・設定・入れ替え
---checklist
- ☐ RustNote の設定の値を、秘密の値（トークン）と秘密でない値に分け、置き場所（fly secrets・[env]・ENV）と勝ち負けを言える
- ☐ ENV で焼き込んだトークンが docker image inspect と docker history で読めることを示し、漏れたら入れ替える理由を言える
- ☐ .dockerignore が無いと .env が builder の段に入りビルドに送られることを、実際のビルドで示せる
- ☐ fly secrets set・--stage・fly secrets deploy・list・unset と、Turso のトークンの入れ替えの順番を説明できる
---glossary
| secret | パスワードやトークンのような秘密の設定の値。Fly.io では fly secrets set で保管し、起動のときに環境変数で渡す |
| 入れ替え（ローテーション） | 秘密の値を、決まった時期や漏れたおそれがあるときに、新しい値に替えること |
| `fly secrets set` | 秘密の値を Fly.io に保管し、マシンを入れ替えて渡すコマンド（`--stage` で、登録だけ） |
| `docker image inspect` | イメージの設定（環境変数・利用者・ポートなど）を見るコマンド |
| `turso db tokens invalidate` | その DB（同じグループ）のトークンを全部使えなくする Turso のコマンド。短い停止がある |
