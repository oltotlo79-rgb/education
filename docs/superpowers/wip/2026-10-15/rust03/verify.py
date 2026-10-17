import re, sys, glob, os
md = open(sys.argv[1], encoding='utf-8').read()
bins = {os.path.basename(p)[:-3]: open(p, encoding='utf-8').read().replace('\r\n','\n') for p in glob.glob('ownership-lab/src/bin/*.rs')}
outs = {os.path.basename(p)[:-4]: re.sub(r'\nexit=\d+\n?$', '\n', open(p, encoding='utf-8').read()) for p in glob.glob('out/*.txt')}
blocks = re.findall(r'```(\w*)\n(.*?)```', md, re.S)
full = excerpt = bad = 0
used_bins = set()
for lang, body in blocks:
    if lang == 'rust':
        b = body.rstrip('\n') + '\n'
        hit = [n for n, src in bins.items() if src == b]
        if hit:
            full += 1; used_bins.update(hit); continue
        # excerpt: every non-empty, non-"..." line appears in some single file in order
        lines = [l for l in b.split('\n') if l.strip() and l.strip() not in ('...',) and '{ ... }' not in l]
        ok = False
        for n, src in bins.items():
            if all(l in src for l in lines):
                ok = True; break
        if ok:
            excerpt += 1
        else:
            bad += 1
            print('RUST BLOCK NOT FOUND:\n' + b[:300])
    elif lang == 'text':
        b = body.rstrip('\n')
        if b.startswith('ownership-lab/') or b.startswith('PS C:'):
            continue
        if any(b in o for o in outs.values()):
            continue
        print('TEXT BLOCK NOT FOUND:\n' + b[:400] + '\n---')
print('rust full', full, 'excerpt', excerpt, 'bad', bad)
print('bins total', len(bins), 'shown in full', len(used_bins))
print('not shown in full:', sorted(set(bins) - used_bins))
