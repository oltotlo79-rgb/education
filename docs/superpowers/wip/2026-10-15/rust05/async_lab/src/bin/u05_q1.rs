use std::sync::{Arc, Mutex};
use std::time::Duration;

async fn handle_request(counter: Arc<Mutex<u64>>) {
    // 鍵を使う部分を { } で囲み、ガードが .await より前に片付くことを形で示す
    let now = {
        let mut count = counter.lock().unwrap();
        *count += 1;
        *count
    };
    tokio::time::sleep(Duration::from_millis(10)).await;
    println!("{now} 回目");
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    tokio::spawn(handle_request(Arc::clone(&counter)))
        .await
        .unwrap();
}
