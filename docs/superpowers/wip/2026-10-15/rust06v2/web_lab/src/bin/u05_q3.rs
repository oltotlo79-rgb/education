//! 問3の解答: 自分の型（NoteId）に NodeViewParts を実装して、view! に (id) と書けるようにする。
//! 実行: cargo run --bin u05_q3

use topcoat::{
    Result,
    context::Cx,
    router::{module_router, page},
    view::{NodeViewParts, PartsWriter, View, view},
};

/// ノートの ID
struct NoteId(i64);

// view! の ( ) に、子の部品として置けるようにする
impl NodeViewParts for NoteId {
    fn into_view_parts(self, _cx: &Cx, parts: &mut PartsWriter<'_>) {
        // push_string で入れた文字は、置く場所に合わせて自動でエスケープされる
        parts.push_string(format!("No.{}", self.0));
    }
}

#[page]
async fn home() -> Result<impl View> {
    let id = NoteId(7);
    Ok(view! { <h1>"ノート " (id)</h1> })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
