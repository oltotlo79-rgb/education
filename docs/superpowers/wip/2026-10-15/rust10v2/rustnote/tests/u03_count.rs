//! 第10巻 UNIT 03: bob が ID を 1 から順に試して、alice のノートの数を知ろうとする。
//! 実行: cargo test --test u03_count -- --nocapture

mod common;

use common::test_app;

const PASSWORD: &str = "correct horse battery";

#[tokio::test]
async fn counting_other_notes() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let bob = app.sign_up("bob@example.com", PASSWORD).await;
    for title in ["1件目", "2件目", "3件目"] {
        app.create_note(&alice, title, "").await;
    }
    println!("alice のノートは 3 件（id は 1・2・3）");
    for id in 1..=5 {
        let path = format!("/notes/{id}");
        let status = app.get(&path, Some(&bob)).await.status();
        println!("bob GET {path} → {status}");
    }
}
