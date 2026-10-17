from html import escape as e

MONO = 'font-family="Consolas, Menlo, monospace"'
HEAD = '<div style="max-width: 720px; margin: 18px auto; text-align: center;">'


def svg_open(h):
    return (f'<svg viewBox="0 0 720 {h}" width="100%" xmlns="http://www.w3.org/2000/svg" '
            'font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">')


def cap(t):
    return f'<div style="font-size: 12px; color: #64748b; margin-top: 6px;">{t}</div>'


def write(name, parts):
    open(name, "w", encoding="utf-8").write("\n".join(parts) + "\n")


# ---------- 図C: 推奨設定と本書の設定の比較 ----------
o = [HEAD, svg_open(345)]
items = [("TypeScript", "Yes", "Yes"), ("Linter", "ESLint", "ESLint"), ("React Compiler", "No", "No"),
         ("Tailwind CSS", "Yes", "Yes"), ("src/ directory", "No", "No"), ("App Router", "Yes", "Yes"),
         ("Cache Components", "Yes", "No"), ("AGENTS.md", "Yes", "Yes"), ("Agent feedback", "Yes", "No")]
o.append('<rect x="20" y="12" width="200" height="40" rx="6" fill="#f8fafc" stroke="#475569" stroke-width="1.5"/>')
o.append('<text x="120" y="37" text-anchor="middle" font-size="13" font-weight="bold" fill="#475569">質問</text>')
o.append('<rect x="235" y="12" width="210" height="40" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="1.5"/>')
o.append('<text x="340" y="30" text-anchor="middle" font-size="13" font-weight="bold" fill="#c2410c">推奨設定（Enter 連打）</text>')
o.append('<text x="340" y="46" text-anchor="middle" font-size="12" fill="#c2410c">Yes, use recommended defaults</text>')
o.append('<rect x="460" y="12" width="240" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="1.5"/>')
o.append('<text x="580" y="30" text-anchor="middle" font-size="13" font-weight="bold" fill="#047857">本書の設定（カスタマイズ）</text>')
o.append('<text x="580" y="46" text-anchor="middle" font-size="12" fill="#047857">No, customize settings</text>')
y = 60
for name, rec, ours in items:
    diff = rec != ours
    fill = "#fff7ed" if diff else "#ffffff"
    o.append(f'<rect x="20" y="{y}" width="680" height="26" rx="5" fill="{fill}" stroke="{"#f97316" if diff else "#e2e8f0"}" stroke-width="{2 if diff else 1}"/>')
    o.append(f'<text x="120" y="{y + 18}" text-anchor="middle" font-size="12" fill="#1e293b" {MONO}>{e(name)}</text>')
    o.append(f'<text x="340" y="{y + 18}" text-anchor="middle" font-size="12" font-weight="{"bold" if diff else "normal"}" fill="{"#c2410c" if diff else "#475569"}">{rec}</text>')
    o.append(f'<text x="580" y="{y + 18}" text-anchor="middle" font-size="12" font-weight="{"bold" if diff else "normal"}" fill="{"#047857" if diff else "#475569"}">{ours}</text>')
    y += 28
o.append(f'<text x="360" y="{y + 18}" text-anchor="middle" font-size="12" fill="#c2410c">違いは橙色の2行。Cache Components が Yes だと第7章で npm run build が失敗する</text>')
o += ['</svg>', cap('図X: 推奨設定と本書の設定の違い（推奨設定のままだと Cache Components が有効になる）'), '</div>']
write("figC.html", o)

