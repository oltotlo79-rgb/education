struct Note {
    title: String,
}

fn main() {
    let note = Note {
        title: String::from("牛乳を買う"),
    };
    println!("{note:?}");
}
