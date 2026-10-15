fn main() {
    // 段階1: 画面から受け取った文字列（前後に空白がある）
    let input = "  42  ";
    println!("[{input}]");

    // 段階2: 同じ名前で「前後の空白を取り除いた版」を作り直す
    let input = input.trim();
    println!("[{input}]");

    // 段階3: さらに同じ名前で「数（u32 型）に変換した版」を作り直す
    let input: u32 = input.parse().expect("数字ではありません");
    // 数になったので足し算ができる
    println!("{}", input + 1);
}
