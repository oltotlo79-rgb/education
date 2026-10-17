import re, sys
n = int(sys.argv[1])
names = ["0001_users", "0002_notes", "0003_sessions", "0004_notes_pinned", "0005_tags"][:n]
import os
p = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'sql-lab/src/bin/u06_migrate.rs')
s = open(p, encoding='utf-8').read()
body = "".join(f'    Migration {{\n        version: "{v}",\n        sql: include_str!("../../migrations/{v}.sql"),\n    }},\n' for v in names)
s = re.sub(r'const MIGRATIONS: &\[Migration\] = &\[\n.*?\n\];', 'const MIGRATIONS: &[Migration] = &[\n' + body + '];', s, flags=re.S)
open(p, 'w', encoding='utf-8', newline='\n').write(s)
