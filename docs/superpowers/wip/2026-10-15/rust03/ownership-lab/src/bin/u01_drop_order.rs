// 片付けられるときにメッセージを出す、実験用の型
struct Noisy {
    name: String,
}

impl Noisy {
    fn new(name: &str) -> Noisy {
        println!("  作成: {name}");
        Noisy {
            name: name.to_string(),
        }
    }
}

// Drop を実装すると「片付けのときに実行する処理」を書ける
impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  片付け: {}", self.name);
    }
}

fn main() {
    println!("main 開始");
    let _a = Noisy::new("a");
    {
        println!("内側のブロック開始");
        let _b = Noisy::new("b");
        let _c = Noisy::new("c");
        println!("内側のブロック終了");
    }
    let d = Noisy::new("d");
    // drop(値) で、スコープの終わりを待たずに今すぐ片付ける
    drop(d);
    println!("main 終了");
}
