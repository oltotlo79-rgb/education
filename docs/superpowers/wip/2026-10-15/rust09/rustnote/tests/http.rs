//! HTTP のテスト: ルーターに `Router::handle` でリクエストを直接渡し、返事を確かめる。
//! ポートを開かないので速く、並列に実行しても衝突しない。
//! 実行: cargo test --test http

mod common;

use common::{body_text, encode, location, session_cookie, test_app};
use topcoat::router::{Body, Method, StatusCode, header, request::Request};

const PASSWORD: &str = "correct horse battery";

#[tokio::test]
async fn health_returns_ok_json() {
    let app = test_app().await;
    let response = app.get("/health", None).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(body_text(response).await, r#"{"status":"ok"}"#);
}

#[tokio::test]
async fn top_page_shows_links_when_logged_out() {
    let app = test_app().await;
    let response = app.get("/", None).await;
    assert_eq!(response.status(), StatusCode::OK);
    let html = body_text(response).await;
    assert!(html.contains("新規登録"));
    assert!(html.contains(r#"<link rel="stylesheet" href="/test-assets/style.css">"#));
}

#[tokio::test]
async fn notes_require_login() {
    let app = test_app().await;

    // 一覧も作成も、未ログインならログイン画面へ 303
    let response = app.get("/notes", None).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&response), "/login");

    let response = app.post_form("/notes", "title=x&body=y", None).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&response), "/login");

    // ログイン画面には「ログインしてください」が出る（フラッシュのクッキーを渡す）
    let flash = response
        .headers()
        .get(header::SET_COOKIE)
        .unwrap()
        .to_str()
        .unwrap()
        .split(';')
        .next()
        .unwrap()
        .to_string();
    let html = body_text(app.get("/login", Some(&flash)).await).await;
    assert!(html.contains("ログインしてください"));
}

#[tokio::test]
async fn signup_validates_input_and_rejects_duplicates() {
    let app = test_app().await;

    // 形式の違うメール・短いパスワード → 400 でフォームを再表示
    let response = app
        .post_form(
            "/signup",
            "email=not-an-email&password=short&password_confirm=short",
            None,
        )
        .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let html = body_text(response).await;
    assert!(html.contains("メールアドレスに @ がありません"));
    assert!(html.contains("パスワードは8文字以上にしてください"));

    // 確認用パスワードの不一致 → 400
    let form = format!(
        "email=a%40example.com&password={}&password_confirm=different1",
        encode(PASSWORD)
    );
    let response = app.post_form("/signup", &form, None).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // 登録済みのメール → 409
    app.sign_up("alice@example.com", PASSWORD).await;
    let form = format!(
        "email=ALICE%40example.com&password={0}&password_confirm={0}",
        encode(PASSWORD)
    );
    let response = app.post_form("/signup", &form, None).await;
    assert_eq!(response.status(), StatusCode::CONFLICT);
    assert!(
        body_text(response)
            .await
            .contains("そのメールアドレスは既に登録されています")
    );
}

#[tokio::test]
async fn login_and_logout() {
    let app = test_app().await;
    app.sign_up("alice@example.com", PASSWORD).await;

    // パスワード違い → 401
    let response = app
        .post_form(
            "/login",
            "email=alice%40example.com&password=wrong-password",
            None,
        )
        .await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert!(
        body_text(response)
            .await
            .contains("メールアドレスまたはパスワードが違います")
    );

    // 登録されていないメール → 同じく 401（区別しない）
    let response = app
        .post_form(
            "/login",
            "email=nobody%40example.com&password=wrong-password",
            None,
        )
        .await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // 正しい組み合わせ → 303 で /notes へ、セッションのクッキーが発行される
    let form = format!("email=alice%40example.com&password={}", encode(PASSWORD));
    let response = app.post_form("/login", &form, None).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&response), "/notes");
    let cookie = session_cookie(&response).unwrap();
    assert_eq!(
        app.get("/notes", Some(&cookie)).await.status(),
        StatusCode::OK
    );

    // ログアウト → 303 で /login へ。古いクッキーはもう使えない
    let response = app.post_form("/logout", "", Some(&cookie)).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&response), "/login");
    let response = app.get("/notes", Some(&cookie)).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
}

