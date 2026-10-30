//! 第11巻 UNIT 03 例3-2: 道具に #[track_caller] を付けると、失敗の場所が、呼んだ行になる。
//! 3つの URL のうち、どれか1つの期待を、わざと書きまちがえている。
//! 実行: cargo test --test e03_track

mod common;

use common::{location, test_app};
use topcoat::router::{StatusCode, response::Response};

#[track_caller]
fn assert_redirect(response: &Response, to: &str) {
    assert_eq!(response.status(), StatusCode::SEE_OTHER, "303 のはず");
    assert_eq!(location(response), to, "移動先");
}

#[tokio::test]
async fn guests_are_redirected() {
    let app = test_app().await;
    assert_redirect(&app.get("/notes", None).await, "/login");
    assert_redirect(&app.get("/notes/1", None).await, "/notes");
    assert_redirect(&app.get("/notes/1/edit", None).await, "/login");
}
