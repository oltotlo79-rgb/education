use std::mem::size_of;

// 「通知を送れるもの」が守る約束（trait）
trait Notifier {
    fn notify(&self, message: &str) -> String;
}

struct Email {
    address: String,
}

struct Slack {
    channel: String,
    mention: bool,
}

impl Notifier for Email {
    fn notify(&self, message: &str) -> String {
        format!("メール送信 → {}: {message}", self.address)
    }
}

impl Notifier for Slack {
    fn notify(&self, message: &str) -> String {
        let at = if self.mention { "@here " } else { "" };
        format!("Slack投稿 → #{}: {at}{message}", self.channel)
    }
}

fn main() {
    // 型が違うものを、1つの Vec にまとめて入れる
    let notifiers: Vec<Box<dyn Notifier>> = vec![
        Box::new(Email {
            address: String::from("alice@example.com"),
        }),
        Box::new(Slack {
            channel: String::from("dev"),
            mention: true,
        }),
    ];

    for n in &notifiers {
        println!("{}", n.notify("ノートが作成されました"));
    }

    println!("Box<Email> の大きさ: {} バイト", size_of::<Box<Email>>());
    println!("Box<dyn Notifier> の大きさ: {} バイト", size_of::<Box<dyn Notifier>>());
}
