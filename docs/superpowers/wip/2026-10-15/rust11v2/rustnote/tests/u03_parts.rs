//! 第11巻 UNIT 03: 1つのレスポンスの4つの部分（状態コード・ヘッダー・クッキー・本文）を見る。
//! 実行: cargo test --test u03_parts -- --nocapture

mod common;

use common::{body_text, encode, session_cookie, test_app};
use topcoat::router::response::Response;

const PASSWORD: &str = "correct horse battery";

/// 状態コードと、ヘッダーを全部表示する（セッションのトークンは、先頭の6文字だけ）
fn show(label: &str, response: &Response) {
    println!("--- {label}");
    println!("状態コード: {}", response.status());
    for (name, value) in response.headers() {
        let value = value.to_str().unwrap_or("（文字にできない）");
        let shown = match value.strip_prefix("__Host-session=") {
            Some(rest) => {
                let (token, attributes) = rest.split_once(';').unwrap_or((rest, ""));
                format!(
                    "__Host-session={}…（{} 文字）;{attributes}",
                    &token[..6],
                    token.len()
                )
            }
            None => value.to_string(),
        };
        println!("{name}: {shown}");
    }
}

#[tokio::test]
async fn four_parts_of_a_response() {
    let app = test_app().await;

    // 1. 登録のフォームを送る（成功すると、303 でノートの一覧へ）
    let form = format!(
        "email={}&password={}&password_confirm={}",
        encode("alice@example.com"),
        encode(PASSWORD),
        encode(PASSWORD)
    );
    let response = app.post_form("/signup", &form, None).await;
    show("POST /signup", &response);
    let cookie = session_cookie(&response).unwrap();

    // 2. ログインしたクッキーで、ノートの一覧を開く
    let response = app.get("/notes", Some(&cookie)).await;
    show("GET /notes", &response);
    let html = body_text(response).await;
    println!("本文: {} バイト", html.len());
    for tag in ["<title>", "<h1>"] {
        let start = html.find(tag).unwrap();
        let end = start + html[start..].find("</").unwrap();
        println!("本文の {tag}: {}", &html[start + tag.len()..end]);
    }
}
