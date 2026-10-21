use std::time::Duration;

use async_lab::mini_server::{self, Response};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // 練習用サーバーを起動する。/hello には 200、それ以外には 404 を返す
    let address = mini_server::start(|request| async move {
        if request.path == "/hello" {
            Response::new(200, "こんにちは、RustNote です")
        } else {
            Response::new(404, "見つかりません")
        }
    })
    .await;
    println!("サーバー: http://{address}");

    // Client は1つ作って使い回す（中で接続をため込んで再利用する）
    let client = reqwest::Client::builder()
        // 接続を確立するまでの上限
        .connect_timeout(Duration::from_secs(2))
        // 1回のリクエスト全体（接続〜本文の受信まで）の上限
        .timeout(Duration::from_secs(5))
        .build()?;

    // ① 成功する GET
    let response = client.get(format!("http://{address}/hello")).send().await?;
    println!("① ステータス: {}", response.status());
    println!("① 本文: {}", response.text().await?);

    // ② 404 は「通信としては成功」なので、send() は Ok を返す
    let response = client
        .get(format!("http://{address}/nothing"))
        .send()
        .await?;
    println!(
        "② ステータス: {} / 成功?: {}",
        response.status(),
        response.status().is_success()
    );

    // ③ error_for_status: 4xx・5xx を Err に変える
    let result = client
        .get(format!("http://{address}/nothing"))
        .send()
        .await?
        .error_for_status();
    match result {
        Ok(_) => println!("③ 成功"),
        Err(error) => println!("③ エラー: {error}"),
    }
    Ok(())
}
