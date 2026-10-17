use std::cell::Cell;
use std::time::Instant;

use libsql::params::IntoParams;
use libsql::{Builder, Connection, Rows};

// 投げた SQL の回数を数える接続（実験用の包み紙）
struct CountingConn {
    conn: Connection,
    count: Cell<usize>,
}

impl CountingConn {
    async fn query(&self, sql: &str, params: impl IntoParams) -> Result<Rows, libsql::Error> {
        self.count.set(self.count.get() + 1);
        self.conn.query(sql, params).await
    }

    // 回数を 0 に戻し、それまでの回数を返す
    fn take_count(&self) -> usize {
        self.count.replace(0)
    }
}

// 会員ごとの集計（管理画面の1行分）
#[derive(Debug, PartialEq)]
struct Summary {
    email: String,
    notes: i64,
    done: i64,
}

// ❌ N+1: 会員の一覧で1回 + 会員1人ごとに1回
async fn summaries_n_plus_1(db: &CountingConn) -> Result<Vec<Summary>, libsql::Error> {
    let mut users = db
        .query("SELECT id, email FROM users ORDER BY id", ())
        .await?;
    let mut list = Vec::new();
    while let Some(user) = users.next().await? {
        let id = user.get::<i64>(0)?;
        // ループの中で SQL を投げている（ここが N 回）
        let mut rows = db
            .query(
                "SELECT COUNT(*), COALESCE(SUM(done), 0) FROM notes WHERE user_id = ?1",
                [id],
            )
            .await?;
        let row = rows.next().await?.expect("COUNT は必ず1行を返す");
        list.push(Summary {
            email: user.get::<String>(1)?,
            notes: row.get::<i64>(0)?,
            done: row.get::<i64>(1)?,
        });
    }
    Ok(list)
}

// ✅ JOIN + GROUP BY: 1回で全員分
async fn summaries_join(db: &CountingConn) -> Result<Vec<Summary>, libsql::Error> {
    let mut rows = db
        .query(
            "SELECT u.email, COUNT(n.id), COALESCE(SUM(n.done), 0)
               FROM users AS u
               LEFT JOIN notes AS n ON n.user_id = u.id
              GROUP BY u.id
              ORDER BY u.id",
            (),
        )
        .await?;
    let mut list = Vec::new();
    while let Some(row) = rows.next().await? {
        list.push(Summary {
            email: row.get::<String>(0)?,
            notes: row.get::<i64>(1)?,
            done: row.get::<i64>(2)?,
        });
    }
    Ok(list)
}

// ✅ IN: 決まった id の集まりを1回で読む（「最近見たノート」など）
async fn titles_by_ids(
    db: &CountingConn,
    owner: i64,
    ids: &[i64],
) -> Result<Vec<(i64, String)>, libsql::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    // ?2, ?3, ?4 … を、id の数だけ作る（値そのものは SQL に書き込まない）
    let marks: Vec<String> = (0..ids.len()).map(|i| format!("?{}", i + 2)).collect();
    let sql = format!(
        "SELECT id, title FROM notes WHERE user_id = ?1 AND id IN ({}) ORDER BY id",
        marks.join(", ")
    );
    // 値は ?1 = 持ち主、?2 以降 = id の順に並べて渡す
    let mut values = vec![libsql::Value::Integer(owner)];
    values.extend(ids.iter().map(|&id| libsql::Value::Integer(id)));
    println!("  SQL: {sql}");
    let mut rows = db.query(&sql, values).await?;
    let mut list = Vec::new();
    while let Some(row) = rows.next().await? {
        list.push((row.get::<i64>(0)?, row.get::<String>(1)?));
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

    println!("--- 会員 3 人で比べる ---");
    let a = summaries_n_plus_1(&db).await?;
    println!("N+1 版: SQL {} 回", db.take_count());
    let b = summaries_join(&db).await?;
    println!("JOIN 版: SQL {} 回", db.take_count());
    for s in &b {
        println!("  {} ノート {} 件（完了 {} 件）", s.email, s.notes, s.done);
    }
    println!("結果は同じ？ {}", a == b);

    println!("--- IN で「最近見たノート」をまとめて読む ---");
    let recent = titles_by_ids(&db, 1, &[6, 1, 4]).await?;
    println!("  {recent:?}（SQL {} 回）", db.take_count());

    println!("--- 会員を 1,000 人増やして比べる ---");
    db.conn
        .execute_batch(
            "WITH RECURSIVE seq(i) AS (SELECT 1 UNION ALL SELECT i + 1 FROM seq WHERE i < 1000)
             INSERT INTO users (email, password_hash)
             SELECT 'user' || i || '@example.com', 'dummy' FROM seq;
             INSERT INTO notes (user_id, title, done)
             SELECT u.id, 'メモ', u.id % 2 FROM users AS u, (SELECT 1 UNION ALL SELECT 2) WHERE u.id > 3;",
        )
        .await?;
    let start = Instant::now();
    let a = summaries_n_plus_1(&db).await?;
    println!("N+1 版: SQL {} 回、{:?}", db.take_count(), start.elapsed());
    let start = Instant::now();
    let b = summaries_join(&db).await?;
    println!("JOIN 版: SQL {} 回、{:?}", db.take_count(), start.elapsed());
    println!("会員 {} 人分。結果は同じ？ {}", b.len(), a == b);
    Ok(())
}
