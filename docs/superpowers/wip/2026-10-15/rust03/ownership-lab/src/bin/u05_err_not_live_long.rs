fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    let a = String::from("ownership");
    let result;
    {
        let b = String::from("borrow");
        result = longest(&a, &b);
    }
    println!("長いほう: {result}");
}
