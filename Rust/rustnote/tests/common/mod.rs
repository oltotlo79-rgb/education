//! テストで共通に使う道具。
//!
//! - 一時フォルダに DB ファイルを作り、マイグレーションを適用する（テストごとに別の DB）
//! - ルーターを組み立て、`Router::handle` でリクエストを直接処理する（ポートを開かない）
//! - フォームの送信・クッキーの受け渡しを短く書くための関数

// テストファイルごとに使う関数が違うので、使っていない関数の警告を出さない
#![allow(dead_code)]

use rustnote::{
    db::{self, Db},
    routes,
    state::AppState,
    views::STYLESHEET,
};
use tempfile::TempDir;
use topcoat::{
    asset::{AssetConfig, Manifest, ManifestEntry},
    router::{
        Body, Method, Router, StatusCode, header, request::Request, response::Response, to_bytes,
    },
};

/// 一時フォルダの DB。TempDir が消えるとフォルダごと削除される
pub struct TestDb {
    pub db: Db,
    _dir: TempDir,
}

/// マイグレーション済みの空の DB を作る
pub async fn test_db() -> TestDb {
    let dir = tempfile::tempdir().expect("一時フォルダを作れません");
    let db = Db::open_local(dir.path().join("test.db"))
        .await
        .expect("DB を開けません");
    db::migrate(&db).await.expect("マイグレーションに失敗");
    TestDb { db, _dir: dir }
}

/// テスト用のアセット設定。
/// 本物は `topcoat asset bundle` が作る束を読むが、テストでは CSS の URL が決まれば十分なので、
/// STYLESHEET の ID と仮のファイル名だけを書いた目録（Manifest）を渡す。
pub fn test_assets() -> AssetConfig {
    let manifest = Manifest {
        version: 1,
        assets: vec![ManifestEntry {
            id: STYLESHEET.id(),
            file: "style.css".to_string(),
            hash: String::new(),
            content_type: "text/css".to_string(),
        }],
    };
    AssetConfig::hosted_at("/test-assets", manifest)
}

/// テスト用のアプリ（ルーターと DB）
pub struct TestApp {
    pub router: Router,
    pub db: TestDb,
}

pub async fn test_app() -> TestApp {
    let db = test_db().await;
    let router = routes::build_router(AppState::new(db.db.clone()), test_assets());
    TestApp { router, db }
}

impl TestApp {
    /// GET リクエスト（cookie があれば付ける）
    pub async fn get(&self, uri: &str, cookie: Option<&str>) -> Response {
        let mut builder = Request::builder().method(Method::GET).uri(uri);
        if let Some(cookie) = cookie {
            builder = builder.header(header::COOKIE, cookie);
        }
        self.router
            .handle(builder.body(Body::empty()).unwrap())
            .await
    }

    /// フォームの POST（application/x-www-form-urlencoded）
    pub async fn post_form(&self, uri: &str, form: &str, cookie: Option<&str>) -> Response {
        let mut builder = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
        if let Some(cookie) = cookie {
            builder = builder.header(header::COOKIE, cookie);
        }
        self.router
            .handle(builder.body(Body::from(form.to_string())).unwrap())
            .await
    }

    /// 新規登録して、ログイン済みのクッキー（"__Host-session=..."）を返す
    pub async fn sign_up(&self, email: &str, password: &str) -> String {
        let form = format!(
            "email={}&password={}&password_confirm={}",
            encode(email),
            encode(password),
            encode(password)
        );
        let response = self.post_form("/signup", &form, None).await;
        assert_eq!(response.status(), StatusCode::SEE_OTHER, "登録に失敗");
        session_cookie(&response).expect("セッションのクッキーがありません")
    }

    /// ノートを作成し、詳細ページの URL（/notes/1 など）を返す
    pub async fn create_note(&self, cookie: &str, title: &str, body: &str) -> String {
        let form = format!("title={}&body={}", encode(title), encode(body));
        let response = self.post_form("/notes", &form, Some(cookie)).await;
        assert_eq!(response.status(), StatusCode::SEE_OTHER, "作成に失敗");
        location(&response).to_string()
    }
}

/// レスポンスの本文を文字列で読む
pub async fn body_text(response: Response) -> String {
    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("本文を読めません");
    String::from_utf8(bytes.to_vec()).expect("UTF-8 ではありません")
}

/// リダイレクト先（Location ヘッダー）
pub fn location(response: &Response) -> &str {
    response
        .headers()
        .get(header::LOCATION)
        .expect("Location がありません")
        .to_str()
        .unwrap()
}

/// Set-Cookie からセッションのクッキー（"__Host-session=値"）を取り出す
pub fn session_cookie(response: &Response) -> Option<String> {
    response
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .find(|value| value.starts_with("__Host-session=") && !value.contains("Max-Age=0"))
        .and_then(|value| value.split(';').next())
        .map(str::to_string)
}

/// フォームの値を URL エンコードする（テストで使う文字だけを扱う簡易版）
pub fn encode(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                encoded.push(byte as char)
            }
            b' ' => encoded.push('+'),
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    encoded
}
