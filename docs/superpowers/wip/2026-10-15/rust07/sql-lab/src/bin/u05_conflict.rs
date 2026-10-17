use libsql::{Builder, Connection};

// 保存先（DB）の層の失敗
#[derive(Debug)]
enum RepoError {
    // version が食い違った（ほかの画面が先に保存した）
    VersionMismatch,
    // ノートが無い（または他人のノート）
    NotFound,
    Database(libsql::Error),
}

impl From<libsql::Error> for RepoError {
    fn from(error: libsql::Error) -> Self {
        RepoError::Database(error)
    }
}

// 利用者への返事を決める層の失敗（第4巻 UNIT 05 の形）
#[derive(Debug)]
enum AppError {
    NotFound,
    Conflict,
    Internal,
}

impl From<RepoError> for AppError {
    fn from(error: RepoError) -> Self {
        match error {
            RepoError::VersionMismatch => AppError::Conflict,
            RepoError::NotFound => AppError::NotFound,
            RepoError::Database(error) => {
                // 原因はログに残し、利用者には詳しく見せない
                eprintln!("DB エラー: {error}");
                AppError::Internal
            }
        }
    }
}

impl AppError {
    fn status_code(&self) -> u16 {
        match self {
            AppError::NotFound => 404,
            AppError::Conflict => 409,
            AppError::Internal => 500,
        }
    }
}

// 画面を開いたときに読んだ version（expected）と一致したときだけ更新する
async fn update_title(
    conn: &Connection,
    owner: i64,
    id: i64,
    title: &str,
    expected: i64,
) -> Result<i64, RepoError> {
    let changed = conn
        .execute(
            "UPDATE notes SET title = ?1, version = version + 1
              WHERE id = ?2 AND user_id = ?3 AND version = ?4",
            libsql::params![title, id, owner, expected],
        )
        .await?;
    if changed == 1 {
        return Ok(expected + 1);
    }
    // 0 行だった: ノートが無いのか、version が違うのかを見分ける
    let mut rows = conn
        .query(
            "SELECT 1 FROM notes WHERE id = ?1 AND user_id = ?2",
            libsql::params![id, owner],
        )
        .await?;
    match rows.next().await? {
        Some(_) => Err(RepoError::VersionMismatch),
        None => Err(RepoError::NotFound),
    }
}

// ノート 1 の今の状態を表示する
async fn show(conn: &Connection) -> Result<(), libsql::Error> {
    let mut rows = conn
        .query("SELECT title, version FROM notes WHERE id = 1", ())
        .await?;
    if let Some(row) = rows.next().await? {
        let title = row.get::<String>(0)?;
        let version = row.get::<i64>(1)?;
        println!("   DB: title = {title}, version = {version}");
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    // Alice（1）がタブ A とタブ B で、同じノート 1（version 1）の編集画面を開いた
    let opened_version = 1;
    show(&conn).await?;

    println!("タブ A が保存（version {opened_version} のつもり）");
    let result = update_title(&conn, 1, 1, "買い物リスト（A）", opened_version).await;
    println!("   結果: {result:?}");
    show(&conn).await?;

    println!("タブ B が保存（version {opened_version} のつもり）");
    let result = update_title(&conn, 1, 1, "買い物リスト（B）", opened_version).await;
    println!("   結果: {result:?}");
    if let Err(error) = result {
        let app_error = AppError::from(error);
        println!("   → {app_error:?} → HTTP {}", app_error.status_code());
    }
    show(&conn).await?;

    println!("Bob のノート 3 を Alice が保存しようとする");
    let result = update_title(&conn, 1, 3, "乗っ取り", 1).await;
    if let Err(error) = result {
        let app_error = AppError::from(error);
        println!("   → {app_error:?} → HTTP {}", app_error.status_code());
    }
    Ok(())
}
