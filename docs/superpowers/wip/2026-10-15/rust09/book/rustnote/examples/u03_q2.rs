//! 問2の解答: 大文字・小文字だけが違うメールアドレスは、DB の UNIQUE で止まるか。
//! 実行: cargo run -q --example u03_q2

use rustnote::db::{self, Db};
use rustnote::domain::Email;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let db = Db::open_local(dir.path().join("u03.db")).await?;
    db::migrate(&db).await?;
    let conn = db.connect().await?;
    let sql = "INSERT INTO users (email, password_hash) VALUES (?1, 'x')";

    println!("--- そろえずに、そのまま入れる");
    for input in ["Alice@Example.com", "alice@example.com"] {
        match conn.execute(sql, [input]).await {
            Ok(_) => println!("{input}: OK"),
            Err(error) => println!("{input}: NG {error}"),
        }
    }

    println!("--- Email::parse でそろえてから入れる");
    conn.execute("DELETE FROM users", ()).await?;
    for input in ["Alice@Example.com", "alice@example.com"] {
        let email = Email::parse(input)?;
        match conn.execute(sql, [email.as_str()]).await {
            Ok(_) => println!("{input} → {email}: OK"),
            Err(error) => println!("{input} → {email}: NG {error}"),
        }
    }
    Ok(())
}
