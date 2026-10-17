use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    set.spawn(async { 42 });
    // 1つ目と違う型の値を返すタスクを入れる
    set.spawn(async { String::from("文字列") });
    while let Some(joined) = set.join_next().await {
        println!("{:?}", joined);
    }
}
