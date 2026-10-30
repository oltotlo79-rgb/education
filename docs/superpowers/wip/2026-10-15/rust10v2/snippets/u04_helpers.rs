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
        .query("SELECT title, body, version, updated_at FROM notes WHERE id = 1", ())
        .await
        .expect("読めません");
    let row = rows.next().await.expect("行を読めません").expect("行が無い");
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
