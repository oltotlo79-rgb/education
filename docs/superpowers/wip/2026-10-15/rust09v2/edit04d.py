import sys
p = sys.argv[1]
t = open(p, encoding="utf-8").read()
def rep(old, new):
    global t
    assert t.count(old) == 1, old[:60]
    t = t.replace(old, new)
rep('<rect x="290" y="90" width="180" height="70" rx="8" fill="#ffffff" stroke="#475569" stroke-width="2"/>',
    '<rect x="280" y="90" width="200" height="70" rx="8" fill="#ffffff" stroke="#475569" stroke-width="2"/>')
rep('<line x1="230" y1="125" x2="286" y2="125" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0404)"/>',
    '<line x1="230" y1="125" x2="276" y2="125" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0404)"/>')
rep('<text x="258" y="80" font-size="11" fill="#475569" text-anchor="middle">約束だけを</text>',
    '<text x="255" y="80" font-size="11" fill="#475569" text-anchor="middle">約束だけを</text>')
rep('<text x="258" y="96" font-size="11" fill="#475569" text-anchor="middle">知っている</text>',
    '<text x="255" y="96" font-size="11" fill="#475569" text-anchor="middle">知っている</text>')
rep('<path d="M520,58 H496 V112 H474"', '<path d="M520,58 H500 V112 H484"')
rep('<path d="M520,192 H496 V138 H474"', '<path d="M520,192 H500 V138 H484"')
rep('<text x="496" y="250" font-size="11"', '<text x="500" y="250" font-size="11"')
for y in ("284", "184", "34"):
    rep(f'<line x1="556" y1="{y}" x2="444" y2="{y}" stroke="#94a3b8" stroke-width="2" stroke-dasharray="5 4"/>\n', "")
rep('<text x="635" y="122" font-size="11" fill="#475569" text-anchor="middle">それぞれの段で</text>',
    '<text x="635" y="122" font-size="11" fill="#475569" text-anchor="middle">同じ高さの段で</text>')
open(p, "w", encoding="utf-8", newline="").write(t)