#[tokio::test]
async fn create_note_follows_prg_and_escapes_html() {
    let app = test_app().await;
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;

    // 作成 → 303 で /notes/{id} へ（Post/Redirect/Get）
    let form = format!(
        "title={}&body={}",
        encode("<b>買い物</b>"),
        encode("<script>alert(1)</script>")
    );
    let response = app.post_form("/notes", &form, Some(&cookie)).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let url = location(&response).to_string();
    assert_eq!(url, "/notes/1");

    // 詳細ページ: 入力した HTML はエスケープされて表示される
    let html = body_text(app.get(&url, Some(&cookie)).await).await;
    assert!(html.contains("&lt;b&gt;買い物&lt;/b&gt;"));
    assert!(html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
    assert!(!html.contains("<script>alert(1)</script>"));
}

#[tokio::test]
async fn empty_title_is_400() {
    let app = test_app().await;
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;

    let response = app
        .post_form("/notes", "title=+++&body=hello", Some(&cookie))
        .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let html = body_text(response).await;
    assert!(html.contains("タイトルを入力してください"));
    // 入力した本文は消えずに残っている
    assert!(html.contains("hello"));

    // 81 文字のタイトルも 400
    let form = format!("title={}&body=", encode(&"あ".repeat(81)));
    let response = app.post_form("/notes", &form, Some(&cookie)).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert!(body_text(response).await.contains("現在81文字"));
}

#[tokio::test]
async fn other_users_notes_are_404() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let bob = app.sign_up("bob@example.com", PASSWORD).await;
    let url = app.create_note(&alice, "alice の秘密", "").await;

    // bob からは、見る・編集・切り替え・削除のどれも 404
    assert_eq!(
        app.get(&url, Some(&bob)).await.status(),
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        app.get(&format!("{url}/edit"), Some(&bob)).await.status(),
        StatusCode::NOT_FOUND
    );
    let response = app
        .post_form(
            &format!("{url}/edit"),
            "title=x&body=&version=1",
            Some(&bob),
        )
        .await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let response = app
        .post_form(&format!("{url}/toggle"), "", Some(&bob))
        .await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let response = app
        .post_form(&format!("{url}/delete"), "", Some(&bob))
        .await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // alice のノートは無事
    let html = body_text(app.get(&url, Some(&alice)).await).await;
    assert!(html.contains("alice の秘密"));
}

#[tokio::test]
async fn invalid_ids_and_unknown_urls_are_404() {
    let app = test_app().await;
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;

    for url in ["/notes/abc", "/notes/0", "/notes/-1", "/notes/999", "/nope"] {
        let response = app.get(url, Some(&cookie)).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND, "{url}");
        assert!(body_text(response).await.contains("ページが見つかりません"));
    }
}

