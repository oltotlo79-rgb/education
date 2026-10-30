//! ドメイン（アプリが扱う「もの」）の型。
//!
//! 外から来た文字列（フォーム・URL・DB の値）は、ここの `parse` / `new` を通して
//! 検査済みの型に変える。中身は private なので、検査を通らずに作る方法はない。
//! 「`NoteTitle` 型の値がある」＝「1〜80 文字の検査に合格した」が必ず成り立つ。

use std::fmt;
use std::str::FromStr;

use thiserror::Error;

// ---------------------------------------------------------------------------
// アプリ全体の定数
// ---------------------------------------------------------------------------

/// タイトルの最大文字数（chars().count() で数える。SQLite の length() と同じ）
pub const MAX_TITLE_CHARS: usize = 80;
/// 本文の最大文字数
pub const MAX_BODY_CHARS: usize = 10_000;
/// パスワードの最小文字数
pub const MIN_PASSWORD_CHARS: usize = 8;
/// パスワードの最大文字数
pub const MAX_PASSWORD_CHARS: usize = 128;
/// 一覧の1ページに表示するノートの数
pub const PER_PAGE: u32 = 20;
/// 検索キーワードの最大文字数（長すぎる入力で DB に負担をかけないため）
pub const MAX_KEYWORD_CHARS: usize = 100;

// ---------------------------------------------------------------------------
// ID（UserId / NoteId）: 中身は同じ i64 でも、取り違えるとコンパイルエラーになる
// ---------------------------------------------------------------------------

/// ID が不正だった理由
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum IdError {
    #[error("ID は数字で指定してください")]
    NotANumber,
    #[error("ID は 1 以上の数です")]
    NotPositive,
}

/// ユーザーの ID（users.id）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UserId(i64);

impl UserId {
    /// 1 以上の数だけを受け付ける
    pub fn new(value: i64) -> Result<UserId, IdError> {
        if value <= 0 {
            return Err(IdError::NotPositive);
        }
        Ok(UserId(value))
    }

    /// 中身の数を取り出す（DB に渡すときに使う）
    pub fn get(self) -> i64 {
        self.0
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// ノートの ID（notes.id）。URL の `/notes/{id}` から作る
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NoteId(i64);

impl NoteId {
    /// 1 以上の数だけを受け付ける
    pub fn new(value: i64) -> Result<NoteId, IdError> {
        if value <= 0 {
            return Err(IdError::NotPositive);
        }
        Ok(NoteId(value))
    }

    /// 中身の数を取り出す（DB に渡すときに使う）
    pub fn get(self) -> i64 {
        self.0
    }
}

// "42".parse::<NoteId>() と書けるようにする。
// Topcoat の module_param! もこの FromStr を使って URL の {id} を NoteId に変える。
impl FromStr for NoteId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: i64 = s.parse().map_err(|_| IdError::NotANumber)?;
        NoteId::new(value)
    }
}

// href! で URL を組み立てるときに使う（/notes/42 の 42 の部分）
impl fmt::Display for NoteId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// メールアドレス
// ---------------------------------------------------------------------------

/// メールアドレスが不正だった理由
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum EmailError {
    #[error("メールアドレスに @ がありません")]
    MissingAt,
    #[error("メールアドレスの形式が正しくありません")]
    Malformed,
    #[error("メールアドレスが長すぎます")]
    TooLong,
}

