struct Note {
    title: String,
}

impl Note {
    // タイトルが 10 文字を超えるなら true
    fn is_long(&self) -> bool {
        self.title.chars().count() > 10
    }
}

fn main() {
    let short = Note {
        title: String::from("牛乳を買う"),
    };
    let long = Note {
        title: String::from("週末にまとめて片付ける部屋の大掃除"),
    };
    println!("{}: {}", short.title, short.is_long());
    println!("{}: {}", long.title, long.is_long());
}
