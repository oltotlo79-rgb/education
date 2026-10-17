struct Note {
    id: u64,
    title: String,
    done: bool,
}

impl Note {
    fn summary(&self) -> String {
        let mark = if self.done { "[x]" } else { "[ ]" };
        format!("{mark} #{} {}", self.id, self.title)
    }
}

// ノートの一覧と「次に振る番号」をまとめて持つ構造体
struct NoteList {
    notes: Vec<Note>,
    next_id: u64,
}

impl NoteList {
    fn new() -> Self {
        Self {
            notes: Vec::new(),
            next_id: 1,
        }
    }

    // ノートを足して、振った ID を返す
    fn add(&mut self, title: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.notes.push(Note {
            id,
            title: title.to_string(),
            done: false,
        });
        id
    }

    // ID が見つかれば完了を切り替えて true、見つからなければ false
    fn toggle(&mut self, id: u64) -> bool {
        for note in &mut self.notes {
            if note.id == id {
                note.done = !note.done;
                return true;
            }
        }
        false
    }

    fn done_count(&self) -> usize {
        let mut count = 0;
        for note in &self.notes {
            if note.done {
                count += 1;
            }
        }
        count
    }

    fn print(&self) {
        for note in &self.notes {
            println!("{}", note.summary());
        }
    }
}

fn main() {
    let mut list = NoteList::new();
    let milk = list.add("牛乳を買う");
    list.add("Rust の勉強");
    list.toggle(milk);
    println!("ID 99 の切り替え: {}", list.toggle(99));
    list.print();
    println!("完了: {} / {}", list.done_count(), list.notes.len());
}
