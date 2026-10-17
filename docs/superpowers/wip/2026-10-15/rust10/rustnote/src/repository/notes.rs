//! ノートの保存・検索・更新・削除。
//!
//! どのメソッドも「持ち主（owner）」を受け取り、SQL の WHERE に必ず
//! `user_id = 持ち主` を入れる。だから他人のノートは「見つからない」扱いになる。

use async_trait::async_trait;
use libsql::Row;

use super::invalid_data;
use crate::db::Db;
use crate::domain::{NewNote, Note, NoteId, NotePage, NoteQuery, PER_PAGE, UserId};
use crate::error::RepoError;

/// ノートの保存先に求める約束
#[async_trait]
pub trait NoteRepository {
    /// ノートを追加し、保存された内容（id や日時つき）を返す
    async fn insert(&self, owner: UserId, note: &NewNote) -> Result<Note, RepoError>;

    /// 自分のノートを1件探す（他人のノート・存在しないノートは None）
    async fn find(&self, owner: UserId, id: NoteId) -> Result<Option<Note>, RepoError>;

    /// 自分のノートを、キーワードとページ番号で検索する（新しい順）
    async fn search(&self, owner: UserId, query: &NoteQuery) -> Result<NotePage, RepoError>;

    /// タイトルと本文を更新する。
    /// `expected_version` が今の version と違えば RepoError::VersionMismatch（楽観的ロック）
    async fn update(
        &self,
        owner: UserId,
        id: NoteId,
        note: &NewNote,
        expected_version: i64,
    ) -> Result<Note, RepoError>;

    /// 完了（done）と未完了を切り替える
    async fn toggle_done(&self, owner: UserId, id: NoteId) -> Result<Note, RepoError>;

    /// ノートを削除する（無ければ RepoError::NotFound）
    async fn delete(&self, owner: UserId, id: NoteId) -> Result<(), RepoError>;
}

/// SELECT や RETURNING で読む列（row_to_note の順番と合わせる）
const NOTE_COLUMNS: &str = "id, user_id, title, body, done, version, created_at, updated_at";

/// libSQL（Turso / ローカルファイル）に保存する実装
pub struct LibsqlNoteRepository {
    db: Db,
}

impl LibsqlNoteRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl NoteRepository for LibsqlNoteRepository {
    async fn insert(&self, owner: UserId, note: &NewNote) -> Result<Note, RepoError> {
        let conn = self.db.connect().await?;
        let sql = format!(
            "INSERT INTO notes (user_id, title, body) VALUES (?1, ?2, ?3) RETURNING {NOTE_COLUMNS}"
        );
        let mut rows = conn
            .query(
                &sql,
                libsql::params![owner.get(), note.title.as_str(), note.body.as_str()],
            )
            .await?;
        let row = rows
            .next()
            .await?
            .ok_or_else(|| RepoError::InvalidData("INSERT が行を返しませんでした".to_string()))?;
        row_to_note(&row)
    }

    async fn find(&self, owner: UserId, id: NoteId) -> Result<Option<Note>, RepoError> {
        let conn = self.db.connect().await?;
        let sql = format!("SELECT {NOTE_COLUMNS} FROM notes WHERE id = ?1 AND user_id = ?2");
        let mut rows = conn
            .query(&sql, libsql::params![id.get(), owner.get()])
            .await?;
        match rows.next().await? {
            Some(row) => Ok(Some(row_to_note(&row)?)),
            None => Ok(None),
        }
    }

