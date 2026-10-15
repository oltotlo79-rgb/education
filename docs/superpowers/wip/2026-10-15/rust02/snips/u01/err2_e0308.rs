fn main() {
    let mut input = " 42 ";
    input = input.trim();
    input = input.parse::<u32>().expect("数字ではありません");
    println!("{input}");
}
