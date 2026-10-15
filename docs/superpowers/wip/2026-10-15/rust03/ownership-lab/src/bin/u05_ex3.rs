fn domain_of(email: &str) -> &str {
    match email.find('@') {
        Some(at) => &email[at + 1..],
        None => "",
    }
}

fn main() {
    let email = String::from("alice@example.com");
    println!("{}", domain_of(&email));
}
