from html import escape as e

MONO = 'font-family="Consolas, Menlo, monospace"'

def term_frame(w, h, title):
    o = []
    o.append(f'<rect x="10" y="6" width="{w-20}" height="{h-12}" rx="10" fill="#0f172a" stroke="#475569" stroke-width="2"/>')
    o.append(f'<rect x="10" y="6" width="{w-20}" height="28" rx="10" fill="#1e293b"/>')
    o.append(f'<rect x="10" y="24" width="{w-20}" height="10" fill="#1e293b"/>')
    o.append('<circle cx="30" cy="20" r="5" fill="#f87171"/><circle cx="48" cy="20" r="5" fill="#fbbf24"/><circle cx="66" cy="20" r="5" fill="#34d399"/>')
    o.append(f'<text x="{w/2:.0f}" y="25" text-anchor="middle" font-size="12" fill="#94a3b8">{e(title)}</text>')
    return o

def span(text, color="#e2e8f0", bold=False, underline=False):
    attrs = f'fill="{color}"'
    if bold:
        attrs += ' font-weight="bold"'
    if underline:
        attrs += ' text-decoration="underline"'
    return f'<tspan {attrs}>{e(text)}</tspan>'

# ---------- 図A: 最初の質問（矢印キーで選ぶ画面） ----------
W, H = 720, 214
o = ['<div style="max-width: 720px; margin: 18px auto; text-align: center;">',
     f'<svg viewBox="0 0 {W} {H}" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">']
o += term_frame(W, H, "ターミナル（npx create-next-app@latest book-management を実行した直後）")
lines = [
    [span("? ", "#22d3ee"), span("Would you like to use the recommended Next.js defaults?", "#f8fafc", True), span(" » - Use arrow-keys. Return to submit.", "#94a3b8")],
    [span("    Yes, use recommended defaults", "#e2e8f0"), span(" - TypeScript, ESLint, No React Compiler,", "#94a3b8")],
    [span("      Tailwind CSS, No src/ directory, App Router, Cache Components, AGENTS.md,", "#94a3b8")],
    [span("      Agent feedback", "#94a3b8")],
    [span("&gt;   ".replace("&gt;", ">"), "#22d3ee"), span("No, customize settings", "#22d3ee", underline=True), span(" - Choose your own preferences", "#94a3b8")],
]
y = 58
for parts in lines:
    o.append(f'<text x="28" y="{y}" font-size="12" {MONO} xml:space="preserve">' + "".join(parts) + '</text>')
    y += 22
o.append('<rect x="24" y="131" width="184" height="22" rx="4" fill="none" stroke="#f97316" stroke-width="2"/>')
o.append('<text x="40" y="176" font-size="12" fill="#fdba74">↑ ↓キーで「No, customize settings」に合わせて Enter を押す</text>')
o.append('<text x="40" y="198" font-size="12" fill="#94a3b8">※ 推奨（Yes）を選ぶと Cache Components が有効になる → 第7章のトップページが build で失敗する</text>')
o.append('</svg>')
o.append('<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図X: 最初の質問の画面イメージ（矢印キーで「カスタマイズ」を選ぶ）</div>')
o.append('</div>')
open("figA.html", "w", encoding="utf-8").write("\n".join(o) + "\n")

# ---------- 図B: すべて答え終わった画面 ----------
Y, N = "Yes", "No"
def yn(choice):
    if choice == "Yes":
        return [span(" ... ", "#94a3b8"), span("No", "#e2e8f0"), span(" / ", "#94a3b8"), span("Yes", "#22d3ee", underline=True)]
    return [span(" ... ", "#94a3b8"), span("No", "#22d3ee", underline=True), span(" / ", "#94a3b8"), span("Yes", "#e2e8f0")]
ok = span("√ ", "#4ade80")
def q(*parts):
    return [ok] + list(parts)
B = [
    (1, q(span("Would you like to use the recommended Next.js defaults?", "#f8fafc", True), span(" » ", "#94a3b8"), span("No, customize settings", "#22d3ee"))),
    (2, q(span("Would you like to use ", "#f8fafc", True), span("TypeScript", "#60a5fa", True), span("?", "#f8fafc", True), *yn(Y))),
    (3, q(span("Which linter would you like to use?", "#f8fafc", True), span(" » ", "#94a3b8"), span("ESLint", "#22d3ee"))),
    (4, q(span("Would you like to use ", "#f8fafc", True), span("React Compiler", "#60a5fa", True), span("?", "#f8fafc", True), *yn(N))),
    (5, q(span("Would you like to use ", "#f8fafc", True), span("Tailwind CSS", "#60a5fa", True), span("?", "#f8fafc", True), *yn(Y))),
    (6, q(span("Would you like your code inside a ", "#f8fafc", True), span("`src/` directory", "#60a5fa", True), span("?", "#f8fafc", True), *yn(N))),
    (7, q(span("Would you like to use ", "#f8fafc", True), span("App Router", "#60a5fa", True), span("? (recommended)", "#f8fafc", True), *yn(Y))),
    (8, q(span("Would you like to use ", "#f8fafc", True), span("Cache Components", "#60a5fa", True), span("?", "#f8fafc", True), *yn(N))),
    (9, q(span("Would you like to customize the ", "#f8fafc", True), span("import alias", "#60a5fa", True), span(" (`@/*` by default)?", "#f8fafc", True), *yn(N))),
    (10, q(span("Would you like to include ", "#f8fafc", True), span("AGENTS.md", "#60a5fa", True), span(" to guide coding agents", "#f8fafc", True))),
    (None, [span("  to write up-to-date Next.js code?", "#f8fafc", True), *yn(Y)]),
    (11, q(span("Would you like to help improve Next.js by letting agents prepare", "#f8fafc", True))),
    (None, [span("  ", "#f8fafc"), span("anonymized feedback", "#60a5fa", True), span(" for your review as you code? (Disable anytime", "#f8fafc", True)]),
    (None, [span("  with `experimental.agentFeedback: false`.)", "#f8fafc", True), *yn(N)]),
    (None, [span(r"Creating a new Next.js app in C:\Users\you\projects\book-management.", "#e2e8f0")]),
]
H = 52 + len(B) * 22 + 14
o = ['<div style="max-width: 720px; margin: 18px auto; text-align: center;">',
     f'<svg viewBox="0 0 {W} {H}" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">']
o += term_frame(W, H, "ターミナル（質問にすべて答え終わったところ）")
y = 58
circ = "①②③④⑤⑥⑦⑧⑨⑩⑪"
for num, parts in B:
    if num:
        o.append(f'<circle cx="30" cy="{y-4}" r="9" fill="#f97316"/>')
        o.append(f'<text x="30" y="{y}" text-anchor="middle" font-size="12" font-weight="bold" fill="#ffffff">{num}</text>')
    o.append(f'<text x="46" y="{y}" font-size="12" {MONO} xml:space="preserve">' + "".join(parts) + '</text>')
    y += 22
o.append('</svg>')
o.append('<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図X: create-next-app の質問に答え終わった画面イメージ（水色の下線が選んだ答え。番号は下の表と対応）</div>')
o.append('</div>')
open("figB.html", "w", encoding="utf-8").write("\n".join(o) + "\n")
print("ok", H)
