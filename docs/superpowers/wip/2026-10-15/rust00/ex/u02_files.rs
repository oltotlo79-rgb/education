use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // 今いるフォルダ（カレントディレクトリ）を調べて表示する
    let here = env::current_dir()?;
    println!("今いるフォルダ: {}", here.display());

    // memo フォルダを作る（すでにあっても OK）
    fs::create_dir_all("memo")?;

    // 「memo フォルダの中の today.txt」という相対パスを組み立てる
    let relative = Path::new("memo").join("today.txt");

    // ファイルに文字を書き込む（無ければ作り、あれば上書き）
    fs::write(&relative, "Rust の勉強を始めた\n")?;

    // 同じファイルを読み戻す
    let text = fs::read_to_string(&relative)?;

    println!("相対パス: {}", relative.display());
    println!("絶対パス: {}", here.join(&relative).display());
    print!("中身: {text}");
    Ok(())
}
