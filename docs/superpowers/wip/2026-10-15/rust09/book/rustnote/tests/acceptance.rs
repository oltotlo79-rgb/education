//! 受入条件（AC）を、そのままテストにしたもの。
//! テストの名前は「ac番号_条件の要約」にそろえ、受入条件の表と1対1で対応させる。
//! 実行: cargo test --test acceptance

mod common;

use common::{location, test_app};
use topcoat::router::StatusCode;

const PASSWORD: &str = "correct horse battery";

// AC-02
// 前提: ログインしていない（alice のノートが1件ある）
// 操作: ノートの詳細・編集を開く／完了の切り替え・削除を送る
// 結果: どれも 303 でログイン画面（/login）へ移動する
#[tokio::test]
async fn ac02_logged_out_user_is_sent_to_login() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let url = app.create_note(&alice, "alice のノート", "").await;

    // 画面を開く（GET）
    for path in [url.clone(), format!("{url}/edit")] {
        let response = app.get(&path, None).await;
        assert_eq!(response.status(), StatusCode::SEE_OTHER, "GET {path}");
        assert_eq!(location(&response), "/login", "GET {path}");
    }
    // 操作を送る（POST）
    for path in [format!("{url}/toggle"), format!("{url}/delete")] {
        let response = app.post_form(&path, "", None).await;
        assert_eq!(response.status(), StatusCode::SEE_OTHER, "POST {path}");
        assert_eq!(location(&response), "/login", "POST {path}");
    }
}

// AC-08
// 前提: ログインしている
// 操作: ログイン画面・登録画面を開く
// 結果: 303 でノート一覧（/notes）へ移動する
#[tokio::test]
async fn ac08_logged_in_user_skips_login_and_signup() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;

    for path in ["/login", "/signup"] {
        let response = app.get(path, Some(&alice)).await;
        assert_eq!(response.status(), StatusCode::SEE_OTHER, "{path}");
        assert_eq!(location(&response), "/notes", "{path}");
    }
}

// AC-05
// 前提: alice と bob が登録済みで、alice のノートが1件ある
// 操作: bob が alice のノートの詳細を開く
// 結果: 404 Not Found になる（ノートが「ある」ことも知らせない）
#[tokio::test]
async fn ac05_other_users_note_is_not_found() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let bob = app.sign_up("bob@example.com", PASSWORD).await;
    let url = app.create_note(&alice, "alice の秘密", "").await;

    let response = app.get(&url, Some(&bob)).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// AC-10
// 前提: alice がログインしていて、ノートが1件ある
// 操作: ログアウトしたあと、ログアウト前のクッキーでノートの詳細を開く
// 結果: 303 でログイン画面（/login）へ移動する（古いクッキーはもう使えない）
#[tokio::test]
async fn ac10_old_cookie_cannot_open_note_after_logout() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let url = app.create_note(&alice, "買い物", "").await;

    let response = app.post_form("/logout", "", Some(&alice)).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);

    let response = app.get(&url, Some(&alice)).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&response), "/login");
}