# ---------- 図D: 生成されたファイルと役割 ----------
cats = {
    "src": ("#eff6ff", "#3b82f6", "#1e40af"),
    "cfg": ("#ecfdf5", "#10b981", "#047857"),
    "pub": ("#fff7ed", "#f97316", "#c2410c"),
    "gen": ("#f8fafc", "#94a3b8", "#64748b"),
    "doc": ("#fdf2f8", "#db2777", "#9d174d"),
}
rows = [
    ("book-management/", 0, None, "プロジェクトのフォルダ（ここで npm run dev する）"),
    (".next/", 1, "gen", "next dev / next build が作る作業場所"),
    ("node_modules/", 1, "gen", "npm install で入った部品（約370個）"),
    ("public/", 1, "pub", "画像などの公開ファイル"),
    ("next.svg ほか4つ", 2, "pub", "/next.svg のような URL でそのまま見える"),
    ("app/", 1, "src", "フォルダの形が URL になる（App Router）"),
    ("favicon.ico", 2, "src", "ブラウザのタブのアイコン"),
    ("globals.css", 2, "src", "全ページ共通の CSS（Tailwind を読み込む）"),
    ("layout.tsx", 2, "src", "全ページの外枠（節7で書き換える）"),
    ("page.tsx", 2, "src", "/ のページ（節9で書き換える）"),
    (".gitignore", 1, "cfg", "Git に入れないファイルの一覧"),
    ("AGENTS.md", 1, "doc", "AI コーディング支援向けの説明書"),
    ("eslint.config.mjs", 1, "cfg", "ESLint（コードチェック）の設定"),
    ("next-env.d.ts", 1, "gen", "Next.js 用の型の読み込み（自動生成）"),
    ("next.config.ts", 1, "cfg", "Next.js の設定"),
    ("package.json", 1, "cfg", "使う部品とスクリプト（npm run ○○）の一覧"),
    ("package-lock.json", 1, "gen", "部品の正確なバージョン記録（自動更新）"),
    ("README.md", 1, "doc", "プロジェクトの説明書"),
    ("tsconfig.json", 1, "cfg", "TypeScript の設定（@/ の意味もここ）"),
]
H = 30 + len(rows) * 24 + 40
o = [HEAD, svg_open(H)]
y = 34
for name, lv, cat, role in rows:
    x = 24 + lv * 20
    if cat:
        fill, stroke, color = cats[cat]
        if all(ord(c) < 128 for c in name):
            w = len(name) * 7.2 + 14
        else:
            w = 130
        o.append(f'<rect x="{x - 6}" y="{y - 16}" width="{w:.0f}" height="22" rx="5" fill="{fill}" stroke="{stroke}" stroke-width="1.5"/>')
        o.append(f'<text x="{x}" y="{y}" font-size="13" fill="{color}" {MONO}>{e(name)}</text>')
        o.append(f'<line x1="{x - 6 + w + 4:.0f}" y1="{y - 5}" x2="296" y2="{y - 5}" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>')
        o.append(f'<text x="304" y="{y}" font-size="12" fill="{color}">{e(role)}</text>')
    else:
        o.append(f'<text x="{x}" y="{y}" font-size="13" font-weight="bold" fill="#1e293b" {MONO}>{e(name)}</text>')
        o.append(f'<text x="304" y="{y}" font-size="12" fill="#475569">{e(role)}</text>')
    y += 24
ly = y + 14
legend = [("src", "自分で書く"), ("cfg", "設定（ときどき編集）"), ("pub", "公開ファイル"),
          ("gen", "自動生成（触らない）"), ("doc", "説明書（読むだけ）")]
x = 24
for cat, label in legend:
    fill, stroke, color = cats[cat]
    o.append(f'<rect x="{x}" y="{ly - 11}" width="14" height="14" rx="3" fill="{fill}" stroke="{stroke}" stroke-width="1.5"/>')
    o.append(f'<text x="{x + 20}" y="{ly}" font-size="12" fill="#475569">{label}</text>')
    x += 20 + len(label) * 12 + 14
o += ['</svg>', cap('図X: create-next-app が作ったファイルと、それぞれの役割（色は「触ってよいか」の目安）'), '</div>']
write("figD.html", o)

