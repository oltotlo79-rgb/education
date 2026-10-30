"""第6巻を Topcoat 0.11.0 / Rust 1.99.1 / tokio 1.54.1 に合わせる（UNIT 01〜02 と巻頭・巻末）。"""
import sys


def edit(p, reps, count_all=False):
    s = open(p, encoding="utf-8").read()
    for a, b in reps:
        n = s.count(a)
        if n == 0 or (n != 1 and not count_all):
            sys.exit(f"{p}: count {n}: {a[:60]}")
        s = s.replace(a, b)
    open(p, "w", encoding="utf-8", newline="\n").write(s)


edit("parts/intro.md", [
    ("Topcoat は 0.5 から 0.10 までに書き方が大きく変わっているので、以前の版のコードは、0.10 では動かないことがあります。",
     "Topcoat は 0.5 から 0.11 までに書き方が大きく変わっているので、以前の版のコードは、0.11 では動かないことがあります。"),
    ("| Rust | **1.98 以上**（本書シリーズの基準）。動作確認は rustc 1.99.0 / cargo 1.99.0（`aarch64-pc-windows-msvc`） |",
     "| Rust | **1.99 以上**（本書シリーズの基準。Topcoat 0.11 が 1.99 以上を求めます）。動作確認は rustc 1.99.1 / cargo 1.99.1（`aarch64-pc-windows-msvc`） |"),
    ("| topcoat | 0.10.0（UNIT 02 から使います。UNIT 01 では使いません。2026-10 には 0.11.0 も出ていますが、この本は 0.10.0 に固定しています） |",
     "| topcoat | 0.11.0（2026-10-24 に出た版。UNIT 02 から使います。UNIT 01 では使いません） |"),
    ("# このフォルダでは Rust 1.99.0 を使う（第13巻 UNIT 01）\nSet-Content rust-toolchain.toml \"[toolchain]`nchannel = `\"1.99.0`\"\"",
     "# このフォルダでは Rust 1.99.1 を使う（第13巻 UNIT 01）\nSet-Content rust-toolchain.toml \"[toolchain]`nchannel = `\"1.99.1`\"\""),
])

edit("parts/unit02.src.md", [
    ("## UNIT 02: Topcoat 0.10 の最小のアプリ", "## UNIT 02: Topcoat 0.11 の最小のアプリ"),
    ("- `cargo add` で Topcoat 0.10.0 と Tokio を足し、", "- `cargo add` で Topcoat 0.11.0 と Tokio を足し、"),
    ("新しいプロジェクトを作り、Rust の版を 1.99.0 に固定してから、`cargo add` で Topcoat（版を 0.10.0 に固定）と Tokio（非同期のランタイム）を足します。",
     "新しいプロジェクトを作り、Rust の版を 1.99.1 に固定してから、`cargo add` で Topcoat（版を 0.11.0 に固定）と Tokio（非同期のランタイム）を足します。"),
    ("Set-Content rust-toolchain.toml \"[toolchain]`nchannel = `\"1.99.0`\"\"\ncargo add topcoat@=0.10.0\ncargo add tokio@=1.53.3 --features rt-multi-thread,macros",
     "Set-Content rust-toolchain.toml \"[toolchain]`nchannel = `\"1.99.1`\"\"\ncargo add topcoat@=0.11.0\ncargo add tokio@=1.54.1 --features rt-multi-thread,macros"),
    ("**▼ 実行結果**（`cargo add topcoat@=0.10.0` の表示）", "**▼ 実行結果**（`cargo add topcoat@=0.11.0` の表示）"),
    ("- `+` の付いた features（`asset`・`router`・`view` など）が、既定で有効になる機能です。`-` の付いたもの（`tailwind`・`websocket` など）は、必要なときに自分で足します。",
     "- `+` の付いた features（`asset`・`router`・`view` など）が、既定で有効になる機能です。そのほかの機能（`tailwind`・`websocket`・`l10n` など）は既定では無効で、数が多いので、`22 deactivated features`（無効の機能が 22 個）とだけ表示されます。一覧は `cargo info -v topcoat@0.11.0` で見られます。必要なときに、自分で足します。"),
    ("- 最後の `Adding topcoat v0.10.0 (available: v0.11.0)` は、「0.10.0 を入れましたが、0.11.0 も出ています」というお知らせです。この本は、すべての巻で **0.10.0 に固定**しています。`@=0.10.0` の `=` を付けているので、勝手に 0.11.0 にはなりません（第13巻 UNIT 01）。Topcoat は、版が上がると書き方が変わることがあるので、版を上げるときは、自分で決めて、まとめて直します。",
     "- `Locking 220 packages to highest Rust 1.99.1 compatible versions` は、「Rust 1.99.1 で使える範囲で、いちばん新しい版を選んで、220 個のクレートの版を `Cargo.lock` に書きました」という意味です。`Adding generic-array v0.14.7 (available: v0.14.9)` は、「もっと新しい 0.14.9 もありますが、0.14.7 にしました」というお知らせです。間接的に使う `crypto-common` が `=0.14.7` と版を固定しているからで、気にしなくて大丈夫です。\n- `@=0.11.0` の `=` を付けているので、Topcoat に新しい版が出ても、勝手には上がりません（第13巻 UNIT 01）。Topcoat は、版が上がると書き方が変わることがあるので、版を上げるときは、自分で決めて、まとめて直します。この本も、0.10.0 から 0.11.0 に上げたときに、すべての巻のコードを動かし直しました。"),
    ("（この PC では、`Compiling` の行が 164 行出て、約 31 秒でした）", "（この PC では、`Compiling` の行が 164 行出て、約 22 秒でした）"),
    ("`topcoat::start` の中身（Topcoat 0.10.0 の `serve.rs`）も、", "`topcoat::start` の中身（Topcoat 0.11.0 の `serve.rs`）も、"),
    ("cargo add tokio@=1.53.3 --features rt-multi-thread,macros,net", "cargo add tokio@=1.54.1 --features rt-multi-thread,macros,net"),
    ("- `cargo add topcoat@=0.10.0` と Tokio を足し、`#[page(\"/\")]` を付けた関数と、`Router::builder().discover().build()`・`topcoat::start(router).await` の数行で、Web アプリが動く。版は `=` で固定する（0.11.0 が出ていても、0.10.0 のまま）。",
     "- `cargo add topcoat@=0.11.0` と Tokio を足し、`#[page(\"/\")]` を付けた関数と、`Router::builder().discover().build()`・`topcoat::start(router).await` の数行で、Web アプリが動く。版は `=` で固定する（新しい版が出ても、勝手には上がらない）。"),
])

edit("parts/outro.md", [
    ("**UNIT 02: Topcoat 0.10 の最小のアプリ**", "**UNIT 02: Topcoat 0.11 の最小のアプリ**"),
    ("- ☐ `cargo add topcoat@=0.10.0` と Tokio で、", "- ☐ `cargo add topcoat@=0.11.0` と Tokio で、"),
])

edit("assemble.py", [("06_Web基礎_HTTPとTopcoat_0_10完全ガイド.md", "06_Web基礎_HTTPとTopcoat_0_11完全ガイド.md")])
print("ok")
