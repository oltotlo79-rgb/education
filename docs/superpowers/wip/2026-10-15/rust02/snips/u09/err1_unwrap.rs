fn main() {
    let raw = "abc";
    let count: u32 = raw.parse().unwrap();
    println!("{count}");
}