    async fn search(&self, owner: UserId, query: &NoteQuery) -> Result<NotePage, RepoError> {
        let conn = self.db.connect().await?;
        // キーワードが無いときは「%」（何にでも一致する）で検索する
        let pattern = match &query.keyword {
            Some(keyword) => format!("%{}%", escape_like(keyword)),
            None => "%".to_string(),
        };
        // LIKE の中の \ を「次の文字をそのまま扱う」という意味にする（ESCAPE '\'）
        let condition = "user_id = ?1 AND (title LIKE ?2 ESCAPE '\\' OR body LIKE ?2 ESCAPE '\\')";

        // 1) 条件に合う件数（ページ数の計算に使う）
        let count_sql = format!("SELECT COUNT(*) FROM notes WHERE {condition}");
        let mut rows = conn
            .query(&count_sql, libsql::params![owner.get(), pattern.as_str()])
            .await?;
        let total_count = match rows.next().await? {
            Some(row) => row.get::<i64>(0)?,
            None => 0,
        };

        // 2) このページに表示する分（新しく更新した順）
        let list_sql = format!(
            "SELECT {NOTE_COLUMNS} FROM notes WHERE {condition}
             ORDER BY updated_at DESC, id DESC
             LIMIT ?3 OFFSET ?4"
        );
        let offset = i64::try_from(query.offset()).unwrap_or(i64::MAX);
        let mut rows = conn
            .query(
                &list_sql,
                libsql::params![owner.get(), pattern.as_str(), i64::from(PER_PAGE), offset],
            )
            .await?;
        let mut notes = Vec::new();
        while let Some(row) = rows.next().await? {
            notes.push(row_to_note(&row)?);
        }

        Ok(NotePage {
            notes,
            page: query.page,
            total_count: u64::try_from(total_count).unwrap_or(0),
        })
    }

    async fn update(
        &self,
        owner: UserId,
        id: NoteId,
        note: &NewNote,
        expected_version: i64,
    ) -> Result<Note, RepoError> {
        let conn = self.db.connect().await?;
        // version が一致したときだけ更新し、version を 1 増やす
        let sql = format!(
            "UPDATE notes
                SET title = ?1, body = ?2, version = version + 1, updated_at = datetime('now')
              WHERE id = ?3 AND user_id = ?4 AND version = ?5
              RETURNING {NOTE_COLUMNS}"
        );
        let mut rows = conn
            .query(
                &sql,
                libsql::params![
                    note.title.as_str(),
                    note.body.as_str(),
                    id.get(),
                    owner.get(),
                    expected_version
                ],
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

    async fn toggle_done(&self, owner: UserId, id: NoteId) -> Result<Note, RepoError> {
        let conn = self.db.connect().await?;
        // done を 0 ⇔ 1 に反転する。中身が変わるので version も上げる
        let sql = format!(
            "UPDATE notes
                SET done = 1 - done, version = version + 1, updated_at = datetime('now')
              WHERE id = ?1 AND user_id = ?2
              RETURNING {NOTE_COLUMNS}"
        );
        let mut rows = conn
            .query(&sql, libsql::params![id.get(), owner.get()])
            .await?;
        match rows.next().await? {
            Some(row) => row_to_note(&row),
            None => Err(RepoError::NotFound),
        }
    }

    async fn delete(&self, owner: UserId, id: NoteId) -> Result<(), RepoError> {
        let conn = self.db.connect().await?;
        let deleted = conn
            .execute(
                "DELETE FROM notes WHERE id = ?1 AND user_id = ?2",
                libsql::params![id.get(), owner.get()],
            )
            .await?;
        if deleted == 0 {
            return Err(RepoError::NotFound);
        }
        Ok(())
    }
}

/// 行を Note に変える（列の順番は NOTE_COLUMNS と同じ）
fn row_to_note(row: &Row) -> Result<Note, RepoError> {
    let id = NoteId::new(row.get::<i64>(0)?).map_err(|e| invalid_data("notes.id", e))?;
    let owner = UserId::new(row.get::<i64>(1)?).map_err(|e| invalid_data("notes.user_id", e))?;
    Ok(Note {
        id,
        owner,
        title: row.get::<String>(2)?,
        body: row.get::<String>(3)?,
        done: row.get::<i64>(4)? != 0,
        version: row.get::<i64>(5)?,
        created_at: row.get::<String>(6)?,
        updated_at: row.get::<String>(7)?,
    })
}

/// LIKE で特別な意味を持つ文字（% と _ と \）を、ただの文字として扱わせる
fn escape_like(keyword: &str) -> String {
    let mut escaped = String::with_capacity(keyword.len());
    for c in keyword.chars() {
        if matches!(c, '\\' | '%' | '_') {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::escape_like;

    #[test]
    fn like_special_characters_are_escaped() {
        assert_eq!(escape_like("100%"), "100\\%");
        assert_eq!(escape_like("a_b"), "a\\_b");
        assert_eq!(escape_like("c:\\tmp"), "c:\\\\tmp");
        assert_eq!(escape_like("買い物"), "買い物");
    }
}
