"""Extract sample code from next/09-styling.md exactly as written and re-run it."""
import os
import re
import shutil
import subprocess
from pathlib import Path

MD = Path(r'C:/Users/oltot/Documents/git-projects/def/next/09-styling.md')
ROOT = Path(r'C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/next09')
W = ROOT / 'verify'
NPX_PREFIX = str(ROOT)

s = MD.read_text(encoding='utf-8')
blocks = [(m.start(), m.group(1), m.group(2)) for m in re.finditer(r'```(\w*)\n(.*?)```', s, re.S)]


def block_starting(first_line, lang=None, nth=0):
    found = [b for b in blocks if b[2].split('\n', 1)[0] == first_line and (lang is None or b[1] == lang)]
    return found[nth]


def next_text_block(pos):
    for b in blocks:
        if b[0] > pos and b[1] in ('text', 'css'):
            return b[2]
    raise ValueError


def run(cmd, cwd):
    r = subprocess.run(cmd, cwd=cwd, shell=True, capture_output=True, text=True, encoding='utf-8')
    return r.stdout + r.stderr


def utilities(css_text):
    i = css_text.find('@layer utilities')
    j = css_text.find('\n}\n', i)
    return css_text[i:j + 2] if j > 0 else css_text[i:]


def cli(d, inp='input.css', out='output.css'):
    log = run(f'npx --prefix "{NPX_PREFIX}" @tailwindcss/cli -i {inp} -o {out}', d)
    return log, (d / out).read_text(encoding='utf-8')


if W.exists():
    shutil.rmtree(W)
W.mkdir()
results = []

# ---- node samples ----
for name in ('// cn.ts', '// breakpoints.ts', '// contrast.ts'):
    pos, lang, code = block_starting(name)
    f = W / name[3:]
    f.write_text(code, encoding='utf-8')
    out = run(f'node {f.name}', W)
    expected = next_text_block(pos)
    results.append((name, out.strip() == expected.strip(), out))

# ---- 0.3 CLI sample ----
d = W / 's1'; d.mkdir()
pos_in, _, css_in = block_starting('/* input.css: Tailwind を読み込むだけの1行 */')
(d / 'input.css').write_text(css_in, encoding='utf-8')
_, _, html = block_starting('<!doctype html>', 'html')
(d / 'index.html').write_text(html, encoding='utf-8')
log, out = cli(d)
exp = next_text_block(s.index('（できた `output.css` の抜粋'))
results.append(('0.3 utilities', utilities(out).strip() in exp, utilities(out)))
results.append(('0.3 log', 'tailwindcss v4.3.3' in log, log))

# ---- 0.4 dynamic class NG/OK ----
for tag, first in (('ng', '// ng/StatusBadge.tsx（NG 版）'), ('ok', '// ok/StatusBadge.tsx（OK 版）')):
    d = W / ('s2' + tag); d.mkdir()
    _, _, code = block_starting(first)
    (d / 'StatusBadge.tsx').write_text(code, encoding='utf-8')
    (d / 'input.css').write_text(css_in, encoding='utf-8')
    log, out = cli(d)
    exp = next_text_block(s.index('`@layer utilities` の部分を見たもの）'))
    u = utilities(out).strip()
    results.append(('0.4 ' + tag, u in exp, u))

# ---- 1.1.1 mini v4 globals ----
d = W / 's4'; (d / 'app').mkdir(parents=True)
_, _, gcss = block_starting('/* app/globals.css（v4 版の最小構成） */')
(d / 'app/globals.css').write_text(gcss, encoding='utf-8')
_, _, page = block_starting('// app/page.tsx（上の CSS で作ったクラスを使ってみる）')
(d / 'app/page.tsx').write_text(page, encoding='utf-8')
log, out = cli(d, 'app/globals.css', 'out.css')
exp = next_text_block(s.index('`page.tsx` で使ったクラスだけが作られている）'))
results.append(('1.1.1 mini', utilities(out).strip() == exp.strip(), utilities(out)))

