struct Guard {
    name: String,
}

impl Drop for Guard {
    fn drop(&mut self) {
        println!("  解放: {}", self.name);
    }
}

fn main() {
    println!("1. let _ = ...");
    // 名前を付けないと、その場ですぐ片付けられる
    let _ = Guard {
        name: String::from("ガード1"),
    };
    println!("   ↑ もう解放されている");

    println!("2. let _guard = ...");
    // _ で始まる名前を付けると、スコープの終わりまで生きる
    let _guard = Guard {
        name: String::from("ガード2"),
    };
    println!("   ↑ まだ解放されていない");
    println!("main 終了");
}
