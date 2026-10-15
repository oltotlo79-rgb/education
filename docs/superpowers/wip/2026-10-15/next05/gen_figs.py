# Generates inline SVG figures for next/05-supabase-setup.md and replaces
# placeholder comments like <!--FIG:5-3--> with the figure block.
import sys
from pathlib import Path

MD = Path(r"C:/Users/oltot/Documents/git-projects/def/next/05-supabase-setup.md")
FONT = "Yu Gothic UI, Yu Gothic, Meiryo, sans-serif"
MONO = "Consolas, Cascadia Mono, monospace"

BLUE = ("#eff6ff", "#3b82f6", "#1e40af")
ORANGE = ("#fff7ed", "#f97316", "#c2410c")
GREEN = ("#ecfdf5", "#10b981", "#047857")
GRAY = ("#f8fafc", "#64748b", "#475569")


def esc(s: str) -> str:
    return s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def block(num: str, w: int, h: int, body: list[str], caption: str, marker: str | None = None, extra_defs: list[str] | None = None) -> str:
    lines = [
        '<div style="max-width: 720px; margin: 18px auto; text-align: center;">',
        f'<svg viewBox="0 0 {w} {h}" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="{FONT}">',
    ]
    defs = []
    if marker:
        defs.append(f'<marker id="{marker}" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>')
    if extra_defs:
        defs += extra_defs
    if defs:
        lines.append("<defs>")
        lines += defs
        lines.append("</defs>")
    lines += [l for l in body if l.strip()]
    lines.append("</svg>")
    lines.append(f'<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図{num}: {caption}</div>')
    lines.append("</div>")
    return "\n".join(lines)


def t(x, y, s, size=12, fill="#1e293b", anchor="start", bold=False, mono=False, extra=""):
    a = f' text-anchor="{anchor}"' if anchor != "start" else ""
    b = ' font-weight="bold"' if bold else ""
    m = f' font-family="{MONO}"' if mono else ""
    return f'<text x="{x}" y="{y}"{a} font-size="{size}"{b}{m} fill="{fill}"{extra}>{esc(s)}</text>'


def rect(x, y, w, h, fill="#ffffff", stroke="#cbd5e1", sw=1, rx=0, extra=""):
    r = f' rx="{rx}"' if rx else ""
    return f'<rect x="{x}" y="{y}" width="{w}" height="{h}"{r} fill="{fill}" stroke="{stroke}" stroke-width="{sw}"{extra}/>'


def line(x1, y1, x2, y2, stroke="#475569", sw=2, marker=None, dash=None):
    m = f' marker-end="url(#{marker})"' if marker else ""
    d = f' stroke-dasharray="{dash}"' if dash else ""
    return f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="{stroke}" stroke-width="{sw}"{d}{m}/>'


def path(d, stroke="#475569", sw=2, marker=None, dash=None, fill="none"):
    m = f' marker-end="url(#{marker})"' if marker else ""
    da = f' stroke-dasharray="{dash}"' if dash else ""
    return f'<path d="{d}" fill="{fill}" stroke="{stroke}" stroke-width="{sw}"{da}{m}/>'


# ---------------------------------------------------------------- 5-3
def mini_table(x0, y0, cols, rows, styles=None):
    """cols: list of (name, width). rows: list of list[str]. styles: {row_index: dict(fill, textfill, strike, cellfill={col: color}, bold_cols=set())}"""
    styles = styles or {}
    out = []
    rh = 22
    total = sum(w for _, w in cols)
    out.append(rect(x0, y0, total, rh, fill="#f1f5f9", stroke="#cbd5e1"))
    cx = x0
    for name, w in cols:
        out.append(t(cx + 6, y0 + 16, name, 12, "#334155", bold=True))
        cx += w
    for i, row in enumerate(rows):
        y = y0 + rh * (i + 1)
        st = styles.get(i, {})
        out.append(rect(x0, y, total, rh, fill=st.get("fill", "#ffffff"), stroke="#cbd5e1"))
        cx = x0
        for j, ((name, w), val) in enumerate(zip(cols, row)):
            if j in st.get("cellfill", {}):
                out.append(rect(cx, y, w, rh, fill=st["cellfill"][j], stroke="#cbd5e1"))
            out.append(t(cx + 6, y + 16, val, 12, st.get("textfill", "#1e293b"), bold=j in st.get("bold_cols", set())))
            cx += w
        if st.get("strike"):
            out.append(line(x0 + 4, y + 11, x0 + total - 4, y + 11, stroke="#64748b", sw=1.5))
        if st.get("outline"):
            out.append(rect(x0, y, total, rh, fill="none", stroke=st["outline"], sw=2.5))
    # vertical lines
    cx = x0
    for _, w in cols[:-1]:
        cx += w
        out.append(line(cx, y0, cx, y0 + rh * (len(rows) + 1), stroke="#cbd5e1", sw=1))
    return out


