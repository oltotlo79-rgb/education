//! 同じ型の値を、2つ登録しようとした（本番の DB と、控えの DB）
use topcoat::router::module_router;

struct Database {
    url: String,
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        .app_context(Database { url: "file:main.db".to_string() })
        .app_context(Database { url: "file:replica.db".to_string() })
        .build();
    topcoat::start(router).await.expect("起動できません");
}
