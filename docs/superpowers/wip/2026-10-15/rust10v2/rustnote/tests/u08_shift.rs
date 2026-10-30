//! 第10巻 UNIT 08: 1ページ目を見ている間にノートが増えたり減ったりすると、
//! OFFSET のページ送りで、2ページ目に何が起きるかを確かめる。最後に、cursor（続きの印）の形と比べる。
//! 実行: cargo test --test u08_shift -- --nocapture --test-threads=1

mod common;

use common::{TestApp, body_text, test_app};

const PASSWORD: &str = "correct horse battery";

/// 一覧の HTML から、ノートのタイトルを、上から順に読む
fn titles(html: &str) -> Vec<String> {
    html.split("<li class=\"note-item")
        .skip(1)
        .filter_map(|item| item.split("<a href=\"/notes/").nth(1))
        .filter_map(|t| t.split_once('>'))
        .map(|(_, rest)| rest.split('<').next().unwrap().to_string())
        .collect()
}

async fn page(app: &TestApp, cookie: &str, n: u32) -> Vec<String> {
    let html = body_text(app.get(&format!("/notes?page={n}"), Some(cookie)).await).await;
    titles(&html)
}

/// 1ページ目と2ページ目を比べて、重なった物と、どちらにも出なかった物を数える
fn compare(all: &[String], page1: &[String], page2: &[String]) {
    let twice: Vec<_> = page2.iter().filter(|t| page1.contains(t)).collect();
    let missing: Vec<_> = all
        .iter()
        .filter(|t| !page1.contains(t) && !page2.contains(t))
        .collect();
    println!("   2ページ目: {page2:?}");
    println!("   2回出た: {twice:?}　どちらにも出なかった: {missing:?}");
}

async fn setup() -> (TestApp, String) {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    for n in 1..=25 {
        app.create_note(&alice, &format!("ノート{n:02}"), "").await;
    }
    (app, alice)
}

/// 見ている間に、1件増えた
#[tokio::test]
async fn a_note_is_added() {
    let (app, alice) = setup().await;
    let page1 = page(&app, &alice, 1).await;
    println!("1. 1ページ目: {} 〜 {}", page1[0], page1[19]);
    app.create_note(&alice, "新しいノート", "").await;
    println!("2. 「新しいノート」を作った");
    let page2 = page(&app, &alice, 2).await;
    // 比べるのは、1ページ目を見たときにあった 25 件
    let all: Vec<String> = (1..=25).rev().map(|n| format!("ノート{n:02}")).collect();
    println!("3. 2ページ目を開く");
    compare(&all, &page1, &page2);
}

/// 見ている間に、1ページ目の1件が消えた
#[tokio::test]
async fn a_note_is_deleted() {
    let (app, alice) = setup().await;
    let page1 = page(&app, &alice, 1).await;
    println!("1. 1ページ目: {} 〜 {}", page1[0], page1[19]);
    // ノート20 は id=20。1ページ目に出ている
    app.post_form("/notes/20/delete", "", Some(&alice)).await;
    println!("2. 「ノート20」を削除した");
    let page2 = page(&app, &alice, 2).await;
    let all: Vec<String> = (1..=25)
        .rev()
        .filter(|n| *n != 20)
        .map(|n| format!("ノート{n:02}"))
        .collect();
    println!("3. 2ページ目を開く");
    compare(&all, &page1, &page2);
}

/// 同じことを、cursor（1ページ目の最後の行の updated_at と id）で続きを読む SQL で行う
#[tokio::test]
async fn cursor_instead_of_offset() {
    let (app, alice) = setup().await;
    let conn = app.db.db.connect().await.unwrap();
    let first = "SELECT title, updated_at, id FROM notes WHERE user_id = 1
                 ORDER BY updated_at DESC, id DESC LIMIT 20";
    let mut rows = conn.query(first, ()).await.unwrap();
    let (mut page1, mut last) = (Vec::new(), (String::new(), 0i64));
    while let Some(row) = rows.next().await.unwrap() {
        page1.push(row.get::<String>(0).unwrap());
        last = (row.get::<String>(1).unwrap(), row.get::<i64>(2).unwrap());
    }
    println!(
        "1. 1ページ目: {} 〜 {}（続きの印: updated_at={} id={}）",
        page1[0], page1[19], last.0, last.1
    );
    app.create_note(&alice, "新しいノート", "").await;
    println!("2. 「新しいノート」を作った");
    let next = "SELECT title FROM notes WHERE user_id = 1 AND (updated_at, id) < (?1, ?2)
                ORDER BY updated_at DESC, id DESC LIMIT 20";
    let mut rows = conn.query(next, (last.0.as_str(), last.1)).await.unwrap();
    let mut page2 = Vec::new();
    while let Some(row) = rows.next().await.unwrap() {
        page2.push(row.get::<String>(0).unwrap());
    }
    let all: Vec<String> = (1..=25).rev().map(|n| format!("ノート{n:02}")).collect();
    println!("3. 続きの印のあとを読む");
    compare(&all, &page1, &page2);
}
