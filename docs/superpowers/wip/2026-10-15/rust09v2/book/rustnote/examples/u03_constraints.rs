//! RustNote と同じマイグレーションで表を作り、わざと決まりを破るデータを入れて、
//! DB がどう断るかを確かめる。
//! 実行: cargo run -q --example u03_constraints

use libsql::Connection;
use rustnote::db::{self, Db};

// SQL を1つ実行し、成功か失敗かを1行で表示する
async fn try_sql(conn: &Connection, label: &str, sql: &str) {
    match conn.execute(sql, ()).await {
        Ok(rows) => println!("OK  {label}（{rows} 行）"),
        Err(error) => println!("NG  {label} → {error}"),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 使い捨ての一時フォルダに DB を作る（プログラムが終わるとフォルダごと消える）
    let dir = tempfile::tempdir()?;
    let db = Db::open_local(dir.path().join("u03.db")).await?;
    // RustNote の起動時と同じ関数で、3つのマイグレーションを適用する
    let applied = db::migrate(&db).await?;
    println!("適用したマイグレーション: {applied:?}");

    // RustNote と同じ接続の作り方（外部キーの確認を ON にした接続）
    let conn = db.connect().await?;
    try_sql(
        &conn,
        "alice を登録",
        "INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'x')",
    )
    .await;
    try_sql(
        &conn,
        "同じメールでもう一度登録",
        "INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'y')",
    )
    .await;
    try_sql(
        &conn,
        "空のタイトル",
        "INSERT INTO notes (user_id, title) VALUES (1, '')",
    )
    .await;
    try_sql(
        &conn,
        "done に 2 を入れる",
        "INSERT INTO notes (user_id, title, done) VALUES (1, '宿題', 2)",
    )
    .await;
    try_sql(
        &conn,
        "いないユーザー 999 のノート",
        "INSERT INTO notes (user_id, title) VALUES (999, '宿題')",
    )
    .await;
    try_sql(
        &conn,
        "alice のノート",
        "INSERT INTO notes (user_id, title) VALUES (1, '宿題')",
    )
    .await;
    try_sql(
        &conn,
        "alice のセッション",
        "INSERT INTO sessions (token_hash, user_id, expires_at) VALUES (x'00', 1, 0)",
    )
    .await;

    // ユーザーを消すと、そのユーザーのノートとセッションも一緒に消える（ON DELETE CASCADE）
    try_sql(&conn, "alice を削除", "DELETE FROM users WHERE id = 1").await;
    for table in ["notes", "sessions"] {
        let sql = format!("SELECT COUNT(*) FROM {table}");
        let mut rows = conn.query(&sql, ()).await?;
        if let Some(row) = rows.next().await? {
            println!("残っている {table}: {} 行", row.get::<i64>(0)?);
        }
    }
    Ok(())
}
