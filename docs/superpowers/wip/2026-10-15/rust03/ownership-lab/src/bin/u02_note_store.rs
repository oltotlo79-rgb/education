// 小さな ID は Copy にしておくと、何度でも気軽に渡せる
#[derive(Debug, Clone, Copy, PartialEq)]
struct NoteId(i64);

// フォームから届いた「これから保存するノート」
#[derive(Debug)]
struct NewNote {
    title: String,
    body: String,
}

// 保存済みのノート
#[derive(Clone)]
struct Note {
    id: NoteId,
    title: String,
    body: String,
    done: bool,
}

impl Note {
    // 中身を1行の文字列にまとめる（&self は「借りて読むだけ」の意味。UNIT 03 で詳しく）
    fn summary(&self) -> String {
        format!(
            "#{} {} / {} / done={}",
            self.id.0, self.title, self.body, self.done
        )
    }
}

struct NoteStore {
    notes: Vec<Note>,
    next_id: i64,
}

impl NoteStore {
    fn new() -> NoteStore {
        NoteStore {
            notes: Vec::new(),
            next_id: 1,
        }
    }

    // NewNote を丸ごと受け取る（所有権を引き取る）
    fn create(&mut self, new_note: NewNote) -> NoteId {
        let id = NoteId(self.next_id);
        self.next_id += 1;
        // new_note の中の String を、コピーせずにそのまま Note へ移す
        let note = Note {
            id,
            title: new_note.title,
            body: new_note.body,
            done: false,
        };
        self.notes.push(note);
        id
    }
}

fn main() {
    let mut store = NoteStore::new();
    let draft = NewNote {
        title: String::from("買い物"),
        body: String::from("牛乳と卵"),
    };
    let id = store.create(draft);

    // NoteId は Copy なので、代入しても id は使い続けられる
    let same = id;
    println!("{id:?} と {same:?} は同じ? {}", id == same);

    // 一覧を別の場所へ渡したいときは、費用を承知で clone する
    let snapshot = store.notes.clone();
    store.notes[0].done = true;
    println!("store:    {}", store.notes[0].summary());
    println!("snapshot: {}", snapshot[0].summary());
}
