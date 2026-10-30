"""style.css の :root の色から、文字と背景の組のコントラスト比（WCAG 2）を計算する。
使い方: python contrast13.py <style.css>
"""
import re, sys

css = open(sys.argv[1], encoding="utf-8").read()
root = re.search(r":root\s*\{(.*?)\}", css, re.S).group(1)
var = dict(re.findall(r"--([\w-]+):\s*(#[0-9a-fA-F]{6})", root))
var["white"] = "#ffffff"


def luminance(hex_color):
    def channel(c):
        c = c / 255
        return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4
    r, g, b = (int(hex_color[i:i + 2], 16) for i in (1, 3, 5))
    return 0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b)


def ratio(fg, bg):
    a, b = sorted((luminance(var[fg]), luminance(var[bg])), reverse=True)
    return (a + 0.05) / (b + 0.05)


pairs = [
    ("本文の文字", "color-text", "color-bg"),
    ("カードの中の文字", "color-text", "color-card"),
    ("薄い文字（カードの中）", "color-muted", "color-card"),
    ("薄い文字（背景の上）", "color-muted", "color-bg"),
    ("リンク（カードの中）", "color-primary", "color-card"),
    ("ボタン", "white", "color-primary"),
    ("ボタン（マウスを乗せたとき）", "white", "color-primary-dark"),
    ("成功のお知らせ", "color-success-text", "color-success-bg"),
    ("注意のお知らせ", "color-warning-text", "color-warning-bg"),
    ("入力のエラー文（背景の上）", "color-danger", "color-bg"),
]
for name, fg, bg in pairs:
    r = ratio(fg, bg)
    verdict = "AA 合格" if r >= 4.5 else ("大きい文字だけ AA" if r >= 3 else "不合格")
    print(f"{name}: {var[fg]} / {var[bg]} → {r:.2f} : 1（{verdict}）")
