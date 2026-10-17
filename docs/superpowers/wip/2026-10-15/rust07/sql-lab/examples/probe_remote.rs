use libsql::Builder;
use std::time::Duration;
#[tokio::main]
async fn main() {
    let r = Builder::new_remote("http://127.0.0.1:9".to_string(), String::new()).build().await;
    println!("remote build: {:?}", r.as_ref().err());
    if let Ok(db) = r {
        let c = db.connect();
        println!("connect: {:?}", c.as_ref().err());
        if let Ok(c) = c {
            let q = c.query("SELECT 1", ()).await;
            println!("query: {:?}", q.err());
            let q = c.execute("CREATE TABLE t(x)", ()).await;
            println!("exec: {:?}", q.err().map(|e| e.to_string()));
        }
    }
    let r = Builder::new_remote("libsql://no-such-db-xyz-oltot.aws-ap-northeast-1.turso.io".to_string(), "bad".to_string()).build().await;
    if let Ok(db) = r { let c = db.connect().unwrap(); let q = c.query("SELECT 1", ()).await; println!("turso bad host: {}", q.err().map(|e|e.to_string()).unwrap_or_default()); }
    let _ = std::fs::remove_file("replica_probe.db");
    let r = Builder::new_remote_replica("replica_probe.db", "http://127.0.0.1:9".to_string(), String::new()).sync_interval(Duration::from_secs(60)).build().await;
    println!("replica build: {:?}", r.as_ref().err().map(|e| e.to_string()));
    if let Ok(db) = r {
        let s = db.sync().await;
        println!("replica sync: {:?}", s.err().map(|e| e.to_string()));
        let c = db.connect().unwrap();
        let q = c.query("SELECT 1", ()).await;
        println!("replica query: {:?}", q.err().map(|e| e.to_string()));
    }
    let local = Builder::new_local(":memory:").build().await.unwrap();
    println!("local sync: {:?}", local.sync().await.err().map(|e| e.to_string()));
}
