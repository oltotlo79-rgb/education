use serde::Serialize;

// Serialize を derive していない型
#[derive(Debug)]
struct NoteTitle(String);

#[derive(Serialize)]
struct NoteView {
    id: i64,
    title: NoteTitle,
}

fn main() {
    let view = NoteView {
        id: 1,
        title: NoteTitle("牛乳を買う".to_string()),
    };
    println!("{}", serde_json::to_string(&view).unwrap());
}
