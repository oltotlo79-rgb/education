//! app_context で借りる型を、ルーターに登録し忘れた
use topcoat::{
    Result,
    context::{Cx, app_context},
    router::{module_router, page},
    view::{View, view},
};

struct AppConfig {
    site_name: String,
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let config = app_context::<AppConfig>(cx);
    Ok(view! { <h1>(config.site_name.as_str())</h1> })
}

#[tokio::main]
async fn main() {
    // .app_context(AppConfig { ... }) を書き忘れた
    let router = module_router!().build();
    topcoat::start(router).await.expect("起動できません");
}
