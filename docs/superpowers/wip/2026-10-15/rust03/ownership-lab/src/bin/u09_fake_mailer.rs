use std::cell::RefCell;

// メールを送る係の約束。送るだけなので &self で十分、という設計
trait Mailer {
    fn send(&self, to: &str, body: &str);
}

// テスト用の偽物。本当には送らず、送った内容を記録するだけ
struct FakeMailer {
    sent: RefCell<Vec<(String, String)>>,
}

impl Mailer for FakeMailer {
    fn send(&self, to: &str, body: &str) {
        self.sent
            .borrow_mut()
            .push((to.to_string(), body.to_string()));
    }
}

// 登録処理。どの Mailer でも動くように &dyn Mailer で受け取る
fn signup(mailer: &dyn Mailer, email: &str) -> Result<(), String> {
    if !email.contains('@') {
        return Err(format!("メールアドレスが不正です: {email}"));
    }
    mailer.send(email, "RustNote へようこそ");
    Ok(())
}

fn main() {
    let fake = FakeMailer {
        sent: RefCell::new(Vec::new()),
    };
    println!("{:?}", signup(&fake, "alice@example.com"));
    println!("{:?}", signup(&fake, "not-an-email"));

    let sent = fake.sent.borrow();
    println!("送信件数: {}", sent.len());
    for (to, body) in sent.iter() {
        println!("  宛先 {to}: {body}");
    }
    assert_eq!(sent.len(), 1);
}
