use libsql::{Builder, Connection, params::IntoParams};
use thiserror::Error;

#[derive(Debug, Error)]
enum GuardError {
    #[error("{found} 件消えるところでした（上限は {max} 件）。取り消しました")]
    TooMany { found: u64, max: u64 },
    #[error(transparent)]
    Db(#[from] libsql::Error),
}

// transaction の中で DELETE を試し、消える行が max より多ければ取り消す
async fn delete_at_most(
    conn: &Connection,
    sql: &str,
    params: impl IntoParams,
    max: u64,
) -> Result<u64, GuardError> {
    let tx = conn.transaction().await?;
    let found = tx.execute(sql, params).await?;
    if found > max {
        tx.rollback().await?;
        return Err(GuardError::TooMany { found, max });
    }
    tx.commit().await?;
    Ok(found)
}

// notes の件数を数える
async fn count_notes(conn: &Connection) -> Result<i64, libsql::Error> {
    let mut rows = conn.query("SELECT COUNT(*) FROM notes", ()).await?;
    let row = rows.next().await?.expect("COUNT(*) は必ず1行返す");
    row.get::<i64>(0)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    let cases: [(&str, &str, Vec<i64>); 3] = [
        ("WHERE の書き忘れ", "DELETE FROM notes", vec![]),
        (
            "id のつもりが user_id",
            "DELETE FROM notes WHERE user_id = ?1",
            vec![1],
        ),
        ("正しい1件", "DELETE FROM notes WHERE id = ?1", vec![3]),
    ];
    for (label, sql, values) in cases {
        match delete_at_most(&conn, sql, values, 1).await {
            Ok(n) => println!("{label}: {n} 件消しました"),
            Err(e) => println!("{label}: {e}"),
        }
        println!("  → notes は今 {} 件", count_notes(&conn).await?);
    }
    Ok(())
}
