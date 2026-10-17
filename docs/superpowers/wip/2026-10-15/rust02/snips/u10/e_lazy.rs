fn main() {
    let titles = ["牛乳を買う", "Rust の勉強", "部屋の掃除"];

    // map を書いただけでは、まだ何も実行されない
    let lengths = titles.iter().map(|t| {
        println!("  「{t}」を処理中");
        t.chars().count()
    });
    println!("まだ何も処理していない");

    // collect で取り出し始めたときに、1つずつ処理が流れる
    let lengths: Vec<usize> = lengths.collect();
    println!("{lengths:?}");
}
