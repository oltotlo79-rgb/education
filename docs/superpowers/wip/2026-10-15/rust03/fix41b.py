p = 'parts/04.md'
s = open(p, encoding='utf-8').read()
a = 'fill="#c2410c">ヒープ（owned の本体 16 バイト）</text>'
assert s.count(a) == 1
s = s.replace(a, 'fill="#c2410c">ヒープ（owned の本体 16 バイト。· は空白）</text>')
open(p, 'w', encoding='utf-8', newline='\n').write(s)
print('ok')
