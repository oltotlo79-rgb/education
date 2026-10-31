title: Turso CLI と libSQL の確認
---checklist
- ☐ libSQL・sqld・Turso・Turso CLI の関係と、Turso CLI を WSL の中で `curl -sSfL https://get.tur.so/install.sh | bash` で入れる手順と、スクリプトがすることを説明できる（UNIT 09）
- ☐ Docker で libSQL のサーバー（`libsql-server`）を動かし、RustNote を `DATABASE_URL=http://127.0.0.1:8080` でつないで、`/health`・登録・SQL の結果を確かめられる（UNIT 09）
- ☐ Windows の開発用ビルドでリモートの DB につなぐと main のスタックがあふれることを知り、`cargo run --release` か `/STACK` で動かせる（UNIT 09）
- ☐ `http://` を忘れるとファイル（の別の流れ）に DB ができること、`libsql://` でトークンが要ることを読んで直せる（UNIT 09）
---glossary
| libSQL | SQLite をもとにした DB。ファイルとしても、サーバー越しにも使える |
| sqld | libSQL のサーバーのプログラム。HTTP で SQL を受け取る |
| Turso | libSQL の DB を貸すクラウドのサービス |
| Turso CLI | Turso を操作するコマンド（`turso`）。Windows 版は無く、WSL の中で使う |
| `DATABASE_URL` | RustNote の DB のつなぎ先。ファイルの名前・`http://`・`libsql://` |
| スタック | 関数を呼ぶたびに一時的な値を置く作業台。Windows の main は既定で 1 MB |
| 代替データストリーム | NTFS で、1つのファイルの中に持てる別の流れ。`名前:流れ` と書く |
