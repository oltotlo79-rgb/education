use std::time::Instant;

// ファイルを読んで行数を数える（同期の処理。ファイルが大きいと時間がかかる）
fn count_lines(path: &str) -> std::io::Result<usize> {
    let text = std::fs::read_to_string(path)?;
    Ok(text.lines().count())
}

// async の中から呼べるように、spawn_blocking で別スレッドへ任せる
async fn count_lines_async(path: String) -> Result<usize, String> {
    tokio::task::spawn_blocking(move || count_lines(&path))
        .await
        .map_err(|e| format!("読むスレッドが異常終了: {e}"))?
        .map_err(|e| format!("読めません: {e}"))
}

#[tokio::main]
async fn main() {
    // 練習用に、10 万行のファイルを作っておく
    let lines: Vec<String> = (1..=100_000).map(|n| format!("{n} 行目")).collect();
    std::fs::write("u07_q2.txt", lines.join("\n")).expect("ファイルを作れません");

    let start = Instant::now();
    let count = count_lines_async(String::from("u07_q2.txt")).await;
    println!("{count:?}（{:.2} 秒）", start.elapsed().as_secs_f64());
    println!(
        "{:?}",
        count_lines_async(String::from("no-such-file.txt")).await
    );
}