# ---------- 図E: .env.local が読み込まれる流れ ----------
o = [HEAD, svg_open(330)]
o.append('<defs><marker id="ar-06-env" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker></defs>')
o.append('<rect x="10" y="90" width="250" height="130" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>')
o.append('<text x="135" y="114" text-anchor="middle" font-size="13" font-weight="bold" fill="#c2410c">.env.local（ファイル）</text>')
o.append(f'<text x="20" y="140" font-size="12" fill="#c2410c" {MONO}>NEXT_PUBLIC_SUPABASE_URL=…</text>')
o.append(f'<text x="20" y="162" font-size="12" fill="#c2410c" {MONO}>NEXT_PUBLIC_SUPABASE_ANON_KEY=…</text>')
o.append(f'<text x="20" y="184" font-size="12" fill="#64748b" {MONO}>SECRET_TOKEN=…</text>')
o.append('<text x="135" y="206" text-anchor="middle" font-size="12" fill="#64748b">（3行目は説明用の例）</text>')
o.append('<rect x="285" y="105" width="160" height="100" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>')
o.append('<text x="365" y="135" text-anchor="middle" font-size="13" font-weight="bold" fill="#1e40af">process.env</text>')
o.append('<text x="365" y="157" text-anchor="middle" font-size="12" fill="#1e40af">サーバー側の</text>')
o.append('<text x="365" y="177" text-anchor="middle" font-size="12" fill="#1e40af">「環境変数の入れ物」</text>')
o.append('<line x1="260" y1="155" x2="283" y2="155" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-env)"/>')
o.append('<text x="160" y="70" text-anchor="middle" font-size="12" fill="#475569">① npm run dev の起動時に読む</text>')
o.append('<text x="160" y="244" text-anchor="middle" font-size="12" fill="#475569">保存し直すと「Reload env」で読み直す</text>')
o.append('<rect x="490" y="20" width="220" height="110" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>')
o.append('<text x="600" y="46" text-anchor="middle" font-size="13" font-weight="bold" fill="#047857">サーバーで動くコード</text>')
o.append('<text x="600" y="68" text-anchor="middle" font-size="12" fill="#047857">Server Component / server.ts</text>')
o.append('<text x="600" y="92" text-anchor="middle" font-size="12" fill="#047857">3つとも読める</text>')
o.append('<text x="600" y="114" text-anchor="middle" font-size="12" fill="#64748b">（ブラウザには届かない）</text>')
o.append('<rect x="490" y="180" width="220" height="130" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>')
o.append('<text x="600" y="206" text-anchor="middle" font-size="13" font-weight="bold" fill="#047857">ブラウザで動くコード</text>')
o.append('<text x="600" y="228" text-anchor="middle" font-size="12" fill="#047857">Client Component / client.ts</text>')
o.append('<text x="600" y="252" text-anchor="middle" font-size="12" fill="#047857">NEXT_PUBLIC_ 付きの2つだけ</text>')
o.append('<text x="600" y="274" text-anchor="middle" font-size="12" fill="#c2410c">SECRET_TOKEN は undefined</text>')
o.append('<text x="600" y="296" text-anchor="middle" font-size="12" fill="#64748b">（値は JS に文字列で埋め込み）</text>')
o.append('<line x1="445" y1="135" x2="488" y2="85" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-env)"/>')
o.append('<line x1="445" y1="180" x2="488" y2="235" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-env)"/>')
o.append('<text x="462" y="164" font-size="12" fill="#475569">②</text>')
o += ['</svg>', cap('図X: .env.local の値がコードに届くまで（NEXT_PUBLIC_ 付きだけがブラウザまで届く）'), '</div>']
write("figE.html", o)

# ---------- 図F: npm run dev のターミナル画面 ----------
BS = chr(92)
lines = [
    ("PS C:" + BS + "Users" + BS + "you" + BS + "projects" + BS + "book-management> npm run dev", "#e2e8f0", ""),
    ("", "#e2e8f0", ""),
    ("> book-management@0.1.0 dev", "#94a3b8", ""),
    ("> next dev", "#94a3b8", ""),
    ("", "#e2e8f0", ""),
    ("▲ Next.js 16.4.1 (Turbopack)", "#f8fafc", "使っている Next.js の版"),
    ("- Local:         http://localhost:3000", "#22d3ee", "ブラウザで開く URL"),
    ("- Network:       http://192.168.x.x:3000", "#94a3b8", "同じ Wi-Fi の別端末用"),
    ("- Environments: .env.local", "#fbbf24", ".env.local を読めた合図"),
    ("✓ Ready in 7.0s", "#4ade80", "準備完了"),
    ("✓ Running next.config.ts took 80ms", "#4ade80", ""),
    ("", "#e2e8f0", ""),
    ("○ Compiling / ...", "#e2e8f0", "初めて開いたページを変換中"),
    (" GET / 200 in 398ms (next.js: 201ms, …)", "#e2e8f0", "200 = 表示に成功"),
]
H = 52 + len(lines) * 21 + 4
o = [HEAD, svg_open(H)]
o.append(f'<rect x="10" y="6" width="455" height="{H - 12}" rx="10" fill="#0f172a" stroke="#475569" stroke-width="2"/>')
o.append('<rect x="10" y="6" width="455" height="28" rx="10" fill="#1e293b"/><rect x="10" y="24" width="455" height="10" fill="#1e293b"/>')
o.append('<circle cx="30" cy="20" r="5" fill="#f87171"/><circle cx="48" cy="20" r="5" fill="#fbbf24"/><circle cx="66" cy="20" r="5" fill="#34d399"/>')
o.append('<text x="250" y="25" text-anchor="middle" font-size="12" fill="#94a3b8">ターミナル</text>')
y = 56
for text, color, note in lines:
    if text:
        o.append(f'<text x="22" y="{y}" font-size="12" fill="{color}" {MONO} xml:space="preserve" style="white-space: pre">{e(text)}</text>')
    if note:
        o.append(f'<line x1="468" y1="{y - 4}" x2="488" y2="{y - 4}" stroke="#f97316" stroke-width="1.5"/>')
        o.append(f'<text x="494" y="{y}" font-size="12" fill="#c2410c">{e(note)}</text>')
    y += 21
