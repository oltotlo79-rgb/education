//! 第9巻 UNIT 10: GET /health が、DB に届くときは 200、届かないときは 503 を返すかを確かめる。
//! 実行: cargo test --test u10_health -- --nocapture

mod common;

use common::{body_text, test_app, test_assets};
use rustnote::{config::Config, db::Db, routes, state::AppState};
use topcoat::router::StatusCode;

#[tokio::test]
async fn health_reports_the_database() {
    // ふつうの DB（テスト用の一時ファイル）
    let app = test_app().await;
    let response = app.get("/health", None).await;
    let status = response.status();
    println!("ふつうの DB      → {status} {}", body_text(response).await);
    assert_eq!(status, StatusCode::OK);

    // 届かない DB（127.0.0.1 の 9 番ポートには、何も待ち受けていない）
    let config = Config::new("http://127.0.0.1:9".to_string(), None).unwrap();
    let db = Db::open(&config)
        .await
        .expect("開くだけなら、まだ接続しない");
    let router = routes::build_router(AppState::new(db), test_assets());
    let app = common::TestApp {
        router,
        db: common::test_db().await,
    };
    let response = app.get("/health", None).await;
    let status = response.status();
    println!("届かない DB      → {status} {}", body_text(response).await);
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
}
