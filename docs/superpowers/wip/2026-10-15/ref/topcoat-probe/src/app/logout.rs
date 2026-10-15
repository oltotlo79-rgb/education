//! POST /logout : clears the cookie and deletes the server-side record.

use topcoat::{
    Result,
    context::Cx,
    router::{
        error::{SeeOther, see_other},
        href, route,
    },
    session,
};

use crate::app::db;

#[route(POST)]
pub(crate) async fn logout(cx: &Cx) -> Result<SeeOther> {
    if let Some(hash) = session::stop(cx).await? {
        db(cx).delete_session(&hash);
    }
    Ok(see_other(href!(crate::app::home).resolve(cx)))
}
