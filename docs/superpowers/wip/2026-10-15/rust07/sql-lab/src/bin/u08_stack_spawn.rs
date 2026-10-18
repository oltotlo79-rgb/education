use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_remote("http://127.0.0.1:9".to_string(), String::new())
        .build()
        .await?;
    let conn = db.connect()?;
    // DB とのやり取りを tokio::spawn で作業用のスレッドに任せ、終わるのを待つ
    let task = tokio::spawn(async move {
        conn.execute("CREATE TABLE memos (id INTEGER PRIMARY KEY)", ())
            .await
    });
    let result = task.await.expect("作業が panic していない");
    println!("結果: {result:?}");
    Ok(())
}
