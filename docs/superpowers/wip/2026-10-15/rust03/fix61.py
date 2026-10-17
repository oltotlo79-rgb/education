p = 'parts/06.md'
s = open(p, encoding='utf-8').read()
bs = chr(92)
a = '牛乳を買う' + bs + 'n</text>'
b = '>' + bs + 'n本を返す</text>'
print(s.count(a), s.count(b))
s = s.replace(a, '牛乳を買う</text>').replace(b, '>本を返す</text>')
open(p, 'w', encoding='utf-8', newline='\n').write(s)
