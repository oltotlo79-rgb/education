trait Notifier {
    fn notify(&self, message: &str) -> String;
}

fn announce(n: dyn Notifier) {
    println!("{}", n.notify("お知らせ"));
}

fn main() {}
