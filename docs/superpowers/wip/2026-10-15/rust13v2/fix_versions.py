"""第13巻の巻頭と UNIT 01 の版の表記を、今の RustNote（Rust 1.99.1・Topcoat 0.11.0・tokio 1.54.1）に合わせる。"""
from pathlib import Path

parts = Path(__file__).resolve().parent / "parts"


def edit(name, reps):
    p = parts / name
    s = p.read_text(encoding="utf-8")
    for old, new in reps:
        n = s.count(old)
        assert n >= 1, (name, old[:60])
        s = s.replace(old, new)
    p.write_text(s, encoding="utf-8", newline="\n")
    print(name, "ok")


edit("intro.md", [
    ("行数は `Cargo.lock` を除くと約 4,700 行です。", "行数は `Cargo.lock` を除くと約 4,600 行です。"),
    ("| `Cargo.lock` | 3,682 | 01 |", "| `Cargo.lock` | 3,753 | 01 |"),
    ("このフォルダで使う Rust の版（1.99.0） |", "このフォルダで使う Rust の版（1.99.1） |"),
    ("> **注意:** 第8巻・第11巻・第12巻は、書き直しの途中です。", "> **注意:** 第12巻は、書き直しの途中です。"),
    ("| Rust | **1.98 以上**（本書シリーズの基準）。動作確認は rustc 1.99.0 / cargo 1.99.0（`aarch64-pc-windows-msvc`） |",
     "| Rust | **1.99 以上**（本書シリーズの基準。Topcoat 0.11 が 1.99 以上を求めます）。動作確認は rustc 1.99.1 / cargo 1.99.1（`aarch64-pc-windows-msvc`） |"),
    ("topcoat 0.10.0、libsql 0.10.0、tokio 1.53.3、", "topcoat 0.11.0、libsql 0.10.0、tokio 1.54.1、"),
    ("| topcoat-cli | 0.10.0（", "| topcoat-cli | 0.11.0（"),
    ("1.99.0-aarch64-pc-windows-msvc (overridden by", "1.99.1-aarch64-pc-windows-msvc (overridden by"),
])

