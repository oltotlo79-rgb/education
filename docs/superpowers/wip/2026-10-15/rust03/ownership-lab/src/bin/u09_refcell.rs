use std::cell::RefCell;

fn main() {
    // mut を付けていない変数でも、RefCell の中身は書き換えられる
    let log = RefCell::new(Vec::new());
    log.borrow_mut().push(String::from("起動"));
    log.borrow_mut().push(String::from("ログイン"));

    {
        // 読み取りの借用は、同時にいくつでも OK
        let r1 = log.borrow();
        let r2 = log.borrow();
        println!("同時に読む: {} 件 / {} 件", r1.len(), r2.len());
    }

    // try_borrow_mut は「今、書き込み用に借りられるか」を panic せずに確かめる
    let reading = log.borrow();
    match log.try_borrow_mut() {
        Ok(_) => println!("読んでいる最中: 書ける"),
        Err(e) => println!("読んでいる最中: 書けない ({e:?})"),
    }
    drop(reading);
    match log.try_borrow_mut() {
        Ok(_) => println!("読み終えた後:   書ける"),
        Err(e) => println!("読み終えた後:   書けない ({e:?})"),
    }

    println!("{:?}", log.borrow());
}
