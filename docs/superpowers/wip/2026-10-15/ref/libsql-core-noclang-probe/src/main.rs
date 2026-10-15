#[tokio::main]
async fn main() {
    let db = libsql::Builder::new_local(":memory:").build().await.unwrap();
    let conn = db.connect().unwrap();
    conn.execute("CREATE TABLE t (x INTEGER)", ()).await.unwrap();
    conn.execute("INSERT INTO t (x) VALUES (?1)", [42]).await.unwrap();
    let mut rows = conn.query("SELECT x FROM t", ()).await.unwrap();
    let row = rows.next().await.unwrap().unwrap();
    println!("x = {}", row.get::<i64>(0).unwrap());
}
