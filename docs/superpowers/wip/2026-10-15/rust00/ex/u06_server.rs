use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    // 自分の PC（127.0.0.1）の 8080 番ポートで待ち受けを始める
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("http://127.0.0.1:8080 で待っています（止めるには Ctrl+C）");

    // 接続が来るたびに、1 つずつ順番に処理する
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut reader = BufReader::new(&stream);

        // リクエストの 1 行目（例: GET / HTTP/1.1）を読む
        let mut first_line = String::new();
        reader.read_line(&mut first_line)?;
        println!("受け取った: {}", first_line.trim_end());

        // 残りのヘッダー行を、空行が来るまで読み飛ばす
        loop {
            let mut line = String::new();
            let n = reader.read_line(&mut line)?;
            if n == 0 || line == "\r\n" {
                break;
            }
        }

        // 返事（レスポンス）を HTTP の決まった形で組み立てて送る
        let body = "<h1>Hello from Rust!</h1>\n";
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(response.as_bytes())?;
    }
    Ok(())
}
