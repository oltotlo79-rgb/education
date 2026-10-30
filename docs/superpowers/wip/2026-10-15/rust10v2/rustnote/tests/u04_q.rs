//! 第10巻 UNIT 04 練習問題。
//! 実行: cargo test --test u04_q -- --nocapture --test-threads=1

mod common;

use common::{TestApp, body_text, encode, test_app};

const PASSWORD: &str = "correct horse battery";

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

/// 問1: 保存したあと、ブラウザの「戻る」で古いフォームに戻り、もう一度保存する
#[tokio::test]
async fn q1_back_button() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    app.create_note(&alice, "牛乳を買う", "1本").await;
    let html = body_text(app.get("/notes/1/edit", Some(&alice)).await).await;
    let (version, _, body) = form_values(&html);
    for title in ["牛乳を2本買う", "牛乳を3本買う"] {
        let form = edit_form(version.as_deref(), title, &body);
        let status = app
            .post_form("/notes/1/edit", &form, Some(&alice))
            .await
            .status();
        println!(
            "問1 version={} で「{title}」を保存 → {status}",
            version.as_deref().unwrap_or("-")
        );
    }
    println!("問1 DB: {}", note_row(&app).await);
}

/// 問2: タブ A は本文だけ、タブ B はタイトルだけを直す
#[tokio::test]
async fn q2_body_only_change() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    app.create_note(&alice, "牛乳を買う", "1本").await;
    let tab_a = body_text(app.get("/notes/1/edit", Some(&alice)).await).await;
    let tab_b = body_text(app.get("/notes/1/edit", Some(&alice)).await).await;
    let (va, title_a, _) = form_values(&tab_a);
    let (vb, _, body_b) = form_values(&tab_b);

    let form = edit_form(va.as_deref(), &title_a, "低脂肪を2本");
    let status = app
        .post_form("/notes/1/edit", &form, Some(&alice))
        .await
        .status();
    println!("問2 タブ A（本文だけ直す）→ {status}");
    let form = edit_form(vb.as_deref(), "牛乳を買う（急ぎ）", &body_b);
    let response = app.post_form("/notes/1/edit", &form, Some(&alice)).await;
    let status = response.status();
    let html = body_text(response).await;
    let (v_new, title_kept, body_kept) = form_values(&html);
    println!("問2 タブ B（タイトルだけ直す）→ {status}");
    println!("問2 お知らせ: {}", notice(&html).unwrap_or("（なし）"));
    let form = edit_form(v_new.as_deref(), &title_kept, &body_kept);
    let status = app
        .post_form("/notes/1/edit", &form, Some(&alice))
        .await
        .status();
    println!("問2 タブ B がそのまま、もう一度保存 → {status}");
    println!("問2 DB: {}", note_row(&app).await);
}

/// 問3: 自分のものでないノートに、正しくない入力を送る
#[tokio::test]
async fn q3_not_found_before_validation() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let bob = app.sign_up("bob@example.com", PASSWORD).await;
    app.create_note(&alice, "牛乳を買う", "").await;
    let cases = [
        ("alice", alice.as_str(), "/notes/1/edit", ""),
        ("bob", bob.as_str(), "/notes/1/edit", ""),
        ("bob", bob.as_str(), "/notes/1/edit", "卵を買う"),
        ("alice", alice.as_str(), "/notes/999/edit", ""),
    ];
    for (who, cookie, path, title) in cases {
        let form = edit_form(Some("1"), title, "");
        let status = app.post_form(path, &form, Some(cookie)).await.status();
        println!("問3 {who:<5} POST {path:<16} title={title:?} → {status}");
    }
}
