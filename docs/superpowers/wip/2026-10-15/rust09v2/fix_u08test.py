"""u08_routes.rs の、Location の読み方を、無いときにも落ちない形にする。"""
import sys

p = sys.argv[1]
s = open(p, encoding="utf-8").read()
reps = [
    ("use common::{location, test_app};", "use common::test_app;"),
    ("        let status = response.status().as_u16();\n        let to_ok = match r.to {\n            Some(prefix) => status == 303 && location(&response).starts_with(prefix),",
     "        let status = response.status().as_u16();\n        // 移り先（無ければ空）\n        let moved_to = response\n            .headers()\n            .get(header::LOCATION)\n            .and_then(|value| value.to_str().ok())\n            .unwrap_or(\"\")\n            .to_string();\n        let to_ok = match r.to {\n            Some(prefix) => status == 303 && moved_to.starts_with(prefix),"),
    ("            status,\n            r.to.map(|_| location(&response).to_string())\n                .unwrap_or_default()\n        );", "            status,\n            moved_to\n        );"),
]
for a, b in reps:
    if s.count(a) != 1:
        sys.exit(f"見つからない: {a[:50]}")
    s = s.replace(a, b)
open(p, "w", encoding="utf-8", newline="\n").write(s)
print("ok")
