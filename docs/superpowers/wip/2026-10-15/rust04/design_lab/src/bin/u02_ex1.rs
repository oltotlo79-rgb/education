trait Summary {
    fn summary(&self) -> String;

    fn headline(&self) -> String {
        format!("【{}】", self.summary())
    }
}

struct Tag {
    name: String,
    count: usize,
}

impl Summary for Tag {
    fn summary(&self) -> String {
        format!("#{} ({}件)", self.name, self.count)
    }
}

fn print_headline(item: &impl Summary) {
    println!("{}", item.headline());
}

fn main() {
    let tag = Tag {
        name: "買い物".to_string(),
        count: 3,
    };
    print_headline(&tag);
}
