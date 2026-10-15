trait Exporter {
    fn name(&self) -> &str;
    fn export(&self, title: &str, body: &str) -> String;
}

struct PlainText;

struct Html;

impl Exporter for PlainText {
    fn name(&self) -> &str {
        "テキスト"
    }

    fn export(&self, title: &str, body: &str) -> String {
        format!("{title}\n{body}")
    }
}

// 新しい形式を足しても、使う側のループは変わらない
impl Exporter for Html {
    fn name(&self) -> &str {
        "HTML"
    }

    fn export(&self, title: &str, body: &str) -> String {
        format!("<h1>{title}</h1>\n<p>{body}</p>")
    }
}

fn main() {
    let exporters: Vec<Box<dyn Exporter>> = vec![Box::new(PlainText), Box::new(Html)];
    for e in &exporters {
        println!("--- {} ---", e.name());
        println!("{}", e.export("牛乳を買う", "低脂肪のもの"));
    }
}
