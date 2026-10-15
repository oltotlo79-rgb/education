use std::env;
use std::process;

fn main() {
    // このプログラムが動いている OS の種類（windows / linux / macos など）
    let os = env::consts::OS;
    // CPU の種類（x86_64 / aarch64 など）
    let arch = env::consts::ARCH;
    // OS がこのプロセスに付けた番号（プロセス ID）
    let pid = process::id();

    println!("OS の種類: {os}");
    println!("CPU の種類: {arch}");
    println!("プロセス ID: {pid}");

    // CPU に足し算を 100 回してもらい、途中の値はメモリ上の total に置く
    let mut total = 0;
    for n in 1..=100 {
        total += n;
    }
    println!("1 から 100 までの合計: {total}");
}
