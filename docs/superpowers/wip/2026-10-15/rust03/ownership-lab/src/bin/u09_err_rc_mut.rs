use std::rc::Rc;

fn main() {
    let shared = Rc::new(String::from("共有メモ"));
    let other = Rc::clone(&shared);
    shared.push_str("（追記）");
    println!("{other}");
}
