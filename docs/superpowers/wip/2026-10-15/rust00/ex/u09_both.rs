// name さんにあいさつする関数
fn greet(name: &str) {
    println!("こんにちは、{name} さん");
}

fn main() {
    let name = String::from("Hanako");
    greet(name);
}
