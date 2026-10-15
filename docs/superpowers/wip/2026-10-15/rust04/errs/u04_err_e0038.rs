use std::fmt::Display;

trait Exporter {
    fn name(&self) -> &str;
    // generics のメソッドを持っている
    fn export_all<T: Display>(&self, items: &[T]) -> String;
}

struct PlainText;

impl Exporter for PlainText {
    fn name(&self) -> &str {
        "テキスト"
    }

    fn export_all<T: Display>(&self, items: &[T]) -> String {
        let lines: Vec<String> = items.iter().map(|x| x.to_string()).collect();
        lines.join("\n")
    }
}

fn main() {
    let e: Box<dyn Exporter> = Box::new(PlainText);
    println!("{}", e.name());
}
