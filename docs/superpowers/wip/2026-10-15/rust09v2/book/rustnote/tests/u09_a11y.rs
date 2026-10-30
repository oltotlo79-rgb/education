//! 第9巻 UNIT 09: どのページも、アクセシビリティの最低限の決まりを守っているかを、HTML から確かめる。
//! - <html lang="ja"> がある
//! - <h1> がちょうど1つ
//! - 入力欄（hidden 以外の input・textarea）には、<label for="id"> か aria-label がある
//! - ボタンには、文字がある
//!
//! 実行: cargo test --test u09_a11y -- --nocapture

mod common;

use common::{body_text, test_app};

/// タグの中の、ある属性の値（無ければ None）
fn attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let key = format!(" {name}=\"");
    let start = tag.find(&key)? + key.len();
    let end = tag[start..].find('"')? + start;
    Some(&tag[start..end])
}

/// HTML の中の、ある名前のタグ（< から > まで）を全部
fn tags<'a>(html: &'a str, name: &str) -> Vec<&'a str> {
    let open = format!("<{name}");
    let mut found = Vec::new();
    let mut rest = html;
    while let Some(at) = rest.find(&open) {
        let tail = &rest[at..];
        let end = tail.find('>').unwrap_or(tail.len() - 1);
        found.push(&tail[..=end]);
        rest = &tail[end + 1..];
    }
    found
}

/// 1ページを確かめて、見つかった問題を返す
fn check(html: &str) -> Vec<String> {
    let mut problems = Vec::new();
    if !html.contains("<html lang=\"ja\">") {
        problems.push("lang が無い".to_string());
    }
    let h1 = tags(html, "h1").len();
    if h1 != 1 {
        problems.push(format!("<h1> が {h1} 個"));
    }
    for tag in tags(html, "input")
        .into_iter()
        .chain(tags(html, "textarea"))
    {
        if matches!(attr(tag, "type"), Some("hidden" | "submit")) {
            continue;
        }
        let labeled = attr(tag, "aria-label").is_some()
            || attr(tag, "id").is_some_and(|id| html.contains(&format!("<label for=\"{id}\"")));
        if !labeled {
            let name = attr(tag, "name").unwrap_or("?");
            problems.push(format!("ラベルの無い入力欄（name={name}）"));
        }
    }
    for button in html.split("<button").skip(1) {
        let text = button.split("</button>").next().unwrap_or("");
        let inner = text.split_once('>').map(|(_, t)| t.trim()).unwrap_or("");
        if inner.is_empty() {
            problems.push("文字の無いボタン".to_string());
        }
    }
    problems
}

#[tokio::test]
async fn pages_follow_the_basic_rules() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "牛乳を買う", "").await;

    let pages = [
        ("/", None),
        ("/signup", None),
        ("/login", None),
        ("/notes", Some(alice.as_str())),
        ("/notes/1", Some(alice.as_str())),
        ("/notes/1/edit", Some(alice.as_str())),
    ];
    let mut total = 0;
    for (path, cookie) in pages {
        let html = body_text(app.get(path, cookie).await).await;
        let title = html
            .split("<title>")
            .nth(1)
            .and_then(|t| t.split("</title>").next())
            .unwrap_or("");
        let problems = check(&html);
        total += problems.len();
        let result = if problems.is_empty() {
            "ok".to_string()
        } else {
            problems.join("・")
        };
        println!("{path:<14} <title>{title}</title>  {result}");
    }
    println!("見つかった問題: {total} 個");
}
