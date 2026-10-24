//! 一覧画面（GET /notes）の「状態」を1つずつ作り、画面に出る文を確かめる。
//! 実行: cargo test --test states -- --nocapture

mod common;

use common::{body_text, test_app};
use rustnote::domain::{NewNote, UserId};
use rustnote::repository::{LibsqlNoteRepository, NoteRepository};
use topcoat::router::response::Response;

const PASSWORD: &str = "correct horse battery";

// 画面の状態の名前・ステータスと、期待する文が見つかったかを表示する
async fn check(state: &str, response: Response, expected: &[&str]) {
    let status = response.status().as_u16();
    let html = body_text(response).await;
    println!("[{state}] {status}");
    for text in expected {
        let found = if html.contains(text) {
            "あり"
        } else {
            "なし"
        };
        println!("    {found}: {text}");
    }
}

#[tokio::test]
async fn list_page_states() {
    let app = test_app().await;
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;

    // 1) 空の状態: まだ1件もない
    let response = app.get("/notes", Some(&cookie)).await;
    check("空", response, &["まだノートがありません。"]).await;

    // 2) 入力エラーの状態: タイトルを空で送った（入力した本文は残る）
    let response = app
        .post_form("/notes", "title=&body=memo", Some(&cookie))
        .await;
    check(
        "入力エラー",
        response,
        &["タイトルを入力してください", ">memo</textarea>"],
    )
    .await;

    // 3) 件数が多い状態: 21 件あると、2 ページに分かれる
    // （画面を通さず、リポジトリで直接 21 件を入れる。最初に登録した alice の ID は 1）
    let notes = LibsqlNoteRepository::new(app.db.db.clone());
    let alice = UserId::new(1).unwrap();
    for i in 1..=21 {
        let note = NewNote::parse(&format!("メモ{i:02}"), "").unwrap();
        notes.insert(alice, &note).await.unwrap();
    }
    let response = app.get("/notes", Some(&cookie)).await;
    check(
        "2ページある",
        response,
        &["1 / 2 ページ（全21件）", "次へ →"],
    )
    .await;

    // 4) 検索して1件も当たらない状態（「空」とは違う文を出す）
    let response = app.get("/notes?q=zzz", Some(&cookie)).await;
    check("検索0件", response, &["条件に合うノートはありません。"]).await;
}
