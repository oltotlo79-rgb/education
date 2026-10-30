import sys
p = sys.argv[1]
t = open(p, encoding="utf-8").read()
old = """    let copy = NewNote::parse(&note.title, &note.body)
        .map_err(|errors| AppError::internal(format!("保存済みのノートが不正です: {errors:?}")))?;
"""
new = """    let copy = NewNote::parse(&note.title, &note.body).map_err(|errors| {
        AppError::internal(format!("保存済みのノートが不正です: {errors:?}"))
    })?;
"""
assert t.count(old) == 1
open(p, "w", encoding="utf-8", newline="").write(t.replace(old, new))
