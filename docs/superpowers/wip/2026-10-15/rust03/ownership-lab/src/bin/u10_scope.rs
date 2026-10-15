use std::thread;

fn main() {
    let notes = vec![String::from("買い物"), String::from("掃除"), String::from("読書")];
    let mut total_chars = 0;

    // scope の中で作ったスレッドは、scope を抜ける前に必ず終わる
    // だから、外の変数を Arc なしで借りられる
    thread::scope(|s| {
        s.spawn(|| {
            println!("件数: {}", notes.len());
        });
        s.spawn(|| {
            total_chars = notes.iter().map(|n| n.chars().count()).sum();
        });
    });

    println!("文字数の合計: {total_chars}");
}
