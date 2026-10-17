//! 画面の部品（コンポーネント）と、エラー画面。
//!
//! `view!` の中の `(式)` は自動で HTML エスケープされる。利用者が入力した
//! `<script>` などは、文字としてそのまま表示されるだけで実行されない。

use topcoat::{
    Error, Result,
    asset::{Asset, asset},
    context::Cx,
    router::{
        StatusCode,
        error::{BadRequestError, NotFoundError},
        href,
    },
    view::{View, class, component, view},
};

use crate::domain::{MAX_BODY_CHARS, MAX_TITLE_CHARS, NoteInputErrors, NotePage, NoteQuery, User};
use crate::error::{AppError, RepoError};
use crate::flash::Flash;

/// 画面全体のスタイルシート（assets/style.css）。
/// `topcoat asset bundle`（または `topcoat dev`）で、ハッシュ付きのファイル名で配信される。
pub const STYLESHEET: Asset = asset!("assets/style.css");

// ---------------------------------------------------------------------------
// 共通の部品
// ---------------------------------------------------------------------------

/// 画面の上の帯（アプリ名・ログイン中のメール・ログアウトボタン）
#[component]
pub async fn site_header(user: Option<User>) -> Result<impl View> {
    Ok(view! {
        <header class="site-header">
            <a class="brand" href=(href!(crate::routes::home))>"RustNote"</a>
            <nav>
                if let Some(user) = user {
                    <a href=(href!(crate::routes::notes::index))>"ノート一覧"</a>
                    <span class="user-email">(user.email.as_str())</span>
                    <form method="post" action=(href!(crate::routes::logout::logout))>
                        <button type="submit" class="link-button">
                            "ログアウト"
                        </button>
                    </form>
                } else {
                    <a href=(href!(crate::routes::login::login_page))>"ログイン"</a>
                    <a href=(href!(crate::routes::signup::signup_page))>
                        "新規登録"
                    </a>
                }
            </nav>
        </header>
    })
}

/// フラッシュメッセージ（無ければ何も出さない）
#[component]
pub async fn flash_banner(flash: Option<Flash>) -> Result<impl View> {
    Ok(view! {
        if let Some(flash) = flash {
            <p class=(class!("flash", flash.kind())) role="status">(flash.message())</p>
        }
    })
}

/// 入力欄の下に出すエラー文（無ければ何も出さない）
#[component]
pub async fn field_error(message: Option<String>) -> Result<impl View> {
    Ok(view! {
        if let Some(message) = message {
            <p class="field-error">(message)</p>
        }
    })
}

// ---------------------------------------------------------------------------
// ノートのフォーム（作成と編集で共通）
// ---------------------------------------------------------------------------

/// フォームに表示する値とエラー
#[derive(Debug, Clone, Default)]
pub struct NoteFormState {
    pub title: String,
    pub body: String,
    /// 編集のときだけ使う（楽観的ロックの version）
    pub version: Option<i64>,
    pub title_error: Option<String>,
    pub body_error: Option<String>,
    /// フォームの上に出すお知らせ（409 のときなど）
    pub notice: Option<String>,
}

impl NoteFormState {
    /// 入力された値をそのまま入れた状態（エラーで再表示するとき、入力を消さないため）
    pub fn from_input(title: &str, body: &str) -> NoteFormState {
        NoteFormState {
            title: title.to_string(),
            body: body.to_string(),
            ..NoteFormState::default()
        }
    }

    /// 検査で見つかった問題を、項目ごとの文章にして入れる
    pub fn with_errors(mut self, errors: &NoteInputErrors) -> NoteFormState {
        self.title_error = errors.title.map(|e| e.to_string());
        self.body_error = errors.body.map(|e| e.to_string());
        self
    }
}

