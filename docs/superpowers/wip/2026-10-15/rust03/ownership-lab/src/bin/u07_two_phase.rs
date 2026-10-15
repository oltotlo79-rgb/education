fn main() {
    let mut v = vec![10, 20];
    // v.len() を先に読んでから push する。この順番なら OK（2段階借用）
    v.push(v.len());
    println!("{v:?}");
}
