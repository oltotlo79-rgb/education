use std::time::Duration;
use tokio::time::timeout;

async fn call_service() -> String {
    tokio::time::sleep(Duration::from_millis(800)).await;
    String::from("応答")
}

async fn fetch(limit: Duration) -> Result<String, String> {
    // 時間切れ（Elapsed）を、この関数のエラーの型（String）に変えてから ? で返す
    let answer = timeout(limit, call_service())
        .await
        .map_err(|_| format!("{} ミリ秒で時間切れ", limit.as_millis()))?;
    Ok(answer)
}

#[tokio::main]
async fn main() {
    println!("{:?}", fetch(Duration::from_millis(500)).await);
    println!("{:?}", fetch(Duration::from_millis(1000)).await);
}
