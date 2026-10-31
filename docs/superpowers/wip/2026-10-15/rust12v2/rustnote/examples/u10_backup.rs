//! 第12巻 UNIT 10: 動いている RustNote の DB を、VACUUM INTO で、1つのファイルに写し取る（バックアップ）。
//! 実行: cargo run -q --example u10_backup -- 元の.db 写し.db

use rustnote::db::Db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let (Some(from), Some(to)) = (args.next(), args.next()) else {
        anyhow::bail!("使い方: u10_backup 元の.db 写し.db");
    };
    let db = Db::open_local(&from).await?;
    let conn = db.connect().await?;
    // VACUUM INTO は、ほかの接続が書いている途中でも、ある瞬間の DB 全体を1つのファイルに書き出す
    conn.execute("VACUUM INTO ?1", [to.as_str()]).await?;
    println!("{from} を {to} に写しました");
    Ok(())
}
