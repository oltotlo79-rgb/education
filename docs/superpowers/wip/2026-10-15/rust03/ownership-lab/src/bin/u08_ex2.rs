trait Notifier {
    fn notify(&self, message: &str) -> String;
}

struct Console;

impl Notifier for Console {
    fn notify(&self, message: &str) -> String {
        format!("画面表示: {message}")
    }
}

struct Email {
    address: String,
}

impl Notifier for Email {
    fn notify(&self, message: &str) -> String {
        format!("メール送信 → {}: {message}", self.address)
    }
}

fn make_notifier(use_email: bool) -> Box<dyn Notifier> {
    if use_email {
        Box::new(Email {
            address: String::from("ops@example.com"),
        })
    } else {
        Box::new(Console)
    }
}

fn main() {
    for flag in [true, false] {
        let n = make_notifier(flag);
        println!("{}", n.notify("バックアップ完了"));
    }
}
