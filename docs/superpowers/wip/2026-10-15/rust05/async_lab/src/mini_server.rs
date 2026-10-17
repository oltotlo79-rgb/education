//! 練習用の、とても小さな HTTP サーバー。
//!
//! 自分のパソコンの中（127.0.0.1）だけで動き、インターネットには出ない。
//! 「決まった答えを返す」だけの最小限のもので、本物の Web アプリは第6巻の Topcoat で作る。

use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// 受け取ったリクエスト（この練習で使う部分だけ）
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Request {
    /// ヘッダーを名前で探す（大文字・小文字は区別しない）
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }
}

/// 返す答え
pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    pub fn new(status: u16, body: impl Into<String>) -> Response {
        Response {
            status,
            body: body.into(),
        }
    }
}

/// 127.0.0.1 の空いているポートでサーバーを起動し、そのアドレスを返す。
/// `handler` はリクエストが1つ来るたびに呼ばれ、答え（Response）を返す。
pub async fn start<F, Fut>(handler: F) -> SocketAddr
where
    F: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    // ポート 0 を指定すると、OS が空いているポートを選んでくれる
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("ポートを開けません");
    let address = listener.local_addr().expect("アドレスを読めません");
    let handler = Arc::new(handler);

    // 接続を受け付け続けるタスク
    tokio::spawn(async move {
        loop {
            let Ok((stream, _)) = listener.accept().await else {
                continue;
            };
            let handler = Arc::clone(&handler);
            // 1つの接続を1つのタスクで処理する
            tokio::spawn(async move {
                serve_one(stream, handler.as_ref()).await;
            });
        }
    });
    address
}

/// 1つの接続から1つのリクエストを読み、答えを書いて閉じる
async fn serve_one<F, Fut>(mut stream: TcpStream, handler: &F)
where
    F: Fn(Request) -> Fut,
    Fut: Future<Output = Response>,
{
    let Some(request) = read_request(&mut stream).await else {
        return;
    };
    let response = handler(request).await;
    let reason = match response.status {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        404 => "Not Found",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "Unknown",
    };
    let text = format!(
        "HTTP/1.1 {} {reason}\r\ncontent-type: text/plain; charset=utf-8\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
        response.status,
        response.body.len(),
        response.body
    );
    // 相手がもう待っていない（タイムアウトで切られた）こともあるので、失敗は無視する
    let _ = stream.write_all(text.as_bytes()).await;
}

/// ヘッダーの終わり（空行）まで読み、Content-Length の分だけ本文を読む
async fn read_request(stream: &mut TcpStream) -> Option<Request> {
    let mut buffer = Vec::new();
    let mut chunk = [0u8; 1024];
    let header_end = loop {
        let n = stream.read(&mut chunk).await.ok()?;
        if n == 0 {
            return None;
        }
        buffer.extend_from_slice(&chunk[..n]);
        if let Some(pos) = buffer.windows(4).position(|w| w == b"\r\n\r\n") {
            break pos;
        }
    };
    let head = String::from_utf8_lossy(&buffer[..header_end]).to_string();
    let mut lines = head.split("\r\n");
    let mut first = lines.next()?.split(' ');
    let method = first.next()?.to_string();
    let path = first.next()?.to_string();
    let headers: Vec<(String, String)> = lines
        .filter_map(|line| line.split_once(':'))
        .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
        .collect();
    let length: usize = headers
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case("content-length"))
        .and_then(|(_, value)| value.parse().ok())
        .unwrap_or(0);
    let mut body = buffer[header_end + 4..].to_vec();
    while body.len() < length {
        let n = stream.read(&mut chunk).await.ok()?;
        if n == 0 {
            break;
        }
        body.extend_from_slice(&chunk[..n]);
    }
    Some(Request {
        method,
        path,
        headers,
        body: String::from_utf8_lossy(&body).to_string(),
    })
}
