use std::time::{Duration, Instant};

struct Announce(&'static str);

impl Drop for Announce {
    fn drop(&mut self) {
        println!("  片付け: {}", self.0);
    }
}

// 本命の DB（遅い）
async fn primary_db(millis: u64) -> &'static str {
    let _announce = Announce("primary_db");
    tokio::time::sleep(Duration::from_millis(millis)).await;
    println!("  primary_db: 最後まで終わりました");
    "本命の DB の結果"
}

// 予備のキャッシュ（速い）
async fn cache(millis: u64) -> &'static str {
    let _announce = Announce("cache");
    tokio::time::sleep(Duration::from_millis(millis)).await;
    println!("  cache: 最後まで終わりました");
    "キャッシュの結果"
}

async fn race(db_millis: u64, cache_millis: u64) {
    let start = Instant::now();
    // 2つの Future を同時に進め、先に終わったほうの枝だけを実行する
    let winner = tokio::select! {
        value = primary_db(db_millis) => format!("DB が勝ち: {value}"),
        value = cache(cache_millis) => format!("キャッシュが勝ち: {value}"),
    };
    println!("{winner}（{:.1} 秒）", start.elapsed().as_secs_f64());
}

#[tokio::main]
async fn main() {
    println!("① DB 500ms / キャッシュ 100ms");
    race(500, 100).await;
    println!("② DB 100ms / キャッシュ 500ms");
    race(100, 500).await;
}
