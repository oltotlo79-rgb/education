trait Exporter {
    fn name(&self) -> &str;

    // where Self: Sized を付けたメソッドは、dyn Exporter からは呼べない代わりに
    // trait 全体を dyn で使えるようにしてくれる
    fn duplicate(&self) -> Self
    where
        Self: Sized;
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

    // 具体的な型（Csv）のままなら duplicate も呼べる
    let original = Csv;
    let copy = original.duplicate();
    println!("{}", copy.name());
}
