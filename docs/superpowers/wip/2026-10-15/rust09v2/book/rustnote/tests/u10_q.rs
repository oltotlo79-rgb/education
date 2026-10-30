//! 第9巻 UNIT 10 問題: DB に届かないとき、/health 以外の URL は何を返すか。
//! 実行: cargo test --test u10_q -- --nocapture

mod common;

use common::{encode, test_assets};
use rustnote::{config::Config, db::Db, routes, state::AppState};

#[tokio::test]
async fn pages_without_the_database() {
    let config = Config::new("http://127.0.0.1:9".to_string(), None).unwrap();
    let db = Db::open(&config).await.unwrap();
    let router = routes::build_router(AppState::new(db), test_assets());
    let app = common::TestApp {
        router,
        db: common::test_db().await,
    };
    for path in ["/health", "/login", "/notes"] {
        let status = app.get(path, None).await.status();
        println!("GET  {path:<8} → {status}");
    }
    let form = format!(
        "email={}&password={}",
        encode("alice@example.com"),
        encode("correct horse battery")
    );
    let status = app.post_form("/login", &form, None).await.status();
    println!("POST /login    → {status}");
}
