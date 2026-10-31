title: rustup・rustc・Cargo
---checklist
- ☐ rustup（版の管理）・rustc（コンパイラ）・cargo（ビルドと部品の段取り）の役目の違いと、`.cargo\bin` の受付が版を選ぶしくみを説明できる（UNIT 04）
- ☐ `https://rustup.rs/` から CPU に合った `rustup-init.exe` を入れ、新しい窓で `rustc --version`・`rustup show` を確かめられる（UNIT 04）
- ☐ `cargo new`・`cargo run` で最初のプログラムを動かし、`Cargo.toml`・`src\main.rs`・`target\debug` の役目を言える（UNIT 04）
- ☐ `rust-toolchain.toml` でフォルダごとに版を決め、`rustup install`・`rustup update` で版を入れたり新しくしたりできる。自動で入れる動きが非推奨なことを知っている（UNIT 04）
---glossary
| rustup | ラストアップ。Rust の版（ツールチェーン）を入れたり選んだりする道具 |
| rustc | Rust のコンパイラ。ソースコードを機械語にする |
| cargo | カーゴ。Rust のビルドと部品（クレート）の管理をする道具 |
| ツールチェーン | ある版の Rust の道具一式。`stable-aarch64-pc-windows-msvc` のような名前 |
| stable | いちばん新しい安定版。`rustup update` で新しくなる |
| `Cargo.toml` | プロジェクトの表紙。名前・版・使う部品を書く |
| `target` フォルダ | cargo がビルドした物を置くフォルダ。Git には入れない |
