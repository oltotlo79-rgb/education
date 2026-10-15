// private な module の中の型を、その道のりで使おうとする
use notes_app::NoteTitle;
use notes_app::repo::NoteStore;

fn main() {
    let mut store = NoteStore::default();
    let title = NoteTitle::parse("牛乳を買う").unwrap();
    let note = store.add(1, title);
    println!("{}", note.id);
}
