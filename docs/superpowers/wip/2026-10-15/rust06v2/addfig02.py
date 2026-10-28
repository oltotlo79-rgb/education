import sys

p = sys.argv[1]
t = open(p, encoding="utf-8").read()
anchor = "> **▼ このコードがやること（先に日本語で）:** Tokio の `TcpListener::bind` で `127.0.0.1:3000` を押さえます。"
fig = """<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0202" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="180" y="22" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">start の前に表示する（困る形）</text>
<rect x="20" y="36" width="320" height="40" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="180" y="61" font-size="12" fill="#0f172a" text-anchor="middle">① 「起動しました」と表示</text>
<rect x="20" y="96" width="320" height="56" rx="6" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="180" y="118" font-size="12" fill="#0f172a" text-anchor="middle">② topcoat::start の中で</text>
<text x="180" y="138" font-size="11" fill="#475569" text-anchor="middle">bind（押さえる）→ serve（さばく）</text>
<rect x="20" y="172" width="320" height="56" rx="6" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="180" y="194" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">bind に失敗しても、①はもう出ている</text>
<text x="180" y="214" font-size="11" fill="#475569" text-anchor="middle">表示と本当の状態が食い違う</text>
<line x1="180" y1="76" x2="180" y2="92" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0202)"/>
<line x1="180" y1="152" x2="180" y2="168" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0202)"/>
<text x="540" y="22" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">押さえてから表示する（例2-3）</text>
<rect x="380" y="36" width="320" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="540" y="61" font-size="12" fill="#0f172a" text-anchor="middle">① TcpListener::bind で押さえる</text>
<rect x="380" y="96" width="320" height="56" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="540" y="118" font-size="12" fill="#0f172a" text-anchor="middle">② 押さえられたときだけ</text>
<text x="540" y="138" font-size="11" fill="#475569" text-anchor="middle">「待っています」と表示</text>
<rect x="380" y="172" width="320" height="56" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="540" y="194" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">③ topcoat::serve(listener, router)</text>
<text x="540" y="214" font-size="11" fill="#475569" text-anchor="middle">失敗したら①で止まり、②は出ない</text>
<line x1="540" y1="76" x2="540" y2="92" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0202)"/>
<line x1="540" y1="152" x2="540" y2="168" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0202)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: 「起動しました」を表示する場所の違い。start は押さえるのとさばくのを中でまとめてするので、間に表示を入れられない。自分で押さえてから表示し、serve に渡せば、表示と状態が食い違わない</div>
</div>

"""
assert t.count(anchor) == 1
open(p, "w", encoding="utf-8", newline="").write(t.replace(anchor, fig + anchor))
print("ok")
