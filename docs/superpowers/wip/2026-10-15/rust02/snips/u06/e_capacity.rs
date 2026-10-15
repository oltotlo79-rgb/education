fn main() {
    let mut v: Vec<i32> = Vec::new();
    println!("最初: len = {}, capacity = {}", v.len(), v.capacity());
    for i in 1..=9 {
        v.push(i);
        println!(
            "push({i}) 後: len = {}, capacity = {}",
            v.len(),
            v.capacity()
        );
    }

    // 入る数が分かっているなら、最初に広さを確保しておける
    let reserved: Vec<i32> = Vec::with_capacity(100);
    println!(
        "with_capacity(100): len = {}, capacity = {}",
        reserved.len(),
        reserved.capacity()
    );
}
