import glob

p = glob.glob("C:/Users/oltot/Documents/git-projects/def/Rust/04_*.md")[0]
s = open(p, encoding="utf-8").read()
OLD = "b940084d7eb6a299eb4bfeb8e34901bc051e7ac4"
NEW = "58cf8f9c35b355199727627d07ae34a4e2846d67"
BS = chr(92)
pairs = [
    ("| Rust | **1.98 以上**（本書シリーズの基準）。動作確認は rustc 1.99.0 / cargo 1.99.0（`aarch64-pc-windows-msvc`） |",
     "| Rust | **1.99 以上**（本書シリーズの基準）。実行結果は rustc 1.99.0（`aarch64-pc-windows-msvc`）で取り、2026-10-31 に 1.99.1 でも同じ表示になることを確かめた（版の番号が出る出力は 1.99.1 で取り直し、エラーの中の rustc のソースの場所の番号は 1.99.1 のものにしている） |", 1),
    ("1.98 より古ければ、`rustup update` で", "1.99 より古ければ、`rustup update` で", 1),
    ("`rust-version = \"1.98\"` を手で1行足します（`cargo new` では入りません）。「このプロジェクトは Rust 1.98 以上で",
     "`rust-version = \"1.99\"` を手で1行足します（`cargo new` では入りません）。「このプロジェクトは Rust 1.99 以上で", 1),
    ('rust-version = "1.98"\n', 'rust-version = "1.99"\n', 3),
    ("     Locking 14 packages to highest Rust 1.99.0 compatible versions", "     Locking 14 packages to highest Rust 1.99.1 compatible versions", 1),
    ("     Locking 6 packages to highest Rust 1.98 compatible versions", "     Locking 6 packages to highest Rust 1.99 compatible versions", 1),
    ("target(s) in 7.50s\n     Running `target" + BS + "debug" + BS + "app.exe`", "target(s) in 5.14s\n     Running `target" + BS + "debug" + BS + "app.exe`", 1),
    ("`Locking 6 packages to highest Rust 1.98 compatible versions`（Rust 1.98 で使える", "`Locking 6 packages to highest Rust 1.99 compatible versions`（Rust 1.99 で使える", 1),
    ("UNIT 06 の workspace で `Locking 6 packages to highest Rust 1.98 compatible versions`", "UNIT 06 の workspace で `Locking 6 packages to highest Rust 1.99 compatible versions`", 1),
    ("     Locking 11 packages to highest Rust 1.99.0 compatible versions", "     Locking 11 packages to highest Rust 1.99.1 compatible versions", 1),
    ("本書では `Cargo.toml` の `[package]` に `rust-version = \"1.98\"` を書きます。これは「この package をビルドするには Rust 1.98 以上が必要」",
     "本書では `Cargo.toml` の `[package]` に `rust-version = \"1.99\"` を書きます。これは「この package をビルドするには Rust 1.99 以上が必要」", 1),
    ("> **注意:** 本書のシリーズは Rust 1.98 以上を基準にしています（RustNote で使う Topcoat 0.10 が Rust 1.98 以上を必要とするため）。",
     "> **注意:** 本書のシリーズは Rust 1.99 以上を基準にしています（RustNote で使う Topcoat 0.11 が Rust 1.99 以上を必要とするため）。", 1),
    ("`rust-version = \"1.98\"` の package を、Rust 1.90 でビルド", "`rust-version = \"1.99\"` の package を、Rust 1.90 でビルド", 1),
    ("  feat_lab@0.1.0 requires rustc 1.98", "  feat_lab@0.1.0 requires rustc 1.99", 1),
    ("feat_lab 0.1.0 は rustc 1.98 が必要", "feat_lab 0.1.0 は rustc 1.99 が必要", 1),
    ("- RustNote の `Cargo.toml` には `rust-version = \"1.98\"` を書き、Topcoat は `=0.10.0` で版を固定します。",
     "- RustNote の `Cargo.toml` には `rust-version = \"1.99\"` を書き、Topcoat は `=0.11.0` で版を固定します。", 1),
    ("（`\"1.0.229\"`・`\"=0.10.0\"`・`\"~1.2.3\"`・`\"0.10\"`）", "（`\"1.0.229\"`・`\"=0.11.0\"`・`\"~1.2.3\"`・`\"0.10\"`）", 1),
    ("| `\"=0.10.0\"` | 0.10.0 ちょうど | 変化の激しい crate を固定したいとき（本書では Topcoat を `=0.10.0` に固定） |",
     "| `\"=0.11.0\"` | 0.11.0 ちょうど | 変化の激しい crate を固定したいとき（本書では Topcoat を `=0.11.0` に固定） |", 1),
    ("- (b) `topcoat = \"=0.10.0\"`", "- (b) `topcoat = \"=0.11.0\"`", 1),
    ("| (b) `\"=0.10.0\"` | 0.10.0 ちょうど |", "| (b) `\"=0.11.0\"` | 0.11.0 ちょうど |", 1),
    ("`\"=0.10.0\"` は「ちょうど」。", "`\"=0.11.0\"` は「ちょうど」。", 1),
    ("版の指定（`\"1.0.229\"`・`\"=0.10.0\"`）", "版の指定（`\"1.0.229\"`・`\"=0.11.0\"`）", 1),
    ("`rust-version = \"1.98\"` を書ける（UNIT 07）", "`rust-version = \"1.99\"` を書ける（UNIT 07）", 1),
    ("（本書は `\"1.98\"`）", "（本書は `\"1.99\"`）", 1),
    (OLD, NEW, 3),
]
for a, b, n in pairs:
    c = s.count(a)
    assert c == n, (a[:50], c)
    s = s.replace(a, b)
open(p, "w", encoding="utf-8", newline="\n").write(s)
print("ok")
