use std::time::Duration;
use tokio::time::timeout;

async fn call_service() -> String {
    tokio::time::sleep(Duration::from_millis(800)).await;
    String::from("応答")
}

// エラーを String で返す関数の中で、timeout の結果に ? を付ける
async fn fetch() -> Result<String, String> {
    let answer = timeout(Duration::from_millis(500), call_service()).await?;
    Ok(answer)
}

#[tokio::main]
async fn main() {
    println!("{:?}", fetch().await);
}
