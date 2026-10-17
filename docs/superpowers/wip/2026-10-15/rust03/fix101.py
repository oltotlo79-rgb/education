p = 'parts/10.md'
s = open(p, encoding='utf-8').read()
pairs = [
 ('<rect x="420" y="98" width="80" height="26" rx="3" fill="#ffffff" stroke="#f97316" stroke-width="2"/>',
  '<rect x="420" y="98" width="60" height="26" rx="3" fill="#ffffff" stroke="#f97316" stroke-width="2"/>'),
 ('<circle cx="488" cy="111" r="3.5" fill="#f97316"/>', '<circle cx="468" cy="111" r="3.5" fill="#f97316"/>'),
 ('<line x1="132" y1="77" x2="170" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-36)"/>',
  '<line x1="132" y1="77" x2="200" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-36)"/>'),
 ('<line x1="310" y1="77" x2="190" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-36)"/>',
  '<line x1="310" y1="77" x2="320" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-36)"/>'),
 ('<line x1="488" y1="77" x2="210" y2="196" stroke="#475569" stroke-width="1.5" stroke-dasharray="3 3" marker-end="url(#ar-03-36)"/>',
  '<line x1="488" y1="77" x2="540" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-36)"/>'),
 ('<line x1="666" y1="77" x2="226" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-36)"/>',
  '<line x1="666" y1="77" x2="590" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-36)"/>'),
 ('<line x1="488" y1="111" x2="510" y2="222" stroke="#f97316" stroke-width="2.5" marker-end="url(#ar-03-37)"/>',
  '<line x1="468" y1="111" x2="468" y2="222" stroke="#f97316" stroke-width="2.5" marker-end="url(#ar-03-37)"/>'),
]
for a, b in pairs:
    assert s.count(a) == 1, a
    s = s.replace(a, b)
open(p, 'w', encoding='utf-8', newline='\n').write(s)
print('ok')
