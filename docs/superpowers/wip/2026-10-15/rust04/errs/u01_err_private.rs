mod domain {
    #[derive(Debug)]
    pub struct NoteTitle(String);

    impl NoteTitle {
        pub fn parse(input: &str) -> Option<NoteTitle> {
            let t = input.trim();
            if t.is_empty() {
                None
            } else {
                Some(NoteTitle(t.to_string()))
            }
        }
    }
}

fn main() {
    // parse を通さずに、空のタイトルを直接作ろうとする
    let title = domain::NoteTitle(String::new());
    println!("{title:?}");
}
