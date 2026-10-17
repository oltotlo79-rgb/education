//! 受入条件（AC）を、そのままテストにしたもの。
//! テストの名前は「ac番号_条件の要約」にそろえ、受入条件の表と1対1で対応させる。
//! 実行: cargo test --test acceptance

mod common;

use common::{location, test_app};
use topcoat::router::StatusCode;

const PASSWORD: &str = "correct horse battery";

// AC-02
// 前提: ログインしていない
// 操作: ノートの詳細・編集を開く／完了切替・削除を送信する
// 結果: どれも 303 でログイン画面（/login）へ移動する
#[tokio::test]
async fn ac02_logged_out_user_is_sent_to_login() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let url = app.create_note(&alice, "alice のノート", "").await;

    for path in [url.clone(), format!("{url}/edit")] {
        let response = app.get(&path, None).await;
        assert_eq!(response.status(), StatusCode::SEE_OTHER, "GET {path}");
        assert_eq!(location(&response), "/login", "GET {path}");
    }
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
