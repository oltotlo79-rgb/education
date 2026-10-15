fn main() {
    // title という名前に、文字列 "Rust入門" を結び付ける
    let title = "Rust入門";
    // count という名前に、数 3 を結び付ける
    let count = 3;

    // {} の位置に、後ろに書いた値が入って表示される
    println!("タイトル: {}", title);
    // {count} のように、波かっこの中へ変数名を直接書いてもよい
    println!("ノートの数: {count}");
}
