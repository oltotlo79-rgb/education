p = 'parts/10.md'
s = open(p, encoding='utf-8').read()
pairs = [
 ('<text x="194" y="118" font-size="11" fill="#64748b">lock() で鍵を待機中</text>',
  '<text x="262" y="48" font-size="11" fill="#64748b">lock() で待機中</text>'),
 ('<text x="550" y="118" font-size="11" fill="#64748b">lock() で鍵を待機中</text>',
  '<text x="620" y="48" font-size="11" fill="#64748b">lock() で待機中</text>'),
]
for a, b in pairs:
    assert s.count(a) == 1, a
    s = s.replace(a, b)
open(p, 'w', encoding='utf-8', newline='\n').write(s)
print('ok')
