import sys
from pathlib import Path
HERE = Path(sys.argv[1])

def patch(name, pairs):
    p = HERE / "parts" / name
    t = p.read_text(encoding="utf-8")
    for old, new in pairs:
        assert t.count(old) == 1, (name, old[:70])
        t = t.replace(old, new)
    p.write_text(t, encoding="utf-8", newline="\n")

patch("intro.md", [
    ("| UNIT 04 | プロジェクト構成と依存の向き（準備中） |", "| UNIT 04 | プロジェクト構成と依存の向き |"),
    ("> **お知らせ:** この版には UNIT 01〜03 を収録しています。UNIT 04〜10 は準備中で、",
     "> **お知らせ:** この版には UNIT 01〜04 を収録しています。UNIT 05〜10 は準備中で、"),
    ("UNIT 01〜03 では使いません", "UNIT 01〜04 では使いません"),
    ("UNIT 01〜03 では、新しいクレートを足しません。", "UNIT 01〜04 では、新しいクレートを足しません。"),
    ("| `tests/*.rs` | 受入条件や画面の流れを確かめるテスト（UNIT 01・02） |",
     "| `tests/*.rs` | 受入条件・画面の流れ・依存の向きを確かめるテスト（UNIT 01・02・04） |"),
    ("| `examples/*.rs` | ドメインの型や DB の制約を確かめる小さなプログラム（UNIT 03） |",
     "| `examples/*.rs` | ドメインの型や DB の制約を確かめ、モジュールの依存を一覧にする小さなプログラム（UNIT 03・04） |"),
])

unit04_check = """

**UNIT 04: プロジェクト構成と依存の向き**

- ☐ RustNote の 10 個のモジュールを6つの段（入口・URL ごとの処理・画面の部品とログイン・共有の状態・保存・土台）に並べ、それぞれの役割を1文で言える
- ☐ 「上の段は下の段を使ってよい・下の段は上の段を使わない」という依存の向きと、下の段ほど変わりにくい理由を説明できる
- ☐ `views`・`auth` が `routes` を指す例外の2本と、同じクレートの中なら互いに使い合ってもコンパイルできるが、クレートを分けると `cyclic package dependency` になることを説明できる
- ☐ ライブラリ・実行ファイル・`tests/`・`examples/` が別々のクレートであることと、`crate::` と `rustnote::` の使い分け（E0432）を説明できる
- ☐ `pub use` で、ファイルの並び（中の都合）と公開する道のり（外との約束）を分けられる（E0603）
- ☐ 依存の向きの決まりをテスト（`tests/architecture.rs`）にして、決まりを破る変更に `cargo test` で気づける。文字を数えるテストの限界も言える
- ☐ `AppState` の保存先を trait のテスト用の実装に差し替えて、DB が壊れたときの画面を確かめられる。ページの中の panic は、Topcoat が本文 `internal server error` の 500 に変えることを知っている
"""

patch("outro.md", [
    ("この版に収録した UNIT 01〜03 で、", "この版に収録した UNIT 01〜04 で、"),
    ("- ☐ 文字数を `len()` ではなく `chars().count()` で数える理由と、`include_str!` のパスの数え方を説明できる\n",
     "- ☐ 文字数を `len()` ではなく `chars().count()` で数える理由と、`include_str!` のパスの数え方を説明できる\n" + unit04_check.rstrip("\n") + "\n"),
    ("> **お知らせ:** この版には UNIT 01〜03 を収録しています。UNIT 04（プロジェクト構成と依存の向き）・UNIT 05（",
     "> **お知らせ:** この版には UNIT 01〜04 を収録しています。UNIT 05（"),
    ("この巻（UNIT 01〜03）に出てきた用語です。", "この巻（UNIT 01〜04）に出てきた用語です。"),
])

p = HERE / "parts" / "outro.md"
t = p.read_text(encoding="utf-8").rstrip("\n")
t += """

### UNIT 04 の用語

| 用語 | 意味 |
|---|---|
| 依存 | あるモジュールが、ほかのモジュールやクレートを使っていること |
| 依存の向き | どのモジュールが、どのモジュールを使ってよいかの決まり。上の段は下の段を使ってよく、下の段は上の段を使わない |
| パッケージ | `Cargo.toml` が1つのまとまり。中にクレートがいくつもある |
| クレート | Rust がひとまとめにコンパイルする単位。ライブラリ・実行ファイル・`tests/` と `examples/` のファイル1つずつが、それぞれ1つ |
| `crate::` | 自分のクレートの根元から道のりを書くときの始まり。ライブラリの外（`main.rs`・`tests/`・`examples/`）からは `rustnote::` を使う |
| `pub use` | 中の物を、別の道のりでも公開する。ファイルの並びと、外に見せる道のりを分けられる |
| 循環（cyclic package dependency） | パッケージどうしが互いに依存している形。クレートは順番にコンパイルするので、Cargo が止める |
| `href!` | ページの関数を指定して、リンクの URL を作る Topcoat のマクロ。関数が無くなるとコンパイルエラーになる |
| E0432 | `use` の道のりを解決できないエラー。別のクレートから `crate::` を書いたときにも出る |
| E0046 | trait の約束のメソッドが、`impl` に全部はそろっていないエラー |
"""
p.write_text(t + "\n", encoding="utf-8", newline="\n")
print("ok")
