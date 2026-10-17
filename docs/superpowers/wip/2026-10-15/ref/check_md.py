"""教材 Markdown の機械チェック。

使い方:
  python check_md.py <file.md>            # チェックのみ
  python check_md.py <file.md> --fix      # 隣り合う引用囲みの結合だけ自動修正

チェック内容:
  1. コードフェンス ``` の数が偶数か
  2. <svg> / </svg>、<div> / </div> の数が一致するか（コードブロックの外）
  3. id="..." の重複
  4. 空行1つを挟んで、ラベル付きの引用（> **用語:** 等）が直前の引用に続く → python-markdown が1つの囲みに結合してしまう
  5. コードブロック外の行で ** の数が奇数（太字の閉じ忘れ）
  6. HTML ブロック内の空行（<div ...> から </div> の間）
  7. 行末コメント（コードブロック内で、コードの後ろに // や # のコメントがある行）の件数（参考）
終了コード: 問題があれば 1
"""
from __future__ import annotations

import re
import sys
from pathlib import Path


def main() -> int:
    path = Path(sys.argv[1])
    fix = "--fix" in sys.argv
    raw = path.read_bytes().decode("utf-8")
    crlf = "\r\n" in raw
    lines = raw.replace("\r\n", "\n").split("\n")
    problems: list[str] = []

    fences = sum(1 for l in lines if l.lstrip().startswith("```"))
    if fences % 2:
        problems.append(f"コードフェンスが奇数個 ({fences})")

    in_code = False
    outside: list[tuple[int, str]] = []
    trailing_comments = 0
    for i, l in enumerate(lines, 1):
        if l.lstrip().startswith("```"):
            in_code = not in_code
            continue
        if in_code:
            if re.search(r"\S.*\s(//|#)\s", l) and not l.lstrip().startswith(("//", "#")) and "http" not in l:
                trailing_comments += 1
            continue
        outside.append((i, l))

    # インラインコード（`<div>` の説明など）は構造チェックの対象外にする
    outside = [(i, re.sub(r"`[^`]*`", "", l)) for i, l in outside]
    text_out = "\n".join(l for _, l in outside)
    svg_count = len(re.findall(r"<svg[\s>]", text_out))
    for tag in ("svg", "div"):
        o = len(re.findall(rf"<{tag}[\s>]", text_out))
        c = len(re.findall(rf"</{tag}>", text_out))
        if o != c:
            problems.append(f"<{tag}> の開き {o} / 閉じ {c} が不一致")

    ids = re.findall(r'\bid="([^"]+)"', text_out)
    dup = sorted({x for x in ids if ids.count(x) > 1})
    if dup:
        problems.append(f"id の重複: {dup[:10]}")

    merge_at: list[int] = []
    in_code = False
    for idx in range(len(lines) - 2):
        if lines[idx].lstrip().startswith("```"):
            in_code = not in_code
        if in_code:
            continue
        if lines[idx].startswith(">") and lines[idx + 1].strip() == "" and lines[idx + 2].startswith("> **"):
            merge_at.append(idx)
    if merge_at:
        if fix:
            for idx in reversed(merge_at):
                lines[idx + 2 : idx + 2] = ["<!-- -->", ""]
            print(f"修正: 隣り合う引用囲み {len(merge_at)} か所を <!-- --> で分離")
        else:
            problems.append(f"隣り合う引用囲みが結合する箇所: {len(merge_at)} か所（行 {[m + 1 for m in merge_at[:10]]}）。--fix で分離可")

    for i, l in outside:
        if l.lstrip().startswith("<"):
            continue
        s = l
        if s.count("**") % 2:
            problems.append(f"太字 ** の閉じ忘れ? 行 {i}: {l[:80]}")

    depth = 0
    for i, l in outside:
        depth += len(re.findall(r"<div[\s>]", l)) - len(re.findall(r"</div>", l))
        if depth > 0 and l.strip() == "":
            problems.append(f"HTML ブロック内の空行: 行 {i}")
            break

    if fix and merge_at:
        out = "\n".join(lines)
        if crlf:
            out = out.replace("\n", "\r\n")
        path.write_bytes(out.encode("utf-8"))

    print(f"{path.name}: 行数 {len(lines)}, フェンス {fences}, svg {svg_count}, 行末コメント(参考) {trailing_comments}")
    for p in problems:
        print("  NG:", p)
    if not problems:
        print("  OK")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
