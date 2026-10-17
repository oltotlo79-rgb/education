struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn get(&self) -> u32 {
        self.count
    }
}

fn main() {
    let mut counter = Counter::new();
    for _ in 0..3 {
        counter.increment();
    }
    println!("count = {}", counter.get());
}
