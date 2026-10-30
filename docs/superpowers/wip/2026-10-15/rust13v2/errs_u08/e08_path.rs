//! 第13巻 UNIT 08 エラー2: URL の {id} を、Id ではなく NoteId の型で読もうとした。
//! 実行: cargo run -q --example e08_path
#![allow(dead_code)]

use rustnote::domain::NoteId;
use topcoat::{Result, context::Cx, router::path_param};

fn current_note_id(cx: &Cx) -> Result<NoteId> {
    let id = *path_param::<NoteId>(cx)?;
    Ok(id)
}

fn main() {}
