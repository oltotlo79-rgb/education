//! Binary entry point. `topcoat::start` binds to $HOST:$PORT (default 127.0.0.1:3000).
//! On Fly.io set HOST=0.0.0.0 and PORT=8080 (e.g. in fly.toml [env]).

#[tokio::main]
async fn main() {
    topcoat::start(topcoat_probe::app::router()).await.unwrap();
}
