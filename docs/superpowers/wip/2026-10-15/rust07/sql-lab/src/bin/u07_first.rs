use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    // ① DB を開く（ファイルが無ければ作られる）
    let db = Builder::new_local("u07.db").build().await?;
    // ② 接続（DB と話すための通り道）を作る
    let conn = db.connect()?;

    // ③ 表を作る（2回目に実行してもエラーにならないよう IF NOT EXISTS を付ける）
    conn.execute(
        "CREATE TABLE IF NOT EXISTS memos (
            id INTEGER PRIMARY KEY,
            text TEXT NOT NULL
        )",
        (),
    )
    .await?;

    // ④ ?1 の場所に値を入れて INSERT する
    let changed = conn
        .execute("INSERT INTO memos (text) VALUES (?1)", ["牛乳を買う"])
        .await?;
    println!(
        "追加した行数: {changed} / 新しい id: {}",
        conn.last_insert_rowid()
    );

    // ⑤ 全部の行を読み出す
    let mut rows = conn
        .query("SELECT id, text FROM memos ORDER BY id", ())
        .await?;
    while let Some(row) = rows.next().await? {
        let id: i64 = row.get(0)?;
        let text: String = row.get(1)?;
        println!("{id}: {text}");
    }
    Ok(())
}
