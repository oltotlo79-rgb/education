fn save(title: &str) {
    println!("保存: {title}");
}

fn main() {
    let title = String::from("買い物");
    for _ in 0..2 {
        save(&title);
    }
    println!("最後に: {title}");
}
