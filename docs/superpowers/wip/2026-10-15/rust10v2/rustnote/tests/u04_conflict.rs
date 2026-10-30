//! 第10巻 UNIT 04: 2つのタブで同じノートの編集フォームを開き、順に保存する（楽観的ロック）。
//! 実行: cargo test --test u04_conflict -- --nocapture

mod common;

use common::{TestApp, body_text, encode, test_app};

/// 編集フォームの HTML から、送り返す値（version・title・body）を読む。
/// version の hidden が無ければ None（ブラウザも、無い項目は送らない）
fn form_values(html: &str) -> (Option<String>, String, String) {
    let attr = |tag: &str, name: &str| -> Option<String> {
        let key = format!("{name}=\"");
        let start = tag.find(&key)? + key.len();
        let end = tag[start..].find('"')? + start;
        Some(tag[start..end].to_string())
    };
    let version = html
        .split("<input")
        .find(|t| t.contains("name=\"version\""))
        .and_then(|t| attr(t, "value"));
    let title = html
        .split("<input")
        .find(|t| t.contains("name=\"title\""))
        .and_then(|t| attr(t, "value"))
        .unwrap_or_default();
    let body = html
        .split("<textarea")
        .nth(1)
        .and_then(|t| t.split_once('>'))
        .map(|(_, rest)| rest.split("</textarea>").next().unwrap_or("").to_string())
        .unwrap_or_default();
    (version, title, body)
}

/// フォームの送信の本文を作る（version が None なら、その項目を入れない）
fn edit_form(version: Option<&str>, title: &str, body: &str) -> String {
    let mut form = format!("title={}&body={}", encode(title), encode(body));
    if let Some(version) = version {
        form = format!("version={version}&{form}");
    }
    form
}

/// notes 表の1件目の、タイトル・本文・version・更新日時
async fn note_row(app: &TestApp) -> String {
    let conn = app.db.db.connect().await.expect("DB に接続できません");
    let mut rows = conn
        .query(
            "SELECT title, body, version, updated_at FROM notes WHERE id = 1",
            (),
        )
        .await
        .expect("読めません");
    let row = rows
        .next()
        .await
        .expect("行を読めません")
        .expect("行が無い");
    format!(
        "title={:?} body={:?} version={} updated_at={}",
        row.get::<String>(0).unwrap(),
        row.get::<String>(1).unwrap(),
        row.get::<i64>(2).unwrap(),
        row.get::<String>(3).unwrap()
    )
}

/// HTML から、class="form-notice" のお知らせの文を取り出す
fn notice(html: &str) -> Option<&str> {
    let start = html.find("<p class=\"form-notice\">")? + "<p class=\"form-notice\">".len();
    let end = html[start..].find("</p>")? + start;
    Some(&html[start..end])
}

#[tokio::test]
async fn two_tabs() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "牛乳を買う", "1本").await;

    // タブ A とタブ B で、同じ編集フォームを開く
    let tab_a = body_text(app.get("/notes/1/edit", Some(&alice)).await).await;
    let tab_b = body_text(app.get("/notes/1/edit", Some(&alice)).await).await;
    let (va, _, body_a) = form_values(&tab_a);
    let (vb, _, body_b) = form_values(&tab_b);
    println!("1. タブ A の version={va:?}、タブ B の version={vb:?}");

    // タブ A が先に保存する
    let form = edit_form(va.as_deref(), "牛乳を2本買う", &body_a);
    let status = app
        .post_form("/notes/1/edit", &form, Some(&alice))
        .await
        .status();
    println!("2. タブ A の保存: {status}");
    println!("   DB: {}", note_row(&app).await);

    // タブ B が、古い version のまま保存する
    let form = edit_form(vb.as_deref(), "牛乳と卵を買う", &body_b);
    let response = app.post_form("/notes/1/edit", &form, Some(&alice)).await;
    let status = response.status();
    let html = body_text(response).await;
    let (v_new, title_kept, _) = form_values(&html);
    println!("3. タブ B の保存: {status}");
    if status != topcoat::router::StatusCode::CONFLICT {
        println!("   DB: {}", note_row(&app).await);
        println!("（409 にならなかったので、ここで終わり）");
        return;
    }
    println!("   お知らせ: {}", notice(&html).unwrap_or("（なし）"));
    println!("   出し直したフォーム: version={v_new:?} title={title_kept:?}");
    println!("   DB: {}", note_row(&app).await);

    // タブ B が、出し直したフォームで、もう一度保存する
    let form = edit_form(v_new.as_deref(), &title_kept, &body_b);
    let status = app
        .post_form("/notes/1/edit", &form, Some(&alice))
        .await
        .status();
    println!("4. タブ B の2回目の保存: {status}");
    println!("   DB: {}", note_row(&app).await);
}
