"""第7巻を、Rust 1.99.1・tokio 1.54.1・rust-version "1.99" にそろえる（出力は rust04v/out の取り直したもの）。"""
import glob
import re
from pathlib import Path

H = Path(__file__).resolve().parent
O = H / "out"
p = Path(glob.glob("C:/Users/oltot/Documents/git-projects/def/Rust/07_*.md")[0])
s = p.read_text(encoding="utf-8")


def rep(a, b, n=1):
    global s
    c = s.count(a)
    assert c == n, (a[:60], c)
    s = s.replace(a, b)


def out(name):
    return (O / f"{name}.txt").read_text(encoding="utf-8").rstrip("\n")


rep("| Rust | **1.98 以上**（本書シリーズの基準）。動作確認は rustc 1.99.0 / cargo 1.99.0（`aarch64-pc-windows-msvc`） |",
    "| Rust | **1.99 以上**（本書シリーズの基準）。実行結果は rustc 1.99.0・tokio 1.53.3 で取り、2026-10-31 に rustc 1.99.1・tokio 1.54.1 で全部の例を動かし直して比べた（違いが出た表示は取り直した。時間など毎回変わる物は、そのまま） |")
rep("| tokio | 1.53.3（`rt-multi-thread` と `macros` の機能つき。`.await` を動かす土台） |", "| tokio | 1.54.1（`rt-multi-thread` と `macros` の機能つき。`.await` を動かす土台） |")
rep("cargo add tokio@1.53.3 --features rt-multi-thread,macros\n", "cargo add tokio@1.54.1 --features rt-multi-thread,macros\n")

# cargo new と cargo add の出力（1つの枠）を差し替える
i = s.index("cargo add thiserror@2.0.21\n")
j = s.index("```text\n", i) + len("```text\n")
k = s.index("\n```", j)
s = s[:j] + out("v07_new") + "\n" + out("v07_add") + s[k:]

rep("`rust-version = \"1.98\"` を手で1行足します（第4巻 UNIT 07 で学んだ「このプロジェクトは Rust 1.98 以上でビルドしてください」という宣言です）。",
    "`rust-version = \"1.99\"` を手で1行足します（第4巻 UNIT 07 で学んだ「このプロジェクトは Rust 1.99 以上でビルドしてください」という宣言です）。")
rep('rust-version = "1.98"\n\n[dependencies]\nlibsql = "0.10.0"\nthiserror = "2.0.21"\ntokio = { version = "1.53.3", features = ["rt-multi-thread", "macros"] }',
    'rust-version = "1.99"\n\n[dependencies]\nlibsql = "0.10.0"\nthiserror = "2.0.21"\ntokio = { version = "1.54.1", features = ["rt-multi-thread", "macros"] }')

# async-trait を足したときの出力
i = s.index("cargo add async-trait@0.1.92\n")
j = s.index("```text\n", i) + len("```text\n")
k = s.index("\n```", j)
s = s[:j] + out("v07_add_async_trait") + s[k:]

leftover = [m.group(0) for m in re.finditer(r"1\.53\.3|1\.98|Topcoat 0\.10", s)]
p.write_text(s, encoding="utf-8", newline="\n")
print("ok; leftover:", leftover)
