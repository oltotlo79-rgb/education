"""第12巻の巻頭と UNIT 01 を、Rust 1.99.1・topcoat 0.11.0・trixie で取り直した結果に合わせる。"""
from pathlib import Path

HERE = Path(__file__).resolve().parent


def edit(name, pairs):
    p = HERE / "parts" / name
    s = p.read_text(encoding="utf-8")
    for a, b in pairs:
        assert s.count(a) == 1, (name, a[:60])
        s = s.replace(a, b)
    p.write_text(s, encoding="utf-8", newline="")


edit("intro.md", [
    ("> **注意:** 第11巻は、書き直しの途中です。この巻の本文は、教材のフォルダの `Rust/rustnote/` のコードと、この巻で実際に動かした結果だけをもとに書いているので、ほかの巻が未完成でも読み進められます。",
     "> **注意:** この巻の本文は、教材のフォルダの `Rust/rustnote/` のコードと、この巻で実際に動かした結果だけをもとに書いています。RustNote の改善点（第11巻・第13巻で見つけたもの）は、教材のコードでは直さずにそのまま使い、関係する所で、その都度ふれます。"),
    ("| Rust | **1.98 以上**（本書シリーズの基準）。動作確認は rustc 1.99.0 / cargo 1.99.0（`aarch64-pc-windows-msvc`） |",
     "| Rust | **1.99 以上**（Topcoat 0.11 の必要条件）。動作確認は rustc 1.99.1 / cargo 1.99.1（`aarch64-pc-windows-msvc`） |"),
    ("| topcoat-cli | 0.10.0（`topcoat asset bundle` で CSS の束を作る。RustNote の topcoat と同じ版） |",
     "| topcoat-cli | 0.11.0（`topcoat asset bundle` で CSS の束を作る。RustNote の topcoat と同じ版） |"),
    ("| Docker | 29.3.1（Docker Desktop。UNIT 02 から使います） |",
     "| Docker | 29.3.1（Docker Desktop。中は linux/arm64。UNIT 02 から使います。イメージは `rust:1.99.1-trixie` と `debian:trixie-slim`） |"),
    ("| flyctl | v0.4.57（Fly.io のコマンド。この PC では `windows/amd64` 版が動きます。UNIT 04 から使います） |",
     "| flyctl | v0.5.1（Fly.io のコマンド。Windows 用の公式の配布は x86_64 版だけで、この PC（ARM64）でも `windows/amd64` 版が動きます。UNIT 04 から使います。アカウントへのログインはしません） |"),
    ("# このフォルダで使われる Rust の版を確かめる（1.99.0 になる）", "# このフォルダで使われる Rust の版を確かめる（1.99.1 になる）"),
    ("cargo install topcoat-cli --version 0.10.0 --locked", "cargo install topcoat-cli --version 0.11.0 --locked"),
    ("```text\ntopcoat 0.10.0\n```", "```text\ntopcoat 0.11.0\n```"),
])

