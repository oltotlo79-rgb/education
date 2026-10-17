use libsql::Builder;
#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let a = db.connect()?;
    a.execute_batch("CREATE TABLE t(x); ").await?;
    {
        let tx = a.transaction().await?;
        tx.execute("INSERT INTO t VALUES (1)", ()).await?;
        // a に直接書く
        a.execute("INSERT INTO t VALUES (2)", ()).await?;
        // 2つめの transaction を開こうとする
        let r = a.transaction().await;
        println!("second tx: {:?}", r.err());
    }
    let mut rows = a.query("SELECT COUNT(*) FROM t", ()).await?;
    println!(
        "count after drop = {}",
        rows.next().await?.unwrap().get::<i64>(0)?
    );
    // codes
    a.execute_batch("CREATE TABLE u(id INTEGER PRIMARY KEY, e TEXT NOT NULL UNIQUE, p INTEGER REFERENCES u(id));").await?;
    a.execute("INSERT INTO u VALUES (1,'a',NULL)", ()).await?;
    for sql in [
        "INSERT INTO u VALUES (2,'a',NULL)",
        "INSERT INTO u VALUES (3,NULL,NULL)",
        "INSERT INTO u VALUES (4,'b',99)",
        "INSERT INTO u VALUES (1,'c',NULL)",
        "SELEC 1",
    ] {
        let e = a.execute(sql, ()).await.unwrap_err();
        println!("{e:?} / {e}");
    }
    Ok(())
}
