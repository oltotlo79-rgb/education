//! 第10巻 UNIT 08 練習問題。
//! 実行: cargo test --test u08_q -- --nocapture --test-threads=1

mod common;

use common::{body_text, test_app};

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

/// <p class="empty"> の文と、ページの行
fn empty_and_line(html: &str) -> (String, String) {
    let empty = html
        .split("<p class=\"empty\">")
        .nth(1)
        .and_then(|t| t.split("</p>").next())
        .unwrap_or("（なし）")
        .to_string();
    let line = html
        .split("<nav class=\"pagination\">")
        .nth(1)
        .and_then(|t| t.split("<span>").nth(1))
        .and_then(|t| t.split("</span>").next())
        .unwrap_or("（なし）")
        .to_string();
    (empty, line)
}

/// 問1・問2: ページの番号が、ページの数より大きいとき・とても大きいとき
#[tokio::test]
async fn q1_q2_page_out_of_range() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    for n in 1..=25 {
        app.create_note(&alice, &format!("ノート{n:02}"), "").await;
    }
    for url in [
        "/notes?page=3",
        "/notes?page=4294967295",
        "/notes?page=4294967296",
    ] {
        let response = app.get(url, Some(&alice)).await;
        let status = response.status();
        let html = body_text(response).await;
        let (empty, line) = empty_and_line(&html);
        println!(
            "問 GET {url} → {status}・{} 件・{line}・空のときの文: {empty}",
            titles(&html).len()
        );
    }
}

/// 問3: 続きの印を updated_at だけにすると、同じ秒のノートはどうなるか
#[tokio::test]
async fn q3_cursor_without_id() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    for n in 1..=25 {
        app.create_note(&alice, &format!("ノート{n:02}"), "").await;
    }
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query(
            "SELECT COUNT(DISTINCT updated_at) FROM notes WHERE user_id = 1",
            (),
        )
        .await
        .unwrap();
    let distinct: i64 = rows.next().await.unwrap().unwrap().get(0).unwrap();
    println!("問3 25 件の updated_at の種類: {distinct}");

    // 1ページ目の最後の行（ノート06）を続きの印にする
    let mut rows = conn
        .query(
            "SELECT updated_at, id FROM notes WHERE title = 'ノート06'",
            (),
        )
        .await
        .unwrap();
    let row = rows.next().await.unwrap().unwrap();
    let (updated_at, id): (String, i64) = (row.get(0).unwrap(), row.get(1).unwrap());
    for (name, sql) in [
        (
            "updated_at だけ",
            "SELECT title FROM notes WHERE user_id = 1 AND updated_at < ?1
             ORDER BY updated_at DESC, id DESC LIMIT 20",
        ),
        (
            "updated_at と id",
            "SELECT title FROM notes WHERE user_id = 1 AND (updated_at, id) < (?1, ?2)
             ORDER BY updated_at DESC, id DESC LIMIT 20",
        ),
    ] {
        let mut rows = if sql.contains("?2") {
            conn.query(sql, (updated_at.as_str(), id)).await.unwrap()
        } else {
            conn.query(sql, [updated_at.as_str()]).await.unwrap()
        };
        let mut found = Vec::new();
        while let Some(row) = rows.next().await.unwrap() {
            found.push(row.get::<String>(0).unwrap());
        }
        println!("問3 続きの印が {name} → 2ページ目: {found:?}");
    }
}
