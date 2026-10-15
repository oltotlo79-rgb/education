trait Exporter {
    fn name(&self) -> &str;
}

struct Csv;

impl Exporter for Csv {
    fn name(&self) -> &str {
        "CSV"
    }
}

fn main() {
    // dyn を書き忘れた
    let e: Box<Exporter> = Box::new(Csv);
    println!("{}", e.name());
}
