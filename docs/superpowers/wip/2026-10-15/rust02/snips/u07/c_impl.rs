struct Note {
    id: u64,
    title: String,
    done: bool,
}

// impl ブロックに、Note 専用の関数をまとめる
impl Note {
    // 関連関数: self を受け取らない。Note::new(...) と呼ぶ。値を「作る」窓口によく使う
    fn new(id: u64, title: &str) -> Self {
        Self {
            id,
            title: title.trim().to_string(),
            done: false,
        }
    }

    // &self: 読むだけのメソッド
    fn summary(&self) -> String {
        let mark = if self.done { "[x]" } else { "[ ]" };
        format!("{mark} #{} {}", self.id, self.title)
    }

    // &mut self: 中身を書き換えるメソッド
    fn toggle(&mut self) {
        self.done = !self.done;
    }

    // self: インスタンスを使い切って、中身を取り出すメソッド
    fn into_title(self) -> String {
        self.title
    }
}

fn main() {
    let mut note = Note::new(1, "  牛乳を買う ");
    println!("{}", note.summary());
    note.toggle();
    println!("{}", note.summary());
    let title = note.into_title();
    println!("取り出したタイトル: {title}");
}