/// ノートの入力フォーム
#[component]
pub async fn note_form(
    action: String,
    submit_label: &str,
    form: NoteFormState,
) -> Result<impl View> {
    Ok(view! {
        <form method="post" action=(action) class="note-form">
            if let Some(notice) = form.notice {
                <p class="form-notice">(notice)</p>
            }
            if let Some(version) = form.version {
                <input type="hidden" name="version" value=(version)>
            }
            <label for="title">
                "タイトル（"
                (MAX_TITLE_CHARS)
                "文字まで）"
            </label>
            <input
                id="title"
                name="title"
                value=(form.title)
                required=""
                maxlength=(MAX_TITLE_CHARS)
            >
            field_error(message: form.title_error)
            <label for="body">
                "本文（"
                (MAX_BODY_CHARS)
                "文字まで）"
            </label>
            <textarea id="body" name="body" rows="6">(form.body)</textarea>
            field_error(message: form.body_error)
            <button type="submit">(submit_label)</button>
        </form>
    })
}

// ---------------------------------------------------------------------------
// ページ送り
// ---------------------------------------------------------------------------

/// 一覧の「前へ・次へ」。検索キーワードはそのまま引き継ぐ
#[component]
pub async fn pagination(page: NotePage, query: NoteQuery) -> Result<impl View> {
    let link = |page_number: u32| {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(keyword) = &query.keyword {
            params.push(("q", keyword.clone()));
        }
        params.push(("page", page_number.to_string()));
        href!(crate::routes::notes::index).query(params)
    };
    let prev = page.has_prev().then(|| link(page.page - 1));
    let next = page.has_next().then(|| link(page.page + 1));

    Ok(view! {
        <nav class="pagination">
            if let Some(prev) = prev {
                <a href=(prev)>"← 前へ"</a>
            }
            <span>
                (page.page)
                " / "
                (page.total_pages())
                " ページ（全"
                (page.total_count)
                "件）"
            </span>
            if let Some(next) = next {
                <a href=(next)>"次へ →"</a>
            }
        </nav>
    })
}

// ---------------------------------------------------------------------------
// エラー画面
// ---------------------------------------------------------------------------

/// レイアウトの error_boundary から呼ばれ、エラーをステータスコード付きの画面にする。
///
/// - AppError        … status_code() と public_message() を使う
/// - RepoError       … AppError に直したときと同じ扱い（多くは 500）
/// - NotFoundError   … Topcoat の 404（URL が無い・ID が数字でない など）
/// - BadRequestError … Topcoat の 400（フォームの形が違う など）
/// - それ以外（リダイレクトなど）はそのまま投げ直し、Topcoat に任せる
pub fn error_page(cx: &Cx, error: Error) -> Result<impl View + '_> {
    let (status, message) = if let Some(app_error) = error.downcast_ref::<AppError>() {
        log_if_internal(app_error.status_code(), &error);
        (app_error.status_code(), app_error.public_message())
    } else if let Some(repo_error) = error.downcast_ref::<RepoError>() {
        // AppError に直したときと同じ分類で答える（From<RepoError> for AppError と同じ表）
        let same_as = match repo_error {
            RepoError::NotFound => AppError::NotFound,
            RepoError::VersionMismatch => AppError::Conflict,
            _ => AppError::internal("database"),
        };
        log_if_internal(same_as.status_code(), &error);
        (same_as.status_code(), same_as.public_message())
    } else if error.downcast_ref::<NotFoundError>().is_some() {
        (StatusCode::NOT_FOUND, AppError::NotFound.public_message())
    } else if error.downcast_ref::<BadRequestError>().is_some() {
        (
            StatusCode::BAD_REQUEST,
            "送信された内容を読み取れませんでした".to_string(),
        )
    } else {
        // 303 などのリダイレクトや、Topcoat が自分で返事を作るエラー
        return Err(error);
    };

    // 普通の関数の中で view! を使うときは、先頭に `cx =>` を書いて文脈を渡す
    Ok(view! {
        cx =>
        (status)
        <section class="error-page">
            <h1>
                (status.as_u16())
                " "
                (status.canonical_reason().unwrap_or("Error"))
            </h1>
            <p>(message)</p>
            <p>
                <a href=(href!(crate::routes::home))>"トップページへ戻る"</a>
            </p>
        </section>
    })
}

/// 500 のときだけ、原因をサーバーのログ（標準エラー出力）に書く
fn log_if_internal(status: StatusCode, error: &Error) {
    if status == StatusCode::INTERNAL_SERVER_ERROR {
        let causes: Vec<String> = error.chain().map(|e| e.to_string()).collect();
        eprintln!("[error] {}", causes.join(" <- "));
    }
}
