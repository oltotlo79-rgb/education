use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    let mut rows = conn.query("SELECT 'メモ', NULL, 1, 'abc'", ()).await?;
    let row = rows.next().await?.expect("1行ある");

    // (a) 文字列を String で
    println!("(a) {:?}", row.get::<String>(0));
    // (b) NULL を String で
    println!("(b) {:?}", row.get::<String>(1));
    // (c) NULL を Option<String> で
    println!("(c) {:?}", row.get::<Option<String>>(1));
    // (d) 1 を bool で
    println!("(d) {:?}", row.get::<bool>(2));
    // (e) 'abc' を Option<i64> で
    println!("(e) {:?}", row.get::<Option<i64>>(3));
    Ok(())
}
