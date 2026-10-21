use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};

use async_lab::mini_server::{self, Response};

// 0 以上 max 未満の「ばらつき」を作る（標準ライブラリだけで作る簡易な乱数）
fn jitter(max: Duration) -> Duration {
    let random = RandomState::new().build_hasher().finish();
    let millis = max.as_millis() as u64;
    Duration::from_millis(if millis == 0 { 0 } else { random % millis })
}

// n 回目の失敗のあとに待つ時間: 100ms, 200ms, 400ms, ...（上限 2 秒）＋ばらつき
fn backoff(attempt: u32) -> Duration {
    let base = Duration::from_millis(100) * 2u32.pow(attempt - 1);
    let capped = base.min(Duration::from_secs(2));
    capped + jitter(capped / 2)
}

// この失敗は、もう一度やれば成功するかもしれないか
fn is_retryable(result: &Result<reqwest::Response, reqwest::Error>) -> bool {
    match result {
        // 503（一時的に使えない）・429（混んでいる）・その他の 5xx はやり直す価値がある
        Ok(response) => {
            let status = response.status();
            status.is_server_error() || status.as_u16() == 429
        }
        // 時間切れ・接続できない、はやり直す価値がある
        Err(error) => error.is_timeout() || error.is_connect(),
    }
}

async fn get_with_retry(
    client: &reqwest::Client,
    url: &str,
    max_attempts: u32,
) -> Result<String, String> {
    let start = Instant::now();
    for attempt in 1..=max_attempts {
        let result = client.get(url).send().await;
        let label = match &result {
            Ok(response) => response.status().to_string(),
            Err(error) => error.to_string(),
        };
        println!(
            "  {:.2} 秒: {attempt} 回目 → {label}",
            start.elapsed().as_secs_f64()
        );

        if !is_retryable(&result) {
            // 成功、または「やり直しても無駄な失敗」（404 など）はここで終わり
            let response = result.map_err(|e| e.to_string())?;
            let response = response.error_for_status().map_err(|e| e.to_string())?;
            return response.text().await.map_err(|e| e.to_string());
        }
        if attempt < max_attempts {
            let wait = backoff(attempt);
            println!("  {}ms 待ってやり直します", wait.as_millis());
            tokio::time::sleep(wait).await;
        }
    }
    Err(format!("{max_attempts} 回やっても成功しませんでした"))
}

#[tokio::main]
async fn main() {
    // 最初の2回は 503、3回目からは 200 を返す「調子の悪い」サーバー
    let count = Arc::new(AtomicU32::new(0));
    let server_count = Arc::clone(&count);
    let address = mini_server::start(move |request| {
        let count = Arc::clone(&server_count);
        async move {
            let n = count.fetch_add(1, Ordering::SeqCst) + 1;
            match request.path.as_str() {
                "/flaky" if n <= 2 => Response::new(503, "混んでいます"),
                "/flaky" => Response::new(200, "やっと成功"),
                _ => Response::new(404, "見つかりません"),
            }
        }
    })
    .await;

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    println!("① 調子の悪いサーバー（503 が2回続く）");
    let result = get_with_retry(&client, &format!("http://{address}/flaky"), 5).await;
    println!("結果: {result:?}");

    println!("② 存在しない URL（404 はやり直さない）");
    let result = get_with_retry(&client, &format!("http://{address}/nothing"), 5).await;
    println!("結果: {result:?}");
}
