use std::cell::RefCell;

fn main() {
    let log = RefCell::new(vec![String::from("起動"), String::from("ログイン")]);
    for line in log.borrow().iter() {
        if line == "ログイン" {
            log.borrow_mut().push(String::from("ログインを検知"));
        }
    }
    println!("{:?}", log.borrow());
}
