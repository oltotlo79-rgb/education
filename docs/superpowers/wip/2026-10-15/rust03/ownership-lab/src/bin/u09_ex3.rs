use std::cell::Cell;

struct Counter {
    hits: Cell<u32>,
}

impl Counter {
    // &self のままで数を増やす。Cell は Copy な値の出し入れ専用
    fn hit(&self) {
        self.hits.set(self.hits.get() + 1);
    }
}

fn main() {
    let c = Counter { hits: Cell::new(0) };
    c.hit();
    c.hit();
    c.hit();
    println!("hits = {}", c.hits.get());
}