o += ['</svg>', cap('図X: npm run dev を実行したときのターミナルの画面イメージ（右は各行の意味）'), '</div>']
write("figF.html", o)

# ---------- 図G: ブラウザの画面イメージ ----------
o = [HEAD, svg_open(400)]
o.append('<rect x="10" y="6" width="700" height="388" rx="10" fill="#ffffff" stroke="#475569" stroke-width="2"/>')
o.append('<rect x="10" y="6" width="700" height="58" rx="10" fill="#e2e8f0"/><rect x="10" y="50" width="700" height="14" fill="#e2e8f0"/>')
o.append('<circle cx="30" cy="22" r="5" fill="#f87171"/><circle cx="48" cy="22" r="5" fill="#fbbf24"/><circle cx="66" cy="22" r="5" fill="#34d399"/>')
o.append('<rect x="90" y="12" width="160" height="22" rx="6" fill="#ffffff"/>')
o.append('<text x="102" y="28" font-size="12" fill="#1e293b">書籍管理アプリ</text>')
o.append('<rect x="24" y="38" width="672" height="22" rx="11" fill="#ffffff" stroke="#cbd5e1"/>')
o.append(f'<text x="40" y="53" font-size="12" fill="#475569" {MONO}>localhost:3000</text>')
o.append('<rect x="11" y="64" width="698" height="329" fill="#f9fafb"/>')
o.append('<rect x="11" y="64" width="698" height="44" fill="#ffffff" stroke="#e5e7eb"/>')
o.append('<text x="34" y="92" font-size="16" font-weight="bold" fill="#111827">書籍管理アプリ</text>')
o.append('<rect x="520" y="74" width="66" height="26" rx="5" fill="#eff6ff"/>')
o.append('<text x="553" y="92" text-anchor="middle" font-size="12" font-weight="bold" fill="#1d4ed8">書籍一覧</text>')
o.append('<rect x="596" y="74" width="100" height="26" rx="5" fill="#2563eb"/>')
o.append('<text x="646" y="92" text-anchor="middle" font-size="12" font-weight="bold" fill="#ffffff">書籍を登録する</text>')
o.append('<text x="34" y="146" font-size="22" font-weight="bold" fill="#111827">書籍一覧</text>')
o.append('<text x="34" y="172" font-size="12" fill="#4b5563">登録された書籍がここに表示されます。</text>')
o.append('<rect x="34" y="190" width="652" height="104" rx="8" fill="#ffffff" stroke="#e5e7eb" stroke-width="1.5"/>')
o.append('<text x="52" y="218" font-size="14" font-weight="bold" fill="#1f2937">セットアップ完了</text>')
o.append('<text x="52" y="242" font-size="12" fill="#4b5563">Next.js + Tailwind CSS + Supabase の開発環境が正しくセットアップされました。</text>')
o.append('<rect x="52" y="256" width="58" height="22" rx="11" fill="#fef9c3"/><text x="81" y="271" text-anchor="middle" font-size="12" fill="#854d0e">読みたい</text>')
o.append('<rect x="116" y="256" width="52" height="22" rx="11" fill="#dbeafe"/><text x="142" y="271" text-anchor="middle" font-size="12" fill="#1e40af">読書中</text>')
o.append('<rect x="174" y="256" width="40" height="22" rx="11" fill="#dcfce7"/><text x="194" y="271" text-anchor="middle" font-size="12" fill="#166534">読了</text>')


def callout(x, y, w, text, tx, ty):
    o.append(f'<line x1="{tx}" y1="{ty}" x2="{x + w / 2:.0f}" y2="{y}" stroke="#f97316" stroke-width="1.5" stroke-dasharray="4 3"/>')
    o.append(f'<rect x="{x}" y="{y}" width="{w}" height="24" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="1.5"/>')
    o.append(f'<text x="{x + w / 2:.0f}" y="{y + 16}" text-anchor="middle" font-size="12" fill="#c2410c">{e(text)}</text>')


callout(146, 114, 284, "① Header.tsx（layout.tsx に置いた共通部分）", 160, 100)
callout(250, 312, 290, "② page.tsx の中身（{children} に入る部分）", 400, 294)
callout(440, 114, 256, "③ 現在地の「書籍一覧」が薄い青になる", 553, 101)
o += ['</svg>', cap('図X: http://localhost:3000 を開いたときの画面イメージ（実際の表示を元に作図）'), '</div>']
write("figG.html", o)
print("done")
