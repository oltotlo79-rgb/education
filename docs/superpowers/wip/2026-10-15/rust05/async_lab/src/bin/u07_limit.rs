use std::sync::Arc;
use std::time::{Duration, Instant};

use argon2::{Argon2, password_hash::PasswordHasher};
use tokio::sync::Semaphore;

// 重い計算の同時実行数に上限を付けた「ハッシュ係」
#[derive(Clone)]
struct Hasher {
    permits: Arc<Semaphore>,
}

impl Hasher {
    fn new(max_concurrent: usize) -> Hasher {
        Hasher {
            permits: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    async fn hash(&self, password: String) -> Result<String, String> {
        // 許可証が空くまで待つ（待っている間はスレッドを使わない）
        let _permit = self.permits.acquire().await.map_err(|e| e.to_string())?;
        tokio::task::spawn_blocking(move || {
            Argon2::default()
                .hash_password(password.as_bytes())
                .map(|h| h.to_string())
                .map_err(|e| e.to_string())
        })
        .await
        .map_err(|e| e.to_string())?
    }
}

#[tokio::main]
async fn main() {
    let hasher = Hasher::new(2);
    let start = Instant::now();
    let mut handles = Vec::new();
    // 6人が同時に登録ボタンを押した
    for n in 1..=6 {
        let hasher = hasher.clone();
        handles.push(tokio::spawn(async move {
            let result = hasher.hash(format!("password-{n}")).await;
            println!(
                "{:.1} 秒: {n} 人目のハッシュ化が終わった（成功: {}）",
                start.elapsed().as_secs_f64(),
                result.is_ok()
            );
        }));
    }
    // その間も、ほかのリクエスト（ここでは軽い処理）は待たされずに動く
    tokio::time::sleep(Duration::from_millis(100)).await;
    println!(
        "{:.1} 秒: 軽いリクエストはすぐ返せる",
        start.elapsed().as_secs_f64()
    );
    for handle in handles {
        handle.await.unwrap();
    }
}
