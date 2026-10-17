use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    // 待ち受けるポート番号。環境変数 PORT があればそれを、無ければ 3000 を使う
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))?;
    println!("http://127.0.0.1:{port} で待っています（Ctrl+C で終了）");

    // 接続が来るたびに1つずつ処理する
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut reader = BufReader::new(&stream);

        // 要求の1行目とヘッダーを、空行が来るまで1行ずつ読む
        let mut lines = Vec::new();
        loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            let line = line.trim_end().to_string();
            if line.is_empty() {
                break;
            }
            lines.push(line);
        }
        println!("--- 届いた要求 ---");
        for line in &lines {
            println!("{line}");
        }

        // 応答を「文字列」として組み立てて送り返す
        let body = "こんにちは、HTTP！\n";
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            body.chars().count(),
            body
        );
        stream.write_all(response.as_bytes())?;
    }
    Ok(())
}
