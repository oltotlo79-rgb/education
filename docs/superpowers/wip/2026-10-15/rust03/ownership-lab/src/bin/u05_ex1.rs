fn shorter<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.chars().count() <= y.chars().count() {
        x
    } else {
        y
    }
}

fn main() {
    let a = String::from("所有権");
    let b = String::from("ライフタイム");
    println!("{}", shorter(&a, &b));
}
