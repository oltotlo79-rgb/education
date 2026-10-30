//! 第6巻 UNIT 06 の例6-2: JSON を受け取り、JSON で返す API（GET と POST /api/memos）。
//! 実行: cargo run --bin u06_api

use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use topcoat::{
    Result,
    router::{StatusCode, content::Json, module_router},
};

/// 保存したメモ
#[derive(Clone, Serialize)]
struct Memo {
    id: u32,
    title: String,
}

static MEMOS: Mutex<Vec<Memo>> = Mutex::new(Vec::new());

mod api {
    pub mod memos {
        use super::super::*;
        use topcoat::router::route;

        /// 受け取る JSON（{"title": "..."}）
        #[derive(Deserialize)]
        pub struct NewMemo {
            title: String,
        }

        /// GET /api/memos: 全部を JSON の配列で返す
        #[route(GET)]
        async fn list() -> Result<Json<Vec<Memo>>> {
            Ok(Json(MEMOS.lock().unwrap().clone()))
        }

        /// POST /api/memos: 1件足して、201 Created と、足したメモを返す
        #[route(POST)]
        async fn create(Json(input): Json<NewMemo>) -> Result<(StatusCode, Json<Memo>)> {
            let mut memos = MEMOS.lock().unwrap();
            let memo = Memo {
                id: memos.len() as u32 + 1,
                title: input.title,
            };
            memos.push(memo.clone());
            Ok((StatusCode::CREATED, Json(memo)))
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
