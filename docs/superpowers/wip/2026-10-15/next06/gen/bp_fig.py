X0, X1, MAXPX = 40, 700, 1600
def px(v): return X0 + (X1 - X0) * v / MAXPX
o = []
o.append('<div style="max-width: 720px; margin: 18px auto; text-align: center;">')
o.append('<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">')
o.append(f'<line x1="{X0}" y1="56" x2="{X1}" y2="56" stroke="#475569" stroke-width="2"/>')
marks = [(0, "0px", ""), (640, "sm", "640"), (768, "md", "768"), (1024, "lg", "1024"), (1280, "xl", "1280"), (1536, "2xl", "1536")]
for v, name, num in marks:
    x = px(v)
    o.append(f'<line x1="{x:.0f}" y1="48" x2="{x:.0f}" y2="64" stroke="#475569" stroke-width="2"/>')
    if name == "0px":
        o.append(f'<text x="{x:.0f}" y="40" text-anchor="middle" font-size="12" fill="#475569">0px</text>')
    else:
        o.append(f'<text x="{x:.0f}" y="24" text-anchor="middle" font-size="12" font-weight="bold" fill="#1e40af">{name}:</text>')
        o.append(f'<text x="{x:.0f}" y="40" text-anchor="middle" font-size="12" fill="#475569">{num}</text>')
o.append(f'<text x="{X1}" y="82" text-anchor="end" font-size="12" fill="#64748b">画面の横幅 →</text>')
bands = [(0, 768, 1, "#eff6ff", "#3b82f6", "#1e40af", "grid-cols-1", "1列（スマホ）"),
         (768, 1024, 2, "#fff7ed", "#f97316", "#c2410c", "md:grid-cols-2", "2列"),
         (1024, 1600, 3, "#ecfdf5", "#10b981", "#047857", "lg:grid-cols-3", "3列（PC）")]
for a, b, n, fill, stroke, color, cls, label in bands:
    x, w = px(a) + 3, px(b) - px(a) - 6
    o.append(f'<rect x="{x:.0f}" y="96" width="{w:.0f}" height="96" rx="8" fill="{fill}" stroke="{stroke}" stroke-width="2"/>')
    gap = 8
    cw = (w - 16 - gap * (n - 1)) / n
    for i in range(n):
        cx = x + 8 + i * (cw + gap)
        o.append(f'<rect x="{cx:.0f}" y="106" width="{cw:.0f}" height="34" rx="4" fill="#ffffff" stroke="{stroke}" stroke-width="1.5"/>')
        if n == 1:
            o.append(f'<rect x="{cx:.0f}" y="146" width="{cw:.0f}" height="16" rx="4" fill="#ffffff" stroke="{stroke}" stroke-width="1.5" stroke-dasharray="4 3"/>')
    o.append(f'<text x="{x + w/2:.0f}" y="182" text-anchor="middle" font-size="12" font-weight="bold" fill="{color}">{label}</text>')
    o.append(f'<text x="{x + w/2:.0f}" y="214" text-anchor="middle" font-size="12" fill="{color}" font-family="Consolas, Menlo, monospace">{cls}</text>')
o.append('<text x="360" y="240" text-anchor="middle" font-size="12" fill="#64748b">プレフィックス付きのクラスは「その幅以上」で効き、より広い幅のクラスが後から上書きする</text>')
o.append('</svg>')
o.append('<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図6-6: ブレークポイントの物差しと、grid-cols-1 md:grid-cols-2 lg:grid-cols-3 の見え方</div>')
o.append('</div>')
open("bp_fig.html", "w", encoding="utf-8").write("\n".join(o) + "\n")
