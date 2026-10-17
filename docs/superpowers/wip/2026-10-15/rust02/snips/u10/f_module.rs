// mod で、関係するものを1つの部屋（モジュール）にまとめる
mod note {
    // pub を付けたものだけが、部屋の外から使える
    pub struct NoteTitle(String);

    impl NoteTitle {
        pub fn parse(raw: &str) -> Option<Self> {
            let title = raw.trim();
            if is_valid_length(title) {
                Some(Self(title.to_string()))
            } else {
                None
            }
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    // pub がないので、部屋の外からは呼べない（部屋の中だけの道具）
    fn is_valid_length(title: &str) -> bool {
        (1..=80).contains(&title.chars().count())
    }
}

// use で、長い住所（note::NoteTitle）を短い名前で書けるようにする
use note::NoteTitle;

fn main() {
    match NoteTitle::parse("  牛乳を買う ") {
        Some(title) => println!("OK: {}", title.as_str()),
        None => println!("NG"),
    }
    println!("空のタイトルは作れない: {}", NoteTitle::parse("").is_none());
}
