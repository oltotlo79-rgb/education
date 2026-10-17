use libsql::Builder;
#[tokio::main]
async fn main() {
    let which = std::env::args().nth(1).unwrap_or_default();
    let db = Builder::new_remote("http://127.0.0.1:9".to_string(), String::new()).build().await.unwrap();
    let c = db.connect().unwrap();
    match which.as_str() {
        "exec" => println!("exec: {:?}", c.execute("CREATE TABLE t(x)", ()).await.err().map(|e| e.to_string())),
        "query" => println!("query: {:?}", c.query("SELECT 1", ()).await.err().map(|e| e.to_string())),
        "spawn" => {
            let h = tokio::spawn(async move { c.execute("CREATE TABLE t(x)", ()).await.err().map(|e| e.to_string()) });
            println!("spawned exec: {:?}", h.await.unwrap());
        }
        _ => {}
    }
}
