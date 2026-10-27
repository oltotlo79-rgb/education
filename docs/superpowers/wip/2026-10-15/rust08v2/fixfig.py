import sys
p = sys.argv[1]
t = open(p, encoding="utf-8").read()
def rep(old, new):
    global t
    assert t.count(old) == 1, old[:60]
    t = t.replace(old, new)
# 攻める人の並びを、入口の並び（フォーム・クッキー・URL）にそろえて、矢印が交わらないようにする
rep('<text x="110" y="64" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">ほかの利用者（bob）</text>\n<text x="110" y="84" font-size="11" fill="#0f172a" text-anchor="middle">URL の数字を変える</text>',
    '<text x="110" y="64" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">パスワードを試す人</text>\n<text x="110" y="84" font-size="11" fill="#0f172a" text-anchor="middle">何回もログインする</text>')
rep('<text x="110" y="140" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">パスワードを試す人</text>\n<text x="110" y="160" font-size="11" fill="#0f172a" text-anchor="middle">何回もログインする</text>',
    '<text x="110" y="140" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">ほかのサイト</text>\n<text x="110" y="160" font-size="11" fill="#0f172a" text-anchor="middle">ブラウザにフォームを送らせる</text>')
rep('<text x="110" y="216" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">ほかのサイト</text>\n<text x="110" y="236" font-size="11" fill="#0f172a" text-anchor="middle">ブラウザにフォームを送らせる</text>',
    '<text x="110" y="216" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">ほかの利用者（bob）</text>\n<text x="110" y="236" font-size="11" fill="#0f172a" text-anchor="middle">URL の数字を変える</text>')
rep('<line x1="210" y1="70" x2="286" y2="150"', '<line x1="210" y1="70" x2="286" y2="110"')
rep('<line x1="210" y1="146" x2="286" y2="112"', '<line x1="210" y1="146" x2="286" y2="130"')
rep('<line x1="210" y1="222" x2="286" y2="132"', '<line x1="210" y1="222" x2="286" y2="150"')
open(p, "w", encoding="utf-8", newline="").write(t)
print("ok")
