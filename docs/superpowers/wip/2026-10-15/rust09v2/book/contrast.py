"""RustNote の style.css の色の組み合わせの、コントラスト比（WCAG 2）を計算する。
使い方: python contrast.py rustnote/assets/style.css
"""
import re
import sys

css = open(sys.argv[1], encoding="utf-8").read()
colors = dict(re.findall(r"--(color-[a-z-]+):\s*(#[0-9a-fA-F]{6})", css))
colors["white"] = "#ffffff"


def luminance(hex_color):
    """相対輝度（0 が黒、1 が白）"""
    rgb = [int(hex_color[i : i + 2], 16) / 255 for i in (1, 3, 5)]
    lin = [c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4 for c in rgb]
    return 0.2126 * lin[0] + 0.7152 * lin[1] + 0.0722 * lin[2]


def ratio(fg, bg):
    a, b = luminance(colors[fg]), luminance(colors[bg])
    hi, lo = max(a, b), min(a, b)
    return (hi + 0.05) / (lo + 0.05)


# どこで使っている組み合わせか
pairs = [
    ("color-text", "color-bg", "本文の文字"),
    ("color-muted", "color-bg", "日時などの薄い文字"),
    ("color-muted", "color-card", "カードの中の薄い文字"),
    ("color-primary", "color-bg", "リンク"),
    ("white", "color-primary", "ボタンの文字"),
    ("color-primary", "color-card", "白いボタン（secondary）の文字"),
    ("color-danger", "color-card", "削除のボタン・エラーの文字"),
    ("color-success-text", "color-success-bg", "成功のお知らせ"),
    ("color-warning-text", "color-warning-bg", "注意のお知らせ"),
]
print("組み合わせ                              比     AA（4.5 以上）")
for fg, bg, where in pairs:
    r = ratio(fg, bg)
    mark = "ok" if r >= 4.5 else "NG"
    print(f"{where:<16} {colors[fg]} / {colors[bg]}  {r:5.2f}  {mark}")
