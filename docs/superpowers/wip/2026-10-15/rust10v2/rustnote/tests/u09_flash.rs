//! 第10巻 UNIT 09: 8種類のお知らせ（フラッシュ）を、予約する操作から、表示する画面まで、順にたどる。
//! ブラウザの代わりに、クッキー（セッションと flash）を覚える小さな入れ物を使う。
//! 実行: cargo test --test u09_flash -- --nocapture

mod common;

use common::{TestApp, body_text, encode, test_app};
use topcoat::router::response::Response;

/// ブラウザのクッキーの入れ物の代わり（名前 → 値）
#[derive(Default)]
struct Jar {
    cookies: Vec<(String, String)>,
}

impl Jar {
    /// Cookie ヘッダーの値（何も無ければ None）
    fn header(&self) -> Option<String> {
        if self.cookies.is_empty() {
            return None;
        }
        let pairs: Vec<String> = self
            .cookies
            .iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect();
        Some(pairs.join("; "))
    }

    /// Set-Cookie を読んで、覚える・消す。flash の Set-Cookie を、そのまま返す
    fn update(&mut self, response: &Response) -> Option<String> {
        let mut flash = None;
        for value in response.headers().get_all("set-cookie") {
            let text = value.to_str().unwrap().to_string();
            let (pair, _) = text.split_once(';').unwrap_or((&text, ""));
            let (name, value) = pair.split_once('=').unwrap();
            self.cookies.retain(|(n, _)| n != name);
            if !text.contains("Max-Age=0") {
                self.cookies.push((name.to_string(), value.to_string()));
            }
            if name == "flash" {
                flash = Some(text);
            }
        }
        flash
    }
}

/// 画面のお知らせ（<p class="flash …" role="status">…</p>）。無ければ "（なし）"
fn banner(html: &str) -> String {
    html.split("<p class=\"flash")
        .nth(1)
        .map(|t| format!("<p class=\"flash{}</p>", t.split("</p>").next().unwrap()))
        .unwrap_or_else(|| "（なし）".to_string())
}

async fn step(app: &TestApp, jar: &mut Jar, n: usize, label: &str, path: &str, form: Option<&str>) {
    let cookie = jar.header();
    let response = match form {
        Some(form) => app.post_form(path, form, cookie.as_deref()).await,
        None => app.get(path, cookie.as_deref()).await,
    };
    let method = if form.is_some() { "POST" } else { "GET" };
    let status = response.status();
    let to = response
        .headers()
        .get("location")
        .map(|v| v.to_str().unwrap().to_string())
        .unwrap_or_default();
    let set = jar
        .update(&response)
        .unwrap_or_else(|| "（なし）".to_string());
    println!("{n}. {label}: {method} {path} → {status} → {to}");
    println!("   予約: {set}");

    // 移動先を開く（ブラウザと同じように、覚えたクッキーを付ける）
    let response = app.get(&to, jar.header().as_deref()).await;
    let removed = jar
        .update(&response)
        .unwrap_or_else(|| "（なし）".to_string());
    let html = body_text(response).await;
    println!("   {to} の画面: {}", banner(&html));
    println!("   消す: {removed}");
    // もう一度開く（再読み込み）
    let html = body_text(app.get(&to, jar.header().as_deref()).await).await;
    println!("   再読み込み: {}", banner(&html));
}

#[tokio::test]
async fn eight_flashes() {
    let app = test_app().await;
    let mut jar = Jar::default();
    let password = encode("correct horse battery");
    let email = encode("alice@example.com");
    let signup = format!("email={email}&password={password}&password_confirm={password}");
    let login = format!("email={email}&password={password}");
    let create = format!("title={}&body=", encode("牛乳を買う"));
    let edit = format!("version=1&title={}&body=", encode("牛乳を2本買う"));

    step(&app, &mut jar, 1, "新規登録", "/signup", Some(&signup)).await;
    step(&app, &mut jar, 2, "作成", "/notes", Some(&create)).await;
    step(&app, &mut jar, 3, "保存", "/notes/1/edit", Some(&edit)).await;
    step(
        &app,
        &mut jar,
        4,
        "完了の切り替え",
        "/notes/1/toggle",
        Some(""),
    )
    .await;
    step(&app, &mut jar, 5, "削除", "/notes/1/delete", Some("")).await;
    step(&app, &mut jar, 6, "ログアウト", "/logout", Some("")).await;
    step(&app, &mut jar, 7, "ログインせずに一覧", "/notes", None).await;
    step(&app, &mut jar, 8, "ログイン", "/login", Some(&login)).await;
}
