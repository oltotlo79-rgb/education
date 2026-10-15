use std::rc::Rc;
use std::thread;

fn main() {
    let shared = Rc::new(String::from("共有メモ"));
    let cloned = Rc::clone(&shared);
    let handle = thread::spawn(move || {
        println!("{cloned}");
    });
    handle.join().unwrap();
}
