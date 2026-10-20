use std::path::Path;

use libsql::Builder;

// dir の中の backup-*.db を新しいほうから keep 個だけ残し、残りを消す。消した名前を返す
fn prune_backups(dir: &Path, keep: usize) -> std::io::Result<Vec<String>> {
    let mut names: Vec<String> = std::fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("backup-") && name.ends_with(".db"))
        .collect();
    // 名前の日付は「年-月-日」の順で、桁もそろえてあるので、文字の順に並べると古い順になる
    names.sort();
    let remove_count = names.len().saturating_sub(keep);
    let removed: Vec<String> = names.drain(..remove_count).collect();
    for name in &removed {
        std::fs::remove_file(dir.join(name))?;
    }
    Ok(removed)
}

// フォルダの中のファイル名を、名前の順に並べて返す
fn list(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .expect("フォルダを読めません")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let dir = Path::new("u10q1");
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).expect("u10q1 フォルダを作れません");
    let db = Builder::new_local("u10q1/app.db").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    // 5日分の写しを作ったことにする（名前に日付を入れる）
    for day in 17..=21 {
        let path = format!("u10q1/backup-2026-10-{day}.db");
        conn.execute("VACUUM INTO ?1", [path]).await?;
    }
    // 写しではないファイルも置いておく（消してはいけない）
    std::fs::write(dir.join("memo.txt"), "手で置いたメモ").expect("メモを書けません");
    println!("整理の前: {:?}", list(dir));

    let removed = prune_backups(dir, 3).expect("写しを整理できません");
    println!("消した写し: {removed:?}");
    println!("整理の後: {:?}", list(dir));
    Ok(())
}
