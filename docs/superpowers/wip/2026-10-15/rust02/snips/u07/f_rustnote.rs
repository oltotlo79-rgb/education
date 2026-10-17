#[derive(Debug, Clone, Copy, PartialEq)]
struct UserId(i64);

#[derive(Debug, Clone, Copy, PartialEq)]
struct NoteId(i64);

#[derive(Debug)]
struct Note {
    id: NoteId,
    user_id: UserId,
    title: String,
    body: String,
    done: bool,
}

impl Note {
    fn new(id: NoteId, user_id: UserId, title: &str, body: &str) -> Self {
        Self {
            id,
            user_id,
            title: title.trim().to_string(),
            body: body.to_string(),
            done: false,
        }
    }

    // 一覧に出す1行
    fn headline(&self) -> String {
        let mark = if self.done { "[x]" } else { "[ ]" };
        let chars = self.body.chars().count();
        format!("{mark} #{} {}（本文 {chars} 文字）", self.id.0, self.title)
    }

    // POST /notes/{id}/toggle で使う: 完了と未完了を切り替える
    fn toggle_done(&mut self) {
        self.done = !self.done;
    }

    // 持ち主かどうか（他人のノートは見せない）
    fn is_owned_by(&self, user: UserId) -> bool {
        self.user_id == user
    }
}

fn main() {
    let alice = UserId(1);
    let bob = UserId(2);
    let mut note = Note::new(NoteId(10), alice, " 牛乳を買う ", "低脂肪のもの");
    println!("{}", note.headline());
    note.toggle_done();
    println!("{}", note.headline());
    println!("alice のノート? {}", note.is_owned_by(alice));
    println!("bob のノート? {}", note.is_owned_by(bob));
    println!("{note:#?}");
}
