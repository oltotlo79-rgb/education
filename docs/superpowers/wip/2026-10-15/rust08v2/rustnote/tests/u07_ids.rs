//! 第8巻 UNIT 07: ノートの ID は、全員で1つの連番。bob が自分のノートを作ったときの ID から、何が分かるか。
//! 実行: cargo test --test u07_ids -- --nocapture

mod common;

use common::test_app;

#[tokio::test]
async fn ids_are_shared() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let bob = app
        .sign_up("bob@example.com", "correct horse battery")
        .await;
    let first = app.create_note(&bob, "bob の1件目", "").await;
    println!("bob の1件目: {first}");
    for n in 1..=5 {
        app.create_note(&alice, &format!("alice の{n}件目"), "")
            .await;
    }
    println!("（この間に、alice が5件作った）");
    let second = app.create_note(&bob, "bob の2件目", "").await;
    println!("bob の2件目: {second}");
    for path in ["/notes/2", "/notes/6"] {
        let status = app.get(path, Some(&bob)).await.status();
        println!("bob GET {path} → {status}");
    }
}
