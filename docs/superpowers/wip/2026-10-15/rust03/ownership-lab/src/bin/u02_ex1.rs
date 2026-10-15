fn shout(mut s: String) -> String {
    s.push_str("!!");
    s
}

fn main() {
    let a = String::from("hey");
    let b = shout(a);
    let c = b.clone();
    let d = shout(c);
    println!("{b} {d}");
}
