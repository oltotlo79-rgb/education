// #[tokio::main] は、tokio の macros の機能が有効なときだけ使える
#[tokio::main]
async fn main() {
    println!("tokio のランタイムの中で動いています");
}
