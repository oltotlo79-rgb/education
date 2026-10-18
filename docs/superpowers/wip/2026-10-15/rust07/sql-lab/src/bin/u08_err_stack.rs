use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    // 手元で何も待ち受けていないポート 9 を、リモートの DB として開く
    let db = Builder::new_remote("http://127.0.0.1:9".to_string(), String::new())
        .build()
        .await?;
    let conn = db.connect()?;
    // 書き込みの文をリモートへ送る
    let result = conn
        .execute("CREATE TABLE memos (id INTEGER PRIMARY KEY)", ())
        .await;
    println!("結果: {result:?}");
    Ok(())
}
