struct Note {
    id: u64,
    title: String,
    done: bool,
}

fn main() {
    let note = Note {
        id: 1,
        title: String::from("牛乳を買う"),
    };
    println!("{} {} {}", note.id, note.title, note.done);
}
