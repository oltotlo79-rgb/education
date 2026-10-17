use libsql::{Builder, Connection};

// メモの数を数える
async fn count(conn: &Connection) -> Result<i64, libsql::Error> {
    let mut rows = conn.query("SELECT COUNT(*) FROM memos", ()).await?;
    let row = rows.next().await?.expect("COUNT(*) は必ず1行返す");
    row.get::<i64>(0)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    for path in ["u08_local.db", ":memory:"] {
        // ① 1回目: 開いて、表を作り、1行足す
        {
            let db = Builder::new_local(path).build().await?;
            let conn = db.connect()?;
            conn.execute(
                "CREATE TABLE IF NOT EXISTS memos (id INTEGER PRIMARY KEY, text TEXT NOT NULL)",
                (),
            )
            .await?;
            conn.execute("INSERT INTO memos (text) VALUES (?1)", ["ローカル"])
                .await?;
            println!("{path}: 1回目に開いたとき {} 件", count(&conn).await?);
            // ここで db と conn が捨てられ、DB が閉じられる
        }

        // ② 2回目: 同じ名前で開き直す
        let db = Builder::new_local(path).build().await?;
        let conn = db.connect()?;
        match count(&conn).await {
            Ok(n) => println!("{path}: 開き直したとき {n} 件"),
            Err(e) => println!("{path}: 開き直したとき エラー: {e}"),
        }

        // ③ ローカルの DB で sync を呼ぶと……
        match db.sync().await {
            Ok(_) => println!("{path}: sync 成功"),
            Err(e) => println!("{path}: sync エラー: {e}"),
        }
    }
    Ok(())
}