/// 検査済みのメールアドレス（前後の空白を除き、小文字にそろえてある）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    /// メールアドレスの最大の長さ（バイト数）
    pub const MAX_LEN: usize = 254;

    pub fn parse(input: &str) -> Result<Email, EmailError> {
        // 同じ人を同じ値にするため、前後の空白を除いて小文字にそろえる
        let normalized = input.trim().to_lowercase();
        if normalized.len() > Self::MAX_LEN {
            return Err(EmailError::TooLong);
        }
        let Some((local, domain)) = normalized.split_once('@') else {
            return Err(EmailError::MissingAt);
        };
        let bad = local.is_empty()
            || domain.is_empty()
            || domain.contains('@')
            || !domain.contains('.')
            || domain.starts_with('.')
            || domain.ends_with('.')
            || normalized.contains(char::is_whitespace);
        if bad {
            return Err(EmailError::Malformed);
        }
        Ok(Email(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ---------------------------------------------------------------------------
// パスワード（ハッシュ化する前の、利用者が入力したもの）
// ---------------------------------------------------------------------------

/// パスワードが不正だった理由
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum PasswordError {
    #[error("パスワードは{min}文字以上にしてください")]
    TooShort { min: usize },
    #[error("パスワードは{max}文字以内にしてください")]
    TooLong { max: usize },
}

/// 検査済みのパスワード。
/// Debug で中身が表示されないよう、Debug は手で実装している（ログへの漏れを防ぐ）。
#[derive(Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn parse(input: &str) -> Result<Password, PasswordError> {
        // パスワードは空白も含めて利用者が決めたものなので、trim しない
        let len = input.chars().count();
        if len < MIN_PASSWORD_CHARS {
            return Err(PasswordError::TooShort {
                min: MIN_PASSWORD_CHARS,
            });
        }
        if len > MAX_PASSWORD_CHARS {
            return Err(PasswordError::TooLong {
                max: MAX_PASSWORD_CHARS,
            });
        }
        Ok(Password(input.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Password(***)")
    }
}

// ---------------------------------------------------------------------------
// ノートのタイトルと本文
// ---------------------------------------------------------------------------

/// タイトルが不正だった理由
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum TitleError {
    #[error("タイトルを入力してください")]
    Empty,
    #[error("タイトルは{max}文字以内にしてください（現在{len}文字）")]
    TooLong { len: usize, max: usize },
}

/// 検査済みのタイトル（前後の空白を除いて 1〜80 文字）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteTitle(String);

impl NoteTitle {
    pub const MAX_CHARS: usize = MAX_TITLE_CHARS;

    /// 文字列を検査し、合格したときだけ NoteTitle を返す
    pub fn parse(input: &str) -> Result<NoteTitle, TitleError> {
        let trimmed = input.trim();
        // 文字数は chars().count() で数える（len() はバイト数なので日本語で狂う）
        let len = trimmed.chars().count();
        if len == 0 {
            return Err(TitleError::Empty);
        }
        if len > Self::MAX_CHARS {
            return Err(TitleError::TooLong {
                len,
                max: Self::MAX_CHARS,
            });
        }
        Ok(NoteTitle(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for NoteTitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// 本文が不正だった理由
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum BodyError {
    #[error("本文は{max}文字以内にしてください（現在{len}文字）")]
    TooLong { len: usize, max: usize },
}

/// 検査済みの本文（空でもよい。最大 10,000 文字）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteBody(String);

impl NoteBody {
    pub const MAX_CHARS: usize = MAX_BODY_CHARS;

    pub fn parse(input: &str) -> Result<NoteBody, BodyError> {
        // ブラウザのフォームは改行を \r\n で送ってくるので \n にそろえる
        let normalized = input.replace("\r\n", "\n");
        let len = normalized.chars().count();
        if len > Self::MAX_CHARS {
            return Err(BodyError::TooLong {
                len,
                max: Self::MAX_CHARS,
            });
        }
        Ok(NoteBody(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ---------------------------------------------------------------------------
// ユーザーとノート
// ---------------------------------------------------------------------------

/// ログインできるユーザー（パスワードのハッシュは含めない）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub email: Email,
}

/// DB から読んだノート
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    pub id: NoteId,
    /// 持ち主（notes.user_id）
    pub owner: UserId,
    pub title: String,
    pub body: String,
    pub done: bool,
    /// 楽観的ロック用の番号。更新のたびに 1 増える
    pub version: i64,
    /// 作成日時（UTC、'YYYY-MM-DD HH:MM:SS'）
    pub created_at: String,
    /// 最終更新日時（UTC、'YYYY-MM-DD HH:MM:SS'）
    pub updated_at: String,
}

/// 作成・編集フォームの入力を検査したもの（タイトルと本文のセット）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewNote {
    pub title: NoteTitle,
    pub body: NoteBody,
}

/// 作成・編集フォームの検査で見つかった問題（項目ごと）
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NoteInputErrors {
    pub title: Option<TitleError>,
    pub body: Option<BodyError>,
}

impl NewNote {
    /// タイトルと本文をまとめて検査する。
    /// 両方の問題を一度に画面へ出せるよう、最初の失敗で止めずに全部集める。
    pub fn parse(title: &str, body: &str) -> Result<NewNote, NoteInputErrors> {
        match (NoteTitle::parse(title), NoteBody::parse(body)) {
            (Ok(title), Ok(body)) => Ok(NewNote { title, body }),
            (title, body) => Err(NoteInputErrors {
                title: title.err(),
                body: body.err(),
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// 一覧の検索条件とページ送り
// ---------------------------------------------------------------------------

/// 一覧の検索条件（キーワードとページ番号）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteQuery {
    /// 検索キーワード。空なら None（すべて表示）
    pub keyword: Option<String>,
    /// 1 から始まるページ番号
    pub page: u32,
}

impl NoteQuery {
    /// URL の ?q= と ?page= から作る。空白だけのキーワードは無いのと同じに扱う
    pub fn new(keyword: Option<&str>, page: Option<u32>) -> NoteQuery {
        let keyword = keyword
            .map(str::trim)
            .filter(|k| !k.is_empty())
            .map(|k| k.chars().take(MAX_KEYWORD_CHARS).collect());
        NoteQuery {
            keyword,
            page: page.unwrap_or(1).max(1),
        }
    }

    /// SQL の OFFSET（何件読み飛ばすか）
    pub fn offset(&self) -> u64 {
        u64::from(self.page - 1) * u64::from(PER_PAGE)
    }
}

/// 一覧の1ページ分の結果
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotePage {
    pub notes: Vec<Note>,
    /// 今のページ番号（1 から）
    pub page: u32,
    /// 条件に合うノートの総数
    pub total_count: u64,
}

impl NotePage {
    /// 全部で何ページあるか（0 件でも 1 ページと数える）
    pub fn total_pages(&self) -> u32 {
        let pages = self.total_count.div_ceil(u64::from(PER_PAGE)).max(1);
        u32::try_from(pages).unwrap_or(u32::MAX)
    }

    pub fn has_prev(&self) -> bool {
        self.page > 1
    }

    pub fn has_next(&self) -> bool {
        self.page < self.total_pages()
    }
}

// ---------------------------------------------------------------------------
// ユニットテスト（cargo test で実行される）
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_title_is_trimmed() {
        let title = NoteTitle::parse("  買い物  ").unwrap();
        assert_eq!(title.as_str(), "買い物");
    }

    #[test]
    fn note_title_rejects_empty_and_spaces() {
        assert_eq!(NoteTitle::parse(""), Err(TitleError::Empty));
        assert_eq!(NoteTitle::parse("   "), Err(TitleError::Empty));
    }

    #[test]
    fn note_title_counts_characters_not_bytes() {
        // 日本語 80 文字は 240 バイトだが、文字数では 80 なので合格
        let eighty = "あ".repeat(80);
        assert!(NoteTitle::parse(&eighty).is_ok());

        let eighty_one = "あ".repeat(81);
        assert_eq!(
            NoteTitle::parse(&eighty_one),
            Err(TitleError::TooLong { len: 81, max: 80 })
        );
    }

    #[test]
    fn note_body_normalizes_newlines_and_limits_length() {
        let body = NoteBody::parse("1行目\r\n2行目").unwrap();
        assert_eq!(body.as_str(), "1行目\n2行目");

        let too_long = "a".repeat(MAX_BODY_CHARS + 1);
        assert!(matches!(
            NoteBody::parse(&too_long),
            Err(BodyError::TooLong { .. })
        ));
    }

    #[test]
    fn new_note_collects_all_errors() {
        let too_long_body = "a".repeat(MAX_BODY_CHARS + 1);
        let errors = NewNote::parse(" ", &too_long_body).unwrap_err();
        assert_eq!(errors.title, Some(TitleError::Empty));
        assert!(errors.body.is_some());
    }

    #[test]
    fn email_is_normalized() {
        let email = Email::parse("  Alice@Example.COM ").unwrap();
        assert_eq!(email.as_str(), "alice@example.com");
    }

    #[test]
    fn email_rejects_bad_forms() {
        assert_eq!(Email::parse("alice"), Err(EmailError::MissingAt));
        assert_eq!(Email::parse("@example.com"), Err(EmailError::Malformed));
        assert_eq!(Email::parse("alice@example"), Err(EmailError::Malformed));
        assert_eq!(Email::parse("a@b@c.com"), Err(EmailError::Malformed));
        assert_eq!(
            Email::parse("al ice@example.com"),
            Err(EmailError::Malformed)
        );
    }

    #[test]
    fn password_length_is_checked_and_hidden_in_debug() {
        assert_eq!(
            Password::parse("short"),
            Err(PasswordError::TooShort { min: 8 })
        );
        let password = Password::parse("correct horse").unwrap();
        assert_eq!(format!("{password:?}"), "Password(***)");
    }

    #[test]
    fn note_id_parses_only_positive_numbers() {
        assert_eq!("42".parse::<NoteId>().map(NoteId::get), Ok(42));
        assert_eq!("0".parse::<NoteId>(), Err(IdError::NotPositive));
        assert_eq!("-3".parse::<NoteId>(), Err(IdError::NotPositive));
        assert_eq!("abc".parse::<NoteId>(), Err(IdError::NotANumber));
    }

    #[test]
    fn note_query_defaults_and_offset() {
        let query = NoteQuery::new(Some("  "), None);
        assert_eq!(query.keyword, None);
        assert_eq!(query.page, 1);
        assert_eq!(query.offset(), 0);

        let query = NoteQuery::new(Some(" rust "), Some(3));
        assert_eq!(query.keyword.as_deref(), Some("rust"));
        assert_eq!(query.offset(), 40);

        // page=0 は 1 として扱う
        assert_eq!(NoteQuery::new(None, Some(0)).page, 1);
    }

    #[test]
    fn note_page_counts_pages() {
        let page = |total_count, page| NotePage {
            notes: Vec::new(),
            page,
            total_count,
        };
        assert_eq!(page(0, 1).total_pages(), 1);
        assert_eq!(page(20, 1).total_pages(), 1);
        assert_eq!(page(21, 1).total_pages(), 2);
        assert!(page(21, 1).has_next());
        assert!(!page(21, 2).has_next());
        assert!(page(21, 2).has_prev());
    }
}
