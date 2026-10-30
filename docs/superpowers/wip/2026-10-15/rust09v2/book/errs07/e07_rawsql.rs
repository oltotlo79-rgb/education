//! 見本のユーザーを、SQL で直接入れてしまった（パスワードをそのまま書いた）
use rustnote::{config::Config, db::{self, Db}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = Db::open(&Config::from_env()?).await?;
    db::migrate(&db).await?;
    let conn = db.connect().await?;
    conn.execute(
        "INSERT INTO users (email, password_hash) VALUES ('carol@example.com', 'correct horse battery')",
        (),
    )
    .await?;
    println!("carol@example.com を入れました");
    Ok(())
}
