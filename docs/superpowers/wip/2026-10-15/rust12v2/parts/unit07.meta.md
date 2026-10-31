title: migration を出す順番と release command
---checklist
- ☐ 起動のたびに migrate をかける作りで、同時に起動したマシンがぶつかる理由を、例7-1の結果で説明できる
- ☐ Fly.io の release_command のしくみ（新しいイメージで1回・失敗で公開を止める・volume は無い）を言え、--migrate-only の動かし方を足して確かめられる
- ☐ 足すだけのマイグレーションと、名前を変えるマイグレーションで、古い版が動くかの違いを示せる
- ☐ 広げてから縮める（expand / contract）の4つの手順を言える
---glossary
| release command | 新しい版のマシンを入れ替える前に、新しいイメージで1回だけ動かすコマンド。マイグレーションをここで済ませる |
| expand / contract | DB の形を変えるとき、まず足し（広げ）、全部のコードが新しい形になってから古い物を消す（縮める）進め方 |
| `--migrate-only` | この巻で RustNote のコピーに足した、マイグレーションだけをして終わる動かし方 |
| rolling | マシンを順番に入れ替える公開のやり方 |
| `ALTER TABLE … ADD COLUMN` | 表に列を足す SQL。古い版のコードはその列を知らなくても動く |
