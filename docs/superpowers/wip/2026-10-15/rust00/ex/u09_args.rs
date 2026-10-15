// name さんに times 回あいさつする関数
fn greet(name: &str, times: u32) {
    for _ in 0..times {
        println!("こんにちは、{name} さん");
    }
}

fn main() {
    greet("Hanako", "3");
}
