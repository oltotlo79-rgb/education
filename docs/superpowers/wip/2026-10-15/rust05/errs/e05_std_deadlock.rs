use std::sync::Mutex;
use std::time::Duration;

async fn task(name: &str, counter: &Mutex<u64>) {
    println!("{name}: 鍵を取りに行きます");
    let mut count = counter.lock().unwrap();
    println!("{name}: 鍵を取りました");
    *count += 1;
    // 標準の Mutex の鍵を持ったまま .await して、スレッドを手放してしまう
    tokio::time::sleep(Duration::from_millis(100)).await;
    println!("{name}: 終わりました（{}）", *count);
}

// スレッドが1本だけのランタイムで動かす
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let counter = Mutex::new(0);
    // spawn せずに join! で2つを同時に進めるので、Send は求められずコンパイルが通る
    tokio::join!(task("A", &counter), task("B", &counter));
    println!("全部終わりました");
}
