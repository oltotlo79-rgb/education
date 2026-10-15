use std::fs;

fn main() {
    // (1) メモリの中だけのカウンター。プログラムが始まるたびに 0 から
    let mut in_memory = 0;
    in_memory += 1;

    // (2) ファイルに保存するカウンター。前回の値を読み、1 足して書き戻す
    //     count.txt が無い（初回）ときは "0" として扱う
    let saved = fs::read_to_string("count.txt").unwrap_or(String::from("0"));
    let mut on_disk: u32 = saved.trim().parse().unwrap();
    on_disk += 1;
    fs::write("count.txt", on_disk.to_string()).unwrap();

    println!("メモリのカウンター  : {in_memory}");
    println!("ファイルのカウンター: {on_disk}");
}
