"""第9巻を Topcoat 0.11.0 / Rust 1.99.1 / tokio 1.54.1 に合わせる。"""
import sys

p = "parts/intro.md"
s = open(p, encoding="utf-8").read()
reps = [
    ("| Rust | **1.98 以上**（本書シリーズの基準）。動作確認は rustc 1.99.0 / cargo 1.99.0（`aarch64-pc-windows-msvc`） |",
     "| Rust | **1.99 以上**（本書シリーズの基準。Topcoat 0.11 が 1.99 以上を求めます）。動作確認は rustc 1.99.1 / cargo 1.99.1（`aarch64-pc-windows-msvc`） |"),
    ("| topcoat | 0.10.0（Web フレームワーク） |", "| topcoat | 0.11.0（Web フレームワーク） |"),
    ("| tokio | 1.53.3（`rt-multi-thread` と `macros` の機能だけ） |", "| tokio | 1.54.1（`rt-multi-thread` と `macros` の機能だけ） |"),
    ("| topcoat-cli | 0.10.0（`topcoat dev` などのコマンド。", "| topcoat-cli | 0.11.0（`topcoat dev` などのコマンド。"),
    ("1.99.0-aarch64-pc-windows-msvc (overridden by '...\\rustnote\\rust-toolchain.toml')",
     "1.99.1-aarch64-pc-windows-msvc (overridden by '...\\rustnote\\rust-toolchain.toml')"),
    ("中に `channel = \"1.99.0\"` と書いてあります。", "中に `channel = \"1.99.1\"` と書いてあります。"),
    ("PC の既定の版に関係なく、Rust 1.99.0 が使われます。", "PC の既定の版に関係なく、Rust 1.99.1 が使われます。"),
    ("- 1.99.0 がまだ入っていない PC では、", "- 1.99.1 がまだ入っていない PC では、"),
    ("rustup toolchain install 1.99.0 --profile minimal --component rustfmt,clippy", "rustup toolchain install 1.99.1 --profile minimal --component rustfmt,clippy"),
]
for a, b in reps:
    n = s.count(a)
    if n != 1:
        sys.exit(f"count {n}: {a[:70]}")
    s = s.replace(a, b)
open(p, "w", encoding="utf-8", newline="\n").write(s)
print("ok")
