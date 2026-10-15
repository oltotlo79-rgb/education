fn main() {
    // (タイトル, 完了したか) のタプルを並べた Vec
    let notes: Vec<(&str, bool)> = vec![
        ("牛乳を買う", true),
        ("Rust の勉強", false),
        ("部屋の掃除", false),
    ];
    // 件数は、ループで notes を使い切る前に数えておく
    let total = notes.len();

    let mut done_count = 0;
    for (title, done) in notes {
        let mark = if done { "[x]" } else { "[ ]" };
        println!("{mark} {title}");
        if done {
            done_count += 1;
        }
    }
    println!("{total} 件中 {done_count} 件完了");
}
