import sys
p = sys.argv[1]
t = open(p, encoding="utf-8").read()
anchor = "- GET は「見るだけ」で、何回送っても DB を変えない（**安全**な）メソッドとして扱います。"
fig = """<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0102" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="60" y="14" width="140" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="130" y="36" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">ブラウザ</text>
<rect x="520" y="14" width="140" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="590" y="36" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">サーバー</text>
<line x1="130" y1="48" x2="130" y2="320" stroke="#94a3b8" stroke-width="2" stroke-dasharray="4 4"/>
<line x1="590" y1="48" x2="590" y2="320" stroke="#94a3b8" stroke-width="2" stroke-dasharray="4 4"/>
<line x1="130" y1="78" x2="586" y2="78" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0102)"/>
<text x="360" y="70" font-size="12" fill="#0f172a" text-anchor="middle">① POST /messages（message=hello）</text>
<text x="600" y="98" font-size="11" fill="#047857">伝言を保存</text>
<line x1="590" y1="122" x2="134" y2="122" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0102)"/>
<text x="360" y="114" font-size="12" fill="#0f172a" text-anchor="middle">② 303 See Other（Location: /messages）</text>
<line x1="130" y1="166" x2="586" y2="166" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0102)"/>
<text x="360" y="158" font-size="12" fill="#0f172a" text-anchor="middle">③ GET /messages（ブラウザが自動で送る）</text>
<line x1="590" y1="210" x2="134" y2="210" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0102)"/>
<text x="360" y="202" font-size="12" fill="#0f172a" text-anchor="middle">④ 200 OK（伝言の一覧）</text>
<text x="20" y="246" font-size="11" fill="#c2410c">再読み込み</text>
<line x1="130" y1="254" x2="586" y2="254" stroke="#c2410c" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-06-0102)"/>
<text x="360" y="246" font-size="12" fill="#c2410c" text-anchor="middle">⑤ もう一度送られるのは GET /messages だけ</text>
<line x1="590" y1="290" x2="134" y2="290" stroke="#c2410c" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-06-0102)"/>
<text x="360" y="282" font-size="12" fill="#c2410c" text-anchor="middle">⑥ 200 OK（一覧を見るだけ。伝言は増えない）</text>
<text x="360" y="316" font-size="11" fill="#475569" text-anchor="middle">②で 200 の一覧を返していたら、再読み込みで ① の POST が送り直され、伝言が二重になる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: PRG（POST → Redirect → GET）の流れ。POST の返事を 303 にして GET の画面へ移すので、再読み込みで送り直されるのは見るだけの GET になる</div>
</div>

"""
assert t.count(anchor) == 1
open(p, "w", encoding="utf-8", newline="").write(t.replace(anchor, fig + anchor))
print("ok")
