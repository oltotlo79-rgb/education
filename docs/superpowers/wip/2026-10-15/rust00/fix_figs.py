import pathlib
import re

P = pathlib.Path(r"C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust00/parts")
MONO = 'font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace"'


def edit(name, pairs):
    p = P / name
    t = p.read_text(encoding="utf-8")
    for a, b in pairs:
        if a not in t:
            raise SystemExit(f"NOT FOUND in {name}: {a[:70]}")
        t = t.replace(a, b)
    p.write_text(t, encoding="utf-8", newline="\n")


# 2. backslash paths inside proportional-font SVG text -> monospace
for name in ["unit02.md", "unit04.md"]:
    p = P / name
    lines = p.read_text(encoding="utf-8").split("\n")
    for i, l in enumerate(lines):
        if l.startswith("<text ") and "\\" in l and "font-family" not in l:
            lines[i] = l.replace("<text ", "<text " + MONO + " ", 1)
    p.write_text("\n".join(lines), encoding="utf-8", newline="\n")

# 1. preserve spaces in monospace groups
edit("unit03.md", [
    ('<g font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" font-size="13">',
     '<g font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" font-size="13" xml:space="preserve">'),
])
edit("unit06.md", [
    ('<g font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" font-size="12" fill="#e2e8f0">',
     '<g font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" font-size="12" fill="#e2e8f0" xml:space="preserve">'),
    # 3. Fig 6-2 labels
    ('<text x="164" y="150" font-size="12" fill="#047857" text-anchor="middle">同じ PC の中からは届く</text>',
     '<text x="30" y="150" font-size="12" fill="#047857">同じ PC の中からは届く</text>'),
    ('<text x="164" y="186" font-size="12" fill="#b91c1c" text-anchor="middle">外の PC からは届かない ×</text>',
     '<text x="30" y="178" font-size="12" fill="#b91c1c">外の PC からは</text>\n<text x="30" y="196" font-size="12" fill="#b91c1c">届かない ×</text>'),
    ('<text x="540" y="150" font-size="12" fill="#047857" text-anchor="middle">外からの https を 8080 番へ取り次ぐ</text>',
     '<text x="620" y="150" font-size="12" fill="#047857" text-anchor="middle">外からの https を</text>\n<text x="620" y="168" font-size="12" fill="#047857" text-anchor="middle">8080 番へ取り次ぐ</text>'),
])

# 4. Fig 7-1 labels, 5. Fig 7-3 arrows
edit("unit07.md", [
    ('<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">\n<defs>\n<marker id="ar-00-17"',
     '<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">\n<defs>\n<marker id="ar-00-17"'),
    ('<text x="324" y="226" font-size="12" fill="#047857" text-anchor="middle">引き継ぐ</text>\n<text x="514" y="226" font-size="12" fill="#047857" text-anchor="middle">引き継ぐ</text>\n<text x="324" y="150" font-size="12" fill="#b91c1c" text-anchor="middle">消える</text>\n<text x="514" y="150" font-size="12" fill="#b91c1c" text-anchor="middle">消える</text>',
     '<text x="20" y="140" font-size="12" fill="#b91c1c">毎回 0 から</text>\n<text x="360" y="258" font-size="12" fill="#334155" text-anchor="middle">メモリの値はプロセスの終わりに消える　／　緑の矢印 = ファイルの値を次の起動が引き継ぐ</text>'),
    ('<path d="M478,84 C380,84 380,79 288,79" fill="none" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-18)"/>\n<path d="M478,114 C390,114 380,82 288,82" fill="none" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-18)"/>\n<path d="M478,144 C380,144 380,109 288,109" fill="none" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-18)"/>',
     '<path d="M400,79 C360,79 330,77 290,77" fill="none" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-18)"/>\n<path d="M400,109 C350,109 340,83 290,83" fill="none" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-18)"/>\n<path d="M400,139 C350,139 340,109 290,109" fill="none" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-18)"/>\n<text x="344" y="168" font-size="12" fill="#c2410c" text-anchor="middle">user_id の値で</text>\n<text x="344" y="184" font-size="12" fill="#c2410c" text-anchor="middle">持ち主を指す</text>'),
    ('<text x="24" y="164" font-size="12" fill="#334155">UNIQUE = 同じメールアドレスは2回登録できない</text>\n<text x="24" y="188" font-size="12" fill="#334155">user_id の矢印 = 「このノートの持ち主は users の何番か」</text>\n<text x="24" y="210" font-size="12" fill="#334155">→ Hanako（1番）のノートは 1・2、Taro（2番）のノートは 3</text>',
     '<text x="24" y="148" font-size="12" fill="#334155">UNIQUE = 同じメールアドレスは</text>\n<text x="24" y="164" font-size="12" fill="#334155">2回登録できない</text>\n<text x="24" y="210" font-size="12" fill="#334155">Hanako（1番）のノートは 1・2、Taro（2番）のノートは 3</text>'),
])