def fig_5_3():
    cols = [("id", 34), ("title", 128), ("status", 82)]
    base = [["1", "リーダブルコード", "done"], ["2", "プロを目指す人〜", "reading"], ["3", "達人プログラマー", "unread"]]
    panels = [
        (8, 8, BLUE, "SELECT（読む）", "SELECT * FROM books", "WHERE status = 'reading';", base,
         {0: {"textfill": "#94a3b8"}, 1: {"fill": "#dbeafe", "outline": "#3b82f6", "bold_cols": {1, 2}}, 2: {"textfill": "#94a3b8"}},
         ["条件に合う", "行だけを", "取り出す。", "表そのものは", "変わらない"]),
        (368, 8, GREEN, "INSERT（追加）", "INSERT INTO books (…)", "VALUES (4, 'SQL入門', 'unread');", base + [["4", "SQL入門", "unread"]],
         {3: {"fill": "#d1fae5", "outline": "#10b981", "bold_cols": {0, 1, 2}}},
         ["一番下に", "行が1つ", "増える"]),
        (8, 184, ORANGE, "UPDATE（書き換え）", "UPDATE books SET status = 'done'", "WHERE id = 2;", base,
         {1: {"cellfill": {2: "#ffedd5"}, "bold_cols": {2}, "outline": "#f97316"}},
         ["id=2 の行の", "status だけ", "reading →", "done に", "変わる"]),
        (368, 184, GRAY, "DELETE（削除）", "DELETE FROM books", "WHERE id = 3;", base,
         {2: {"fill": "#e2e8f0", "textfill": "#94a3b8", "strike": True}},
         ["id=3 の行が", "消える。", "元に戻す", "命令は無い"]),
    ]
    body = []
    for (px, py, col, title, sql1, sql2, rows, styles, note) in panels:
        if title.startswith("UPDATE"):
            rows = [r[:] for r in rows]
            rows[1][2] = "done"
        body.append(rect(px, py, 344, 168, fill=col[0], stroke=col[1], sw=2, rx=10))
        body.append(t(px + 14, py + 24, title, 14, col[2], bold=True))
        body.append(t(px + 14, py + 42, sql1, 12, "#334155", mono=True))
        body.append(t(px + 14, py + 57, sql2, 12, "#334155", mono=True))
        body += mini_table(px + 14, py + 64, cols, rows, styles)
        for k, s in enumerate(note):
            body.append(t(px + 268, py + 86 + k * 17, s, 12, col[2] if k == 0 else "#475569", bold=(k == 0)))
    return block("5-3", 720, 360, body, "同じ3行の表に4つの命令を実行したときの変化（色の付いた行が変わった所）")


# ---------------------------------------------------------------- 5-4
def fig_5_4():
    cols = [("id", 40), ("title", 128), ("status", 82)]
    before = [["1", "リーダブルコード", "done"], ["2", "プロを目指す人〜", "reading"], ["4", "SQL入門", "unread"]]
    body = []
    mk = "ar-05-04"
    for (y, col, label, sql, sub, after, styles, result) in [
        (14, ORANGE, "NG", "UPDATE practice_books", "SET status = 'done';",
         [["1", "リーダブルコード", "done"], ["2", "プロを目指す人〜", "done"], ["4", "SQL入門", "done"]],
         {i: {"cellfill": {2: "#ffedd5"}, "bold_cols": {2}} for i in range(3)}, "UPDATE 3（全行が変わった）"),
        (164, GREEN, "OK", "UPDATE practice_books", "SET status = 'done' WHERE id = 2;",
         [["1", "リーダブルコード", "done"], ["2", "プロを目指す人〜", "done"], ["4", "SQL入門", "unread"]],
         {1: {"cellfill": {2: "#d1fae5"}, "bold_cols": {2}}}, "UPDATE 1（狙った1行だけ）"),
    ]:
        body.append(rect(6, y - 8, 708, 140, fill=col[0], stroke=col[1], sw=1.5, rx=10))
        body.append(t(20, y + 14, "変更前", 12, "#475569", bold=True))
        body += mini_table(20, y + 22, cols, before)
        body.append(t(360, y + 22, label, 18, col[2], anchor="middle", bold=True))
        body.append(t(360, y + 44, sql, 12, "#334155", anchor="middle", mono=True))
        body.append(t(360, y + 60, sub, 12, "#334155", anchor="middle", mono=True))
        body.append(line(282, y + 76, 446, y + 76, stroke="#475569", sw=2.5, marker=mk))
        body.append(t(360, y + 100, result, 12, col[2], anchor="middle", bold=True))
        body.append(t(452, y + 14, "変更後", 12, "#475569", bold=True))
        body += mini_table(452, y + 22, cols, after, styles)
    return block("5-4", 720, 300, body, "WHERE を忘れた UPDATE（上）と、正しく絞り込んだ UPDATE（下）", marker=mk)


FIGS = {"5-3": fig_5_3, "5-4": fig_5_4}

if __name__ == "__main__":
    text = MD.read_text(encoding="utf-8")
    for key, fn in FIGS.items():
        ph = f"<!--FIG:{key}-->"
        if ph in text:
            text = text.replace(ph, fn())
            print("replaced", key)
    MD.write_text(text, encoding="utf-8")
