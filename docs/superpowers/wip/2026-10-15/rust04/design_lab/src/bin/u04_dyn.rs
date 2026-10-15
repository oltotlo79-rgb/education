use std::mem::size_of;

// ノートを書き出す「形式」の約束
trait Exporter {
    fn name(&self) -> &str;
    fn export(&self, title: &str, body: &str) -> String;
}

struct PlainText;

struct Markdown {
    heading_level: usize,
}

struct Csv;

impl Exporter for PlainText {
    fn name(&self) -> &str {
        "テキスト"
    }

    fn export(&self, title: &str, body: &str) -> String {
        format!("{title}\n{body}")
    }
}

impl Exporter for Markdown {
    fn name(&self) -> &str {
        "Markdown"
    }

    fn export(&self, title: &str, body: &str) -> String {
        let hashes = "#".repeat(self.heading_level);
        format!("{hashes} {title}\n\n{body}")
    }
}

impl Exporter for Csv {
    fn name(&self) -> &str {
        "CSV"
    }

    fn export(&self, title: &str, body: &str) -> String {
        format!("\"{title}\",\"{body}\"")
    }
}

fn main() {
    // 型の違う 3 つの値を、1 つの Vec に入れる
    let exporters: Vec<Box<dyn Exporter>> = vec![
        Box::new(PlainText),
        Box::new(Markdown { heading_level: 2 }),
        Box::new(Csv),
    ];

    for e in &exporters {
        println!("--- {} ---", e.name());
        println!("{}", e.export("牛乳を買う", "低脂肪のもの"));
    }

    // ポインタの大きさを比べる（64 ビット環境）
    println!("&Markdown の大きさ: {} バイト", size_of::<&Markdown>());
    println!(
        "&dyn Exporter の大きさ: {} バイト",
        size_of::<&dyn Exporter>()
    );
    println!(
        "Box<dyn Exporter> の大きさ: {} バイト",
        size_of::<Box<dyn Exporter>>()
    );
}
