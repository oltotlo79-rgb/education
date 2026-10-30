from pathlib import Path

p = Path(__file__).resolve().parent / "parts" / "unit09.src.md"
s = p.read_text(encoding="utf-8")
reps = [
    (
        "Windows では、断られた接続のたびに、約 1 秒待ってから諦めるので、20 回で 20 秒かかりました（第9巻 UNIT 10）。",
        "Windows では、断られた接続のたびに約 2 秒待ってから諦めるので（第9巻 UNIT 10）、同時 2 本で 20 回に、約 20 秒かかりました。",
    ),
    (
        "curl は、Chrome と同じく（第10巻）、`127.0.0.1` と `localhost` を「安全な相手」として扱います。",
        "curl も、Chrome と同じく（第10巻）、`127.0.0.1` への http には、`Secure` のクッキーを送りました。",
    ),
    (
        "ただし、混みすぎると、ブラウザや Fly.io の入口の時間切れ（たとえば 60 秒）まで待たされ、結局エラーになります。",
        "ただし、混みすぎると、ブラウザや、前に置いた入口（プロキシ）の時間切れまで待たされ、結局エラーになります。",
    ),
]
for old, new in reps:
    assert s.count(old) == 1, old[:40]
    s = s.replace(old, new)
p.write_text(s, encoding="utf-8", newline="\n")
print("ok")
