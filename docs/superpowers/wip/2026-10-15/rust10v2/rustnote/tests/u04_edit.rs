//! 第10巻 UNIT 04: 編集のフォームを開き、ブラウザと同じように、hidden の version ごと送り返して保存する。
//! 実行: cargo test --test u04_edit -- --nocapture

mod common;

use common::{TestApp, body_text, encode, location, test_app};

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

#[tokio::test]
async fn edit_and_save() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "牛乳を買う", "1本").await;
    println!("1. 作った直後の DB: {}", note_row(&app).await);

    let html = body_text(app.get("/notes/1/edit", Some(&alice)).await).await;
    let (version, title, body) = form_values(&html);
    println!("2. 編集フォームの値: version={version:?} title={title:?} body={body:?}");

    // updated_at は秒の単位なので、1秒あけてから保存する
    tokio::time::sleep(std::time::Duration::from_millis(1100)).await;
    let form = edit_form(version.as_deref(), "牛乳を2本買う", &body);
    let response = app.post_form("/notes/1/edit", &form, Some(&alice)).await;
    let status = response.status();
    let to = response
        .headers()
        .get("location")
        .map(|_| location(&response).to_string())
        .unwrap_or_else(|| "（なし）".to_string());
    let text = body_text(response).await;
    println!("3. 保存の POST: {status}（Location: {to}）");
    if !status.is_redirection() {
        // エラーの画面の、見出しと文だけを表示する
        let section = text
            .split("<section class=\"error-page\">")
            .nth(1)
            .and_then(|t| t.split("</section>").next())
            .unwrap_or(text.as_str());
        println!("   画面: {section}");
    }
    println!("4. 保存したあとの DB: {}", note_row(&app).await);
}
