use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    // ① ?1 と ?2 があるのに、値を1つしか渡さない
    let result = conn
        .execute(
            "UPDATE notes SET title = ?1 WHERE id = ?2",
            ["新しいタイトル"],
        )
        .await;
    println!("① UPDATE（値が1つ足りない）: {result:?}");

    // ② INSERT で値が足りない
    let result = conn
        .execute("INSERT INTO notes (user_id, title) VALUES (?1, ?2)", [1])
        .await;
    println!("② INSERT（値が1つ足りない）: {result:?}");

    // ③ 値が1つ多い（3つ目の値はどこにも使われない）
    let result = conn
        .execute(
            "UPDATE notes SET title = ?1 WHERE id = ?2",
            libsql::params!["新しいタイトル", 1, 2],
        )
        .await;
    println!("③ UPDATE（値が1つ多い）: {result:?}");

    // ④ 対策: prepare して、? の数と渡す値の数を確かめてから実行する
    let stmt = conn
        .prepare("UPDATE notes SET title = ?1 WHERE id = ?2")
        .await?;
    println!("④ この文の ? の数: {}", stmt.parameter_count());
    Ok(())
}
