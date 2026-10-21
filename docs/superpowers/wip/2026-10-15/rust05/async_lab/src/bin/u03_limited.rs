use std::time::{Duration, Instant};
use tokio::task::JoinSet;

// 外部へのリクエストの代わり。どれも 200ms かかる
async fn download(id: u32) -> u32 {
    tokio::time::sleep(Duration::from_millis(200)).await;
    id
}

#[tokio::main]
async fn main() {
    // 同時に動かすのは最大 2 個まで
    const LIMIT: usize = 2;
    let start = Instant::now();
    let mut set = JoinSet::new();
    let mut done = Vec::new();

    for id in 1..=6 {
        // 袋がいっぱいなら、どれか1つ終わるまで待ってから次を入れる
        if set.len() >= LIMIT {
            if let Some(joined) = set.join_next().await {
                done.push(joined.unwrap());
            }
        }
        println!("{:.2} 秒: {id} 番を開始", start.elapsed().as_secs_f64());
        set.spawn(download(id));
    }
    // 残りを全部受け取る。join_all は全部終わるのを待って Vec で返す
    done.extend(set.join_all().await);

    done.sort();
    println!("{done:?}（{:.2} 秒）", start.elapsed().as_secs_f64());
}
