use async_lab::job_queue;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let _ = std::fs::remove_dir_all("e10status");
    std::fs::create_dir_all("e10status").expect("e10status フォルダを作れません");
    let db = job_queue::open("e10status/jobs.db").await?;
    let conn = job_queue::connect(&db).await?;
    let id = job_queue::enqueue(&conn, "send_email", "hanako@example.com", 3).await?;

    // 'done' と書くつもりで 'finished' と書いてしまった
    conn.execute("UPDATE jobs SET status = 'finished' WHERE id = ?1", [id])
        .await?;
    println!("書き換えました");
    Ok(())
}
