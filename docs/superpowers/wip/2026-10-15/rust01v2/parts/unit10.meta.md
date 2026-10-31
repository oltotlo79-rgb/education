title: flyctl と環境の診断
---checklist
- ☐ flyctl を公式の `iwr https://fly.io/install.ps1 -useb | iex` で入れ、`fly version` で版と CPU の種類（`windows/amd64`）を確かめられる（UNIT 10）
- ☐ `fly version` はアカウント不要、`fly auth login`・`whoami`・`logout`・`signup` はアカウントにつながることを区別できる。トークンが `.fly\config.yml` に保存されることを知っている（UNIT 10）
- ☐ `doctor.ps1` で、この巻の道具を順に確かめ、NG の行の括弧の単元に戻れる。`-File` には `$HOME` を使い、ふつうの PowerShell で動かす（UNIT 10）
---glossary
| flyctl | Fly.io を操作するコマンド。`fly` と打っても同じ |
| `fly auth` | ログインまわりのコマンド。login・whoami・logout・signup |
| `.fly\config.yml` | ログインしたときのトークンが保存されるファイル。人に見せない |
| 環境の診断 | 道具がそろっているか・版が合っているかを、決まった順番で確かめること |
| `$HOME` | ホームフォルダ。PowerShell が先に `C:\Users\（名前）` に直す |
