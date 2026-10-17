fn main() {
    tracing_subscriber::fmt().init();
    tracing::info!("1回目の登録");
    // 別の場所（テストの準備など）で、もう一度登録してしまう
    tracing_subscriber::fmt().json().init();
}
