use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("http://127.0.0.1:8080 で待っています（止めるには Ctrl+C）");

    for stream in listener.incoming() {
        handle(stream?)?;
    }
    Ok(())
}

// 1 つの接続（1 回のリクエスト）を処理する
fn handle(mut stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(&stream);

    // 1 行目「GET /health HTTP/1.1」を、空白で 3 つに分ける
    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");

    // 残りのヘッダー行は空行まで読み飛ばす
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        if n == 0 || line == "\r\n" {
            break;
        }
    }

    // メソッドとパスの組み合わせで、返す内容を決める（ルーティング）
    let (status, body) = match (method, path) {
        ("GET", "/") => ("200 OK", "<h1>トップページ</h1>\n"),
        ("GET", "/health") => ("200 OK", "ok\n"),
        ("GET", "/about") => ("200 OK", "<h1>このサーバーについて</h1>\n"),
        _ => ("404 Not Found", "<h1>ページが見つかりません</h1>\n"),
    };
    println!("{method} {path} -> {status}");

    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(response.as_bytes())?;
    Ok(())
}
