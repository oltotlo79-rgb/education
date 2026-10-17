import re, sys, difflib
md = open(sys.argv[1], encoding='utf-8').read()
for name in sys.argv[2:]:
    src = open(f'ownership-lab/src/bin/{name}.rs', encoding='utf-8').read()
    i = md.find(f'`src/bin/{name}.rs`')
    j = md.find('```rust\n', i)
    k = md.find('```', j + 8)
    body = md[j+8:k]
    d = list(difflib.unified_diff(body.splitlines(), src.splitlines(), 'md', 'file', lineterm='', n=0))
    print('=====', name); print('\n'.join(d))
