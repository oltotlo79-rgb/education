//! 第10巻 UNIT 06: ノートを削除し、一覧・詳細・もう一度の削除が、どうなるかを確かめる。
//! 実行: cargo test --test u06_delete -- --nocapture

mod common;

use common::{TestApp, body_text, location, test_app};

/// notes 表の id を、小さい順に読む
async fn note_ids(app: &TestApp) -> Vec<i64> {
    let conn = app.db.db.connect().await.expect("DB に接続できません");
    let mut rows = conn
        .query("SELECT id FROM notes ORDER BY id", ())
        .await
        .expect("読めません");
    let mut ids = Vec::new();
    while let Some(row) = rows.next().await.expect("行を読めません") {
        ids.push(row.get(0).expect("id"));
    }
    ids
}

#[tokio::test]
async fn delete_a_note() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "牛乳を買う", "").await;
    app.create_note(&alice, "卵を買う", "").await;
    println!("1. 削除の前の id: {:?}", note_ids(&app).await);

    let response = app.post_form("/notes/1/delete", "", Some(&alice)).await;
    let status = response.status();
    if !status.is_redirection() {
        let allow = response
            .headers()
            .get("allow")
            .map(|v| format!("（allow: {}）", v.to_str().unwrap()))
            .unwrap_or_default();
        println!("2. POST /notes/1/delete → {status}{allow}");
        println!("   id: {:?}", note_ids(&app).await);
        return;
    }
    let to = location(&response).to_string();
    let flash = response
        .headers()
        .get_all("set-cookie")
        .iter()
        .map(|v| v.to_str().unwrap().split(';').next().unwrap().to_string())
        .find(|c| c.starts_with("flash="))
        .unwrap_or_default();
    println!("2. POST /notes/1/delete → {status}（Location: {to}・{flash}）");
    println!("   削除のあとの id: {:?}", note_ids(&app).await);

    let html = body_text(app.get(&to, Some(&format!("{alice}; {flash}"))).await).await;
    println!(
        "3. 一覧: お知らせ「ノートを削除しました」={}・「牛乳を買う」={}・「卵を買う」={}",
        html.contains("ノートを削除しました"),
        html.contains("牛乳を買う"),
        html.contains("卵を買う")
    );
    let status = app.get("/notes/1", Some(&alice)).await.status();
    println!("4. GET /notes/1 → {status}");
    let status = app
        .post_form("/notes/1/delete", "", Some(&alice))
        .await
        .status();
    println!("5. もう一度 POST /notes/1/delete → {status}");
}
