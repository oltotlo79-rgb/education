"""第5巻を、Rust 1.99.1・tokio 1.54.1・rust-version "1.99"・Topcoat 0.11 にそろえる。
出力は rust04v/out の取り直したもの（cap0507v.sh）と、比べ合わせ（cmp_labs.py）で確かめた tokio 1.54.1 のエラーを使う。
"""
import glob
import re
from pathlib import Path

H = Path(__file__).resolve().parent
O = H / "out"
BS = chr(92)
p = Path(glob.glob("C:/Users/oltot/Documents/git-projects/def/Rust/05_*.md")[0])
s = p.read_text(encoding="utf-8")


def rep(a, b, n=1):
    global s
    c = s.count(a)
    assert c == n, (a[:60], c)
    s = s.replace(a, b)


def out(name):
    return (O / f"{name}.txt").read_text(encoding="utf-8").rstrip("\n")


def replace_text_block_after(command_line, new_body, nth_block=1):
    """command_line を含む powershell の枠のあとの、nth_block 番目の text の枠の中身を差し替える"""
    global s
    i = s.index(command_line)
    j = i
    for _ in range(nth_block):
        j = s.index("```text\n", j) + len("```text\n")
    k = s.index("\n```", j)
    s = s[:j] + new_body + s[k:]


def upto_locking(text):
    lines = text.splitlines()
    for idx, line in enumerate(lines):
        if "Locking" in line:
            return "\n".join(lines[: idx + 1])
    return text


def split_two(text, second_marker):
    i = text.index(second_marker)
    # second_marker の前の "    Updating crates.io index" から2つ目とする
    j = text.rfind("    Updating crates.io index", 0, i)
    return text[:j].rstrip("\n"), text[j:]


# ---- 版の表
rep("| Rust | **1.98 以上**（本書シリーズの基準）。動作確認は rustc 1.99.0 / cargo 1.99.0（`aarch64-pc-windows-msvc`） |",
    "| Rust | **1.99 以上**（本書シリーズの基準）。実行結果は rustc 1.99.0・tokio 1.53.3 で取り、2026-10-31 に rustc 1.99.1・tokio 1.54.1 で全部の例を動かし直して比べた（違いが出た表示は取り直した。時間・スレッドの順番など毎回変わる物は、そのまま） |")
rep("| tokio | 1.53.3（`full` の機能つき。この巻の全単元で使う） |", "| tokio | 1.54.1（`full` の機能つき。この巻の全単元で使う） |")

# ---- 練習用プロジェクトの用意
rep("cargo add tokio@=1.53.3 --features full\n", "cargo add tokio@=1.54.1 --features full\n")
rep("      Adding tokio =1.53.3 to dependencies", "      Adding tokio =1.54.1 to dependencies")
rep("     Locking 24 packages to highest Rust 1.99.0 compatible versions\n      Adding tokio v1.53.3 (available: v1.54.0)",
    "     Locking 24 packages to highest Rust 1.99.1 compatible versions")
old_bullet = re.search(r"- `tokio@=1\.53\.3` の `=` は「ちょうど 1\.53\.3」という意味です（第4巻 UNIT 07）。[^\n]*\n", s).group(0)
rep(old_bullet, "- `tokio@=1.54.1` の `=` は「ちょうど 1.54.1」という意味です（第4巻 UNIT 07）。`=` を付けずに `tokio@1.54.1` と書くと、Cargo は 1.x の中でいちばん新しい版を選ぶので、新しい版が出ると、それが入ります。この巻の出力と同じ結果になるように、版を固定しておきます（2026-10-31 の時点で、1.54.1 がいちばん新しい版でした）。\n")
rep("`edition = \"2024\"` の次の行に `rust-version = \"1.98\"` を手で1行足します。", "`edition = \"2024\"` の次の行に `rust-version = \"1.99\"` を手で1行足します。")
rep('rust-version = "1.98"\n\n[dependencies]\ntokio = { version = "=1.53.3", features = ["full"] }', 'rust-version = "1.99"\n\n[dependencies]\ntokio = { version = "=1.54.1", features = ["full"] }')
rep("（この PC では約12秒でした）", "（この PC では約9秒でした）")
rep("   Compiling tokio v1.53.3\n   Compiling async_lab v0.1.0 (..." + BS + "async_lab)\n    Finished `dev` profile [unoptimized + debuginfo] target(s) in 11.89s",
    "   Compiling tokio v1.54.1\n   Compiling async_lab v0.1.0 (..." + BS + "async_lab)\n    Finished `dev` profile [unoptimized + debuginfo] target(s) in 9.15s")

# ---- tokio の中の場所（1.53.3 と 1.54.1 で、行の番号は同じだった）
rep(BS + "tokio-1.53.3" + BS + "src" + BS + "task" + BS + "spawn.rs:176:21", BS + "tokio-1.54.1" + BS + "src" + BS + "task" + BS + "spawn.rs:176:21", 3)
rep(BS + "tokio-1.53.3" + BS + "src" + BS + "task" + BS + "join_set.rs:144:19", BS + "tokio-1.54.1" + BS + "src" + BS + "task" + BS + "join_set.rs:144:19")
# ---- エラー3（block_on）: tokio 1.54.1 では、場所の行と、メッセージの最後の一文が変わった
rep("thread 'main' (22388) panicked at ..." + BS + "tokio-1.53.3" + BS + "src" + BS + "runtime" + BS + "scheduler" + BS + "multi_thread" + BS + "mod.rs:91:9:\n"
    "Cannot start a runtime from within a runtime. This happens because a function (like `block_on`) attempted to block the current thread while the thread is being used to drive asynchronous tasks.\n",
    "thread 'main' (47444) panicked at ..." + BS + "tokio-1.54.1" + BS + "src" + BS + "runtime" + BS + "scheduler" + BS + "multi_thread" + BS + "mod.rs:89:9:\n"
    "Cannot start a runtime from within a runtime. This happens because a function (like `block_on`) attempted to block the current thread while the thread is being used to drive asynchronous tasks. If you are in an async function, use `.await` on the future instead of blocking on it.\n")
