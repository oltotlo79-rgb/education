from pathlib import Path

p = Path(__file__).resolve().parent / "parts" / "unit04.src.md"
s = p.read_text(encoding="utf-8")
reps = [
    (
        "- `POST /notes → 500` が 92 回あるのは、縮める途中で、何度も試したからです。このとき、標準エラー出力には、RustNote の `[error] サーバーの中でエラーが起きました <- DB の操作に失敗しました <- SQLite failure: CHECK constraint failed: length(title) BETWEEN 1 AND 80` という行が、92 行出ていました（上の出力からは省いています）。",
        "- `POST /notes → 500` が 147 回あるのは、縮める途中で、失敗する入力を何度も試したからです。このとき、標準エラー出力には、RustNote の `[error] サーバーの中でエラーが起きました <- DB の操作に失敗しました <- SQLite failure: CHECK constraint failed: length(title) BETWEEN 1 AND 80` という行が、147 行出ていました（上の出力からは省いています）。",
    ),
    (
        "（第3巻 UNIT 05）。",
        "（第3巻 UNIT 09）。",
    ),
    (
        r"「制御文字（`\PC` の `C`）以外の文字を、0〜100 個」という正規表現です。",
        "「制御文字などの目に見えない文字（Unicode の分類 `C`）以外の文字を、0〜100 個」という正規表現です。",
    ),
    (
        "（`proptest: FileFailurePersistence::SourceParallel set, but failed to find lib.rs or main.rs` という行も2回出ますが、省いています。解説3で説明します。）",
        "（`test parse_… ...` の行のうしろに出る `proptest: FileFailurePersistence …` は、解説3で説明します。）",
    ),
    (
        "- 3回とも、6〜8 個目くらいで見つかりました（上は2回目。`successes: 5` のあとで失敗）。",
        "- 3回とも、6〜9 個目の入力で見つかりました（上は2回目。5 個通ったあとの、6 個目で失敗）。",
    ),
    (
        r"@@OUTGREP:u04_err_escape:^(error|  -->| *[0-9]* \||help|  =)@@",
        "@@OUTHEAD:u04_err_escape:12@@\n" + r"@@OUTGREP:u04_err_escape:^error(\[E0277\]|: could not)@@",
    ),
    (
        r"@@OUTGREP:u04_err_fnmut:^(error|  -->|[0-9 ]+\||   \||help|For more)@@",
        "@@OUT:u04_err_fnmut@@",
    ),
]
for old, new in reps:
    assert s.count(old) == 1, old[:70]
    s = s.replace(old, new)
p.write_text(s, encoding="utf-8", newline="\n")
print("ok")
