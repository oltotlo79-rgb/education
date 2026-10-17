// タプル構造体: フィールドに名前がなく、.0 で取り出す
// 中身は同じ i64 でも型が別なので、取り違えをコンパイラが見つけてくれる
#[derive(Debug, Clone, Copy, PartialEq)]
struct NoteId(i64);

#[derive(Debug, Clone, Copy, PartialEq)]
struct UserId(i64);

fn find_note(id: NoteId) -> String {
    format!("ノート {} を探します", id.0)
}

fn main() {
    let note_id = NoteId(7);
    let user_id = UserId(7);
    println!("{}", find_note(note_id));
    println!("{note_id:?} と {user_id:?}");
    // PartialEq を derive したので == で比べられる
    println!("同じ? {}", note_id == NoteId(7));
}
