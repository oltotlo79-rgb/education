use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let file = "notes.tsv";

    // 引数が渡されたら、それをタイトルとして 1 行（1 件）追加する
    if let Some(title) = env::args().nth(1) {
        // 今ある行数 + 1 を、新しい行の id にする
        let existing = fs::read_to_string(file).unwrap_or_default();
        let next_id = existing.lines().count() + 1;

        // ファイルの末尾に追記する（無ければ作る）
        let mut f = OpenOptions::new().create(true).append(true).open(file)?;
        // 列の順番: id, user_id, title, done（タブ文字 \t で区切る）
        writeln!(f, "{next_id}\t1\t{title}\tfalse")?;
    }

    // ファイルの全行を、表（テーブル）の形で表示する
    let text = fs::read_to_string(file).unwrap_or_default();
    println!("id | user_id | done  | title");
    for line in text.lines() {
        let cols: Vec<&str> = line.split('\t').collect();
        println!(
            "{:<2} | {:<7} | {:<5} | {}",
            cols[0], cols[1], cols[3], cols[2]
        );
    }
    Ok(())
}
