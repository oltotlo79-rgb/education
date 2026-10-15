// main.rs は、同じ package のライブラリ（notes_app）を使う「別の crate」
use notes_app::domain::user::count_owned;
use notes_app::{NoteStore, NoteTitle};

fn main() {
    let mut store = NoteStore::default();
    for (owner, title) in [(1, "牛乳を買う"), (2, "本を返す"), (1, "電気代を払う")] {
        let title = NoteTitle::parse(title).expect("正しいタイトル");
        let note = store.add(owner, title);
        println!("追加: #{} {}", note.id, note.title.as_str());
    }
    println!("ユーザー1のノート数: {}", count_owned(store.all(), 1));
    println!("#1 の持ち主: {}", store.all()[0].owner_id());
}
