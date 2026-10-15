use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    fs::create_dir_all("memo")?;

    // 2 つのファイルを書く
    fs::write(Path::new("memo").join("today.txt"), "今日の予定\n")?;
    fs::write(Path::new("memo").join("tomorrow.txt"), "明日の予定\n")?;

    // memo フォルダの中身を一覧表示する
    for entry in fs::read_dir("memo")? {
        let entry = entry?;
        println!("{}", entry.path().display());
    }
    Ok(())
}
