fn main() {
    let mut remaining = 3;
    // while は「条件が true の間」くり返す
    while remaining > 0 {
        println!("残り {remaining}");
        remaining -= 1;
    }
    println!("発射！");
}
