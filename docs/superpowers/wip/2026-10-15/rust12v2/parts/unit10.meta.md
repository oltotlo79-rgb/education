title: rollback・backup・障害からの復旧・更新
---checklist
- ☐ ほかの接続が開いている間に DB のファイルだけをコピーすると、-wal の分が抜けることを示し、VACUUM INTO でバックアップを取れる
- ☐ 戻すときは rustnote.db・-wal・-shm を全部よけてから写しを置き、件数と integrity_check で確かめられる
- ☐ 壊れた版を、前の版のイメージで戻せる。Fly.io の fly releases --image・fly deploy --image と、戻らない物（DB・secrets・fly.toml）を言える
- ☐ 依存の更新を、dry-run → 別のフォルダで更新 → テスト → 公開の順に進め、版を書く場所を組で直せる
---glossary
| WAL | 書き込みを、まず -wal のファイルに書き、あとで本体に書き戻す SQLite のしくみ |
| `-wal`・`-shm` | WAL の書き込みの記録と、その目次のファイル。DB の一部なので、本体と組で扱う |
| `VACUUM INTO` | 動いている DB の、ある瞬間の全体を、1つのファイルに写す SQL |
| `PRAGMA integrity_check` | DB のファイルの形が壊れていないかを確かめる SQL。壊れていなければ ok |
| rollback | 新しい版に問題があったとき、前の版（イメージ）に戻すこと。DB の中身は戻らない |
| point-in-time recovery | Turso で、過去の時刻の状態の DB を、新しい DB として作ること |