# ---- 1.1.1 tsumazuki @config ----
for tag, cssfirst in (('ng', '/* NG: app/globals.css が @import だけ → 上の設定ファイルは読まれない */'),
                      ('ok', '/* OK その1: @config で設定ファイルの場所を教える（globals.css から見た相対パス） */')):
    d = W / ('cfg' + tag); (d / 'app').mkdir(parents=True)
    _, _, cfg = block_starting('// tailwind.config.ts（v3 時代の記事どおりに作った設定ファイル）')
    (d / 'tailwind.config.ts').write_text(cfg, encoding='utf-8')
    _, _, c = block_starting(cssfirst)
    (d / 'app/globals.css').write_text(c, encoding='utf-8')
    (d / 'app/page.tsx').write_text('export default function Page() {\n  return <button className="bg-primary-600 text-white">保存</button>;\n}\n', encoding='utf-8')
    log, out = cli(d, 'app/globals.css', 'out.css')
    exp = next_text_block(s.index('NG と OK その1 を CLI で実際に比べた結果が次です。'))
    results.append(('config ' + tag, utilities(out).strip() in exp, utilities(out) + log[-200:]))

# ---- 5.1 dark NG/OK ----
for tag, first in (('ng', '/* NG: v4 で何も設定していない → dark: は OS の設定にしか反応しない */'),
                   ('ok', '/* OK: dark: を「dark クラスが付いた要素とその中身」で効かせる */')):
    d = W / ('dark' + tag); d.mkdir()
    _, _, c = block_starting(first)
    (d / 'input.css').write_text(c, encoding='utf-8')
    (d / 'index.html').write_text('<div class="bg-white dark:bg-slate-900">本棚</div>\n', encoding='utf-8')
    log, out = cli(d)
    u = utilities(out)
    exp = next_text_block(s.index('（それぞれの出力の `dark:bg-slate-900` の部分）'))
    part = u[u.index('dark') - 40:] if 'prefers' in u else u
    seg = [l for l in u.split('\n') if 'dark' in l or 'slate-900' in l or 'prefers' in l]
    results.append(('dark ' + tag, all(x.strip() in exp for x in seg), '\n'.join(seg)))

# ---- Q4 ----
pos = s.index('### 問題4')
q = [b for b in blocks if b[0] > pos][:2]
for tag, (_, _, code) in zip(('ng', 'ok'), q):
    d = W / ('q4' + tag); d.mkdir()
    (d / 'Button.tsx').write_text(code, encoding='utf-8')
    (d / 'input.css').write_text(css_in, encoding='utf-8')
    log, out = cli(d)
    exp = next_text_block(s.index('（直す前と後のファイルを CLI に通した'))
    results.append(('Q4 ' + tag, utilities(out).strip() in exp, utilities(out)))

# ---- Q5 ----
pos = s.index('### 問題5')
code = [b for b in blocks if b[0] > pos][0][2]
d = W / 'q5'; d.mkdir()
(d / 'input.css').write_text(code, encoding='utf-8')
(d / 'index.html').write_text('<div class="bg-brand-500 dark:bg-slate-900"></div>\n', encoding='utf-8')
log, out = cli(d)
exp = next_text_block(s.index('（`<div class="bg-brand-500 dark:bg-slate-900"></div>` を CLI に通した'))
results.append(('Q5', utilities(out).strip() == exp.strip(), utilities(out)))

# ---- main 1.1 globals.css compiled with all chapter code ----
d = W / 'app'; (d / 'app').mkdir(parents=True)
_, _, main_css = block_starting('/* app/globals.css', 'css')
(d / 'app/globals.css').write_text(main_css, encoding='utf-8')
for k, (_, lang, code) in enumerate(blocks):
    if lang in ('typescript', 'tsx', 'ts'):
        (d / 'app' / f'ex{k:03d}.tsx').write_text(code, encoding='utf-8')
log, out = cli(d, 'app/globals.css', 'out.css')
needed = ['.btn-primary', '.card-hover', '.badge-warning', '.container', '.glass', '.animate-shimmer',
          '.animate-fade-in', '.animate-toast-out', '.text-2xs', '.line-clamp-2', '.bg-background',
          'hover\\:shadow-card-hover', '.dark\\:bg-gray-900']
missing = [n for n in needed if n not in out]
results.append(('main globals.css', 'Done' in log and not missing and 'rror' not in log, f'missing={missing} log={log.strip()[-120:]}'))

for name, ok, detail in results:
    print(('OK  ' if ok else 'NG  ') + name)
    if not ok:
        print(detail)
