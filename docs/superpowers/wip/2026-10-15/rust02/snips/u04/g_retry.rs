const MAX_ATTEMPTS: u32 = 5;

// サーバーに問い合わせた結果のステータスコードを返す（練習用のにせ物）
fn fake_request(attempt: u32) -> u16 {
    match attempt {
        1 | 2 => 503,
        3 => 200,
        _ => 500,
    }
}

fn main() {
    let mut attempt = 0;
    let outcome = loop {
        attempt += 1;
        let status = fake_request(attempt);
        match status {
            // 2xx なら成功。結果の文を持って loop を抜ける
            200..=299 => break format!("{attempt} 回目で成功（{status}）"),
            // 503 は一時的な失敗。回数が残っていれば、もう一度くり返す
            503 if attempt < MAX_ATTEMPTS => {
                println!("{attempt} 回目: {status} のため再試行します");
                continue;
            }
            // それ以外はあきらめる
            _ => break format!("{attempt} 回目で中止（{status}）"),
        }
    };
    println!("{outcome}");
}
