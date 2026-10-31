title: Cargo の品質コマンド
---checklist
- ☐ `cargo check`（型と文法）・`cargo fmt`（体裁）・`cargo clippy`（書き方の注意）・`cargo test`（答え合わせ）を、目的ごとに使い分けられる（UNIT 06）
- ☐ `cargo fmt --check` の差分、clippy の注意（`-->` の場所と `help:`）、テストの `left`・`right` を読んで、コードを直せる（UNIT 06）
- ☐ `-D warnings` で注意を失敗にでき、終了コード（0・1・101）を `$LASTEXITCODE` で確かめられる（UNIT 06）
- ☐ `'cargo-fmt.exe' is not installed` を読み、`rustup component add rustfmt clippy` で足せる（UNIT 06）
---glossary
| `cargo check` | 型と文法だけを確かめる。実行ファイルは作らないので速い |
| `cargo fmt` | rustfmt で、字下げや空白を決まった形に整える。`--check` は差分を見せるだけ |
| `cargo clippy` | よくある書き方の注意を出す。`-- -D warnings` で注意をエラーにする |
| `cargo test` | テストを動かして、結果が期待どおりかを確かめる |
| 終了コード | コマンドが終わるときに返す数。0 は成功。PowerShell では `$LASTEXITCODE` |
| `--profile minimal` | rustup で、rustc・cargo・標準ライブラリだけを入れる組み合わせ |
