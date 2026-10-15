use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // example.com の 80 番ポート（http の窓口）に接続する
    let mut stream = TcpStream::connect("example.com:80")?;

    // HTTP リクエストは「決まった形の文字」。1 行ずつ \r\n で区切り、空行で終わる
    let request = "GET / HTTP/1.1\r\n\
                   Host: example.com\r\n\
                   User-Agent: rust-study\r\n\
                   Connection: close\r\n\
                   \r\n";
    stream.write_all(request.as_bytes())?;

    // サーバーからの返事（レスポンス）を最後まで受け取る
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // 空行（\r\n\r\n）の前がヘッダー、後ろが本文
    let (head, body) = response.split_once("\r\n\r\n").unwrap_or((&response, ""));
    println!("----- ヘッダー部分 -----");
    println!("{head}");
    println!("----- 本文の先頭 60 文字 -----");
    let preview: String = body.chars().take(60).collect();
    println!("{preview}");
    Ok(())
}
