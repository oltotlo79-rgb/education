use std::cell::RefCell;

fn main() {
    let log = RefCell::new(vec![String::from("起動"), String::from("ログイン")]);
    // 1. 先に「足すべきか」を調べ、読み取りの借用をここで終わらせる
    let found = log.borrow().iter().any(|line| line == "ログイン");
    // 2. それから書き込みの借用を取る
    if found {
        log.borrow_mut().push(String::from("ログインを検知"));
    }
    println!("{:?}", log.borrow());
}
