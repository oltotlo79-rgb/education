"""第12巻 UNIT 02 の本文の、結果が出てから書く所を埋める。"""
from pathlib import Path

HERE = Path(__file__).resolve().parent


def edit(name, pairs):
    p = HERE / "parts" / name
    s = p.read_text(encoding="utf-8")
    for a, b in pairs:
        assert s.count(a) == 1, (name, a[:60])
        s = s.replace(a, b)
    p.write_text(s, encoding="utf-8", newline="")


edit("unit02.src.md", [
    ("ソースを1行変えたときの cargo build: @@CACHE2@@ 秒（依存クレートはコンパイルし直さない）",
     "ソースを1行変えたときの cargo build: 14.1 秒（依存クレートはコンパイルし直さない）"),
    ("- 工房とお店の**電気の電圧**（Debian の版と glibc）は、そろえておきます。工房の新しい機械で焼いたパンの作り方を、古い電圧のお店で再現しようとすると、動きません（エラー1）。",
     "- 工房とお店の**電気の方式**（Linux の種類と glibc）は、そろえておきます。少し古い方式のお店でも動くことはありますが、約束はありません。方式がまるで違うお店では、まったく動きません（エラー1）。"),
    ("| 電気の電圧をそろえる | builder と実行用を、同じ Debian 13（trixie）にする |",
     "| 電気の方式をそろえる | builder と実行用を、同じ Debian 13（trixie）にする |"),
    ("""#### エラー1: builder と実行用で、Debian の版がちがう（GLIBC の版が見つからない）

builder の段は trixie（Debian 13）のまま、実行用の段だけを `debian:bookworm-slim`（Debian 12）にしてビルドし、起動しました。

```text
@@OUT:u02_err_glibc_build_result@@
```

```text
@@OUT:u02_err_glibc@@
```

@@GLIBC_NOTE@@""",
     """#### エラー1: 実行用の段を alpine にした（exec … no such file or directory）

小さなイメージとしてよく使われる alpine を、実行用の段に使ってみました（alpine には `useradd` が無いので、利用者を作る行も alpine の書き方に直しました）。

```diff
@@OUT:u02_err_alpine_diff@@
```

```text
@@OUT:u02_err_alpine_build_result@@
```

```text
@@OUT:u02_err_alpine@@
```

- **何が起きたか:** ビルドは通りましたが、起動すると `exec /app/rustnote: no such file or directory`（そのファイルがありません）で止まりました。ところが、ファイルはあります。

```text
@@OUT:u02_err_alpine_ls@@
```

- **読み方:** 実行ファイルは `/app/rustnote` にあります。無いのは、実行ファイルを読み込むための道具（glibc の `ld-linux-aarch64.so.1`）です。alpine は glibc ではなく musl という別の C のライブラリを使うので、glibc 向けに作った RustNote の実行ファイルは動きません。「ファイルが無い」という文は、実行ファイルそのものではなく、それを読み込む道具が無いという意味でした。
- **直し方:** 実行用の段は、builder の段と同じ Debian（`debian:trixie-slim`）にします。どうしても alpine にしたいなら、builder の段も musl 向け（`x86_64-unknown-linux-musl` などのターゲット）で作ります。

比べるために、実行用の段だけを1つ古い Debian 12（`debian:bookworm-slim`。glibc 2.36）にしたイメージも作りました。

```text
@@OUT:u02_err_glibc@@
```

- こちらは、glibc 2.41 の trixie で作った実行ファイルが、glibc 2.36 の上でも、エラー無く動きました。RustNote が、新しい glibc にしか無い機能を使っていなかったからです。ただし、これはたまたまで、Rust や依存クレートの版が上がると、`GLIBC_2.38 not found` のようなエラーで止まることがあります。だから、builder と実行用は、同じ版にそろえておきます（`Dockerfile` の先頭のコメントのとおり）。"""),
    ("@@COPY_NOTE@@",
     """- **何が起きたか:** ビルドの最後の段で、「`/app/target/rustnote` が見つかりません（not found）」と止まりました。builder の段の中に、その場所のファイルが無いからです。
- **読み方:** `failed to calculate checksum of ref …` は、Docker がコピーする物の指紋を計算しようとして、物が無かった、という意味です。止まるのはビルドのときなので、壊れたイメージが公開されることはありません。CI の `docker build` で、毎回確かめられます。
- **直し方:** builder の段で、物がどこにできるかを確かめます（release ビルドは `target/release/`）。"""),
    ("@@NODATA_NOTE@@",
     """- **何が起きたか:** ビルドは通りましたが、起動すると「DB（/data/rustnote.db）を開けませんでした」で止まりました。原因の `14` は、SQLite の「ファイルを開けません（SQLITE_CANTOPEN）」の番号です。
- **読み方:** `/data` が無いので、DB のファイルを作る場所がありません。root なら `/` の下に作れるかもしれませんが、利用者 `app` には書く権限がありません。最小権限にしたので、書ける場所を、こちらで用意する必要があるのです。
- **直し方:** `mkdir -p /data && chown app:app /data` で、`app` が書けるフォルダを用意します。Fly.io で volume を使うときも、同じ `/data` に付けます。本番の RustNote は Turso を使うので、`/data` は使いません（UNIT 05・09）。"""),
    ("@@CACHE_NOTE@@",
     """- 1回目（キャッシュが空）は、`cargo build` の段が 95.0 秒（依存クレートを含めて全部）でした。ソースを1行変えた2回目は **14.1 秒**で、コンパイルしたのは RustNote の1つだけ（`Compiling` の行が1行）でした。ビルド全体も 100 秒 → 24 秒です。
- できたイメージは 170MB で、`/health` も 200 でした。キャッシュのマウントは、イメージの中身を変えずに、作る時間だけを短くします。
- ただし、キャッシュのマウントは、ビルドする PC（や CI の機械）の中に残る物です。毎回まっさらな機械でビルドする CI では、効きません。その場合は、依存クレートだけを先にビルドする段を分ける（`cargo-chef` などの道具を使う）方法があります。"""),
    ("- builder と実行用の Debian の版はそろえる（ずれると GLIBC の版のエラー）。`COPY --from` の場所の書き間違いはビルドで止まり、`/data` を作り忘れると起動で止まる。",
     "- builder と実行用の Linux はそろえる。alpine（musl）では `exec … no such file or directory` で動かない。1つ古い Debian では今は動いたが、約束は無い。`COPY --from` の場所の書き間違いはビルドで止まり、`/data` を作り忘れると起動で止まる（SQLite の 14）。"),
])

edit("unit02.meta.md", [
    ("- ☐ Debian の版のずれ（GLIBC）・COPY --from の場所・/data の持ち主のまちがいが、それぞれどこで、どう止まるかを言える",
     "- ☐ 実行用の段の Linux のちがい（alpine で no such file or directory）・COPY --from の場所・/data の持ち主のまちがいが、それぞれどこで、どう止まるかを言える"),
])
print("ok")
