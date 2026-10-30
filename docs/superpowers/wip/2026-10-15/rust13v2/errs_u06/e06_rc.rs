//! 第13巻 UNIT 06 エラー3: 共有の状態に、Arc ではなく Rc を使った。
//! 実行: cargo run -q --example e06_rc

use std::rc::Rc;

use topcoat::context::{Cx, app_context};

fn visit_count(cx: &Cx) -> &Rc<u32> {
    app_context::<Rc<u32>>(cx)
}

fn main() {
    let _ = visit_count;
}
