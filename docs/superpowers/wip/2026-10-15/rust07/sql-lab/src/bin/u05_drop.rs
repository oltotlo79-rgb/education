use libsql::{Builder, Connection};

// ノートの件数を数える
async fn count(conn: &Connection) -> Result<i64, libsql::Error> {
    let mut rows = conn.query("SELECT COUNT(*) FROM notes", ()).await?;
    let row = rows.next().await?.expect("COUNT(*) は必ず1行返す");
    row.get::<i64>(0)
}

// 2件のノートを「両方とも保存」か「どちらも保存しない」かにする
async fn save_two(conn: &Connection, first: &str, second: &str) -> Result<(), libsql::Error> {
    let tx = conn.transaction().await?;
    tx.execute("INSERT INTO notes (user_id, title) VALUES (1, ?1)", [first])
        .await?;
    // ここで失敗すると ? で関数を抜け、tx は commit されないまま片付けられる
    tx.execute(
        "INSERT INTO notes (user_id, title) VALUES (1, ?1)",
        [second],
    )
    .await?;
    tx.commit().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    // 前回の実行で作ったファイルを消してから始める
    let path = "u05_drop.db";
    let _ = std::fs::remove_file(path);

    let db = Builder::new_local(path).build().await?;
    // 同じ DB への接続を2本作る（a が書く人、b が別の画面の人）
    let a = db.connect()?;
    let b = db.connect()?;
    a.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    a.execute_batch(include_str!("../../sql/seed.sql")).await?;

    println!(
        "① 始める前: a = {}, b = {}",
        count(&a).await?,
        count(&b).await?
    );

    {
        let tx = a.transaction().await?;
        tx.execute(
            "INSERT INTO notes (user_id, title) VALUES (1, 'drop されるノート')",
            (),
        )
        .await?;
        println!(
            "② tx の中: tx = {}, b = {}",
            count(&tx).await?,
            count(&b).await?
        );
        println!("   a.is_autocommit() = {}", a.is_autocommit());
        // commit も rollback もしないまま、ここで tx が片付けられる（drop）
    }
    println!(
        "③ drop の後: a = {}, b = {}",
        count(&a).await?,
        count(&b).await?
    );
    println!("   a.is_autocommit() = {}", a.is_autocommit());

    let result = save_two(&a, "1件目", "").await;
    println!("④ save_two(\"1件目\", \"\") = {result:?}");
    println!("   その後: a = {}", count(&a).await?);

    save_two(&a, "1件目", "2件目").await?;
    println!(
        "⑤ save_two(\"1件目\", \"2件目\") の後: a = {}, b = {}",
        count(&a).await?,
        count(&b).await?
    );
    Ok(())
}
