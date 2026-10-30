//! 第8巻 UNIT 04: 「登録していないメール」と「パスワードが違う」で、ログインの失敗にかかる時間を比べる。
//! 実行: cargo test --test u04_timing -- --nocapture

mod common;

use std::time::Instant;

use common::{encode, test_app};

#[tokio::test]
async fn failure_timing() {
    let app = test_app().await;
    app.sign_up("alice@example.com", "correct horse battery")
        .await;
    // ダミーのハッシュの準備（最初の1回だけ遅い。UNIT 02 の問2）を、先に済ませておく
    let warm = format!("email={}&password=x", encode("warm@example.com"));
    app.post_form("/login", &warm, None).await;

    // 続けて計算すると PC の都合で遅くなる回があるので、2つを交互に送って、同じ条件で比べる
    let mut rows = [Vec::new(), Vec::new()];
    for _ in 0..4 {
        for (i, email) in ["dave@example.com", "alice@example.com"]
            .into_iter()
            .enumerate()
        {
            let form = format!(
                "email={}&password={}",
                encode(email),
                encode("wrong password")
            );
            let started = Instant::now();
            let status = app.post_form("/login", &form, None).await.status();
            rows[i].push(format!(
                "{}（{} ms）",
                status.as_u16(),
                started.elapsed().as_millis()
            ));
        }
    }
    println!("登録していないメール: {}", rows[0].join(" / "));
    println!("パスワードが違う:     {}", rows[1].join(" / "));
}
