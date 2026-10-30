//! .sessions(…) は書いたが、.cookies() を書き忘れた

use std::collections::HashMap;
use std::sync::Mutex;

use serde::Deserialize;
use topcoat::{
    Result,
    context::{Cx, app_context},
    cookie::{Cookie, Cookies, RouterBuilderCookieExt, cookies},
    router::{content::Form, error::see_other, module_router, page},
    session::{self, RouterBuilderSessionExt, SessionConfig, TokenHash},
    view::{View, view},
};

/// セッションの記録（トークンのハッシュ → 名前）。本物のアプリでは DB に保存する
struct Sessions(Mutex<HashMap<TokenHash, String>>);

/// 今の要求のセッションから、名前を探す
async fn current_name(cx: &Cx) -> Result<Option<String>> {
    let Some(hash) = session::token_hash(cx).await? else {
        return Ok(None);
    };
    Ok(app_context::<Sessions>(cx).0.lock().unwrap().get(&hash).cloned())
}

/// GET /: クッキー（テーマ）と、セッション（名前）を読む
#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let theme = cookies(cx)
        .get("theme")
        .map(|c| c.value().to_string())
        .unwrap_or_else(|| "light".to_string());
    let name = current_name(cx).await?;
    Ok(view! {
        <p>"テーマ: " (theme)</p>
        <p>"ログイン: " (name.unwrap_or_else(|| "（していない）".to_string()))</p>
    })
}

mod theme {
    use super::*;

    /// POST /theme: テーマのクッキーを書いて、トップへ
    #[page(POST)]
    async fn toggle(cx: &Cx) -> Result<()> {
        let jar = cookies(cx);
        let next = match jar.get("theme") {
            Some(theme) if theme.value() == "dark" => "light",
            _ => "dark",
        };
        jar.add(Cookie::build(("theme", next)).path("/").build());
        Err(see_other("/").into())
    }
}

mod login {
    use super::*;

    #[derive(Deserialize)]
    struct LoginForm {
        name: String,
    }

    /// POST /login: 新しいセッションを始めて、ハッシュと名前を記録する
    #[page(POST)]
    async fn login(cx: &Cx, Form(input): Form<LoginForm>) -> Result<()> {
        let started = session::start(cx).await?;
        app_context::<Sessions>(cx)
            .0
            .lock()
            .unwrap()
            .insert(started.token_hash, input.name);
        Err(see_other("/").into())
    }
}

mod logout {
    use super::*;

    /// POST /logout: セッションを終えて、記録も消す
    #[page(POST)]
    async fn logout(cx: &Cx) -> Result<()> {
        if let Some(hash) = session::stop(cx).await? {
            app_context::<Sessions>(cx).0.lock().unwrap().remove(&hash);
        }
        Err(see_other("/").into())
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        .sessions(SessionConfig::default())
        .app_context(Sessions(Mutex::new(HashMap::new())))
        .build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
