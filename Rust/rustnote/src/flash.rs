//! フラッシュメッセージ: 「ノートを作成しました」のように、次の画面で一度だけ出すお知らせ。
//!
//! しくみ:
//! 1. フォームの処理が成功したら `set` でクッキー `flash` にメッセージの種類（コード）を入れ、
//!    303 で次の画面へリダイレクトする
//! 2. 次の画面（GET）の処理の最初で `take` を呼ぶと、クッキーを読んで消す
//!
//! クッキーには決まったコード（例: note_created）しか入れない。利用者が値を書き換えても、
//! 知らないコードは無視されるだけで、任意の文字が画面に出ることはない。
//! クッキーの書き込みは「ページの処理が値を返す前」に行う必要がある（Topcoat の決まり）。

use topcoat::{
    context::Cx,
    cookie::{Cookie, Cookies, cookie, cookies, time::Duration},
};

/// クッキーの名前
const COOKIE_NAME: &str = "flash";

/// 出せるメッセージの種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flash {
    SignedUp,
    LoggedIn,
    LoggedOut,
    LoginRequired,
    NoteCreated,
    NoteUpdated,
    NoteToggled,
    NoteDeleted,
}

impl Flash {
    /// クッキーに入れる短いコード
    fn code(self) -> &'static str {
        match self {
            Flash::SignedUp => "signed_up",
            Flash::LoggedIn => "logged_in",
            Flash::LoggedOut => "logged_out",
            Flash::LoginRequired => "login_required",
            Flash::NoteCreated => "note_created",
            Flash::NoteUpdated => "note_updated",
            Flash::NoteToggled => "note_toggled",
            Flash::NoteDeleted => "note_deleted",
        }
    }

    /// コードから種類に戻す（知らないコードなら None）
    fn from_code(code: &str) -> Option<Flash> {
        let all = [
            Flash::SignedUp,
            Flash::LoggedIn,
            Flash::LoggedOut,
            Flash::LoginRequired,
            Flash::NoteCreated,
            Flash::NoteUpdated,
            Flash::NoteToggled,
            Flash::NoteDeleted,
        ];
        all.into_iter().find(|flash| flash.code() == code)
    }

    /// 画面に出す文章
    pub fn message(self) -> &'static str {
        match self {
            Flash::SignedUp => "登録しました。ようこそ RustNote へ！",
            Flash::LoggedIn => "ログインしました",
            Flash::LoggedOut => "ログアウトしました",
            Flash::LoginRequired => "ログインしてください",
            Flash::NoteCreated => "ノートを作成しました",
            Flash::NoteUpdated => "ノートを保存しました",
            Flash::NoteToggled => "完了の状態を切り替えました",
            Flash::NoteDeleted => "ノートを削除しました",
        }
    }

    /// 見た目の種類（CSS のクラス名に使う）
    pub fn kind(self) -> &'static str {
        match self {
            Flash::LoginRequired => "warning",
            _ => "success",
        }
    }
}

/// 次の画面で出すメッセージを予約する
pub fn set(cx: &Cx, flash: Flash) {
    cookies(cx).add(cookie! {
        "flash" = flash.code();
        Path = "/";
        HttpOnly;
        SameSite = Lax;
        MaxAge = Duration::minutes(5)
    });
}

/// 予約されたメッセージを取り出し、クッキーを消す（一度だけ表示するため）
pub fn take(cx: &Cx) -> Option<Flash> {
    let jar = cookies(cx);
    let code = jar.get(COOKIE_NAME)?.value().to_string();
    // 消すときは、書いたときと同じ Path を指定する
    jar.remove(Cookie::build((COOKIE_NAME, "")).path("/").build());
    Flash::from_code(&code)
}

#[cfg(test)]
mod tests {
    use super::Flash;

    #[test]
    fn codes_round_trip() {
        for flash in [Flash::SignedUp, Flash::NoteDeleted, Flash::LoginRequired] {
            assert_eq!(Flash::from_code(flash.code()), Some(flash));
        }
        assert_eq!(Flash::from_code("<script>"), None);
    }
}
