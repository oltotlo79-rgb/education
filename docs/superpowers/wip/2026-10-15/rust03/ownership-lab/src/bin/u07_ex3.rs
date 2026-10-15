fn main() {
    let mut scores = vec![50, 80, 65, 90];
    // 60 未満を取り除く（借用の衝突なく、その場で消せる）
    scores.retain(|&s| s >= 60);
    println!("{scores:?}");
}
