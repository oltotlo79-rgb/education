//! 第8巻 UNIT 07: alice のノート1件に、ノートの5つの入口から、alice・bob・ログインしていない人（guest）が
//! 近づいたときの返事を、表にして確かめる。最後に、alice のノートが変わっていないかを見る。
//! 実行: cargo test --test u07_matrix -- --nocapture

mod common;

use common::{TestApp, encode, test_app};

/// alice のノート（id=1）の、タイトル・完了・version
async fn note(app: &TestApp) -> String {
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query("SELECT title, done, version FROM notes WHERE id = 1", ())
        .await
        .unwrap();
    match rows.next().await.unwrap() {
        Some(row) => format!(
            "title={:?} done={} version={}",
            row.get::<String>(0).unwrap(),
            row.get::<i64>(1).unwrap(),
            row.get::<i64>(2).unwrap()
        ),
        None => "（消えた）".to_string(),
    }
}

#[tokio::test]
async fn authorization_matrix() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let bob = app
        .sign_up("bob@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "牛乳を買う", "").await;
    println!("はじめ: {}", note(&app).await);

    let edit = format!("version=1&title={}&body=", encode("書き換えた"));
    let entrances: [(&str, &str, Option<&str>); 5] = [
        ("GET ", "/notes/1", None),
        ("GET ", "/notes/1/edit", None),
        ("POST", "/notes/1/edit", Some(edit.as_str())),
        ("POST", "/notes/1/toggle", Some("")),
        ("POST", "/notes/1/delete", Some("")),
    ];
    println!("・{:<21} {:>8} {:>8}", "入口", "bob", "guest");
    for (method, path, form) in entrances {
        let mut cells = Vec::new();
        for cookie in [Some(bob.as_str()), None] {
            let response = match form {
                Some(form) => app.post_form(path, form, cookie).await,
                None => app.get(path, cookie).await,
            };
            cells.push(response.status().as_u16().to_string());
        }
        println!("・{method} {path:<17} {:>8} {:>8}", cells[0], cells[1]);
    }
    println!("bob と guest のあと: {}", note(&app).await);

    // 最後に、alice 本人なら、同じ入口が使えることを確かめる（GET の2つだけ）
    for path in ["/notes/1", "/notes/1/edit"] {
        let status = app.get(path, Some(&alice)).await.status().as_u16();
        println!("alice GET {path} → {status}");
    }
}
