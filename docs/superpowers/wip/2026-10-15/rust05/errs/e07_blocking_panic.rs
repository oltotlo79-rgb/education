#[tokio::main]
async fn main() {
    let joined = tokio::task::spawn_blocking(|| {
        let sizes: Vec<u32> = Vec::new();
        // 空の Vec の最大値を unwrap して panic する
        sizes.iter().max().copied().unwrap()
    })
    .await;
    println!("{joined:?}");
}
