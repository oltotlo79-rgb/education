use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc: スレッドをまたいで共有できる持ち主カウンタ付きの箱
    // Mutex: 中身を触れるのは鍵を持った1人だけ、という仕組み
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..4 {
        // スレッドごとに持ち手を1つ増やして、それを move で渡す
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                // lock() で鍵を取る。返ってくる guard を通して中身を書き換える
                let mut n = counter.lock().unwrap();
                *n += 1;
                // ここで guard が片付けられ、鍵が自動で返される
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("合計 = {}", *counter.lock().unwrap());
    println!("持ち主の数 = {}", Arc::strong_count(&counter));
}
