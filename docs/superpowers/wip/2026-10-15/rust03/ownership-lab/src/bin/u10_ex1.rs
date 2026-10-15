use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let names = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for id in 1..=3 {
        let names = Arc::clone(&names);
        handles.push(thread::spawn(move || {
            names.lock().unwrap().push(format!("worker{id}"));
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let mut result = names.lock().unwrap().clone();
    // 追加された順番は実行ごとに変わるので、並べ替えてから表示する
    result.sort();
    println!("{result:?}");
}
