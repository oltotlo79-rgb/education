#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UserId(i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NoteId(i64);

fn find_note(owner: UserId, id: NoteId) -> String {
    format!("ユーザー{}のノート{}を探します", owner.0, id.0)
}

fn main() {
    let owner = UserId(7);
    let note = NoteId(42);
    // わざと順番を取り違える
    println!("{}", find_note(note, owner));
}
