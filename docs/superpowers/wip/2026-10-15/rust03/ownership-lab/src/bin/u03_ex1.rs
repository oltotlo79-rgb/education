fn main() {
    let mut text = String::from("abc");
    let a = &text;
    let b = &text;
    println!("{a} {b}");
    let c = &mut text;
    c.push('d');
    println!("{text}");
}
