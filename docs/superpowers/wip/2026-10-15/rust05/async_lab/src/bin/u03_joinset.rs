use std::time::{Duration, Instant};
use tokio::task::JoinSet;

// 画像の縮小などの仕事の代わり: 指定の時間だけ待ってから結果を返す
async fn make_thumbnail(name: &'static str, millis: u64) -> String {
    tokio::time::sleep(Duration::from_millis(millis)).await;
    format!("{name} の縮小画像")
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // JoinSet: 子タスクをまとめて持つ「親の袋」
    let mut set = JoinSet::new();
    set.spawn(make_thumbnail("a.png", 300));
    set.spawn(make_thumbnail("b.png", 100));
    set.spawn(make_thumbnail("c.png", 200));
    println!("子タスクの数: {}", set.len());

    // join_next: 「終わった順に」1つずつ結果を受け取る。全部受け取ると None
    while let Some(joined) = set.join_next().await {
        let thumbnail = joined.expect("子タスクが panic しました");
        println!("{:.1} 秒: {thumbnail}", start.elapsed().as_secs_f64());
    }
    println!("全部終わりました（残り {} 個）", set.len());
}
