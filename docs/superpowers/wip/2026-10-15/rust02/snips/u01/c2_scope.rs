fn main() {
    let level = 1;
    {
        // このブロック { } の中だけで有効な、新しい level を作る
        let level = level * 10;
        println!("ブロックの中: {level}");
    }
    // ブロックを出ると、新しい level は消え、元の level が見える
    println!("ブロックの外: {level}");
}
