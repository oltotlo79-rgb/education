//! 第10巻 UNIT 05 練習問題。
//! 実行: cargo test --test u05_q -- --nocapture --test-threads=1

mod common;

use common::{body_text, encode, test_app};

const PASSWORD: &str = "correct horse battery";

/// 問1: 編集のフォームを開いたあとで完了を切り替え、それから編集を保存する
#[tokio::test]
async fn q1_edit_after_toggle() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    app.create_note(&alice, "牛乳を買う", "").await;
    let html = body_text(app.get("/notes/1/edit", Some(&alice)).await).await;
    let version = html
        .split("name=\"version\" value=\"")
        .nth(1)
        .and_then(|t| t.split('"').next())
        .unwrap()
        .to_string();
    println!("問1 編集のフォームを開く（version={version}）");
    let status = app
        .post_form("/notes/1/toggle", "", Some(&alice))
        .await
        .status();
    println!("問1 完了を切り替える → {status}");
    let form = format!("version={version}&title={}&body=", encode("牛乳を2本買う"));
    let status = app
        .post_form("/notes/1/edit", &form, Some(&alice))
        .await
        .status();
    println!("問1 開いておいたフォームで保存 → {status}");
}

/// 問2: 「反転する」SQL と「値を決めて入れる」SQL を、それぞれ2回ずつ実行する
#[tokio::test]
async fn q2_toggle_or_set() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    app.create_note(&alice, "反転する", "").await;
    app.create_note(&alice, "値を決める", "").await;
    let conn = app.db.db.connect().await.unwrap();
    for (id, sql) in [
        (1, "UPDATE notes SET done = 1 - done WHERE id = ?1"),
        (2, "UPDATE notes SET done = 1 WHERE id = ?1"),
    ] {
        let mut seen = Vec::new();
        for _ in 0..2 {
            conn.execute(sql, [id]).await.unwrap();
            let mut rows = conn
                .query("SELECT done FROM notes WHERE id = ?1", [id])
                .await
                .unwrap();
            seen.push(rows.next().await.unwrap().unwrap().get::<i64>(0).unwrap());
        }
        println!(
            "問2 {sql}\n問2   → 1回目のあと done={}、2回目のあと done={}",
            seen[0], seen[1]
        );
    }
}

/// 問3: 3件のうち、いちばん古いノートの完了を切り替えると、一覧の順番はどうなるか
#[tokio::test]
async fn q3_list_order_after_toggle() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    for title in ["1件目", "2件目", "3件目"] {
        app.create_note(&alice, title, "").await;
    }
    // 一覧の1行（<li class="note-item …">）ごとに、タイトルと、完了の印があるかを読む
    let titles = |html: &str| -> Vec<String> {
        html.split("<li class=\"note-item")
            .skip(1)
            .map(|item| {
                let item = item.split("</li>").next().unwrap();
                let title = item
                    .split("<a href=\"/notes/")
                    .nth(1)
                    .and_then(|t| t.split_once('>'))
                    .map(|(_, rest)| rest.split('<').next().unwrap())
                    .unwrap();
                if item.contains("class=\"badge\"") {
                    format!("{title}（完了）")
                } else {
                    title.to_string()
                }
            })
            .collect()
    };
    let html = body_text(app.get("/notes", Some(&alice)).await).await;
    println!("問3 切り替える前の一覧: {:?}", titles(&html));
    tokio::time::sleep(std::time::Duration::from_millis(1100)).await;
    app.post_form("/notes/1/toggle", "", Some(&alice)).await;
    let html = body_text(app.get("/notes", Some(&alice)).await).await;
    println!("問3 1件目を切り替えたあと: {:?}", titles(&html));
}