edit("unit01.src.md", [
    ("Rust 1.99.0 を使う・整形の決まり", "Rust 1.99.1 を使う・整形の決まり"),
    ("使う Rust の最低の版（1.98）", "使う Rust の最低の版（1.99）"),
    ("同じ Rust 1.99.0 でも、", "同じ Rust 1.99.1 でも、"),
    ("- `rust-version = \"1.98\"` は、「このパッケージをビルドするには、Rust 1.98 以上が必要」という宣言です。Topcoat 0.10 が 1.98 以上を求めているので、それに合わせています。",
     "- `rust-version = \"1.99\"` は、「このパッケージをビルドするには、Rust 1.99 以上が必要」という宣言です。Topcoat 0.11 が 1.99 以上を求めているので、それに合わせています。"),
    ("##### 解説2: \"=1.53.3\" は「この版だけ」", "##### 解説2: \"=1.54.1\" は「この版だけ」"),
    ("- `tokio = { version = \"=1.53.3\", features = [\"rt-multi-thread\", \"macros\"] }` は、",
     "- `tokio = { version = \"=1.54.1\", features = [\"rt-multi-thread\", \"macros\"] }` は、"),
    ("使う Rust の版を 1.99.0 に決め、", "使う Rust の版を 1.99.1 に決め、"),
    ("`Cargo.toml` の `rust-version = \"1.98\"` は「1.98 以上なら何でもよい」、`rust-toolchain.toml` の `channel = \"1.99.0\"` は「このフォルダでは 1.99.0 を使う」です。",
     "`Cargo.toml` の `rust-version = \"1.99\"` は「1.99 以上なら何でもよい」、`rust-toolchain.toml` の `channel = \"1.99.1\"` は「このフォルダでは 1.99.1 を使う」です。"),
    ("この PC の既定の版は 1.97.1 ですが、RustNote のフォルダでは 1.99.0 が使われています。", "この PC の既定の版は 1.97.1 ですが、RustNote のフォルダでは 1.99.1 が使われています。"),
    ("全員が同じ 1.99.0 でビルドします。", "全員が同じ 1.99.1 でビルドします。"),
    # 例1-5
    ("""> **▼ このコマンドがやること（先に日本語で）:** `cargo update -p tokio --dry-run` は、「tokio を新しくしたら、どの版になるか」を、`Cargo.lock` を書き換えずに表示します（`--dry-run` は「試しに動かすだけ」）。まず今の `Cargo.toml` のまま実行し、次に tokio の行の `=` を消して、もう一度実行します。確かめたら `=` を元に戻します。

```powershell
# 今のまま（version = "=1.53.3"）
cargo update -p tokio --dry-run
```
""",
     """> **▼ このコマンドがやること（先に日本語で）:** `cargo update -p tokio --dry-run` は、「tokio を新しくしたら、どの版になるか」を、`Cargo.lock` を書き換えずに表示します（`--dry-run` は「試しに動かすだけ」）。ただし、今の RustNote の tokio（1.54.1）は、いちばん新しい版なので、このままでは違いが見えません。そこで、練習用のコピーで、tokio を1つ前の 1.53.3 に固定し直してから試します。確かめたら、`Cargo.toml` と `Cargo.lock` を教材のものに戻します。

```powershell
# Cargo.toml の tokio の行を version = "=1.53.3" に変えてから、Cargo.lock も 1.53.3 にする
cargo update -p tokio --precise 1.53.3
```

```text
@@OUT:u01_downgrade@@
```

```powershell
# =1.53.3 のまま
cargo update -p tokio --dry-run
```
"""),
    ("- `to highest Rust 1.98 compatible version` は、「Rust 1.98 で使える版の中で、いちばん新しい版」という意味です。",
     "- `to highest Rust 1.99 compatible version` は、「Rust 1.99 で使える版の中で、いちばん新しい版」という意味です。"),
    ("- `26 unchanged dependencies behind latest` は、「もっと新しい版が出ているのに、今の決まりでは選べないクレートが 26 個ある」という意味です。",
     "- `2 unchanged dependencies behind latest` は、「もっと新しい版が出ているのに、今の決まりでは選べないクレートが 2 個ある」という意味です。`=` を消すと 1 個に減りました。tokio が、新しい版を選べるようになったからです。"),
    ("<text x=\"565\" y=\"96\" font-size=\"13\" font-weight=\"700\" fill=\"#047857\" text-anchor=\"middle\">tokio v1.53.3（1つにまとめる）</text>",
     "<text x=\"565\" y=\"96\" font-size=\"13\" font-weight=\"700\" fill=\"#047857\" text-anchor=\"middle\">tokio v1.54.1（1つにまとめる）</text>"),
    # エラー1
    ("（`requires rustc 1.98` の行が 35 行続くので、途中を省いています）", "（`requires rustc 1.99` の行が 35 行続くので、途中を省いています）"),
    ("`rustnote@0.1.0 requires rustc 1.98` は、RustNote の `Cargo.toml` の `rust-version = \"1.98\"`、`topcoat@0.10.0 requires rustc 1.98` は、Topcoat 側の同じ宣言です。",
     "`rustnote@0.1.0 requires rustc 1.99` は、RustNote の `Cargo.toml` の `rust-version = \"1.99\"`、`topcoat@0.11.0 requires rustc 1.99` は、Topcoat 側の同じ宣言です。"),
    ("読むのは、パッケージの名前と `requires rustc 1.98` の部分だけで十分です。", "読むのは、パッケージの名前と `requires rustc 1.99` の部分だけで十分です。"),
    ("`rust-toolchain.toml` があるので 1.99.0 が使われます。1.99.0 がまだ入っていない PC では、`rustup toolchain install 1.99.0 --profile minimal --component rustfmt,clippy` で入れます",
     "`rust-toolchain.toml` があるので 1.99.1 が使われます。1.99.1 がまだ入っていない PC では、`rustup toolchain install 1.99.1 --profile minimal --component rustfmt,clippy` で入れます"),
    # エラー2
    ("tokio = { version = \"=1.53.3\", features = [\"rt-multi-thread\", \"macro\"] }", "tokio = { version = \"=1.54.1\", features = [\"rt-multi-thread\", \"macro\"] }"),
    ("- `versions that meet the requirements =1.53.3 (locked to 1.53.3) are: 1.53.3` は、「条件に合う版は 1.53.3 だけで、",
     "- `versions that meet the requirements =1.54.1 (locked to 1.54.1) are: 1.54.1` は、「条件に合う版は 1.54.1 だけで、"),
    # RustNote ではここで使う
    ("| `Dockerfile` | `FROM rust:1.99.0-bookworm AS builder` | `rust-toolchain.toml` と同じ 1.99.0 |", "| `Dockerfile` | `FROM rust:1.99.1-bookworm AS builder` | `rust-toolchain.toml` と同じ 1.99.1 |"),
    ("| `Dockerfile` | `RUN cargo install topcoat-cli --version 0.10.0 --locked` | `Cargo.toml` の topcoat と同じ 0.10.0 |", "| `Dockerfile` | `RUN cargo install topcoat-cli --version 0.11.0 --locked` | `Cargo.toml` の topcoat と同じ 0.11.0 |"),
    ("| `ci.yml` | `toolchain: 1.99.0` と", "| `ci.yml` | `toolchain: 1.99.1` と"),
    ("- Rust の版（1.99.0）は、", "- Rust の版（1.99.1）は、"),
    ("- topcoat の版（0.10.0）も、", "- topcoat の版（0.11.0）も、"),
    # 問2
    ("- 下から読むと、`rustnote → libsql → hyper-rustls → rustls → ring` とつながっています。libsql が Turso（ネットの向こうの DB）と暗号化して通信するために rustls（TLS という暗号化の仕組みを Rust で書いたクレート）を使い、rustls が暗号の計算に ring を使っているのです。",
     "- 下から読むと、`rustnote → libsql → hyper-rustls → rustls → ring` とつながっています。libsql が Turso（ネットの向こうの DB）と暗号化して通信するために rustls（TLS という暗号化の仕組みを Rust で書いたクレート）を使い、rustls と、証明書を確かめる rustls-webpki が、暗号の計算に ring を使っているのです。"),
])
