use std::thread;

fn main() {
    let title = String::from("買い物");
    let handle = thread::spawn(|| {
        println!("{title}");
    });
    handle.join().unwrap();
}
