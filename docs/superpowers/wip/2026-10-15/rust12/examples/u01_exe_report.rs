//! Windows の実行ファイル（.exe）を調べて、大きさ・リンクした時刻・ユーザーフォルダの
//! 場所が何か所入っているかを表示する。2つ渡すと、違うバイトの数も数える。
//! 実行: cargo run -q --example u01_exe_report -- 1つ目.exe [2つ目.exe]

use std::env;
use std::fs;

/// PE 形式（Windows の実行ファイルの形式）の見出しから、リンクした時刻を読む。
/// 時刻は「1970-01-01 からの秒数」（UNIX 時刻）で入っている
fn link_time(bytes: &[u8]) -> Option<u32> {
    // 先頭から 0x3C の位置に、PE の見出しがどこから始まるかが書いてある
    let header: [u8; 4] = bytes.get(0x3C..0x40)?.try_into().ok()?;
    let pe = usize::try_from(u32::from_le_bytes(header)).ok()?;
    // 見出しは "PE\0\0" で始まり、その 8 バイト後ろに時刻がある
    if bytes.get(pe..pe + 4)? != b"PE\0\0" {
        return None;
    }
    let time: [u8; 4] = bytes.get(pe + 8..pe + 12)?.try_into().ok()?;
    Some(u32::from_le_bytes(time))
}

/// `needle` が何か所に入っているかを数える
fn count(haystack: &[u8], needle: &[u8]) -> usize {
    haystack
        .windows(needle.len())
        .filter(|window| *window == needle)
        .count()
}

fn main() -> std::io::Result<()> {
    let paths: Vec<String> = env::args().skip(1).collect();
    // ユーザーフォルダの場所（例: C:\Users\taro）。場所そのものは表示しない
    let profile = env::var("USERPROFILE").unwrap_or_default();

    let mut files = Vec::new();
    for path in &paths {
        let bytes = fs::read(path)?;
        println!("[{path}]");
        println!("    大きさ: {} バイト", bytes.len());
        match link_time(&bytes) {
            Some(time) => println!("    リンクした時刻（UNIX 秒）: {time}"),
            None => println!("    リンクした時刻: 読めません（PE 形式ではない）"),
        }
        if !profile.is_empty() {
            let found = count(&bytes, profile.as_bytes());
            println!("    ユーザーフォルダの場所が入っている数: {found} か所");
        }
        files.push(bytes);
    }

    if let [first, second] = files.as_slice() {
        let same_length = first.len() == second.len();
        let differ = first.iter().zip(second).filter(|(a, b)| a != b).count();
        println!("--- 2つを比べる");
        println!("    大きさが同じ: {same_length}");
        println!("    違うバイトの数: {differ}");
    }
    Ok(())
}
