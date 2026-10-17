use std::env;

fn main() {
    println!("Hello, Rust!");

    // このプログラムが「どの OS・どの CPU 向け」にコンパイルされたか
    println!("OS          : {}", env::consts::OS);
    println!("CPU         : {}", env::consts::ARCH);

    // Cargo.toml に書いたパッケージの名前と版（コンパイル時に埋め込まれる）
    println!(
        "パッケージ  : {} {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
}
