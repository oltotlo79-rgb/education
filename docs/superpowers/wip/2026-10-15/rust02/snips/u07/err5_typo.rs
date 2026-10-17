struct Note {
    title: String,
}

impl Note {
    fn summary(&self) -> String {
        format!("ノート: {}", self.title)
    }
}

fn main() {
    let note = Note {
        title: String::from("牛乳を買う"),
    };
    println!("{}", note.summery());
}
