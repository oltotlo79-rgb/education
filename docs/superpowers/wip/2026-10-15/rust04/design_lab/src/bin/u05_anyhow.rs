use anyhow::{Context, Result, bail};
use std::fs;

// シード（最初に入れておくノート）を、1 行 1 件のテキストファイルから読む
fn load_seed(path: &str) -> Result<Vec<String>> {
    let text = fs::read_to_string(path)
        .with_context(|| format!("シードファイル {path} を読めませんでした"))?;

    let titles: Vec<String> = text
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    if titles.is_empty() {
        bail!("シードファイル {path} にノートが1件もありません");
    }
    Ok(titles)
}

// main が anyhow::Result を返すと、失敗したときに原因の連鎖を表示して終了する
fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "seed.txt".to_string());
    let titles = load_seed(&path).context("シードの投入に失敗しました")?;
    for title in &titles {
        println!("投入: {title}");
    }
    println!("{} 件のノートを投入しました", titles.len());
    Ok(())
}
