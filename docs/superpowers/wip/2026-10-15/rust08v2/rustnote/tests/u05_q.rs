//! 第8巻 UNIT 05 練習問題1: PC とスマートフォンでログインしていて、PC でだけログアウトする。
//! 実行: cargo test --test u05_q -- --nocapture

mod common;

use common::{encode, session_cookie, test_app};

#[tokio::test]
async fn q1_logout_on_one_device() {
    let app = test_app().await;
    let pc = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let form = format!(
        "email={}&password={}",
        encode("alice@example.com"),
        encode("correct horse battery")
    );
    let phone = session_cookie(&app.post_form("/login", &form, None).await).unwrap();
    let conn = app.db.db.connect().await.unwrap();
    let count = || async {
        let mut rows = conn
            .query("SELECT COUNT(*) FROM sessions", ())
            .await
            .unwrap();
        rows.next().await.unwrap().unwrap().get::<i64>(0).unwrap()
    };
    println!(
        "問1 PC とスマートフォンでログイン（sessions: {} 行）",
        count().await
    );
    let status = app.post_form("/logout", "", Some(&pc)).await.status();
    println!(
        "問1 PC で POST /logout → {status}（sessions: {} 行）",
        count().await
    );
    for (name, cookie) in [("PC", &pc), ("スマートフォン", &phone)] {
        let status = app.get("/notes", Some(cookie)).await.status();
        println!("問1 {name} のクッキーで GET /notes → {status}");
    }
}
