import sys
p = sys.argv[1]
t = open(p, encoding="utf-8").read()
def rep(old, new):
    global t
    assert t.count(old) == 1, old[:60]
    t = t.replace(old, new)

fig_a = """
<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 200" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-0103" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<line x1="180" y1="50" x2="690" y2="50" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0103)"/>
<line x1="220" y1="44" x2="220" y2="56" stroke="#475569" stroke-width="2"/>
<line x1="330" y1="44" x2="330" y2="56" stroke="#475569" stroke-width="2"/>
<line x1="400" y1="44" x2="400" y2="56" stroke="#475569" stroke-width="2"/>
<line x1="630" y1="44" x2="630" y2="56" stroke="#475569" stroke-width="2"/>
<text x="220" y="34" font-size="12" fill="#0f172a" text-anchor="middle">1.53.3</text>
<text x="330" y="34" font-size="12" fill="#0f172a" text-anchor="middle">1.54.0</text>
<text x="400" y="34" font-size="12" fill="#0f172a" text-anchor="middle">1.54.1</text>
<text x="630" y="34" font-size="12" fill="#0f172a" text-anchor="middle">2.0.0</text>
<text x="690" y="76" font-size="11" fill="#475569" text-anchor="end">新しい版 →</text>
<text x="20" y="104" font-size="13" font-weight="700" fill="#1e40af">"=1.53.3"</text>
<circle cx="220" cy="100" r="8" fill="#3b82f6"/>
<text x="240" y="104" font-size="12" fill="#1e40af">この版だけ（cargo update でも変わらない）</text>
<text x="20" y="150" font-size="13" font-weight="700" fill="#047857">"1.53.3"</text>
<text x="20" y="168" font-size="11" fill="#475569">（"^1.53.3" と同じ）</text>
<rect x="220" y="140" width="410" height="16" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<circle cx="220" cy="148" r="6" fill="#10b981"/>
<circle cx="630" cy="148" r="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<circle cx="400" cy="148" r="6" fill="#047857"/>
<text x="400" y="186" font-size="12" fill="#047857" text-anchor="middle">この範囲でいちばん新しい 1.54.1 を選べる（2.0.0 は含まない）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: 版の書き方と、選べる版の範囲。= を付けるとその版だけ、付けないと「上の桁が同じ範囲」の新しい版まで選べる</div>
</div>
"""

rep("""- `26 unchanged dependencies behind latest` は""", fig_a.lstrip("\n") + """
- `26 unchanged dependencies behind latest` は""")

fig_b = """
<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-0104" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="250" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="145" y="43" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">rustnote の Cargo.toml</text>
<text x="145" y="64" font-size="12" fill="#0f172a" text-anchor="middle">rt-multi-thread・macros</text>
<rect x="20" y="96" width="250" height="56" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="145" y="119" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">topcoat の Cargo.toml</text>
<text x="145" y="140" font-size="12" fill="#0f172a" text-anchor="middle">macros・net・signal</text>
<rect x="20" y="172" width="250" height="56" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="145" y="195" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">libsql_replication の Cargo.toml</text>
<text x="145" y="216" font-size="12" fill="#0f172a" text-anchor="middle">full（ほぼ全部）</text>
<line x1="270" y1="48" x2="426" y2="110" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0104)"/>
<line x1="270" y1="124" x2="426" y2="124" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0104)"/>
<line x1="270" y1="200" x2="426" y2="138" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0104)"/>
<text x="330" y="70" font-size="20" font-weight="800" fill="#475569" text-anchor="middle">＋</text>
<text x="330" y="186" font-size="20" font-weight="800" fill="#475569" text-anchor="middle">＋</text>
<rect x="430" y="70" width="270" height="108" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="565" y="96" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">tokio v1.53.3（1つにまとめる）</text>
<text x="565" y="120" font-size="12" fill="#0f172a" text-anchor="middle">全部を足した機能でコンパイル</text>
<text x="565" y="142" font-size="11" fill="#475569" text-anchor="middle">full・macros・net・signal・</text>
<text x="565" y="160" font-size="11" fill="#475569" text-anchor="middle">rt-multi-thread・fs・time ほか</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: features の足し算。1つのビルドの中の tokio は1つにまとめられ、誰かが求めた機能を全部足したものでコンパイルされる。だから自分が書き忘れた機能も、ほかのクレートのおかげで使えてしまうことがある</div>
</div>
"""

rep("""同じ「足し算」の考え方で、版の違うクレートが同時に入ることもあります。""", fig_b.lstrip("\n") + """
同じ「足し算」の考え方で、とはいえ少し違う話として、版の違うクレートが同時に入ることもあります。""")
open(p, "w", encoding="utf-8", newline="").write(t)
