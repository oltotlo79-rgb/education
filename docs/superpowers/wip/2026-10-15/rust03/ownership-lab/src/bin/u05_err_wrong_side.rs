fn after_prefix<'a>(text: &'a str, prefix: &str) -> &'a str {
    if text.starts_with(prefix) {
        prefix
    } else {
        text
    }
}

fn main() {
    println!("{}", after_prefix("/notes/42", "/notes/"));
}