# 6. Fig 8-3 text below curve
edit("unit08.md", [
    ('<text x="360" y="160" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">× 逆向きには戻せない（一方向）</text>\n<text x="360" y="200" font-size="12" fill="#334155" text-anchor="middle">',
     '<text x="360" y="184" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">× 逆向きには戻せない（一方向）</text>\n<text x="360" y="208" font-size="12" fill="#334155" text-anchor="middle">'),
    ('<svg viewBox="0 0 720 220" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">\n<defs>\n<marker id="ar-00-21"',
     '<svg viewBox="0 0 720 222" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">\n<defs>\n<marker id="ar-00-21"'),
])

# 7. Fig 9-1 preserve + leader lines from right edge of terminal
edit("unit09.md", [
    ('<g font-family="Cascadia Mono, Consolas, monospace" font-size="12.5" fill="#e2e8f0">',
     '<g font-family="Cascadia Mono, Consolas, monospace" font-size="12.5" fill="#e2e8f0" xml:space="preserve">'),
    ('<line x1="132" y1="32" x2="452" y2="32" stroke="#dc2626" stroke-width="1.5"/>\n<line x1="210" y1="54" x2="452" y2="80" stroke="#3b82f6" stroke-width="1.5"/>\n<line x1="330" y1="120" x2="452" y2="134" stroke="#dc2626" stroke-width="1.5"/>\n<line x1="300" y1="200" x2="452" y2="188" stroke="#3b82f6" stroke-width="1.5"/>\n<line x1="210" y1="230" x2="452" y2="246" stroke="#10b981" stroke-width="1.5"/>\n<line x1="260" y1="318" x2="452" y2="306" stroke="#0284c7" stroke-width="1.5"/>\n<line x1="330" y1="392" x2="452" y2="372" stroke="#475569" stroke-width="1.5"/>',
     '<circle cx="420" cy="32" r="3" fill="#dc2626"/>\n<line x1="420" y1="32" x2="452" y2="32" stroke="#dc2626" stroke-width="1.5"/>\n<circle cx="420" cy="54" r="3" fill="#3b82f6"/>\n<line x1="420" y1="54" x2="452" y2="80" stroke="#3b82f6" stroke-width="1.5"/>\n<circle cx="420" cy="120" r="3" fill="#dc2626"/>\n<line x1="420" y1="120" x2="452" y2="134" stroke="#dc2626" stroke-width="1.5"/>\n<circle cx="420" cy="186" r="3" fill="#3b82f6"/>\n<line x1="420" y1="186" x2="452" y2="188" stroke="#3b82f6" stroke-width="1.5"/>\n<circle cx="420" cy="230" r="3" fill="#10b981"/>\n<line x1="420" y1="230" x2="452" y2="246" stroke="#10b981" stroke-width="1.5"/>\n<circle cx="420" cy="318" r="3" fill="#0284c7"/>\n<line x1="420" y1="318" x2="452" y2="306" stroke="#0284c7" stroke-width="1.5"/>\n<circle cx="420" cy="392" r="3" fill="#475569"/>\n<line x1="420" y1="392" x2="452" y2="372" stroke="#475569" stroke-width="1.5"/>'),
    ('図9-1: エラーメッセージの部品（紙面に合わせて長い行は折り返している）。①→②→③の順に読み、⑥の提案を検討する',
     '図9-1: エラーメッセージの部品（紙面に合わせて長い行は折り返している。右端の点がその行）。①→②→③の順に読み、⑥の提案を検討する'),
])

# 8. Fig 10-4 viewBox
edit("unit10.md", [
    ('<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">\n<defs>\n<marker id="ar-00-26"',
     '<svg viewBox="0 72 720 176" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">\n<defs>\n<marker id="ar-00-26"'),
])
print("done")
