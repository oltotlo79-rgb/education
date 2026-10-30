import sys

p = "parts/unit06.src.md"
s = open(p, encoding="utf-8").read()
T = "`"
B = chr(92)
reps = [
    (T + "missing field " + B + T + "title" + B + T + T,
     T + T + "missing field " + T + "title" + T + " " + T + T),
    ("（このあとに、同じ所を指す " + T + "E0277" + T + " がもう1つ続き、その中に " + T + "note: for local types consider adding #[derive(serde::Deserialize)] to your MemoForm type" + T + " という案内があります）",
     "（このあとに、同じ所を指す " + T + "E0277" + T + " がもう1つ続き、その中に " + T + T + "note: for local types consider adding " + T + "#[derive(serde::Deserialize)]" + T + " to your " + T + "MemoForm" + T + " type" + T + T + " という案内があります）"),
    ("最後の " + T + "note:" + T + " の " + T + "required for Form<MemoForm> to implement FromRequest" + T + " は、",
     "最後の " + T + "note:" + T + " の " + T + T + "required for " + T + "Form<MemoForm>" + T + " to implement " + T + "FromRequest" + T + " " + T + T + " は、"),
    ("'{\"title\":\"milk\"}' | Out-File -Encoding ascii memo.json", "Set-Content memo.json '{\"title\":\"milk\"}' -Encoding ascii"),
    ("'{\"title\":\"' + ('a' * 3145728) + '\"}' | Out-File -Encoding ascii big.json", "Set-Content big.json ('{\"title\":\"' + ('a' * 3145728) + '\"}') -Encoding ascii"),
]
for a, b in reps:
    n = s.count(a)
    if n != 1:
        sys.exit(f"count {n}: {a}")
    s = s.replace(a, b)
if (B + T) in s:
    sys.exit("escaped backtick remains")
open(p, "w", encoding="utf-8", newline="\n").write(s)
print("ok")
