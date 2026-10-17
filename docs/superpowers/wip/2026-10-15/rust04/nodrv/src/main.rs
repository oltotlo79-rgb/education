use serde::Serialize;

// derive 機能を付けずに serde を入れた状態で、derive しようとする
#[derive(Serialize)]
struct Note {
    title: String,
}

fn main() {
    let note = Note {
        title: "牛乳を買う".to_string(),
    };
    println!("{}", note.title);
}
