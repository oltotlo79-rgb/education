import sys, re
path = sys.argv[1]
lines = open(path, encoding='utf-8').read().split('\n')
out = []
i = 0
n = len(lines)
while i < n:
    if lines[i].startswith('>'):
        # collect blockquote group
        j = i
        while j < n and lines[j].startswith('>'):
            j += 1
        group = lines[i:j]
        if any(l.startswith('> ```') for l in group):
            segs = []  # (kind, [lines])
            k = 0
            cur_text = []
            while k < len(group):
                l = group[k]
                if l.startswith('> ```'):
                    if cur_text:
                        segs.append(('t', cur_text)); cur_text = []
                    code = [l[2:]]
                    k += 1
                    while k < len(group) and not group[k].startswith('> ```'):
                        code.append(group[k][2:] if group[k].startswith('> ') else group[k][1:])
                        k += 1
                    code.append(group[k][2:])
                    k += 1
                    segs.append(('c', code))
                else:
                    cur_text.append(l)
                    k += 1
            if cur_text:
                segs.append(('t', cur_text))
            blocks = []
            for kind, ls in segs:
                if kind == 't':
                    # strip leading/trailing empty '>' lines
                    while ls and ls[0].strip() == '>':
                        ls = ls[1:]
                    while ls and ls[-1].strip() == '>':
                        ls = ls[:-1]
                    if ls:
                        blocks.append('\n'.join(ls))
                else:
                    blocks.append('\n'.join(ls))
            out.append('\n\n'.join(blocks))
            i = j
            continue
        out.extend(group)
        i = j
        continue
    out.append(lines[i])
    i += 1
open(path, 'w', encoding='utf-8', newline='\n').write('\n'.join(out))
print('done')
