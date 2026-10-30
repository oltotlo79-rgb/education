//! 第13巻 UNIT 04 問2: NoteQuery の項目は pub なので、new を通らずに page: 0 を作れる。
//! 実行: cargo run -q --example u04_q2（--release を付けた場合とも比べる）

use rustnote::domain::NoteQuery;

fn main() {
    let made_by_new = NoteQuery::new(None, Some(0));
    println!(
        "new で作った: page {}・offset {}",
        made_by_new.page,
        made_by_new.offset()
    );

    let made_directly = NoteQuery {
        keyword: None,
        page: 0,
    };
    println!("直接作った: page {}", made_directly.page);
    println!("直接作った: offset {}", made_directly.offset());
}
