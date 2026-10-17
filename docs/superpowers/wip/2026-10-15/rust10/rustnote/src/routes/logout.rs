//! POST /logout : セッションを終わらせて、303 でログイン画面へ。
//!
//! 状態を変える操作なので GET ではなく POST にする（Topcoat の OriginPolicy が
//! 他のサイトからの POST を 403 で止めてくれる）。

use topcoat::{
    Result,
    context::Cx,
    router::{error::see_other, href, page},
};

use crate::auth::log_out;
use crate::flash::{self, Flash};

// レイアウト（エラー画面）を通すため #[page(POST)] にしている。成功すると必ず 303 なので、
// 画面として返す中身は無い（`()`）。
#[page(POST)]
pub async fn logout(cx: &Cx) -> Result<()> {
    log_out(cx).await?;
    flash::set(cx, Flash::LoggedOut);
    Err(see_other(href!(crate::routes::login::login_page).resolve(cx)).into())
}
