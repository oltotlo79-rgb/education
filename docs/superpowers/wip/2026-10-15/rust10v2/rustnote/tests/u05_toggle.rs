//! 第10巻 UNIT 05: 完了の切り替え（POST /notes/{id}/toggle）を2回送り、画面と DB を確かめる。
//! 実行: cargo test --test u05_toggle -- --nocapture

mod common;

use common::{TestApp, body_text, location, test_app};

/// notes 表の1件目の done と version
async fn done_and_version(app: &TestApp) -> String {
    let conn = app.db.db.connect().await.expect("DB に接続できません");
    let mut rows = conn
        .query("SELECT done, version FROM notes WHERE id = 1", ())
        .await
        .expect("読めません");
    let row = rows
        .next()
        .await
        .expect("行を読めません")
        .expect("行が無い");
    format!(
        "done={} version={}",
        row.get::<i64>(0).unwrap(),
        row.get::<i64>(1).unwrap()
    )
}

#[tokio::test]
async fn toggle_twice() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "牛乳を買う", "").await;
    println!("作った直後の DB: {}", done_and_version(&app).await);

    for n in 1..=2 {
        let response = app.post_form("/notes/1/toggle", "", Some(&alice)).await;
        let status = response.status();
        if !status.is_redirection() {
            // 失敗したとき（500 など）は、移動先が無い
            println!("{n} 回目の POST: {status}");
            println!("   DB: {}", done_and_version(&app).await);
            continue;
        }
        let to = location(&response).to_string();
        let cookie = response
            .headers()
            .get_all("set-cookie")
            .iter()
            .map(|v| v.to_str().unwrap().split(';').next().unwrap().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("{n} 回目の POST: {status}（Location: {to}・{cookie}）");

        // ブラウザと同じように、受け取ったお知らせのクッキーも付けて、移動先を開く
        let cookies = format!("{alice}; {cookie}");
        let html = body_text(app.get(&to, Some(&cookies)).await).await;
        let badge = html.contains("<span class=\"badge\">完了</span>");
        let button = if html.contains("未完了に戻す") {
            "未完了に戻す"
        } else {
            "完了にする"
        };
        let flash = html.contains("完了の状態を切り替えました");
        println!("   詳細の画面: 完了の印={badge}・ボタン「{button}」・お知らせ={flash}");
        println!("   DB: {}", done_and_version(&app).await);
    }
}
