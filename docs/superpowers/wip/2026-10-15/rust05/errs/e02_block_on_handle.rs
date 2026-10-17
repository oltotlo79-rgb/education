use tokio::runtime::Handle;

fn load_title() -> String {
    // 今動いているランタイムを借りて、その上で待ちきろうとする
    Handle::current().block_on(async { String::from("買い物メモ") })
}

#[tokio::main]
async fn main() {
    let title = load_title();
    println!("{title}");
}
