use std::time::{Duration, Instant};
use tokio::task::JoinSet;

// 外部サービスへの問い合わせの代わり。name が "B" のときだけ失敗する
async fn fetch(name: &'static str, millis: u64) -> Result<String, String> {
    tokio::time::sleep(Duration::from_millis(millis)).await;
    if name == "B" {
        return Err(format!("{name} は応答しませんでした"));
    }
    Ok(format!("{name} の結果"))
}

// 失敗があっても止めずに、全部の結果を集める
async fn fetch_all() -> Vec<Result<String, String>> {
    let mut set = JoinSet::new();
    set.spawn(fetch("A", 100));
    set.spawn(fetch("B", 200));
    set.spawn(fetch("C", 1000));

    let mut results = Vec::new();
    while let Some(joined) = set.join_next().await {
        // panic は Err に変えて、アプリのエラーと同じように集める
        let result = joined.unwrap_or_else(|e| Err(e.to_string()));
        results.push(result);
    }
    results
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    for result in fetch_all().await {
        println!("{result:?}");
    }
    println!("{:.1} 秒", start.elapsed().as_secs_f64());
}
