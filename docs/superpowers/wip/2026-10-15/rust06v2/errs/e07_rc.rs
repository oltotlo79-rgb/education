//! スレッドをまたいで共有できない Rc を、app_context に入れようとした
use std::rc::Rc;

use topcoat::router::module_router;

struct AppConfig {
    site_name: Rc<String>,
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        .app_context(AppConfig { site_name: Rc::new("メモ帳".to_string()) })
        .build();
    topcoat::start(router).await.expect("起動できません");
}
