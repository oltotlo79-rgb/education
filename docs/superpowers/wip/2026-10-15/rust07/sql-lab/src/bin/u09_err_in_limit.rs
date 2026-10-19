use libsql::{Builder, Value};

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    for count in [32766, 32767] {
        // ?1, ?2, … を count 個作り、id として 1, 2, … を渡す
        let marks: Vec<String> = (1..=count).map(|i| format!("?{i}")).collect();
        let sql = format!(
            "SELECT COUNT(*) FROM notes WHERE id IN ({})",
            marks.join(", ")
        );
        let values: Vec<Value> = (1..=count).map(|i| Value::Integer(i64::from(i))).collect();
        match conn.query(&sql, values).await {
            Ok(mut rows) => {
                let row = rows.next().await?.expect("COUNT は必ず1行を返す");
                println!("{count} 個: 見つかったノート {} 件", row.get::<i64>(0)?);
            }
            Err(e) => println!("{count} 個: エラー: {e}"),
        }
    }
    Ok(())
}
