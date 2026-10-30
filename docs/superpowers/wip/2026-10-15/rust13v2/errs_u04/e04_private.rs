//! 第13巻 UNIT 04 エラー1: 検査を通らずに、NoteId を直接作ろうとした。
//! 実行: cargo run -q --example e04_private

use rustnote::domain::NoteId;

fn main() {
    let id = NoteId(0);
    println!("{id}");
}
