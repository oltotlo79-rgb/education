//! 第13巻 UNIT 05 エラー1: #[async_trait] を付けずに、async fn を持つ trait を dyn で使おうとした。
//! 実行: cargo run -q --example e05_dyn

use std::sync::Arc;

trait NoteCounter {
    async fn count(&self) -> u64;
}

struct Fixed;

impl NoteCounter for Fixed {
    async fn count(&self) -> u64 {
        3
    }
}

#[tokio::main]
async fn main() {
    let counter: Arc<dyn NoteCounter + Send + Sync> = Arc::new(Fixed);
    println!("{}", counter.count().await);
}
