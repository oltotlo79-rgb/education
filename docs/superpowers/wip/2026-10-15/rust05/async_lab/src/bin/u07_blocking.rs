use std::time::{Duration, Instant};

use argon2::{Argon2, password_hash::PasswordHasher};

// CPU を使う重い処理: Argon2id でパスワードをハッシュ化する
fn hash(password: &str) -> String {
    Argon2::default()
        .hash_password(password.as_bytes())
        .expect("ハッシュ化に失敗しました")
        .to_string()
}

// 「心臓の鼓動」役: 200ms ごとに経過時間を表示する。止まったら、ほかのタスクが動けていない証拠
async fn heartbeat(start: Instant) {
    for _ in 0..12 {
        println!("  ♥ {:.1} 秒", start.elapsed().as_secs_f64());
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

// ワーカースレッドが1本だけのランタイム（違いが見やすいように）
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let start = Instant::now();
    let beat = tokio::spawn(heartbeat(start));
    tokio::time::sleep(Duration::from_millis(300)).await;

    // ① 悪い例: async の中で、そのまま重い計算をする
    println!(
        "① そのまま計算 開始 {:.1} 秒",
        start.elapsed().as_secs_f64()
    );
    let _ = hash("password-1");
    println!(
        "① そのまま計算 終了 {:.1} 秒",
        start.elapsed().as_secs_f64()
    );

    tokio::time::sleep(Duration::from_millis(300)).await;

    // ② 良い例: spawn_blocking で、重い計算専用の別スレッドへ任せる
    println!(
        "② spawn_blocking 開始 {:.1} 秒",
        start.elapsed().as_secs_f64()
    );
    let _ = tokio::task::spawn_blocking(|| hash("password-2"))
        .await
        .expect("計算のスレッドが panic しました");
    println!(
        "② spawn_blocking 終了 {:.1} 秒",
        start.elapsed().as_secs_f64()
    );

    beat.await.unwrap();
}
