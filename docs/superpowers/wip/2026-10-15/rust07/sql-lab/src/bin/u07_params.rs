use libsql::{Builder, Value};

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    // ① params! マクロ: 型の違う値を並べられる（いちばんよく使う）
    conn.execute(
        "INSERT INTO notes (user_id, title, body) VALUES (?1, ?2, ?3)",
        libsql::params![1, "映画を見る", "土曜の夜"],
    )
    .await?;

    // ② タプル: 値が少ないときに手軽
    conn.execute("UPDATE notes SET done = ?1 WHERE id = ?2", (1, 9))
        .await?;

    // ③ 配列: 同じ型の値だけなら [..] でもよい
    let mut rows = conn
        .query("SELECT id, title, done FROM notes WHERE id = ?1", [9])
        .await?;
    if let Some(row) = rows.next().await? {
        let id = row.get::<i64>(0)?;
        let title = row.get::<String>(1)?;
        // done は 0 / 1 の整数。bool として取り出すこともできる
        let done = row.get::<bool>(2)?;
        println!("③ id = {id}, title = {title}, done = {done}");
    }

    // ④ 名前付き: :owner のように名前で値を渡す（値が多いときに取り違えにくい）
    let mut rows = conn
        .query(
            "SELECT COUNT(*) FROM notes WHERE user_id = :owner AND done = :done",
            libsql::named_params! { ":owner": 1, ":done": 0 },
        )
        .await?;
    if let Some(row) = rows.next().await? {
        println!("④ Alice の未完了: {} 件", row.get::<i64>(0)?);
    }

    // ⑤ 型が決まっていない値は Value として取り出し、match で見分ける
    let mut rows = conn.query("SELECT 42, 'テキスト', NULL, 1.5", ()).await?;
    if let Some(row) = rows.next().await? {
        for i in 0..row.column_count() {
            match row.get_value(i)? {
                Value::Integer(n) => println!("⑤ 列{i}: 整数 {n}"),
                Value::Text(s) => println!("⑤ 列{i}: 文字列 {s}"),
                Value::Null => println!("⑤ 列{i}: NULL"),
                Value::Real(x) => println!("⑤ 列{i}: 小数 {x}"),
                Value::Blob(b) => println!("⑤ 列{i}: バイト列 {} バイト", b.len()),
            }
        }
    }

    // ⑥ NULL かもしれない値は Option で受け取る
    let mut rows = conn.query("SELECT NULL", ()).await?;
    if let Some(row) = rows.next().await? {
        let maybe: Option<String> = row.get(0)?;
        println!("⑥ Option で受け取ると: {maybe:?}");
    }
    Ok(())
}
