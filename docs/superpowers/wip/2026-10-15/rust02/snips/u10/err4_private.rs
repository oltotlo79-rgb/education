mod note {
    pub struct NoteTitle(String);

    impl NoteTitle {
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }
}

fn main() {
    // parse を通さずに、空のタイトルを直接作ろうとする
    let title = note::NoteTitle(String::new());
    println!("{}", title.as_str());
}