#[tokio::test]
async fn edit_with_stale_version_is_409() {
    let app = test_app().await;
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;
    let url = app.create_note(&cookie, "初版", "").await;
    let edit_url = format!("{url}/edit");

    // 編集フォームには今の version（1）が埋め込まれている
    let html = body_text(app.get(&edit_url, Some(&cookie)).await).await;
    assert!(html.contains(r#"name="version" value="1""#));

    // version 1 で保存 → 303。version は 2 になる
    let response = app
        .post_form(&edit_url, "title=第2版&body=&version=1", Some(&cookie))
        .await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&response), url);

    // 古い version 1 のまま保存 → 409。最新の version 2 でフォームを出し直す
    let response = app
        .post_form(&edit_url, "title=古い画面&body=&version=1", Some(&cookie))
        .await;
    assert_eq!(response.status(), StatusCode::CONFLICT);
    let html = body_text(response).await;
    assert!(html.contains("ほかの画面で先に更新されました"));
    assert!(html.contains(r#"name="version" value="2""#));

    // 空タイトルで保存 → 400
    let response = app
        .post_form(&edit_url, "title=&body=&version=2", Some(&cookie))
        .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // 中身は第2版のまま
    let html = body_text(app.get(&url, Some(&cookie)).await).await;
    assert!(html.contains("第2版"));
}

#[tokio::test]
async fn toggle_and_delete_redirect() {
    let app = test_app().await;
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;
    let url = app.create_note(&cookie, "宿題", "").await;

    // 完了に切り替え → 303 で詳細へ。詳細には「完了」のバッジが出る
    let response = app
        .post_form(&format!("{url}/toggle"), "", Some(&cookie))
        .await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&response), url);
    let html = body_text(app.get(&url, Some(&cookie)).await).await;
    assert!(html.contains(r#"<span class="badge">完了</span>"#));

    // 削除 → 303 で一覧へ。もう見えない
    let response = app
        .post_form(&format!("{url}/delete"), "", Some(&cookie))
        .await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&response), "/notes");
    assert_eq!(
        app.get(&url, Some(&cookie)).await.status(),
        StatusCode::NOT_FOUND
    );
}

#[tokio::test]
async fn list_searches_and_paginates() {
    let app = test_app().await;
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;
    for i in 1..=21 {
        app.create_note(&cookie, &format!("メモ{i:02}"), "").await;
    }
    app.create_note(&cookie, "Rust の本", "所有権").await;

    // 1ページ目: 20 件と「次へ」（一覧の項目は <li class="..."> で数える）
    let html = body_text(app.get("/notes", Some(&cookie)).await).await;
    assert_eq!(html.matches("<li class=").count(), 20);
    assert!(html.contains("1 / 2 ページ（全22件）"));
    assert!(html.contains("/notes?page=2"));

    // 2ページ目: 残りの 2 件
    let html = body_text(app.get("/notes?page=2", Some(&cookie)).await).await;
    assert_eq!(html.matches("<li class=").count(), 2);

    // キーワード検索（本文にも当たる）
    let html = body_text(app.get("/notes?q=所有権", Some(&cookie)).await).await;
    assert!(html.contains("Rust の本"));
    assert!(html.contains("全1件"));

    // page が数字でなければ 400
    let response = app.get("/notes?page=abc", Some(&cookie)).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn cross_site_post_is_403() {
    let app = test_app().await;
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;

    // ほかのサイトのフォームから送られた POST（ブラウザが Sec-Fetch-Site: cross-site を付ける）
    let request = Request::builder()
        .method(Method::POST)
        .uri("/notes")
        .header(header::COOKIE, &cookie)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header("sec-fetch-site", "cross-site")
        .body(Body::from("title=attack&body="))
        .unwrap();
    let response = app.router.handle(request).await;
    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    // 同じサイトからの POST は通る
    let request = Request::builder()
        .method(Method::POST)
        .uri("/notes")
        .header(header::COOKIE, &cookie)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header("sec-fetch-site", "same-origin")
        .body(Body::from("title=ok&body="))
        .unwrap();
    let response = app.router.handle(request).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
}

#[tokio::test]
async fn flash_message_is_shown_once() {
    let app = test_app().await;
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;

    let form = "title=flash&body=";
    let response = app.post_form("/notes", form, Some(&cookie)).await;
    let url = location(&response).to_string();
    // 作成のレスポンスには flash クッキーが付いている
    let flash = response
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .find(|v| v.starts_with("flash="))
        .unwrap()
        .split(';')
        .next()
        .unwrap()
        .to_string();
    assert_eq!(flash, "flash=note_created");

    // 次の画面で表示され、同時にクッキーを消す指示（Max-Age=0）が出る
    let both = format!("{cookie}; {flash}");
    let response = app.get(&url, Some(&both)).await;
    let removal = response
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .any(|v| v.starts_with("flash=") && v.contains("Max-Age=0"));
    assert!(removal);
    assert!(body_text(response).await.contains("ノートを作成しました"));
}
