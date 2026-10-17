struct NoteId(i64);
struct UserId(i64);

fn find_note(id: NoteId) -> i64 {
    id.0
}

fn main() {
    let user_id = UserId(7);
    println!("{}", find_note(user_id));
}
