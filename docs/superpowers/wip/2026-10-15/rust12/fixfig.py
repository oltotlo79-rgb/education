import re
import sys
p = sys.argv[1]
t = open(p, encoding="utf-8").read()
bs = chr(92)

def rep(old, new):
    global t
    assert t.count(old) == 1, old[:60]
    t = t.replace(old, new)

# SVG の中で \ を含む文字は、¥ に見えないよう等幅のフォントにする
def mono(m):
    tag = m.group(0)
    if bs in m.group(2) and "font-family" not in m.group(1):
        return "<text" + m.group(1) + ' font-family="Consolas, monospace">' + m.group(2) + "</text>"
    return tag
t = re.sub(r"<text([^>]*)>([^<]*)</text>", mono, t)

rep('<rect x="20" y="20" width="160" height="40" rx="6" fill="#f8fafc"/>',
    '<rect x="20" y="20" width="160" height="40" rx="6" fill="#f8fafc"/>\n<text x="100" y="45" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">比べたこと</text>')
rep("図12-?: 本番用の実行ファイルができるまで。① release ビルドの後に ② アセットの束を作り、実行ファイルと assets フォルダを必ず一緒に置く",
    "図12-?: 本番用の実行ファイルができるまで。① release ビルドの後に ② アセットの束を作り、2つを必ず一緒に置く")
rep("図12-?: 探す場所の決まりの違い。アセットは実行ファイルの隣、DB ファイルは今いるフォルダ（本番では DATABASE_URL で場所を決める）",
    "図12-?: 探す場所の決まりの違い。アセットは実行ファイルの隣を、DB ファイルは今いるフォルダを使う")
rep("動きは遅い。target" + bs + "debug にできる）", "動きは遅い。`target" + bs + "debug` にできる）")
rep("速く動く。target" + bs + "release にできる）", "速く動く。`target" + bs + "release` にできる）")
open(p, "w", encoding="utf-8", newline="").write(t)
print("monospace texts:", t.count('font-family="Consolas, monospace"'))
