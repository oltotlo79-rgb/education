use std::sync::Arc;
use std::thread;

fn main() {
    let title = Arc::new(String::from("共有メモ"));
    let cloned = Arc::clone(&title);
    let handle = thread::spawn(move || {
        println!("スレッド側: {cloned}");
    });
    handle.join().unwrap();
    println!("main 側: {title} / 持ち主 {} 人", Arc::strong_count(&title));
}
