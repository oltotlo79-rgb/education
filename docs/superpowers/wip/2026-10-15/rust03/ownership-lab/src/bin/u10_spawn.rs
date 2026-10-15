use std::thread;

fn main() {
    let mut handles = Vec::new();
    for id in 1..=3 {
        // move を付けると、クロージャが id の値を「持って」新しいスレッドへ行く
        let handle = thread::spawn(move || {
            for step in 1..=2 {
                println!("スレッド{id}: {step}回目");
            }
            // スレッドの最後の式が、join で受け取れる結果になる
            id * 10
        });
        handles.push(handle);
    }

    println!("main: 全員の終了を待ちます");
    for handle in handles {
        // join はスレッドの終了を待つ。中で panic していたら Err になる
        let result = handle.join().unwrap();
        println!("main: 結果 {result}");
    }
}
