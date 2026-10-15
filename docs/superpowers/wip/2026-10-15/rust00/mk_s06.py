import pathlib, sys
S = pathlib.Path(sys.argv[1])
t = (S / "u06_router.rs").read_text(encoding="utf-8")
bs = chr(92)
old = '("GET", "/health") => ("200 OK", "ok' + bs + 'n"),\n'
assert old in t
new = old + '        ("GET", "/about") => ("200 OK", "<h1>このサーバーについて</h1>' + bs + 'n"),\n'
t = t.replace(old, new)
(S / "s06_about.rs").write_text(t, encoding="utf-8")
