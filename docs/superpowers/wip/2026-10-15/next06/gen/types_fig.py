rows = ["id", "title", "author", "status", "rating", "memo", "created_at", "updated_at"]
book = ["Y"] * 8
ins = ["N", "R", "R", "O", "O", "O", "N", "N"]
upd = ["N", "O", "O", "O", "O", "O", "N", "N"]
style = {
    "Y": ("#ecfdf5", "#10b981", "#047857", "あり"),
    "R": ("#ecfdf5", "#10b981", "#047857", "必須"),
    "O": ("#fff7ed", "#f97316", "#c2410c", "省略可（?）"),
    "N": ("#f8fafc", "#cbd5e1", "#94a3b8", "なし"),
}
out = []
out.append('<div style="max-width: 720px; margin: 18px auto; text-align: center;">')
out.append('<svg viewBox="0 0 720 345" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">')
cols = [(180, "Book", "取得（SELECT）"), (360, "BookInsert", "登録（INSERT）"), (540, "BookUpdate", "更新（UPDATE）")]
out.append('<rect x="20" y="14" width="150" height="44" rx="6" fill="#f8fafc" stroke="#475569" stroke-width="1.5"/>')
out.append('<text x="95" y="41" text-anchor="middle" font-size="13" font-weight="bold" fill="#475569">項目名</text>')
for x, name, sub in cols:
    out.append(f'<rect x="{x}" y="14" width="170" height="44" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>')
    out.append(f'<text x="{x+85}" y="33" text-anchor="middle" font-size="13" font-weight="bold" fill="#1e40af">{name}</text>')
    out.append(f'<text x="{x+85}" y="50" text-anchor="middle" font-size="12" fill="#64748b">{sub}</text>')
for i, r in enumerate(rows):
    y = 66 + i * 30
    out.append(f'<text x="95" y="{y+18}" text-anchor="middle" font-size="13" fill="#1e293b" font-family="Consolas, Menlo, monospace">{r}</text>')
    for (x, _, _), v in zip(cols, [book[i], ins[i], upd[i]]):
        fill, stroke, color, label = style[v]
        out.append(f'<rect x="{x}" y="{y}" width="170" height="26" rx="5" fill="{fill}" stroke="{stroke}" stroke-width="1.5"/>')
        out.append(f'<text x="{x+85}" y="{y+18}" text-anchor="middle" font-size="12" fill="{color}">{label}</text>')
ly = 318
items = [("#ecfdf5", "#10b981", "必ず入っている／必ず送る"), ("#fff7ed", "#f97316", "送っても送らなくてもよい"), ("#f8fafc", "#cbd5e1", "型に含めない（DBが自動で付ける）")]
x = 30
for fill, stroke, label in items:
    out.append(f'<rect x="{x}" y="{ly}" width="14" height="14" fill="{fill}" stroke="{stroke}" stroke-width="1.5"/>')
    out.append(f'<text x="{x+20}" y="{ly+12}" font-size="12" fill="#475569">{label}</text>')
    x += 220
out.append('</svg>')
out.append('<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図6-10: Book / BookInsert / BookUpdate に入る項目の比較</div>')
out.append('</div>')
open("types_fig.html", "w", encoding="utf-8").write("\n".join(out) + "\n")
