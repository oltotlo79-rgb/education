// 「一行で要約できる」という約束（trait）
trait Summary {
    // 実装する型が必ず書かなければならないメソッド
    fn summary(&self) -> String;

    // 既定の実装つきメソッド（書かなければこれが使われる）
    fn headline(&self) -> String {
        format!("【{}】", self.summary())
    }
}

struct Note {
    title: String,
    done: bool,
}

struct User {
    email: String,
}

// Note は Summary の約束を守ります、という宣言
impl Summary for Note {
    fn summary(&self) -> String {
        let mark = if self.done { "済" } else { "未" };
        format!("{mark} {}", self.title)
    }
}

impl Summary for User {
    fn summary(&self) -> String {
        format!("ユーザー {}", self.email)
    }

    // 既定の実装を上書きする
    fn headline(&self) -> String {
        format!("<<{}>>", self.email)
    }
}

// Summary を実装した型なら何でも受け取れる関数
fn print_headline(item: &impl Summary) {
    println!("{}", item.headline());
}

fn main() {
    let note = Note {
        title: "牛乳を買う".to_string(),
        done: false,
    };
    let user = User {
        email: "alice@example.com".to_string(),
    };

    println!("{}", note.summary());
    print_headline(&note);
    print_headline(&user);
}
