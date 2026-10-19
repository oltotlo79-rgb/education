use std::cell::Cell;

use libsql::params::IntoParams;
use libsql::{Builder, Connection, Rows};

// 投げた SQL の回数を数える接続（例9-3 と同じ）
struct CountingConn {
    conn: Connection,
    count: Cell<usize>,
}

impl CountingConn {
    async fn query(&self, sql: &str, params: impl IntoParams) -> Result<Rows, libsql::Error> {
        self.count.set(self.count.get() + 1);
        self.conn.query(sql, params).await
    }

    fn take_count(&self) -> usize {
        self.count.replace(0)
    }
}

// ❌ 問題のコード: ノートの一覧で1回 + ノート1件ごとに作者を1回
async fn latest_n_plus_1(db: &CountingConn) -> Result<Vec<(String, String)>, libsql::Error> {
    let mut notes = db
        .query(
            "SELECT user_id, title FROM notes ORDER BY id DESC LIMIT 5",
            (),
        )
        .await?;
    let mut list = Vec::new();
    while let Some(note) = notes.next().await? {
        let mut users = db
            .query(
                "SELECT email FROM users WHERE id = ?1",
                [note.get::<i64>(0)?],
            )
            .await?;
        let user = users.next().await?.expect("作者は必ずいる");
        list.push((note.get::<String>(1)?, user.get::<String>(0)?));
    }
    Ok(list)
}

// ✅ 直した版: JOIN で1回
async fn latest_join(db: &CountingConn) -> Result<Vec<(String, String)>, libsql::Error> {
    let mut rows = db
        .query(
            "SELECT n.title, u.email
               FROM notes AS n
               JOIN users AS u ON u.id = n.user_id
              ORDER BY n.id DESC
              LIMIT 5",
            (),
        )
        .await?;
    let mut list = Vec::new();
    while let Some(row) = rows.next().await? {
        list.push((row.get::<String>(0)?, row.get::<String>(1)?));
    }
    Ok(list)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;
    let db = CountingConn {
        conn,
        count: Cell::new(0),
    };

    let a = latest_n_plus_1(&db).await?;
    println!("N+1 版: SQL {} 回", db.take_count());
    let b = latest_join(&db).await?;
    println!("JOIN 版: SQL {} 回", db.take_count());
    for (title, email) in &b {
        println!("  {title}（{email}）");
    }
    println!("結果は同じ？ {}", a == b);
    Ok(())
}