rep("- **何が起きたか:** 「ランタイムの中で、ランタイムを始めることはできません。`block_on` のような関数が、async のタスクを動かしているスレッドを止めて待とうとしました」という panic です。",
    "- **何が起きたか:** 「ランタイムの中で、ランタイムを始めることはできません。`block_on` のような関数が、async のタスクを動かしているスレッドを止めて待とうとしました。async の関数の中なら、止めて待つのではなく、Future に `.await` を使ってください」という panic です（最後の一文は、tokio 1.54 で足されました）。")

# ---- tokio のドキュメント・ソースの話
rep("（tokio 1.53.3 のドキュメントに「公平（書き込み優先）」とあります）", "（tokio 1.54.1 のドキュメントに「公平（書き込み優先）」とあります）")
rep("tokio 1.53.3 のソース（`runtime/builder.rs`）でも、`max_blocking_threads` の既定は 512 です。", "tokio 1.54.1 のソース（`runtime/builder.rs`）でも、`max_blocking_threads` の既定は 512 です。")

# ---- 後の単元の cargo add（取り直した出力に差し替え）
replace_text_block_after("cargo add tokio-util@=0.7.20 --features rt", out("v05_add_tokio_util"))
rep("- `Locking 7 packages to highest Rust 1.98 compatible versions` は、`Cargo.toml` の `rust-version = \"1.98\"` に合う版の中から選んだ、という意味です。",
    "- `Locking 7 packages to highest Rust 1.99 compatible versions` は、`Cargo.toml` の `rust-version = \"1.99\"` に合う版の中から選んだ、という意味です。")
replace_text_block_after("cargo add argon2@=0.6.0", out("v05_add_argon2"))
reqwest, serde = split_two(out("v05_add_reqwest_serde"), "      Adding serde =1.0.229")
n_req = re.search(r"Locking (\d+) packages", reqwest).group(1)
replace_text_block_after("cargo add reqwest@=0.13.5 --features json", upto_locking(reqwest), 1)
replace_text_block_after("cargo add reqwest@=0.13.5 --features json", serde.rstrip("\n"), 2)
rep("（`Locking 147 packages` の後ろに、`Adding …` の行が 147 行続きますが、省いています）", f"（`Locking {n_req} packages` の後ろに、`Adding …` の行が {n_req} 行続きますが、省いています）")
rep("147 個ものクレートが入るのは", f"{n_req} 個ものクレートが入るのは")
tracing1, tracing2 = split_two(out("v05_add_tracing"), "      Adding tracing-subscriber =0.3.23")
replace_text_block_after("cargo add tracing@=0.1.44", tracing1, 1)
replace_text_block_after("cargo add tracing@=0.1.44", tracing2.rstrip("\n"), 2)
replace_text_block_after("cargo add libsql@=0.10.0", upto_locking(out("v05_add_libsql")))

# ---- 第5巻の後半の Cargo.toml（UNIT 08）
rep('tokio = { version = "=1.53.3", features = ["full"] }', 'tokio = { version = "=1.54.1", features = ["full"] }')

# ---- Topcoat の中（0.10.0 と 0.11.0 で、抜き出した部分のコードは同じだった）
rep("Topcoat 0.10.0 のソース（`topcoat-router-0.10.0/src/service.rs` の `internal_serve`）", "Topcoat 0.11.0 のソース（`topcoat-router-0.11.0/src/service.rs` の `internal_serve`）")
rep("`internal_serve`（`topcoat-router-0.10.0/src/service.rs`）では", "`internal_serve`（`topcoat-router-0.11.0/src/service.rs`）では")
rep("Topcoat 0.10.0 のソースから、channel を作る部分", "Topcoat 0.11.0 のソースから、channel を作る部分")
rep(">Topcoat 0.10 の中（RustNote）</text>", ">Topcoat 0.11 の中（RustNote）</text>")
rep("Topcoat 0.10.0 のソースで、そのことを確かめます。", "Topcoat 0.11.0 のソースで、そのことを確かめます（0.10.0 から 0.11.0 で、ここに抜き出した部分のコードは変わっていないことを、2つの版のソースを比べて確かめました）。")
rep("まず `topcoat-0.10.0/src/serve.rs` です。", "まず `topcoat-0.11.0/src/serve.rs` です。")
rep("（`topcoat-router-0.10.0/src/service.rs` の `internal_serve` の後半。", "（`topcoat-router-0.11.0/src/service.rs` の `internal_serve` の後半。")

leftover = [m.group(0) for m in re.finditer(r"1\.53\.3|1\.98|Topcoat 0\.10|topcoat-0\.10|router-0\.10", s)]
p.write_text(s, encoding="utf-8", newline="\n")
print("ok; leftover:", leftover)
