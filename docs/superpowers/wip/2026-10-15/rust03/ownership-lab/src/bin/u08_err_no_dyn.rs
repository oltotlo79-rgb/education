trait Notifier {
    fn notify(&self, message: &str) -> String;
}

fn main() {
    let notifiers: Vec<Box<Notifier>> = Vec::new();
    println!("{}", notifiers.len());
}
