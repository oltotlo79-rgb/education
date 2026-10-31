mod app;
mod features;

#[tokio::main]
async fn main() {
    topcoat::start(app::router()).await.unwrap();
}
