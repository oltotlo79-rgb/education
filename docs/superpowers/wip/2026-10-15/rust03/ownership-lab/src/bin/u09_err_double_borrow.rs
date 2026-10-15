use std::cell::RefCell;

fn main() {
    let log = RefCell::new(vec![String::from("起動")]);
    let first = log.borrow_mut();
    let second = log.borrow_mut();
    println!("{} {}", first.len(), second.len());
}
