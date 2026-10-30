//! 第10巻 UNIT 06: 「削除」を、ボタン以外のやり方で引き起こせないかを確かめる。
//! やり方ごとに alice のノートを1件ずつ作り、そのノートを消そうとする。
//! 実行: cargo test --test u06_guard -- --nocapture

mod common;

use common::test_app;
use topcoat::router::{Body, Method, header, request::Request};

/// 1つのやり方: (説明, メソッド, クッキー, 付けるヘッダー)
type Case<'a> = (&'a str, Method, &'a str, &'a [(&'a str, &'a str)]);

#[tokio::test]
async fn ways_to_trigger_delete() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let bob = app
        .sign_up("bob@example.com", "correct horse battery")
        .await;

    let cross_site_link = [
        ("sec-fetch-site", "cross-site"),
        ("sec-fetch-mode", "navigate"),
    ];
    let cases: [Case; 6] = [
        ("RustNote の中のリンクを開く", Method::GET, &alice, &[]),
        (
            "ほかのサイトのリンクを開く",
            Method::GET,
            &alice,
            &cross_site_link,
        ),
        (
            "ほかのサイトのフォーム（Origin）",
            Method::POST,
            &alice,
            &[("origin", "https://evil.example")],
        ),
        (
            "ほかのサイトのフォーム（Sec-Fetch-Site）",
            Method::POST,
            &alice,
            &[("sec-fetch-site", "cross-site")],
        ),
        ("bob のボタン", Method::POST, &bob, &[]),
        ("alice のボタン", Method::POST, &alice, &[]),
    ];
    for (n, (what, method, cookie, extra)) in cases.into_iter().enumerate() {
        let url = app.create_note(&alice, &format!("{}件目", n + 1), "").await;
        let mut builder = Request::builder()
            .method(method.clone())
            .uri(format!("{url}/delete"))
            .header(header::COOKIE, cookie);
        if method == Method::POST {
            builder = builder.header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
        }
        for (name, value) in extra {
            builder = builder.header(*name, *value);
        }
        let response = app
            .router
            .handle(builder.body(Body::empty()).unwrap())
            .await;
        let allow = response
            .headers()
            .get("allow")
            .map(|v| format!("（allow: {}）", v.to_str().unwrap()))
            .unwrap_or_default();
        let exists = app.get(&url, Some(&alice)).await.status().is_success();
        println!(
            "{}. {what}: {method} {url}/delete → {}{allow}／ノートは{}",
            n + 1,
            response.status(),
            if exists {
                "残っている"
            } else {
                "消えた"
            }
        );
    }
}
