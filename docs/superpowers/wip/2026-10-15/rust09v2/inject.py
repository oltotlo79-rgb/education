import sys
path = sys.argv[1]
text = open(path, encoding="utf-8").read()
anchor = """        .map_err(AppError::from)?;
    let keyword = query.keyword.clone();
"""
add = """        .map_err(AppError::from)?;
    // 急いで足した行: 自分のノートの件数を、SQL で直接数えてログに出す
    let conn = app_state(cx).db.connect().await.map_err(AppError::internal)?;
    let mut rows = conn
        .query(
            "SELECT COUNT(*) FROM notes WHERE user_id = ?1",
            libsql::params![owner.get()],
        )
        .await
        .map_err(AppError::internal)?;
    if let Some(row) = rows.next().await.map_err(AppError::internal)? {
        eprintln!("ノートの件数: {:?}", row.get::<i64>(0));
    }
    let keyword = query.keyword.clone();
"""
assert text.count(anchor) == 1
open(path, "w", encoding="utf-8", newline="").write(text.replace(anchor, add))
