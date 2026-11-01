//! /attachments/{id} … 添付ファイルのダウンロード（その依頼を見てよい人だけ）

use topcoat::{
    Result,
    context::Cx,
    router::{error::RouterErrorExt, module_param, path_param, response::Response, route},
};

use crate::{app::requests::id::load_visible, auth::require_user, files, queries, state::state};

module_param!(pub id: i64, error = not_found);

#[route(GET)]
pub async fn download(cx: &Cx) -> Result<Response> {
    let user = require_user(cx).await?;
    let id = *path_param::<Id>(cx)?;
    let st = state(cx);
    let a = queries::find_attachment(&st.pool, id).await?.ok_or_not_found()?;
    load_visible(cx, &user, a.request_id).await?; // 依頼を見られない人には、404
    files::download(&st.config.file_dir, &a).await
}