edit("unit01.src.md", [
    ("21,698,048 バイト（約 21.7 MB）", "21,751,808 バイト（約 21.8 MB）"),
    ("11,820,032 バイト（約 11.8 MB）", "11,732,480 バイト（約 11.7 MB）"),
    ("<text x=\"315\" y=\"190\" font-size=\"12\" fill=\"#0f172a\" text-anchor=\"middle\">約 380〜470 ms</text>",
     "<text x=\"315\" y=\"190\" font-size=\"12\" fill=\"#0f172a\" text-anchor=\"middle\">約 400 ms</text>"),
    ("- **② アセットの束**は、`topcoat asset bundle --release` で作ります。topcoat CLI は、①でできた実行ファイルの中から `asset!(\"assets/style.css\")` の印を探し、CSS を `style-e9b983efabcfc6c3.css` のような名前（中身から計算した指紋付き）にして、実行ファイルの隣の `assets\\` に置きます。だから、①が先です（くわしくは UNIT 03（準備中））。",
     "- **② アセットの束**は、`topcoat asset bundle --release` で作ります。topcoat CLI は、中で `cargo build --release` を動かして実行ファイルを用意し（①で作ってあれば、そのまま使います）、その中から `asset!(\"assets/style.css\")` の印を探して、CSS を `style-e9b983efabcfc6c3.css` のような名前（中身から計算した指紋付き）にして、実行ファイルの隣の `assets\\` に置きます。topcoat CLI の中の `cargo build` には `--locked` が付かないので、先に①で、`Cargo.lock` のとおりの版で作っておきます（くわしくは UNIT 03（準備中））。"),
    ("この PC では、何もない状態からの release ビルドに 1 分 25 秒かかりました。debug ビルドは 1 分 2 秒でした。",
     "この PC では、何もない状態からの release ビルドに 1 分 26 秒かかりました。debug ビルドは 1 分 6 秒でした。"),
    ("- ハッシュ化は、debug で約 380〜470 ms、release で約 20 ms でした。",
     "- ハッシュ化は、debug で約 400 ms、release で約 20 ms でした。"),
    ("- debug の1回目のハッシュ化（473.06 ms）は、2回目以降より少し遅くなりました。最初の1回は、計算に使うメモリを用意したり、`spawn_blocking` 用のスレッドを作ったりする分が加わるからだと考えられます。速さを比べるときは、1回だけで決めず、何回か測ります。",
     "- debug の1回目の照合（445.38 ms）は、2回目以降（410 ms ほど）より少し遅くなりました。最初の1回は、計算に使うメモリを用意したり、`spawn_blocking` 用のスレッドを作ったりする分が加わることがあるからだと考えられます。ただし、ハッシュ化のほうは、1回目と2回目以降でほとんど差がありませんでした。速さを比べるときは、1回だけで決めず、何回か測ります。"),
    ("- 違いは、**リンクした時刻**でした。1793046265 と 1793046676 の差は 411 秒で、この PC で2回のビルドの間にたった時間です。",
     "- 違いは、**リンクした時刻**でした。1793398680 と 1793398806 の差は 126 秒で、この PC で2回のビルドの間にたった時間です。"),
    ("- 違うバイトの数が 0 になりました。「リンクした時刻」は、本当の時刻ではなく、中身から計算した値（277490482）になっています。",
     "- 違うバイトの数が 0 になりました。「リンクした時刻」は、本当の時刻ではなく、中身から計算した値（2520511643。日付にすると 2049 年になる、ありえない値）になっています。"),
    ("- 「ユーザーフォルダの場所が入っている数」は、この PC では 1,820 か所でした。", "- 「ユーザーフォルダの場所が入っている数」は、この PC では 1,811 か所でした。"),
    ("- 1,820 か所が 1 か所に減りました。", "- 1,811 か所が 1 か所に減りました。"),
    ("（topcoat-asset 0.10.0 の `asset!` のソースで確かめました）", "（topcoat-asset 0.11.0 の `asset!` のソースで確かめました）"),
    ("RustNote の `Dockerfile` が、`rust:1.99.0-bookworm` のイメージの中の `/app` フォルダでビルドしているのは、このためです（UNIT 02（準備中））。",
     "RustNote の `Dockerfile` が、`rust:1.99.1-trixie` のイメージの中の `/app` フォルダでビルドしているのは、このためです（UNIT 02（準備中））。"),
    ("<text x=\"120\" y=\"158\" font-size=\"20\" font-weight=\"800\" fill=\"#c2410c\" text-anchor=\"middle\">1,820 か所</text>",
     "<text x=\"120\" y=\"158\" font-size=\"20\" font-weight=\"800\" fill=\"#c2410c\" text-anchor=\"middle\">1,811 か所</text>"),
    ("- **気をつけたい点:** 1行目に `RustNote を起動しました: http://127.0.0.1:3000` と出ています。ところが、このプログラムは起動できずに止まりました。RustNote の `main.rs` は、ポートを押さえる（`topcoat::start`）**前に**「起動しました」を表示しているからです。ログを見る人が「起動した」と勘違いしないよう、ポートを押さえたあとに表示するのが本来の形です。これも、RustNote に残っている改善点です（コードは変えていません）。",
     "- **気をつけたい点:** 1行目に `RustNote を起動しました: http://127.0.0.1:3000` と出ています。ところが、このプログラムは起動できずに止まりました。RustNote の `main.rs` は、ポートを押さえる（`topcoat::start`）**前に**「起動しました」を表示しているからです。ログを見る人が「起動した」と勘違いしないよう、ポートを押さえたあとに表示するのが本来の形です。これも、RustNote に残っている改善点です（教材のコードは変えていません。直し方は、第13巻の総合演習2で確かめています）。"),
    ("""```dockerfile
FROM rust:1.99.0-bookworm AS builder
WORKDIR /app

# アセットの束を作る topcoat CLI（アプリの Topcoat と同じ 0.10.0）
RUN cargo install topcoat-cli --version 0.10.0 --locked
""", """```dockerfile
# builder と実行用は、同じ Debian 13（trixie）にそろえる（実行ファイルが使う glibc の版を合わせるため）
FROM rust:1.99.1-trixie AS builder
WORKDIR /app

# アセットの束を作る topcoat CLI（アプリの Topcoat と同じ 0.11.0）
RUN cargo install topcoat-cli --version 0.11.0 --locked
"""),
    ("- Rust の版（1.99.0）と、作るフォルダ（`/app`）が、いつも同じです。", "- Rust の版（1.99.1）と、作るフォルダ（`/app`）が、いつも同じです。"),
    ("`rustc --version` は、このフォルダで使われる Rust の版（`rust-toolchain.toml` の 1.99.0）です。", "`rustc --version` は、このフォルダで使われる Rust の版（`rust-toolchain.toml` の 1.99.1）です。"),
    ("この例では、例1-3で調べた「リンクした時刻」（1793046676 秒）と同じ、2026-10-26 20:31:16（UTC）になりました。",
     "この例では、例1-3で調べた2回目のビルドの「リンクした時刻」（1793398806 秒）と同じ、2026-10-30 22:20:06（UTC）になりました。"),
])
print("ok")
