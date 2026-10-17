p = 'parts/04.md'
s = open(p, encoding='utf-8').read()
pairs = [
 ('<path d="M204,154 C330,154 420,150 442,74" fill="none" stroke="#10b981" stroke-width="2" marker-end="url(#ar-03-16)"/>',
  '<path d="M204,154 C320,154 400,118 436,74" fill="none" stroke="#10b981" stroke-width="2" marker-end="url(#ar-03-16)"/>'),
 ('<text x="494" y="122" text-anchor="middle" font-size="12" fill="#047857">part = &amp;owned[6..10]（6 から 4 バイト分）</text>',
  '<text x="560" y="140" text-anchor="middle" font-size="12" fill="#047857">part = &amp;owned[6..10]（6 から 4 バイト分）</text>'),
]
for a, b in pairs:
    assert s.count(a) == 1, a
    s = s.replace(a, b)
n = s.count('font-size="13" fill="#64748b" font-family="Consolas, Menlo, monospace">␣</text>')
s = s.replace('font-size="13" fill="#64748b" font-family="Consolas, Menlo, monospace">␣</text>', 'font-size="16" font-weight="700" fill="#64748b" font-family="Consolas, Menlo, monospace">·</text>')
print('spaces replaced', n)
open(p, 'w', encoding='utf-8', newline='\n').write(s)
