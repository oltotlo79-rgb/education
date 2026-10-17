use libsql::{Builder, Connection};

// 危険: 入力をそのまま SQL の文字列に混ぜて、会員を探す
async fn find_unsafe(conn: &Connection, email: &str) -> Result<Vec<String>, libsql::Error> {
    let sql = format!("SELECT email FROM users WHERE email = '{email}'");
    println!("   実行した SQL: {sql}");
    let mut rows = conn.query(&sql, ()).await?;
    let mut found = Vec::new();
    while let Some(row) = rows.next().await? {
        found.push(row.get::<String>(0)?);
    }
    Ok(found)
}

// 安全: 入力は ?1 に「値」として渡す（SQL の文そのものは変わらない）
async fn find_safe(conn: &Connection, email: &str) -> Result<Vec<String>, libsql::Error> {
    let mut rows = conn
        .query("SELECT email FROM users WHERE email = ?1", [email])
        .await?;
    let mut found = Vec::new();
    while let Some(row) = rows.next().await? {
        found.push(row.get::<String>(0)?);
    }
    Ok(found)
}

// 危険: URL から来た id の文字列を、そのまま DELETE 文に混ぜる
async fn delete_unsafe(conn: &Connection, owner: i64, id_text: &str) -> Result<u64, libsql::Error> {
    let sql = format!("DELETE FROM notes WHERE id = {id_text} AND user_id = {owner}");
    println!("   実行した SQL: {sql}");
    conn.execute(&sql, ()).await
}

// 安全: まず数に変換し（できなければ 404 にする）、値として渡す
async fn delete_safe(conn: &Connection, owner: i64, id_text: &str) -> Result<u64, String> {
    let id: i64 = id_text
        .parse()
        .map_err(|_| format!("id が数ではありません: {id_text:?}（404 にする）"))?;
    conn.execute(
        "DELETE FROM notes WHERE id = ?1 AND user_id = ?2",
        libsql::params![id, owner],
    )
    .await
    .map_err(|e| e.to_string())
}

async fn count_notes(conn: &Connection) -> Result<i64, libsql::Error> {
    let mut rows = conn.query("SELECT COUNT(*) FROM notes", ()).await?;
    let row = rows.next().await?.expect("COUNT(*) は必ず1行返す");
    row.get::<i64>(0)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    // 攻撃者が入力欄に打ち込む文字列
    let evil_email = "x' OR '1'='1";
    println!("① 文字列を混ぜる版で {evil_email:?} を探す");
    println!("   見つかった: {:?}", find_unsafe(&conn, evil_email).await?);
    println!("② bind する版で同じ文字列を探す");
    println!("   見つかった: {:?}", find_safe(&conn, evil_email).await?);

    // Alice（1）が URL の id を書き換えて送ってくる
    let evil_id = "0 OR 1=1 --";
    println!(
        "③ 文字列を混ぜる版で id = {evil_id:?} を削除（ノートは {} 件）",
        count_notes(&conn).await?
    );
    let deleted = delete_unsafe(&conn, 1, evil_id).await?;
    println!(
        "   消えた行: {deleted}（残り {} 件）",
        count_notes(&conn).await?
    );

    // 消えてしまったので、会員ごと消してからデータを入れ直す
    conn.execute("DELETE FROM users", ()).await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;
    println!(
        "④ bind する版で同じ id を削除（ノートは {} 件）",
        count_notes(&conn).await?
    );
    println!("   結果: {:?}", delete_safe(&conn, 1, evil_id).await);
    println!(
        "   Bob のノート 3 を指定: {:?}",
        delete_safe(&conn, 1, "3").await
    );
    println!("   残り {} 件", count_notes(&conn).await?);
    Ok(())
}
