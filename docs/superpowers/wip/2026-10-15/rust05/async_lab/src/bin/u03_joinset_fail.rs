use std::time::{Duration, Instant};
use tokio::task::JoinSet;

// 片付けのときに表示する印。タスクが途中で取り消されると、ここで分かる
struct Announce(&'static str);

impl Drop for Announce {
    fn drop(&mut self) {
        println!("  片付け: {}", self.0);
    }
}

// 外部サービスへの問い合わせの代わり。name が "B" のときだけ失敗する
async fn fetch(name: &'static str, millis: u64) -> Result<String, String> {
    let _announce = Announce(name);
    tokio::time::sleep(Duration::from_millis(millis)).await;
    if name == "B" {
        return Err(format!("{name} は応答しませんでした"));
    }
    Ok(format!("{name} の結果"))
}

async fn fetch_all() -> Result<Vec<String>, String> {
    let mut set = JoinSet::new();
    set.spawn(fetch("A", 100));
    set.spawn(fetch("B", 200));
    set.spawn(fetch("C", 1000));

    let mut results = Vec::new();
    while let Some(joined) = set.join_next().await {
        // 1つ目の ? は「panic したか」、2つ目の ? は「Err を返したか」
        let value = joined.map_err(|e| e.to_string())??;
        results.push(value);
    }
    Ok(results)
    // 途中で return Err した場合も、ここで set が片付けられ、残りの子タスクは取り消される
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let result = fetch_all().await;
    println!("{result:?}（{:.1} 秒）", start.elapsed().as_secs_f64());
    // 取り消された子タスクの片付けが表示される時間を少しだけ待つ
    tokio::time::sleep(Duration::from_millis(50)).await;
}
