use notes_app::{NoteStore, NoteTitle};

fn main() {
    let mut store = NoteStore::default();
    let title = NoteTitle::parse("牛乳を買う").unwrap();
    let note = store.add(1, title);
    // pub(crate) のフィールドを、crate の外（main.rs）から読もうとする
    println!("{}", note.owner_id);
}
