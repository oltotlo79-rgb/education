"""第0巻〜第13巻の「用語ミニ辞典」を集めて、全巻の用語集・索引（parts/glossary_all.md）を作る。

- 用語ごとに1行。意味は、いちばん前の巻の説明を使う。「出てくる所」に、載っている巻と単元を全部並べる（索引）。
- 読みの五十音順に、行（あ行・か行…）に分ける。英字・記号で始まる用語は最初にまとめる。
- 読みは、第0・2・3巻の表の「読み」の列があればそれを、無ければ pykakasi で用語から作る。
使い方: venv13 の python で  python glossary13.py
"""
import re
import sys
import unicodedata
from pathlib import Path

import pykakasi

REPO = Path("C:/Users/oltot/Documents/git-projects/def/Rust")
HERE = Path(__file__).resolve().parent
kks = pykakasi.kakasi()

GROUPS = [
    ("あ行", "あいうえおぁぃぅぇぉゔ"),
    ("か行", "かきくけこ"),
    ("さ行", "さしすせそ"),
    ("た行", "たちつてとっ"),
    ("な行", "なにぬねの"),
    ("は行", "はひふへほ"),
    ("ま行", "まみむめも"),
    ("や行", "やゆよゃゅょ"),
    ("ら行", "らりるれろ"),
    ("わ行", "わをんゎ"),
]


def to_hira(text: str) -> str:
    return "".join(item["hira"] for item in kks.convert(text))


def base_kana(ch: str) -> str:
    """が → か、ぱ → は のように、濁点・半濁点を外す"""
    return unicodedata.normalize("NFD", ch)[0]


def group_of(reading: str) -> str:
    first = base_kana(reading[:1]) if reading else ""
    for name, chars in GROUPS:
        if first in chars:
            return name
    return "英字・記号"


def plain(term: str) -> str:
    return unicodedata.normalize("NFKC", term.replace("`", "")).strip()


def split_row(line: str, ncols: int):
    cells = [c.strip() for c in line.strip().strip("|").split("|")]
    if len(cells) > ncols:
        # 意味の中の | （コードの中など）を、もとに戻す
        extra = len(cells) - ncols
        mid = 1 if ncols == 2 else (2 if ncols == 4 else 1)
        cells = cells[:mid] + [" | ".join(cells[mid:mid + extra + 1])] + cells[mid + extra + 1:]
    return cells


def unit_text(raw: str) -> str:
    raw = raw.strip()
    if re.fullmatch(r"[\d・、, ]+", raw):
        return "UNIT " + raw
    return raw


entries = {}  # plain(term) -> dict
order = []
for md in sorted(REPO.glob("[0-9][0-9]_*.md")):
    vol = int(md.name[:2])
    text = md.read_text(encoding="utf-8")
    start = text.find("\n## 用語ミニ辞典")
    if start < 0:
        print("no glossary:", md.name, file=sys.stderr)
        continue
    end = text.find("\n## ", start + 5)
    section = text[start: end if end > 0 else len(text)]
    unit = ""
    ncols = 0
    reading_col = False
    for line in section.splitlines():
        m = re.match(r"^### (UNIT \d\d) の用語", line)
        if m:
            unit = m.group(1)
            continue
        if not line.startswith("|"):
            continue
        if re.match(r"^\|\s*-", line):
            continue
        head = [c.strip() for c in line.strip().strip("|").split("|")]
        if head[0] == "用語":
            ncols = len(head)
            # 第2巻の2列目は「読み・別名」（別名が入る）なので、読みとしては使わない
            reading_col = ncols == 4 and head[1] == "読み"
            continue
        cells = split_row(line, ncols)
        term = cells[0]
        reading = ""
        if ncols == 4:
            reading, meaning, where = (cells[1] if reading_col else ""), cells[2], unit_text(cells[3])
        elif ncols == 3:
            meaning, where = cells[1], unit_text(cells[2])
        else:
            meaning, where = cells[1], unit
        # 「冪等」と「冪等（べきとう）」のように、全角のかっこの補足だけが違う物は1つにまとめる
        key = plain(re.sub(r"（.*?）", "", term)) or plain(term)
        place = f"第{vol}巻 {where}".strip()
        if key not in entries:
            entries[key] = {"term": term, "meaning": meaning, "reading": reading, "places": []}
            order.append(key)
        e = entries[key]
        if not e["reading"] and reading:
            e["reading"] = reading
        if place not in e["places"]:
            e["places"].append(place)

rows = []
for key in order:
    e = entries[key]
    first = plain(e["term"])[:1]
    if first and (first.isascii() or not unicodedata.category(first).startswith("L")):
        # 英字は、最初の英字ごとに分ける（# や ` などの記号は飛ばして見る）
        letters = re.sub(r"^[^A-Za-z0-9]+", "", plain(e["term"]))
        head_ch = letters[:1].upper()
        group = head_ch if head_ch.isalpha() else "記号・数字"
        sort_key = letters.lower()
    else:
        reading = to_hira(e["reading"] or re.sub(r"（.*?）|\(.*?\)", "", plain(e["term"])))
        group = group_of(reading)
        sort_key = reading
    rows.append((group, sort_key, e))

names = ["記号・数字"] + [chr(c) for c in range(ord("A"), ord("Z") + 1)] + [g for g, _ in GROUPS]
out = []
out.append("## 全巻の用語集・索引")
out.append("")
out.append(f"第0巻から第13巻までの「用語ミニ辞典」に載せた用語を、1つにまとめました（{len(rows)} 語）。英字で始まる用語を最初にアルファベット順に（`#[…]` のような記号は飛ばして、最初の英字で）、そのあとは読みの五十音順に並べています。")
out.append("")
out.append("- **意味**は、その用語を、いちばん前の巻で説明したときの言葉です。")
out.append("- **出てくる所**は、その用語を用語ミニ辞典に載せた巻と単元です。索引として、くわしい説明を探すときに使います。")
out.append("- 同じ用語が、巻によって少し違う言葉で説明されていることがあります。迷ったら、出てくる所の、いちばん後ろの巻を読みます。")
for name in names:
    part = sorted((r for r in rows if r[0] == name), key=lambda r: (r[1], plain(r[2]["term"])))
    if not part:
        continue
    out.append("")
    out.append(f"### {name}")
    out.append("")
    out.append("| 用語 | 意味 | 出てくる所 |")
    out.append("|---|---|---|")
    for _, _, e in part:
        out.append(f"| {e['term']} | {e['meaning']} | {'、'.join(e['places'])} |")
(HERE / "parts" / "glossary_all.md").write_text("\n".join(out) + "\n", encoding="utf-8", newline="\n")
counts = {n: sum(1 for r in rows if r[0] == n) for n in names}
print(len(rows), counts)
