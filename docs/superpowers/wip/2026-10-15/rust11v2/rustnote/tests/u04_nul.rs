//! 第11巻 UNIT 04: property test が見つけた「題が NUL 文字だと 500」を、1つずつ確かめる。
//!
//! 実行: cargo test --test u04_nul -- --nocapture --test-threads=1

mod common;

use common::{encode, test_app};
use rustnote::domain::NoteTitle;

#[tokio::test]
async fn titles_with_nul() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let conn = app.db.db.connect().await.unwrap();

    for title in ["\0", "\0a", "a\0", "a\0b"] {
        // Rust の側: NoteTitle::parse の結果と、文字数
        let parsed = NoteTitle::parse(title).is_ok();
        let chars = title.chars().count();
        // SQLite の側: length() が数える文字数
        let mut rows = conn.query("SELECT length(?1)", [title]).await.unwrap();
        let sqlite: i64 = rows.next().await.unwrap().unwrap().get(0).unwrap();
        // HTTP: 作成のフォームで送る
        let form = format!("title={}&body=", encode(title));
        let status = app.post_form("/notes", &form, Some(&alice)).await.status();
        println!(
            "題 {title:?}: chars().count()={chars}・parse の合格={parsed}・SQLite の length()={sqlite} → {status}"
        );
    }

    // おまけ: UTF-8 として読めないバイト（%FF と、途中で切れた「あ」の %E3%81）を送ると
    let status = app
        .post_form("/notes", "title=%FFa%E3%81&body=", Some(&alice))
        .await
        .status();
    let mut rows = conn
        .query(
            "SELECT title, hex(title) FROM notes ORDER BY id DESC LIMIT 1",
            (),
        )
        .await
        .unwrap();
    let row = rows.next().await.unwrap().unwrap();
    let stored: String = row.get(0).unwrap();
    let hex: String = row.get(1).unwrap();
    println!("題 %FFa%E3%81 → {status}・DB には {stored:?}（16 進数で {hex}）");
}
