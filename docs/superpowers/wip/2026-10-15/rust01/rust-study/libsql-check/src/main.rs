use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    // ローカルのファイル check.db を開く（無ければ新しく作られる）
    let db = Builder::new_local("check.db").build().await?;
    // 開いた DB との「接続」を作る。SQL はこの接続を通して送る
    let conn = db.connect()?;

    // DB エンジンの版を問い合わせる（libSQL は SQLite と互換）
    let mut rows = conn.query("SELECT sqlite_version()", ()).await?;
    if let Some(row) = rows.next().await? {
        let version: String = row.get(0)?;
        println!("SQLite 互換エンジンの版: {version}");
    }

    // 表が無ければ作り、確認のしるしを1行足す
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pings (id INTEGER PRIMARY KEY, note TEXT NOT NULL)",
        (),
    )
    .await?;
    conn.execute("INSERT INTO pings (note) VALUES (?1)", ["hello libSQL"])
        .await?;

    // 何行たまったかを数える（実行するたびに 1 ずつ増える）
    let mut rows = conn.query("SELECT COUNT(*) FROM pings", ()).await?;
    if let Some(row) = rows.next().await? {
        let count: i64 = row.get(0)?;
        println!("pings の行数: {count}");
    }

    println!("接続 OK: check.db");
    Ok(())
}
