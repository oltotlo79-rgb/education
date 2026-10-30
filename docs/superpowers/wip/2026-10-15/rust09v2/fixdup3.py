import sys
p = sys.argv[1]
t = open(p, encoding="utf-8").read()
old = """    // DB から読んだタイトルと本文も、型の検査を通してから保存する
    let copy = NewNote::parse(&note.title, &note.body)
        .map_err(|errors| AppError::internal(format!("保存済みのノートが不正です: {errors:?}")))?;
"""
new = """    // DB から読んだタイトルと本文も、型の検査を通してから保存する。
    // 保存済みの値が検査に落ちるのは DB の異常なので、500 にする
    let copy = NewNote::parse(&note.title, &note.body)
        .map_err(|errors| AppError::internal(format!("{errors:?}")))?;
"""
assert t.count(old) == 1
open(p, "w", encoding="utf-8", newline="").write(t.replace(old, new))
