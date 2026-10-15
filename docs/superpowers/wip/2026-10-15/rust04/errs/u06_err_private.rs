mod app {
    pub mod domain {
        pub struct Note {
            pub title: String,
            version: i64,
        }

        impl Note {
            pub fn new(title: &str) -> Note {
                Note {
                    title: title.to_string(),
                    version: 1,
                }
            }
        }

        fn next_version(v: i64) -> i64 {
            v + 1
        }
    }
}

use app::domain::Note;

fn main() {
    let note = Note::new("本を返す");
    // private なフィールドを読む
    println!("{}", note.version);
    // private な関数を呼ぶ
    println!("{}", app::domain::next_version(1));
}
