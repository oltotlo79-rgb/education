use libsql::{Builder, Connection, Row};
use thiserror::Error;

// ---- ドメインの型（第4巻 UNIT 01 の形を小さくしたもの） ----
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UserId(i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NoteId(i64);

impl NoteId {
    // 1 以上の数だけを受け付ける
    fn new(value: i64) -> Result<NoteId, String> {
        if value <= 0 {
            return Err(format!("{value} は 1 以上ではありません"));
        }
        Ok(NoteId(value))
    }
}

#[derive(Debug)]
struct NoteTitle(String);

impl NoteTitle {
    // 1〜80 文字だけを受け付ける
    fn parse(input: &str) -> Result<NoteTitle, String> {
        let len = input.chars().count();
        if len == 0 || len > 80 {
            return Err(format!("タイトルは 1〜80 文字です（{len} 文字）"));
        }
        Ok(NoteTitle(input.to_string()))
    }
}

#[derive(Debug)]
struct Note {
    id: NoteId,
    owner: UserId,
    title: NoteTitle,
    done: bool,
}

// ---- 保存先の層の失敗 ----
#[derive(Debug, Error)]
enum RepoError {
    #[error("DB の値を読み取れません: {0}")]
    InvalidData(String),
    #[error("DB の操作に失敗しました: {0}")]
    Database(#[from] libsql::Error),
}

// ---- Repository: notes 表の SQL はすべてここに集める ----
struct NoteRepo {
    conn: Connection,
}

// SELECT で読む列。row_to_note の番号と順番をそろえる
const NOTE_COLUMNS: &str = "id, user_id, title, done";

impl NoteRepo {
    // 自分のノートを新しい順に全部返す
    async fn list_by_owner(&self, owner: UserId) -> Result<Vec<Note>, RepoError> {
        let sql = format!("SELECT {NOTE_COLUMNS} FROM notes WHERE user_id = ?1 ORDER BY id DESC");
        let mut rows = self.conn.query(&sql, [owner.0]).await?;
        let mut notes = Vec::new();
        while let Some(row) = rows.next().await? {
            notes.push(row_to_note(&row)?);
        }
        Ok(notes)
    }

    // 自分のノートを1件探す。他人のノートや無いノートは None
    async fn find(&self, owner: UserId, id: NoteId) -> Result<Option<Note>, RepoError> {
        let sql = format!("SELECT {NOTE_COLUMNS} FROM notes WHERE id = ?1 AND user_id = ?2");
        let mut rows = self.conn.query(&sql, [id.0, owner.0]).await?;
        match rows.next().await? {
            Some(row) => Ok(Some(row_to_note(&row)?)),
            None => Ok(None),
        }
    }
}

// 1行を Note に変える。変換できなければ「どの列か」を添えて InvalidData にする
fn row_to_note(row: &Row) -> Result<Note, RepoError> {
    let id = NoteId::new(row.get::<i64>(0)?)
        .map_err(|e| RepoError::InvalidData(format!("notes.id: {e}")))?;
    let title = NoteTitle::parse(&row.get::<String>(2)?)
        .map_err(|e| RepoError::InvalidData(format!("notes.title (id={}): {e}", id.0)))?;
    Ok(Note {
        id,
        owner: UserId(row.get::<i64>(1)?),
        title,
        done: row.get::<i64>(3)? != 0,
    })
}

#[tokio::main]
async fn main() -> Result<(), RepoError> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    let repo = NoteRepo { conn };
    let alice = UserId(1);

    println!("--- Alice のノート一覧 ---");
    for note in repo.list_by_owner(alice).await? {
        let mark = if note.done { "済" } else { "  " };
        println!(
            "[{mark}] #{} {}（持ち主 {}）",
            note.id.0, note.title.0, note.owner.0
        );
    }

    println!("--- 1件だけ探す ---");
    let mine = repo.find(alice, NoteId(2)).await?;
    println!("Alice が #2 を探す: {mine:?}");
    let others = repo.find(alice, NoteId(3)).await?;
    println!("Alice が #3（Bob のノート）を探す: {others:?}");
    Ok(())
}
