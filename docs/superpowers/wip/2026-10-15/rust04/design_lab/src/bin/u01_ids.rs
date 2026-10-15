// 素の i64 のままだと、どちらが何の番号か型からは分からない
fn find_note_raw(owner_id: i64, note_id: i64) -> String {
    format!("ユーザー{owner_id}のノート{note_id}を探します")
}

// 意味ごとに別の型を作る（newtype パターン）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UserId(i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NoteId(i64);

// 引数の型そのものが「何の番号か」を表している
fn find_note(owner: UserId, id: NoteId) -> String {
    format!("ユーザー{}のノート{}を探します", owner.0, id.0)
}

fn main() {
    let owner_id: i64 = 7;
    let note_id: i64 = 42;
    // 正しい順番
    println!("{}", find_note_raw(owner_id, note_id));
    // 順番を取り違えても、コンパイルは通ってしまう
    println!("{}", find_note_raw(note_id, owner_id));

    let owner = UserId(7);
    let note = NoteId(42);
    // newtype なら、正しい順番でしか呼べない
    println!("{}", find_note(owner, note));
    // Debug 表示では型の名前も出る
    println!("{owner:?} / {note:?}");
}
