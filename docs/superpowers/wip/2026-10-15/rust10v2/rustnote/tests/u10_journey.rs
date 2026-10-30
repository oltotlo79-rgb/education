//! 第10巻 UNIT 10: 総合の試験。この巻の画面を、利用者が使う順番に1本の流れでたどり、
//! 段ごとに「こうなるはず」を assert で確かめる。どこかで約束が破れたら、その段の名前で止まる。
//! 実行: cargo test --test u10_journey -- --nocapture

mod common;

use common::{TestApp, body_text, encode, location, test_app};
use topcoat::router::StatusCode;

const PASSWORD: &str = "correct horse battery";

/// notes 表の件数
async fn count(app: &TestApp) -> i64 {
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn.query("SELECT COUNT(*) FROM notes", ()).await.unwrap();
    rows.next().await.unwrap().unwrap().get(0).unwrap()
}

/// 1つの段を表示する
fn ok(step: &str) {
    println!("✓ {step}");
}

#[tokio::test]
async fn whole_journey() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    ok("1. alice が新規登録できる");

    let html = body_text(app.get("/notes", Some(&alice)).await).await;
    assert!(
        html.contains("まだノートがありません"),
        "2. 最初の一覧は空のはず"
    );
    ok("2. 最初の一覧は空で、次にすることを伝える");

    let to = app.create_note(&alice, "<b>牛乳</b>を買う", "1本").await;
    assert_eq!(to, "/notes/1", "3. 作成したら詳細へ移るはず");
    ok("3. 作成すると 303 で詳細へ移る");

    let r = app.post_form("/notes", "title=&body=", Some(&alice)).await;
    assert_eq!(
        r.status(),
        StatusCode::BAD_REQUEST,
        "4. 空のタイトルは 400 のはず"
    );
    assert_eq!(count(&app).await, 1, "4. 400 のときは保存しないはず");
    ok("4. 空のタイトルは 400 で、保存しない");

    let html = body_text(app.get("/notes/1", Some(&alice)).await).await;
    assert!(
        html.contains("&lt;b&gt;牛乳&lt;/b&gt;を買う"),
        "5. タイトルはエスケープされるはず"
    );
    ok("5. 詳細では、タイトルの < > が文字として出る");

    let html = body_text(
        app.get(&format!("/notes?q={}", encode("牛乳")), Some(&alice))
            .await,
    )
    .await;
    assert!(html.contains("全1件"), "6. 牛乳で1件見つかるはず");
    let html = body_text(
        app.get(&format!("/notes?q={}", encode("%")), Some(&alice))
            .await,
    )
    .await;
    assert!(html.contains("全0件"), "6. % はただの文字として探すはず");
    ok("6. 検索は、% をただの文字として探す");

    let form = format!("version=1&title={}&body=", encode("牛乳を2本買う"));
    let r = app.post_form("/notes/1/edit", &form, Some(&alice)).await;
    assert_eq!(
        r.status(),
        StatusCode::SEE_OTHER,
        "7. 新しい version の保存は 303 のはず"
    );
    ok("7. 編集を保存すると 303");

    let r = app.post_form("/notes/1/edit", &form, Some(&alice)).await;
    assert_eq!(
        r.status(),
        StatusCode::CONFLICT,
        "8. 古い version の保存は 409 のはず"
    );
    ok("8. 古い version で保存すると 409");

    let r = app.post_form("/notes/1/toggle", "", Some(&alice)).await;
    assert_eq!(location(&r), "/notes/1", "9. 切り替えたら詳細へ戻るはず");
    let html = body_text(app.get("/notes/1", Some(&alice)).await).await;
    assert!(html.contains("未完了に戻す"), "9. 完了になっているはず");
    ok("9. 完了を切り替えると、ボタンが「未完了に戻す」になる");

    let bob = app.sign_up("bob@example.com", PASSWORD).await;
    let r = app.get("/notes/1", Some(&bob)).await;
    assert_eq!(
        r.status(),
        StatusCode::NOT_FOUND,
        "10. bob には alice のノートが 404 のはず"
    );
    let r = app.post_form("/notes/1/delete", "", Some(&bob)).await;
    assert_eq!(
        r.status(),
        StatusCode::NOT_FOUND,
        "10. bob は alice のノートを消せないはず"
    );
    ok("10. bob には、alice のノートが見えず、消せない");

    let r = app.post_form("/notes/1/delete", "", Some(&alice)).await;
    assert_eq!(location(&r), "/notes", "11. 削除したら一覧へ移るはず");
    assert_eq!(count(&app).await, 0, "11. 削除したら 0 件のはず");
    ok("11. alice が削除すると、一覧へ移り、0 件になる");

    let r = app.post_form("/logout", "", Some(&alice)).await;
    assert_eq!(
        location(&r),
        "/login",
        "12. ログアウトしたらログイン画面へ移るはず"
    );
    let r = app.get("/notes", Some(&alice)).await;
    assert_eq!(
        location(&r),
        "/login",
        "12. 古いクッキーでは一覧を開けないはず"
    );
    ok("12. ログアウトすると、古いクッキーでは一覧を開けない");
}
