import sys
from pathlib import Path

HERE = Path(sys.argv[1])


def patch(name, pairs):
    p = HERE / "parts" / name
    t = p.read_text(encoding="utf-8")
    for old, new in pairs:
        assert t.count(old) == 1, (name, old[:60])
        t = t.replace(old, new)
    p.write_text(t, encoding="utf-8", newline="\n")


patch(
    "unit02.src.md",
    [
        (
            "ページの処理に変える**属性マクロ**です（第4巻）。",
            "ページの処理に変える**属性マクロ**です。",
        )
    ],
)
patch(
    "intro.md",
    [
        (
            "| UNIT 02 | Topcoat 0.10 の最小のアプリ（準備中） |",
            "| UNIT 02 | Topcoat 0.10 の最小のアプリ |",
        ),
        (
            "> **お知らせ:** この版には UNIT 01 を収録しています。UNIT 02〜10 は準備中で、",
            "> **お知らせ:** この版には UNIT 01〜02 を収録しています。UNIT 03〜10 は準備中で、",
        ),
        (
            "| topcoat | 0.10.0（UNIT 02 から使います。UNIT 01 では使いません） |",
            "| topcoat | 0.10.0（UNIT 02 から使います。UNIT 01 では使いません。2026-10 には 0.11.0 も出ていますが、この本は 0.10.0 に固定しています） |",
        ),
    ],
)

unit02_check = """
**UNIT 02: Topcoat 0.10 の最小のアプリ**

- ☐ `cargo add topcoat@=0.10.0` と Tokio で、ページが1つのアプリを作って起動できる。`=` で版を固定する理由を言える
- ☐ `#[page("/")]`・`Router::builder().discover().build()`・`topcoat::start(router).await` の役割を説明できる
- ☐ 404・405 と `allow`・HEAD・`Content-Length`・圧縮を、Topcoat が代わりにしてくれることを、`curl.exe` で確かめられる
- ☐ `HOST` と `PORT` の環境変数で待ち受ける場所を変えられる。`PORT` が数でないときのエラーを読める
- ☐ `topcoat::start` が何も表示しないことと、`TcpListener::bind` で押さえてから表示して `topcoat::serve` に渡す書き方を説明できる
- ☐ `.await` の書き忘れ（E0599）、`RouterBuilderDiscoverExt` の `use` の書き忘れ（E0599）、実行ファイルが2つあるときの `--bin` のエラーを読める
"""

patch(
    "outro.md",
    [
        ("この版に収録した UNIT 01 で、", "この版に収録した UNIT 01〜02 で、"),
        (
            "- ☐ curl の `(7)`（たどり着けない）と、404（たどり着いたが、ページが無い）を見分けられる\n",
            "- ☐ curl の `(7)`（たどり着けない）と、404（たどり着いたが、ページが無い）を見分けられる\n" + unit02_check,
        ),
        (
            "> **お知らせ:** この版には UNIT 01 を収録しています。UNIT 02（Topcoat 0.10 の最小のアプリ）・UNIT 03（",
            "> **お知らせ:** この版には UNIT 01〜02 を収録しています。UNIT 03（",
        ),
        ("この巻（UNIT 01）に出てきた用語です。", "この巻（UNIT 01〜02）に出てきた用語です。"),
    ],
)

p = HERE / "parts" / "outro.md"
t = p.read_text(encoding="utf-8").rstrip("\n")
t += """

### UNIT 02 の用語

| 用語 | 意味 |
|---|---|
| Web フレームワーク | 要求を読み、URL ごとの処理に振り分け、応答を返す、といった決まった仕事を引き受けてくれる道具 |
| ルーター | 要求の URL とメソッドを見て、どの関数に処理させるかを決める仕組み |
| `#[page("/")]` | この関数は `GET /`（と HEAD）に答える、という Topcoat の属性マクロ |
| `discover()` | プログラムの中の `#[page]` の関数を、全部自動で集める（`RouterBuilderDiscoverExt` を use して使う） |
| `topcoat::start` | 環境変数 `HOST`・`PORT`（無ければ 127.0.0.1:3000）で待ち受けを始める。起動しても何も表示しない |
| `topcoat::serve` | 自分で押さえた待ち受けの場所（`TcpListener` など）とルーターを受け取って、要求をさばく |
| `view!` | HTML をほぼそのまま書いて、ページの中身を作るマクロ |
| `vary` | この応答が、要求のどのヘッダーによって変わるかを伝えるヘッダー |
| `content-encoding: gzip` | 本文を gzip で圧縮してある、という印 |
| `default-run` | 実行ファイルが2つ以上あるとき、`cargo run` で動かすものを決める `Cargo.toml` の設定 |
"""
p.write_text(t + "\n", encoding="utf-8", newline="\n")
print("ok")
