p = 'parts/09.md'
s = open(p, encoding='utf-8').read()
i = s.index('<svg viewBox="0 0 720 230"', s.index('借用の状態**を持っていて'))
j = s.index('</svg>', i)
seg = s[i:j]
seg2 = seg.replace('height="222" rx="10"', 'height="196" rx="10"').replace('<text x="360" y="218" text-anchor="middle"', '<text x="360" y="220" text-anchor="middle"')
assert seg2.count('height="196" rx="10"') == 4, seg2.count('height="196" rx="10"')
s = s[:i] + seg2 + s[j:]
open(p, 'w', encoding='utf-8', newline='\n').write(s)
print('ok')
