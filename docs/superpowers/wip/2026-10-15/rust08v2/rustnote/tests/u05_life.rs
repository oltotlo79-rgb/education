//! 第8巻 UNIT 05: セッションの一生（発行・使う・ログアウト）を、クッキーと sessions 表の両方で見る。
//! 実行: cargo test --test u05_life -- --nocapture

mod common;

use common::{TestApp, test_app};

/// sessions 表の行（トークンのハッシュの先頭・ユーザー・期限・期限までの秒）
async fn rows(app: &TestApp) -> Vec<String> {
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query(
            "SELECT substr(hex(token_hash), 1, 8), length(token_hash), user_id, expires_at,
                    expires_at - unixepoch(created_at)
               FROM sessions ORDER BY created_at",
            (),
        )
        .await
        .unwrap();
    let mut found = Vec::new();
    while let Some(row) = rows.next().await.unwrap() {
        found.push(format!(
            "token_hash={}…（{} バイト） user_id={} expires_at={}（作ってから {} 秒）",
            row.get::<String>(0).unwrap(),
            row.get::<i64>(1).unwrap(),
            row.get::<i64>(2).unwrap(),
            row.get::<i64>(3).unwrap(),
            row.get::<i64>(4).unwrap()
        ));
    }
    found
}

/// Set-Cookie の __Host-session の行（値は先頭6文字だけ）
fn set_cookie(response: &topcoat::router::response::Response) -> String {
    response
        .headers()
        .get_all("set-cookie")
        .iter()
        .map(|v| v.to_str().unwrap().to_string())
        .find(|v| v.starts_with("__Host-session="))
        .map(|v| {
            let (pair, rest) = v.split_once(';').unwrap();
            let value = pair.trim_start_matches("__Host-session=");
            let shown = if value.is_empty() {
                String::new()
            } else {
                format!("{}…", &value[..6])
            };
            format!("__Host-session={shown};{rest}")
        })
        .unwrap_or_else(|| "（なし）".to_string())
}

#[tokio::test]
async fn session_life() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    println!("1. 登録した（ログインもした）");
    for row in rows(&app).await {
        println!("   {row}");
    }

    // 少し待ってから使っても、期限は延びない
    tokio::time::sleep(std::time::Duration::from_millis(1100)).await;
    let response = app.get("/notes", Some(&alice)).await;
    println!(
        "2. 1秒後に GET /notes → {}（Set-Cookie: {}）",
        response.status(),
        set_cookie(&response)
    );
    for row in rows(&app).await {
        println!("   {row}");
    }

    let response = app.post_form("/logout", "", Some(&alice)).await;
    println!("3. POST /logout → {}", response.status());
    println!("   Set-Cookie: {}", set_cookie(&response));
    println!("   sessions の行: {}", rows(&app).await.len());

    let response = app.get("/notes", Some(&alice)).await;
    println!("4. 前のクッキーのまま GET /notes → {}", response.status());
}
