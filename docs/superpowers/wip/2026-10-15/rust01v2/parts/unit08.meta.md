title: Topcoat 0.11 と CLI
---checklist
- ☐ `cargo install topcoat-cli --version 0.11.0 --locked` で CLI を入れ、`topcoat --version` で確かめられる。アプリの Topcoat と CLI の版をそろえる理由を言える（UNIT 08）
- ☐ `topcoat new` で新しいアプリを作り、`Cargo.toml`・`src\app.rs`・`styles.css` の役目を言える（UNIT 08）
- ☐ `cargo run` だけでは「no asset bundle」で止まる理由を説明し、`topcoat asset bundle` か `topcoat dev` で動かせる（UNIT 08）
- ☐ `requires rustc 1.99`（古い Rust）と `AddrInUse`・10048（使われているポート）のエラーを読んで直せる（UNIT 08）
---glossary
| Topcoat | Rust で Web アプリを作るライブラリ。このシリーズは 0.11.0 |
| topcoat-cli | Topcoat の CLI（`topcoat` コマンド）のパッケージ名 |
| `cargo install` | crates.io からソースを取ってきてビルドし、`.cargo\bin` に置く |
| `--locked` | 作った人が確かめた `Cargo.lock` のとおりの版で作る指定 |
| アセットの束 | CSS などを、指紋付きの名前でまとめたもの。`target\debug\assets` |
| `topcoat dev` | ビルド・束・起動をまとめてして、保存のたびにやり直す開発用のサーバー |
