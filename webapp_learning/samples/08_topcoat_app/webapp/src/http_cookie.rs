//! HTTP（暗号化なし）の社内環境で使う、セッションのクッキーの運び方。
//!
//! Topcoat 0.11 の既定（`CookieTokenStore`）は、クッキーに `__Host-` の名前と
//! `Secure` を必ず付ける。ブラウザは HTTP のページでは `Secure` のクッキーを
//! 受け取らないので、HTTP ではログインが続かない。
//! ここでは `Secure` と `__Host-` だけを外し、`HttpOnly`・`SameSite=Lax`・`Path=/` は残す。
//! HTTPS にしたら、このファイルは使わず、決まりのクッキーに戻す（期限の 60 分は残す）。

use std::time::Duration;

use topcoat::{
    context::Cx,
    cookie::{Cookie, Cookies, SameSite, time},
    session::{Token, TokenStore, TokenStoreFuture},
};

/// セッションのクッキーの名前。
const NAME: &str = "session";

/// HTTP 用のセッションのクッキー（`Secure` なし）。
pub struct HttpCookieTokenStore;

fn cookies(cx: &Cx) -> impl Cookies {
    topcoat::cookie::cookies(cx)
        .override_same_site(SameSite::Lax)
        .override_http_only(true)
        .override_secure(false)
        .override_path("/")
}

impl TokenStore for HttpCookieTokenStore {
    fn read<'a>(&'a self, cx: &'a Cx) -> TokenStoreFuture<'a, Option<Token>> {
        Box::pin(async move {
            let Some(cookie) = cookies(cx).get(NAME) else {
                return Ok(None);
            };
            Ok(Token::decode(cookie.value_trimmed()).ok())
        })
    }

    fn write<'a>(
        &'a self,
        cx: &'a Cx,
        token: Token,
        max_age: Duration,
    ) -> TokenStoreFuture<'a, ()> {
        Box::pin(async move {
            let max_age = time::Duration::try_from(max_age)?;
            cookies(cx)
                .override_max_age(max_age)
                .add(Cookie::new(NAME, token.encode()));
            Ok(())
        })
    }

    fn delete<'a>(&'a self, cx: &'a Cx) -> TokenStoreFuture<'a, ()> {
        Box::pin(async move {
            cookies(cx).remove(Cookie::new(NAME, ""));
            Ok(())
        })
    }
}
