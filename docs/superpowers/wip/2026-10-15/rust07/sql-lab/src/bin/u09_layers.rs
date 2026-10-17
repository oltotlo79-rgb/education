use std::error::Error as _;
use std::sync::Arc;

use async_trait::async_trait;
use libsql::{Builder, Connection, Row};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
struct UserId(i64);

#[derive(Debug, Clone, Copy)]
struct NoteId(i64);

#[derive(Debug)]
struct Note {
    id: NoteId,
    title: String,
    version: i64,
}

// ---- 保存先の層の失敗（RustNote の RepoError と同じ分類） ----
#[derive(Debug, Error)]
enum RepoError {
    #[error("version が一致しません")]
    VersionMismatch,
    #[error("対象のデータが見つかりません")]
    NotFound,
    #[error("DB の値を読み取れません: {0}")]
    InvalidData(String),
    #[error("DB の操作に失敗しました")]
    Database(#[from] libsql::Error),
}

// ---- アプリの層の失敗（利用者への返事を決める） ----
#[derive(Debug, Error)]
enum AppError {
    #[error("ページが見つかりません")]
    NotFound,
    #[error("ほかの画面で先に更新されました。読み込み直してください")]
    Conflict,
    #[error("サーバーの中でエラーが起きました")]
    Internal(#[source] RepoError),
}

impl From<RepoError> for AppError {
    fn from(error: RepoError) -> Self {
        match error {
            RepoError::VersionMismatch => AppError::Conflict,
            RepoError::NotFound => AppError::NotFound,
            other => AppError::Internal(other),
        }
    }
}

impl AppError {
    fn status_code(&self) -> u16 {
        match self {
            AppError::NotFound => 404,
            AppError::Conflict => 409,
            AppError::Internal(_) => 500,
        }
    }
}

// ---- Repository の約束（trait）。async fn を dyn で使うため #[async_trait] を付ける ----
#[async_trait]
trait NoteRepository {
    async fn find(&self, owner: UserId, id: NoteId) -> Result<Option<Note>, RepoError>;
    async fn rename(
        &self,
        owner: UserId,
        id: NoteId,
        title: &str,
        expected_version: i64,
    ) -> Result<Note, RepoError>;
}

// ---- libSQL で約束を守る実装。SQL はこの中にしか書かない ----
struct LibsqlNoteRepository {
    conn: Connection,
}

const NOTE_COLUMNS: &str = "id, title, version";

#[async_trait]
impl NoteRepository for LibsqlNoteRepository {
    async fn find(&self, owner: UserId, id: NoteId) -> Result<Option<Note>, RepoError> {
        let sql = format!("SELECT {NOTE_COLUMNS} FROM notes WHERE id = ?1 AND user_id = ?2");
        let mut rows = self.conn.query(&sql, [id.0, owner.0]).await?;
        match rows.next().await? {
            Some(row) => Ok(Some(row_to_note(&row)?)),
            None => Ok(None),
        }
    }

    async fn rename(
        &self,
        owner: UserId,
        id: NoteId,
        title: &str,
        expected_version: i64,
    ) -> Result<Note, RepoError> {
        // version が一致したときだけ更新し、version を 1 増やす（UNIT 05 の楽観的ロック）
        let sql = format!(
            "UPDATE notes SET title = ?1, version = version + 1
              WHERE id = ?2 AND user_id = ?3 AND version = ?4
              RETURNING {NOTE_COLUMNS}"
        );
        let mut rows = self
            .conn
            .query(
                &sql,
                libsql::params![title, id.0, owner.0, expected_version],
            )
            .await?;
        if let Some(row) = rows.next().await? {
            return row_to_note(&row);
        }
        // 1行も更新されなかった: ノートが無いのか、version が違うのかを見分ける
        match self.find(owner, id).await? {
            Some(_) => Err(RepoError::VersionMismatch),
            None => Err(RepoError::NotFound),
        }
    }
}

// 1行を Note に変える。おかしな値なら、どの列のどの行かを添えて InvalidData にする
fn row_to_note(row: &Row) -> Result<Note, RepoError> {
    let id = row.get::<i64>(0)?;
    let title = row.get::<String>(1)?;
    let len = title.chars().count();
    if !(1..=80).contains(&len) {
        return Err(RepoError::InvalidData(format!(
            "notes.title (id={id}): 1〜80 文字ではありません（{len} 文字）"
        )));
    }
    Ok(Note {
        id: NoteId(id),
        title,
        version: row.get::<i64>(2)?,
    })
}

// ---- ハンドラーのつもりの関数。SQL を1行も知らず、trait だけを使う ----
async fn show_note(
    repo: &(dyn NoteRepository + Send + Sync),
    owner: UserId,
    id: NoteId,
) -> Result<String, AppError> {
    let note = repo.find(owner, id).await?.ok_or(AppError::NotFound)?;
    Ok(format!(
        "#{} {}（version {}）",
        note.id.0, note.title, note.version
    ))
}

async fn rename_note(
    repo: &(dyn NoteRepository + Send + Sync),
    owner: UserId,
    id: NoteId,
    title: &str,
    expected_version: i64,
) -> Result<String, AppError> {
    let note = repo.rename(owner, id, title, expected_version).await?;
    Ok(format!(
        "保存しました: {}（version {}）",
        note.title, note.version
    ))
}

// 結果を「ステータス 文章」の形で表示する。500 のときは原因をログ（標準エラー）へ
fn report(label: &str, result: Result<String, AppError>) {
    match result {
        Ok(message) => println!("{label} → 200 {message}"),
        Err(error) => {
            println!("{label} → {} {error}", error.status_code());
            if let Some(source) = error.source() {
                eprintln!("  [log] 原因: {source}");
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    // 本物の実装を、trait の型（dyn）として持つ。RustNote の AppState と同じ形
    let repo: Arc<dyn NoteRepository + Send + Sync> =
        Arc::new(LibsqlNoteRepository { conn: conn.clone() });
    let alice = UserId(1);

    report(
        "① 自分の #2 を表示",
        show_note(&*repo, alice, NoteId(2)).await,
    );
    report(
        "② Bob の #3 を表示",
        show_note(&*repo, alice, NoteId(3)).await,
    );
    report(
        "③ #2 を version 3 で更新",
        rename_note(&*repo, alice, NoteId(2), "Rust の復習", 3).await,
    );
    report(
        "④ #2 を古い version 3 で更新",
        rename_note(&*repo, alice, NoteId(2), "別のタブから", 3).await,
    );

    // CHECK 制約を一時的に無視して、壊れた行をわざと作る（実験のためだけ）
    conn.execute_batch(
        "PRAGMA ignore_check_constraints = ON;
         UPDATE notes SET title = '' WHERE id = 4;
         PRAGMA ignore_check_constraints = OFF;",
    )
    .await?;
    report(
        "⑤ 壊れた #4 を表示",
        show_note(&*repo, alice, NoteId(4)).await,
    );
    Ok(())
}
