use std::env;
use std::time::Duration;

use libsql::Builder;

// 組み込みレプリカの使い方の例（Turso の本物の DB が必要。この本の PC ではコンパイルだけ確認）
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL とトークンは環境変数から読む（コードに書かない）
    let url = env::var("DATABASE_URL")?;
    let token = env::var("DATABASE_AUTH_TOKEN")?;

    // 手元のファイル replica.db を、Turso の DB のコピー置き場にする
    let db = Builder::new_remote_replica("replica.db", url, token)
        // 60 秒ごとに、裏で自動的にコピーを最新にする
        .sync_interval(Duration::from_secs(60))
        .build()
        .await?;
    // 起動したら、まず1回コピーを最新にする
    db.sync().await?;

    let conn = db.connect()?;
    // 書き込みは Turso（向こうの本体）へ送られる
    conn.execute("INSERT INTO memos (text) VALUES (?1)", ["レプリカから追加"])
        .await?;
    // 読み込みは手元のコピーから。自分が書いた行はすぐ見える（read-your-writes）
    let mut rows = conn.query("SELECT COUNT(*) FROM memos", ()).await?;
    if let Some(row) = rows.next().await? {
        println!("メモの数: {}", row.get::<i64>(0)?);
    }
    Ok(())
}
