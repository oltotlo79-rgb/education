trait Exporter {
    fn name(&self) -> &str;
    // 自分と同じ型の新しい値を返す
    fn duplicate(&self) -> Self;
}

struct Csv;

impl Exporter for Csv {
    fn name(&self) -> &str {
        "CSV"
    }

    fn duplicate(&self) -> Self {
        Csv
    }
}

fn main() {
    let e: Box<dyn Exporter> = Box::new(Csv);
    println!("{}", e.name());
}
