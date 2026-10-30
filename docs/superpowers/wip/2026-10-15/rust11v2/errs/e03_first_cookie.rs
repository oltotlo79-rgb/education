//! 第11巻 UNIT 03 エラー2: 1つ目の Set-Cookie が、セッションのクッキーだと思い込んだテスト。
//! 実行: cargo test --test e03_first_cookie

mod common;

use common::{encode, test_app};
use topcoat::router::header;

const PASSWORD: &str = "correct horse battery";

#[tokio::test]
async fn first_set_cookie_is_the_session() {
    let app = test_app().await;
    let form = format!(
        "email={}&password={}&password_confirm={}",
        encode("alice@example.com"),
        encode(PASSWORD),
        encode(PASSWORD)
    );
    let response = app.post_form("/signup", &form, None).await;

    // get は、同じ名前のヘッダーが2つあっても、1つ目しか返さない
    let first = response.headers()[header::SET_COOKIE].to_str().unwrap();
    assert!(
        first.starts_with("__Host-session="),
        "1つ目の Set-Cookie: {first}"
    );
}
