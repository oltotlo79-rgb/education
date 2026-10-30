//! 画面を順番に開き、ステータスコードと見出し（h1）を表示する。
//! 実行: cargo test --test screens -- --nocapture

mod common;

use common::{body_text, test_app};
use topcoat::router::response::Response;

const PASSWORD: &str = "correct horse battery";

// HTML から最初の <h1>…</h1> の中身を取り出す
fn h1(html: &str) -> String {
    let start = html.find("<h1>").map(|i| i + "<h1>".len());
    let end = html.find("</h1>");
    match (start, end) {
        (Some(start), Some(end)) if start <= end => html[start..end].to_string(),
        _ => "（h1 なし）".to_string(),
    }
}

// 1 画面分を「[画面の名前] 送ったリクエスト -> ステータス 見出し」の形で表示する
async fn show(screen: &str, request: &str, response: Response) -> String {
    let status = response.status().as_u16();
    let html = body_text(response).await;
    println!("[{screen}] {request} -> {status} 見出し「{}」", h1(&html));
    html
}

#[tokio::test]
async fn walk_all_screens() {
    let app = test_app().await;

    // ログインしていない人が見る画面
    show("トップ", "GET /", app.get("/", None).await).await;
    show("登録", "GET /signup", app.get("/signup", None).await).await;
    show("ログイン", "GET /login", app.get("/login", None).await).await;

    // 登録すると、ログインした状態になる
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;
    let html = show(
        "一覧（空）",
        "GET /notes",
        app.get("/notes", Some(&cookie)).await,
    )
    .await;
    assert!(html.contains("まだノートがありません"));

    // タイトルを空にして作成を送ると、一覧の画面が 400 で出直す
    let response = app
        .post_form("/notes", "title=&body=memo", Some(&cookie))
        .await;
    show("作成（失敗）", "POST /notes", response).await;

    let url = app.create_note(&cookie, "牛乳を買う", "2本").await;
    show("詳細", "GET /notes/1", app.get(&url, Some(&cookie)).await).await;
    let edit = format!("{url}/edit");
    show(
        "編集",
        "GET /notes/1/edit",
        app.get(&edit, Some(&cookie)).await,
    )
    .await;
    show("エラー", "GET /nope", app.get("/nope", Some(&cookie)).await).await;
}
