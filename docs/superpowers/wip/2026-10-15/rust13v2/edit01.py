import sys
p = sys.argv[1]
t = open(p, encoding="utf-8").read()
def rep(old, new):
    global t
    assert t.count(old) == 1, old[:60]
    t = t.replace(old, new)
rep("`cargo tree -e features -i tokio` で調べると、たとえば `libsql` が tokio の `full`（ほぼ全部の機能）を、`topcoat` が `macros` を有効にしていることが分かります。",
    "`cargo tree -e features -i tokio` で調べると、libsql が使う `libsql_replication` が tokio の `full`（ほぼ全部の機能）を、`topcoat` が `macros`・`net`・`signal` を有効にしていることが分かります（それぞれのクレートの `Cargo.toml` にも、そう書いてあります）。")
rep("同じ名前が2回出るのは、ライブラリと実行ファイルのように、パッケージの中の作る物（ターゲット）ごとに調べているからです。",
    "同じ名前が何回か出ていますが、読むのは、パッケージの名前と `requires rustc 1.98` の部分だけで十分です。")
rep("- topcoat の版（0.10.0）も、`Cargo.toml` のクレートと、`Dockerfile` の CLI の2か所です。クレートと CLI の版をそろえることは、Topcoat の README でも求められています（第1巻）。",
    "- topcoat の版（0.10.0）も、`Cargo.toml` のクレートと、`Dockerfile` の CLI の2か所です。クレートと CLI は同じ Topcoat の一部なので、版を上げるときは両方を一緒に上げます。")
rep("@@OUTTAILSKIP:u01_err_msrv:2@@", "@@OUTTAIL:u01_err_msrv:2@@")
rep("@@TEXT:feat_demo_wrong_toml@@", "@@FILE:snippets/feat_demo_wrong.toml@@")
rep("@@TEXT:q1_snippet@@", "@@FILE:snippets/q1_snippet.rs@@")
open(p, "w", encoding="utf-8", newline="").write(t)
