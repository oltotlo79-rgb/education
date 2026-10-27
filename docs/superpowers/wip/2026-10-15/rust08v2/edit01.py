import sys
from pathlib import Path
d = Path(sys.argv[1])
def patch(name, pairs):
    p = d / "parts" / name
    t = p.read_text(encoding="utf-8")
    for old, new in pairs:
        assert t.count(old) == 1, (name, old[:50])
        t = t.replace(old, new)
    p.write_text(t, encoding="utf-8", newline="\n")
patch("unit01.src.md", [(
"- **自分のサイトなのに 403 になるとき:** よくある原因は、ブラウザで開いている URL と、サーバーが受け取る `Host` が違うことです。たとえば `http://localhost:3000` で開いたページから送ると、`Origin` は `http://localhost:3000` になります。RustNote の画面は、案内どおり `http://127.0.0.1:3000` で開いてください。また、別のサイト（たとえば第11巻（準備中）で使う、別のポートの道具）からわざと送る必要があるときは、Topcoat の `OriginPolicy` で、信じてよい送り元を指定できます。守りそのものを外す設定もありますが、使わないでください。",
"- **自分のサイトなのに 403 になるとき:** フォームのページを表示したサイトと、送り先の RustNote が、別のサイトとして扱われている場合です。たとえば、フォームだけを書いた HTML ファイルを別の開発用のサーバー（別のポート）で開いて、そこから RustNote へ送ると、`Origin` と `Host` のポートが違うので止まります。また、RustNote の手前に置いた中継のサーバー（リバースプロキシ）が `Host` を書き換える設定だと、同じサイトからでも食い違います。フォームは RustNote 自身の画面から送り、中継のサーバーは `Host` をそのまま渡す設定にします。ほかのサイトからの送信をわざと受け付けたいときは、Topcoat の `OriginPolicy` で、信じてよい送り元を指定できます。守りそのものを外す設定もありますが、使わないでください。")])
patch("intro.md", [(
"UNIT 01 のエラー3で、型が「他人のデータ」の事故を防ぐ様子を見ます。",
"UNIT 01 のエラー2で、型が「他人のデータ」の事故を防ぐ様子を見ます。")])
print("ok")
