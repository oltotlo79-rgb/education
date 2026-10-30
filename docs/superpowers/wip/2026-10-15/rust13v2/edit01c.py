import sys, re
p = sys.argv[1]
t = open(p, encoding="utf-8").read()
def rep(old, new):
    global t
    assert t.count(old) == 1, old[:70]
    t = t.replace(old, new)
# 図13-3: 6つの箱を同じ高さ（40）にそろえる
boxes = [
    ("Cargo.toml", "#eff6ff", "#3b82f6", "#1e40af", 20),
    ("Cargo.lock", "#eff6ff", "#3b82f6", "#1e40af", 70),
    ("rust-toolchain.toml", "#ecfdf5", "#10b981", "#047857", 120),
    ("rustfmt.toml", "#ecfdf5", "#10b981", "#047857", 170),
    (".gitignore", "#fff7ed", "#f97316", "#c2410c", 220),
    (".env.example", "#f8fafc", "#64748b", "#475569", 270),
]
start = t.index('<rect x="20" y="20" width="210" height="44"')
end = t.index('<rect x="300" y="44" width="130" height="56"')
new_boxes = ""
for name, fill, stroke, color, y in boxes:
    new_boxes += f'<rect x="20" y="{y}" width="210" height="40" rx="6" fill="{fill}" stroke="{stroke}" stroke-width="2"/>\n'
    new_boxes += f'<text x="125" y="{y + 25}" font-size="13" font-weight="700" fill="{color}" text-anchor="middle">{name}</text>\n'
t = t[:start] + new_boxes + t[end:]
old_lines = [
    '<line x1="230" y1="42" x2="296" y2="64"', '<line x1="230" y1="98" x2="296" y2="80"',
    '<line x1="230" y1="154" x2="296" y2="162"', '<line x1="230" y1="210" x2="296" y2="178"',
    '<line x1="230" y1="266" x2="296" y2="264"', '<line x1="230" y1="312" x2="296" y2="282"',
]
new_lines = [
    '<line x1="230" y1="40" x2="296" y2="64"', '<line x1="230" y1="90" x2="296" y2="80"',
    '<line x1="230" y1="140" x2="296" y2="160"', '<line x1="230" y1="190" x2="296" y2="176"',
    '<line x1="230" y1="240" x2="296" y2="258"', '<line x1="230" y1="290" x2="296" y2="274"',
]
for o, n in zip(old_lines, new_lines):
    rep(o, n)
# 図13-6: ＋ を矢印から離し、左の箱の間に置く
rep('<text x="330" y="70" font-size="20" font-weight="800" fill="#475569" text-anchor="middle">＋</text>',
    '<text x="145" y="92" font-size="16" font-weight="800" fill="#475569" text-anchor="middle">＋</text>')
rep('<text x="330" y="186" font-size="20" font-weight="800" fill="#475569" text-anchor="middle">＋</text>',
    '<text x="145" y="168" font-size="16" font-weight="800" fill="#475569" text-anchor="middle">＋</text>')
open(p, "w", encoding="utf-8", newline="").write(t)
