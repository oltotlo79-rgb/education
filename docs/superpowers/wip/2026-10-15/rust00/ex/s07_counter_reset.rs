use std::env;
use std::fs;

fn main() {
    // 1 つ目の引数が "reset" なら、ファイルを消して 0 に戻す
    if env::args().nth(1).as_deref() == Some("reset") {
        // ファイルが無い場合のエラーは気にしないので、結果を捨てる
        let _ = fs::remove_file("count.txt");
        println!("カウンターを 0 に戻しました");
        return;
    }

    let saved = fs::read_to_string("count.txt").unwrap_or(String::from("0"));
    let mut on_disk: u32 = saved.trim().parse().unwrap();
    on_disk += 1;
    fs::write("count.txt", on_disk.to_string()).unwrap();
    println!("ファイルのカウンター: {on_disk}");
}
