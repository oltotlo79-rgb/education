use std::error::Error;
use std::time::Duration;

use async_lab::mini_server::{self, Response};
use serde::Deserialize;

// 外の API が返す JSON の形を、struct で書いておく
#[derive(Debug, Deserialize)]
struct Weather {
    city: String,
    celsius: f64,
}

#[tokio::main]
async fn main() {
    // /weather は正しい JSON、/broken は celsius が抜けた JSON を返す
    let address = mini_server::start(|request| async move {
        match request.path.as_str() {
            "/weather" => Response::new(200, r#"{"city":"東京","celsius":18.5}"#),
            "/broken" => Response::new(200, r#"{"city":"東京"}"#),
            _ => Response::new(404, "見つかりません"),
        }
    })
    .await;

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    for path in ["/weather", "/broken"] {
        let result = client
            .get(format!("http://{address}{path}"))
            .send()
            .await
            .and_then(|response| response.error_for_status());
        let weather = match result {
            // 本文を JSON として読み、Weather に変換する
            Ok(response) => response.json::<Weather>().await,
            Err(error) => Err(error),
        };
        match weather {
            Ok(weather) => println!("{path}: {} は {} 度", weather.city, weather.celsius),
            Err(error) => {
                println!("{path}: 失敗 {error}");
                if let Some(cause) = error.source() {
                    println!("    原因: {cause}");
                }
            }
        }
    }
}
