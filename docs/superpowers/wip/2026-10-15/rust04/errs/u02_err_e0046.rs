trait Summary {
    fn summary(&self) -> String;
    fn headline(&self) -> String {
        format!("【{}】", self.summary())
    }
}

struct User {
    email: String,
}

// summary を書き忘れた
impl Summary for User {
    fn headline(&self) -> String {
        format!("<<{}>>", self.email)
    }
}

fn main() {
    let user = User {
        email: "alice@example.com".to_string(),
    };
    println!("{}", user.headline());
}
