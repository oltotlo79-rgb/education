p = 'parts/08.md'
s = open(p, encoding='utf-8').read()
pairs = [
 ('<rect x="330" y="50" width="114" height="44" rx="4" fill="#ffffff" stroke="#c2410c"/>', '<rect x="330" y="38" width="114" height="44" rx="4" fill="#ffffff" stroke="#c2410c"/>'),
 ('<text x="338" y="68" font-size="11" font-weight="700" fill="#0f172a">Email</text>', '<text x="338" y="56" font-size="11" font-weight="700" fill="#0f172a">Email</text>'),
 ('<text x="338" y="86" font-size="11" fill="#0f172a">address</text>', '<text x="338" y="74" font-size="11" fill="#0f172a">address</text>'),
 ('<line x1="290" y1="72" x2="328" y2="72" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-30)"/>', '<line x1="290" y1="72" x2="328" y2="66" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-30)"/>'),
]
for a, b in pairs:
    assert s.count(a) == 1, a
    s = s.replace(a, b)
open(p, 'w', encoding='utf-8', newline='\n').write(s)
print('ok')
