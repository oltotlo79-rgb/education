//! UNIT 10 の練習用の「仕事の列」（ジョブキュー）。
//!
//! 仕事（ジョブ）を DB の jobs 表に保存するので、プログラムを止めても消えない。
//! 取り出すときは1つの UPDATE 文で「選ぶ」と「自分のものにする」を同時に行うので、
//! 何人の worker が同時に取りに来ても、同じ仕事を2人が同時に持つことはない。

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use libsql::{Builder, Connection, Database, params};

/// jobs 表を作る SQL（列の意味は本文の表を参照）
const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS jobs (
    id INTEGER PRIMARY KEY,
    kind TEXT NOT NULL,
    payload TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'queued'
        CHECK (status IN ('queued', 'running', 'done', 'dead')),
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,
    run_at INTEGER NOT NULL,
    locked_until INTEGER,
    last_error TEXT
);
";

/// 取り出した仕事
#[derive(Debug)]
pub struct Job {
    pub id: i64,
    pub kind: String,
    pub payload: String,
    /// これまでに取り出された回数（今回を含む）
    pub attempts: i64,
}

/// 今の時刻（1970年1月1日からのミリ秒）
pub fn now_ms() -> i64 {
    let since = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("時計が 1970 年より前になっています");
    since.as_millis() as i64
}

/// DB ファイルを開き（無ければ作られる）、jobs 表を用意する
pub async fn open(path: &str) -> Result<Database, libsql::Error> {
    let db = Builder::new_local(path).build().await?;
    let conn = db.connect()?;
    conn.execute_batch(SCHEMA).await?;
    Ok(db)
}

/// 新しい接続を作る（worker ごとに1本ずつ使う）
pub async fn connect(db: &Database) -> Result<Connection, libsql::Error> {
    let conn = db.connect()?;
    // ほかの接続が書き込み中なら、最大 5 秒待つ（第7巻 UNIT 05）
    conn.execute_batch("PRAGMA busy_timeout = 5000;").await?;
    Ok(conn)
}

/// 仕事を1つ列に入れ、その id を返す
pub async fn enqueue(
    conn: &Connection,
    kind: &str,
    payload: &str,
    max_attempts: i64,
) -> Result<i64, libsql::Error> {
    let mut rows = conn
        .query(
            "INSERT INTO jobs (kind, payload, max_attempts, run_at)
             VALUES (?1, ?2, ?3, ?4)
             RETURNING id",
            params![kind, payload, max_attempts, now_ms()],
        )
        .await?;
    let row = rows.next().await?.expect("INSERT ... RETURNING は1行返す");
    row.get::<i64>(0)
}

/// 実行してよい仕事を1つ取り出し、lease（持ち時間）の間だけ自分のものにする。無ければ None。
/// 対象は「待っていて、実行の時刻が来た仕事」と「持ち時間が切れたまま running の仕事」
pub async fn claim(conn: &Connection, lease: Duration) -> Result<Option<Job>, libsql::Error> {
    let now = now_ms();
    let mut rows = conn
        .query(
            "UPDATE jobs
             SET status = 'running', attempts = attempts + 1, locked_until = ?1
             WHERE id = (
                 SELECT id FROM jobs
                 WHERE (status = 'queued' AND run_at <= ?2)
                    OR (status = 'running' AND locked_until < ?2)
                 ORDER BY id
                 LIMIT 1
             )
             RETURNING id, kind, payload, attempts",
            params![now + lease.as_millis() as i64, now],
        )
        .await?;
    match rows.next().await? {
        Some(row) => Ok(Some(Job {
            id: row.get(0)?,
            kind: row.get(1)?,
            payload: row.get(2)?,
            attempts: row.get(3)?,
        })),
        None => Ok(None),
    }
}

/// 仕事が成功したことを記録する
pub async fn complete(conn: &Connection, id: i64) -> Result<(), libsql::Error> {
    conn.execute(
        "UPDATE jobs SET status = 'done', locked_until = NULL WHERE id = ?1",
        [id],
    )
    .await?;
    Ok(())
}

/// 失敗を記録する。まだやり直せるなら retry_after だけ後に実行するよう列に戻し、
/// 回数を使い切ったら dead（もうやり直さない）にする。新しい状態を返す
pub async fn fail(
    conn: &Connection,
    id: i64,
    error: &str,
    retry_after: Duration,
) -> Result<String, libsql::Error> {
    let mut rows = conn
        .query(
            "UPDATE jobs
             SET status = CASE WHEN attempts >= max_attempts THEN 'dead' ELSE 'queued' END,
                 run_at = ?2, locked_until = NULL, last_error = ?3
             WHERE id = ?1
             RETURNING status",
            params![id, now_ms() + retry_after.as_millis() as i64, error],
        )
        .await?;
    let row = rows.next().await?.expect("その id の仕事があるはず");
    row.get::<String>(0)
}

/// 状態ごとの件数を、状態の名前の順に返す
pub async fn counts(conn: &Connection) -> Result<Vec<(String, i64)>, libsql::Error> {
    let mut rows = conn
        .query(
            "SELECT status, COUNT(*) FROM jobs GROUP BY status ORDER BY status",
            (),
        )
        .await?;
    let mut result = Vec::new();
    while let Some(row) = rows.next().await? {
        result.push((row.get(0)?, row.get(1)?));
    }
    Ok(result)
}
