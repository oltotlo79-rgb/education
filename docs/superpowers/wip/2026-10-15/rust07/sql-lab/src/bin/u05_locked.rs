use std::time::{Duration, Instant};

use libsql::{Builder, TransactionBehavior};

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let path = "u05_locked.db";
    let _ = std::fs::remove_file(path);
    let db = Builder::new_local(path).build().await?;
    let a = db.connect()?;
    let b = db.connect()?;
    a.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    a.execute_batch(include_str!("../../sql/seed.sql")).await?;

    // a が「書き込み用の鍵」を先に取る（BEGIN IMMEDIATE）
    let tx = a
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .await?;
    tx.execute("UPDATE notes SET done = 1 WHERE id = 1", ())
        .await?;

    // ① 待たない設定のまま、b が書き込もうとする
    let result = b
        .execute("UPDATE notes SET done = 1 WHERE id = 3", ())
        .await;
    println!("① 待たない b: {result:?}");

    // ② b に「最大 5 秒待つ」と設定し、別のタスクで書き込ませる
    b.busy_timeout(Duration::from_secs(5))?;
    let start = Instant::now();
    let waiter = tokio::spawn(async move {
        let result = b
            .execute("UPDATE notes SET done = 1 WHERE id = 3", ())
            .await;
        (result, start.elapsed())
    });

    // a は 300 ミリ秒後に確定して鍵を手放す
    tokio::time::sleep(Duration::from_millis(300)).await;
    tx.commit().await?;
    println!("② a が COMMIT（{} ミリ秒後）", start.elapsed().as_millis());

    let (result, waited) = waiter.await.expect("タスクが panic しない");
    println!(
        "② 待つ b: {result:?}（{} ミリ秒待った）",
        waited.as_millis()
    );
    Ok(())
}
