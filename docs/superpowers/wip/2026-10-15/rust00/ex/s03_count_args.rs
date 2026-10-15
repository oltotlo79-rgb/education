use std::env;

fn main() {
    // 0 番目（プログラム自身）を skip(1) で飛ばしてから数える
    let count = env::args().skip(1).count();
    println!("渡された値は {count} 個です");
}
