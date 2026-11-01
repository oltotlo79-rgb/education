//! /logout … ログアウト（POST だけ。GET で消えると、リンクを踏ませるだけでログアウトさせられるため）

use topcoat::{
    Result,
    context::Cx,
    router::{
        error::{SeeOther, see_other},
        href, route,
    },
};

use crate::auth;

#[route(POST)]
pub async fn logout(cx: &Cx) -> Result<SeeOther> {
    auth::logout(cx).await?;
    Ok(see_other(href!(crate::app::login::login_page).resolve(cx)))
}
