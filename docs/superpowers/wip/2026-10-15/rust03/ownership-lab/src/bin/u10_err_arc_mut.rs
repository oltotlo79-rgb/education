use std::sync::Arc;

fn main() {
    let count = Arc::new(0);
    let other = Arc::clone(&count);
    *count += 1;
    println!("{other}");
}
