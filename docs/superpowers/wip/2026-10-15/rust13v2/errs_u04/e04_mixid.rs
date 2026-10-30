//! 第13巻 UNIT 04 エラー2: ユーザーの ID を、ノートの ID の代わりに渡した。
//! 実行: cargo run -q --example e04_mixid

use rustnote::domain::{NoteId, UserId};

fn show_note(id: NoteId) {
    println!("ノート {id} を表示します");
}

fn main() {
    let user_id = UserId::new(7).unwrap();
    show_note(user_id);
}
