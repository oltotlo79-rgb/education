//! 第10巻 UNIT 05: 同じノートへ「完了の切り替え」を2つ同時に送る、を 20 回くり返し、最後の done を数える。
//! 実行: cargo test --test u05_race -- --nocapture

mod common;

use common::test_app;

#[tokio::test]
async fn two_toggles_at_once() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let conn = app.db.db.connect().await.expect("DB に接続できません");
    let (mut back_to_0, mut stuck_at_1) = (0, 0);
    for round in 1..=20 {
        // 毎回、未完了（done=0）のノートを1件作ってから始める
        let url = app.create_note(&alice, &format!("{round}件目"), "").await;
        let toggle = format!("{url}/toggle");
        let (a, b) = tokio::join!(
            app.post_form(&toggle, "", Some(&alice)),
            app.post_form(&toggle, "", Some(&alice)),
        );
        let id: i64 = url.trim_start_matches("/notes/").parse().unwrap();
        let mut rows = conn
            .query("SELECT done, version FROM notes WHERE id = ?1", [id])
            .await
            .expect("読めません");
        let row = rows.next().await.unwrap().unwrap();
        let (done, version): (i64, i64) = (row.get(0).unwrap(), row.get(1).unwrap());
        if round <= 3 {
            println!(
                "{round} 回目: 返事 {} と {} → done={done} version={version}",
                a.status().as_u16(),
                b.status().as_u16()
            );
        }
        if done == 0 {
            back_to_0 += 1;
        } else {
            stuck_at_1 += 1;
        }
    }
    println!("20 回のうち、2回押して元に戻った（done=0）: {back_to_0} 回");
    println!("20 回のうち、1回分が消えた（done=1）: {stuck_at_1} 回");
}
