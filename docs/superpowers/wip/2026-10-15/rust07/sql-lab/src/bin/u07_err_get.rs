use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    let mut rows = conn
        .query("SELECT id, title, NULL, 2 FROM notes WHERE id = 1", ())
        .await?;
    let row = rows.next().await?.expect("ノート 1 はある");

    // ① NULL を String で受け取る
    println!("① NULL → String: {:?}", row.get::<String>(2));
    // ② 列の番号の数え間違い（列は 0〜3 の 4 つしかない）
    println!("② 存在しない列 4: {:?}", row.get::<String>(4));
    // ③ 0 と 1 以外の整数を bool で受け取る
    println!("③ 2 → bool: {:?}", row.get::<bool>(3));
    // ④ 行を返す SELECT を execute で実行する
    let result = conn.execute("SELECT id FROM notes", ()).await;
    println!("④ execute で SELECT: {result:?}");
    // ⑤ 1行も無いのに next() の結果を unwrap せずに確かめる
    let mut rows = conn
        .query("SELECT id FROM notes WHERE id = 999", ())
        .await?;
    println!("⑤ 999 番の行: {:?}", rows.next().await?.is_some());
    Ok(())
}
