trait Summary {
    fn summary(&self) -> String;
}

struct Note {
    title: String,
}

impl Summary for Note {
    fn summary(&self) -> String {
        self.title.clone()
    }
}

fn print_summary(item: &impl Summary) {
    println!("{}", item.summary());
}

fn main() {
    let note = Note {
        title: "牛乳を買う".to_string(),
    };
    print_summary(&note);
    // i32 は Summary を実装していない
    print_summary(&42);
}
