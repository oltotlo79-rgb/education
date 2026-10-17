use topcoat::{
    Result,
    router::{Router, RouterBuilderDiscoverExt, page, route},
    view::{View, view},
};

#[tokio::main]
async fn main() {
    // このファイルに書いた #[page] と #[route] を集めて、ルーター（URL の案内係）を作る
    let router = Router::builder().discover().build();

    // topcoat::start と同じ規則で、待ち受ける場所を表示する（既定は 127.0.0.1:3000）
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    eprintln!("起動しました: http://{host}:{port}");

    // HOST と PORT を読んで待ち受ける。Ctrl+C で止まる
    if let Err(error) = topcoat::start(router).await {
        eprintln!("サーバーを起動できませんでした: {error}");
        std::process::exit(1);
    }
}

// GET / で表示するページ
#[page("/")]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>"Topcoat の確認"</title>
            </head>
            <body>
                <h1>"Hello, Topcoat!"</h1>
                <p>"環境構築ができました。"</p>
            </body>
        </html>
    })
}

// GET /health で、動いているかどうかを文字で返す
#[route(GET "/health")]
async fn health() -> Result<&'static str> {
    Ok("ok")
}
