//! 第11巻 UNIT 03: レスポンスを確かめる道具（ヘルパー）を作り、HTTP のテストを短く書く。
//! 道具に #[track_caller] を付けて、失敗したときに「テストのどの行か」が出るようにする。
//!
//! 実行: cargo test --test u03_helpers

mod common;

use common::{encode, location, test_app};
use topcoat::router::{StatusCode, header, response::Response};

const PASSWORD: &str = "correct horse battery";

/// 303 で `to` へ移動するレスポンスか
#[track_caller]
fn assert_redirect(response: &Response, to: &str) {
    assert_eq!(response.status(), StatusCode::SEE_OTHER, "303 のはず");
    assert_eq!(location(response), to, "移動先");
}

/// `name` のクッキーの属性（`HttpOnly`・`Path=/` など）を、並んだ順に返す。無ければ止まる
#[track_caller]
fn cookie_attributes(response: &Response, name: &str) -> Vec<String> {
    let prefix = format!("{name}=");
    let value = response
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .find(|value| value.starts_with(&prefix))
        .unwrap_or_else(|| panic!("{name} のクッキーがありません"));
    value
        .split(';')
        .skip(1)
        .map(|attribute| attribute.trim().to_string())
        .collect()
}

#[tokio::test]
async fn signup_sets_two_cookies_and_redirects() {
    let app = test_app().await;
    let form = format!(
        "email={}&password={}&password_confirm={}",
        encode("alice@example.com"),
        encode(PASSWORD),
        encode(PASSWORD)
    );
    let response = app.post_form("/signup", &form, None).await;

    assert_redirect(&response, "/notes");
    // セッション: JavaScript から読めず・HTTPS だけ・ほかのサイトからの POST では送られず・30 日
    assert_eq!(
        cookie_attributes(&response, "__Host-session"),
        [
            "HttpOnly",
            "SameSite=Lax",
            "Secure",
            "Path=/",
            "Max-Age=2592000"
        ]
    );
    // お知らせ: 5 分だけ
    assert_eq!(
        cookie_attributes(&response, "flash"),
        ["HttpOnly", "SameSite=Lax", "Path=/", "Max-Age=300"]
    );
}

#[tokio::test]
async fn pages_for_members_send_guests_to_login() {
    let app = test_app().await;
    for path in ["/notes", "/notes/1", "/notes/1/edit"] {
        let response = app.get(path, None).await;
        assert_redirect(&response, "/login");
    }
}

#[tokio::test]
async fn html_pages_say_utf8() {
    let app = test_app().await;
    let response = app.get("/login", None).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()[header::CONTENT_TYPE],
        "text/html; charset=utf-8"
    );
}
