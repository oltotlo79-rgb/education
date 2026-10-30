//! 第6巻 UNIT 04 の例4-2: URL の {id} を、自分で作った型（1 以上の数だけ）で読む。
//! 実行: cargo run --bin u04_noteid

use std::fmt;
use std::str::FromStr;

use topcoat::{
    Result,
    context::Cx,
    router::{module_router, page, path_param},
    view::{View, view},
};

/// ノートの ID。0 や負の数は作れない
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoteId(i64);

/// ID を作れなかった理由
#[derive(Debug)]
pub enum IdError {
    NotANumber,
    NotPositive,
}

// "42".parse::<NoteId>() と書けるようにする。module_param! もこれを使う
impl FromStr for NoteId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: i64 = s.parse().map_err(|_| IdError::NotANumber)?;
        if value <= 0 {
            return Err(IdError::NotPositive);
        }
        Ok(NoteId(value))
    }
}

// 画面に出すときと、href! で URL を作るときに使う
impl fmt::Display for NoteId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

mod notes {
    mod id {
        use super::super::*;
        use topcoat::router::module_param;

        // 区切りの文字を NoteId::from_str で読む。読めなければ 404
        module_param!(id: NoteId, error = not_found);

        /// GET /notes/{id}
        #[page]
        async fn show(cx: &Cx) -> Result<impl View> {
            let id: &NoteId = path_param::<Id>(cx)?;
            // view! に入れられるのは文字や数などなので、to_string() で文字にする
            Ok(view! { <h1>"ノート " (id.to_string()) " 番"</h1> })
        }
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
