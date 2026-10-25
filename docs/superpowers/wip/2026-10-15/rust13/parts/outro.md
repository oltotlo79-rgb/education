## この巻のまとめ

この版に収録した UNIT 01 で、次のことができるようになったかを確かめましょう。できたものに印を付けてください。

**UNIT 01: 版を固定する設定ファイル**

- ☐ `Cargo.toml`・`Cargo.lock`・`rust-toolchain.toml`・`rustfmt.toml`・`.gitignore`・`.env.example` の6つについて、読む道具と、決めていることを言える
- ☐ `"=1.53.3"`（その版だけ）と `"1.53.3"`（1.53.3 以上 2.0.0 未満）の違いを、`cargo update -p tokio --dry-run` の結果で説明できる
- ☐ `rust-version`（最低の版）と `rust-toolchain.toml`（このフォルダで使う版）の違いと、`overridden by` の意味を説明できる
- ☐ `--locked` が `Cargo.lock` の書き換えを止めることと、`Dockerfile` と `ci.yml` が `--locked` を付けている理由を説明できる
- ☐ features は依存のつながり全体で足し算されることを、`cargo tree -f "{p} [{f}]"` の出力で確かめられる。自分が使う機能は、自分の `Cargo.toml` に書く
- ☐ `cargo tree -i クレート名` で、あるクレートを誰が使っているかをたどれる（`ring` ← rustls ← hyper-rustls ← libsql）
- ☐ 版が古い（`requires rustc 1.98`）・機能の名前の打ち間違い・`--locked` の食い違い・checksum の食い違い・`#[tokio::main]` の E0433 を読み、直すファイルを決められる
- ☐ `git check-ignore -v` で、ファイルが `.gitignore` のどの決まりで外されるかを確かめられる

> **お知らせ:** この版には UNIT 01 を収録しています。UNIT 02（表を作る SQL と DB の入口）・UNIT 03（設定と起動の順番）・UNIT 04（ドメインの型とエラーの分類）・UNIT 05（リポジトリ）・UNIT 06（ログイン・お知らせ・共有の状態）・UNIT 07（画面の枠・部品・登録とログインの画面）・UNIT 08（ノートの CRUD）・UNIT 09（見た目）・UNIT 10（テスト・Docker・Fly.io・CI）と、巻末の総合演習・解答・全巻の用語集・索引は準備中で、次の更新から順に追加します。それまでの間、RustNote のファイルは、教材のフォルダの `Rust/rustnote/` で直接読めます。どのファイルも、先頭のコメントに役割が書いてあります。

## 用語ミニ辞典

この巻（UNIT 01）に出てきた用語です。

### UNIT 01 の用語

| 用語 | 意味 |
|---|---|
| 版の固定（pin） | 使う道具や部品の版を、1つに決めて書いておくこと |
| 依存クレート | 自分のプログラムが使う、ほかの人が作ったクレート。そのクレートが使うクレートも、間接的な依存になる |
| `Cargo.toml` | パッケージの名前と、使うクレートと版を、人が書くファイル |
| `Cargo.lock` | cargo が選んだ全クレートの版と checksum を記録するファイル。手で書き換えない |
| checksum | クレートの中身から計算した指紋（SHA-256）。取ってきた中身が記録と同じかを確かめる |
| `--locked` | `Cargo.lock` の書き換えが必要なら、ビルドせずにエラーで止める指定 |
| `rust-version` | そのパッケージに必要な、Rust の最低の版 |
| `rust-toolchain.toml` | そのフォルダで使う Rust の版を決めるファイル。rustup が読む |
| features | クレートの機能のスイッチ。依存のつながり全体で足し算される |
| `[dev-dependencies]` | テスト・examples でだけ使うクレート。本番の実行ファイルには入らない |
| `cargo tree` | 依存のつながりを木の形で表示する。`-i` で逆向きにたどる |
| `.gitignore` | Git に入れないファイルの名前の決まり |
| `.env` / `.env.example` | 環境変数を書くファイルと、その見本。`.env` は Git に入れない |
| E0433 | 名前（モジュールやクレートの中の物）が見つからないエラー |
| E0752 | `main` 関数に `async` を付けたエラー |
