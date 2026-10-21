use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_lab::mini_server::{self, Request, Response};

// サーバー側の記録: 実際に「送金」した回数と、冪等キーごとの答え
#[derive(Default)]
struct Ledger {
    charged: u32,
    answers: HashMap<String, String>,
}

// POST /payments の処理。最初の1回だけ、答えを返すのがとても遅い（ネットワークが詰まった想定）
async fn handle_payment(ledger: Arc<Mutex<Ledger>>, request: Request) -> Response {
    let key = request.header("Idempotency-Key").map(str::to_string);
    let answer = {
        let mut ledger = ledger.lock().unwrap();
        // 同じキーで処理済みなら、記録しておいた答えをそのまま返す（もう一度は送金しない）
        if let Some(key) = &key
            && let Some(saved) = ledger.answers.get(key)
        {
            return Response::new(200, format!("{saved}（記録済みの答え）"));
        }
        ledger.charged += 1;
        let answer = format!(
            "{} を受け付けました（送金 {} 回目）",
            request.body, ledger.charged
        );
        if let Some(key) = key {
            ledger.answers.insert(key, answer.clone());
        }
        answer
    };
    // 送金はもう終わったが、答えが相手に届くまで時間がかかる
    if answer.contains("送金 1 回目") {
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    Response::new(201, answer)
}

// 冪等キー（このリクエストの「整理番号」）を作る。本物では uuid クレートなどを使う
fn new_key() -> String {
    format!("{:016x}", RandomState::new().build_hasher().finish())
}

async fn pay(client: &reqwest::Client, url: &str, key: Option<&str>) {
    for attempt in 1..=3 {
        let mut request = client.post(url).body("1000 円");
        if let Some(key) = key {
            request = request.header("Idempotency-Key", key);
        }
        match request.send().await {
            Ok(response) => {
                println!("  {attempt} 回目: {}", response.text().await.unwrap());
                return;
            }
            Err(error) if error.is_timeout() => println!("  {attempt} 回目: 時間切れ → やり直す"),
            Err(error) => {
                println!("  {attempt} 回目: 失敗 {error}");
                return;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::builder()
        // 200ms で諦める（サーバーの最初の答えは 500ms かかるので、必ず時間切れになる）
        .timeout(Duration::from_millis(200))
        .build()
        .unwrap();

    for use_key in [false, true] {
        let ledger = Arc::new(Mutex::new(Ledger::default()));
        let server_ledger = Arc::clone(&ledger);
        let address =
            mini_server::start(move |request| handle_payment(Arc::clone(&server_ledger), request))
                .await;
        let url = format!("http://{address}/payments");

        if use_key {
            println!("② 冪等キーあり");
            let key = new_key();
            pay(&client, &url, Some(&key)).await;
        } else {
            println!("① 冪等キーなし");
            pay(&client, &url, None).await;
        }
        println!(
            "  → サーバーが実際に送金した回数: {}",
            ledger.lock().unwrap().charged
        );
    }
}
