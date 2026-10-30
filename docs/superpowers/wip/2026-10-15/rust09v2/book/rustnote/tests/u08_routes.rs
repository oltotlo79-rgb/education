//! 第9巻 UNIT 08: ルートの一覧（メソッド・URL・誰が・期待する返事）を表にし、本物のルーターで1行ずつ確かめる。
//! 実行: cargo test --test u08_routes -- --nocapture

mod common;

use common::test_app;
use topcoat::router::{Body, Method, header, request::Request};

/// 誰として送るか
#[derive(Clone, Copy)]
enum Who {
    /// ログインしていない
    Anon,
    /// alice（ノート 1 の持ち主）
    Alice,
    /// bob（ノート 1 の持ち主ではない）
    Bob,
}

/// 表の1行
struct Row {
    method: Method,
    path: &'static str,
    body: &'static str,
    who: Who,
    /// 期待する状態コード
    status: u16,
    /// 303 のときの移り先（の始まり）
    to: Option<&'static str>,
}

fn row(method: Method, path: &'static str, who: Who, status: u16) -> Row {
    Row {
        method,
        path,
        body: "",
        who,
        status,
        to: None,
    }
}

#[tokio::test]
async fn route_table() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let bob = app
        .sign_up("bob@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "alice のノート", "").await; // /notes/1

    let table = [
        Row {
            to: Some("/notes"),
            ..row(Method::GET, "/", Who::Alice, 303)
        },
        row(Method::GET, "/", Who::Anon, 200),
        row(Method::GET, "/health", Who::Anon, 200),
        row(Method::GET, "/signup", Who::Anon, 200),
        row(Method::GET, "/login", Who::Anon, 200),
        Row {
            body: "email=alice%40example.com&password=wrong-password",
            ..row(Method::POST, "/login", Who::Anon, 401)
        },
        Row {
            to: Some("/login"),
            ..row(Method::GET, "/notes", Who::Anon, 303)
        },
        row(Method::GET, "/notes", Who::Alice, 200),
        Row {
            body: "title=&body=",
            ..row(Method::POST, "/notes", Who::Alice, 400)
        },
        Row {
            body: "title=%E6%96%B0%E3%81%97%E3%81%84&body=",
            to: Some("/notes/"),
            ..row(Method::POST, "/notes", Who::Alice, 303)
        },
        row(Method::GET, "/notes/1", Who::Alice, 200),
        row(Method::GET, "/notes/1", Who::Bob, 404),
        row(Method::GET, "/notes/abc", Who::Alice, 404),
        row(Method::GET, "/notes/1/edit", Who::Alice, 200),
        Row {
            to: Some("/notes/1"),
            ..row(Method::POST, "/notes/1/toggle", Who::Alice, 303)
        },
        row(Method::POST, "/notes/1/toggle", Who::Bob, 404),
        row(Method::POST, "/notes/1/delete", Who::Bob, 404),
        row(Method::DELETE, "/notes/1", Who::Alice, 405),
        row(Method::GET, "/nothing", Who::Anon, 404),
        Row {
            to: Some("/login"),
            ..row(Method::POST, "/logout", Who::Bob, 303)
        },
    ];

    let mut failures = 0;
    for r in &table {
        let mut builder = Request::builder().method(r.method.clone()).uri(r.path);
        let cookie = match r.who {
            Who::Anon => None,
            Who::Alice => Some(alice.as_str()),
            Who::Bob => Some(bob.as_str()),
        };
        if let Some(cookie) = cookie {
            builder = builder.header(header::COOKIE, cookie);
        }
        if !r.body.is_empty() {
            builder = builder.header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
        }
        let response = app
            .router
            .handle(builder.body(Body::from(r.body)).unwrap())
            .await;
        let status = response.status().as_u16();
        // 移り先（無ければ空）
        let moved_to = response
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("")
            .to_string();
        let to_ok = match r.to {
            Some(prefix) => status == 303 && moved_to.starts_with(prefix),
            None => true,
        };
        let ok = status == r.status && to_ok;
        if !ok {
            failures += 1;
        }
        let who = match r.who {
            Who::Anon => "未ログイン",
            Who::Alice => "alice",
            Who::Bob => "bob",
        };
        println!(
            "{} {:<6} {:<18} {:<10} → {} {}",
            if ok { "ok" } else { "NG" },
            r.method.as_str(),
            r.path,
            who,
            status,
            moved_to
        );
    }
    assert_eq!(failures, 0, "表と違う行が {failures} 行あります");
}
