p = r'C:/Users/oltot/Documents/git-projects/def/next/09-styling.md'
s = open(p, encoding='utf-8').read()
new = open(r'C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/next09/new_kaisetsu.md', encoding='utf-8').read()
a = s.index('##### 解説1: Tailwind を読み込む3行')
b = s.index('### 1.1.1 ')
region = s[a:b]
l0 = region.index('3つの層は、**積み重なった棚**')
l1 = region.index('安心してできます。', l0) + len('安心してできます。')
layer = region[l0:l1]
t0 = region.index('完成した `.btn-primary`（本文のコード全体）')
t1 = region.index('消さないようにしましょう。', t0) + len('消さないようにしましょう。')
states = region[t0:t1]
assert '{{LAYER}}' in new and '{{STATES}}' in new
new = new.replace('{{LAYER}}', layer).replace('{{STATES}}', states)
s = s[:a] + new + s[b:]
open(p, 'w', encoding='utf-8').write(s)
print('replaced', len(region), '->', len(new))
