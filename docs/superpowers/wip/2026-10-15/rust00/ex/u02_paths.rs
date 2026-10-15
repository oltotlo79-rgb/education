use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // 1 つのパスを部品に分けてみる
    let path = Path::new("src/main.rs");
    println!("パス全体  : {}", path.display());
    println!("ファイル名: {:?}", path.file_name());
    println!("拡張子    : {:?}", path.extension());
    println!("親フォルダ: {:?}", path.parent());
    println!();

    // いろいろな相対パスが「今いるフォルダから見て」存在するか調べる
    let candidates = [
        "Cargo.toml",
        "src/main.rs",
        "main.rs",
        "./src/../Cargo.toml",
    ];
    for candidate in candidates {
        let exists = Path::new(candidate).exists();
        println!("{candidate:<22} -> 存在する? {exists}");
    }
    println!();

    // 今いるフォルダの中身を一覧表示する
    println!("今いるフォルダの中身:");
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let kind = if entry.path().is_dir() {
            "フォルダ"
        } else {
            "ファイル"
        };
        println!("  [{kind}] {}", entry.file_name().display());
    }
    Ok(())
}
