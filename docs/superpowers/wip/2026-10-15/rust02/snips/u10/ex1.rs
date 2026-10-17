use std::collections::{BTreeMap, HashMap};

fn main() {
    let text = "rust web rust db web rust";
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    let sorted: BTreeMap<&str, u32> = counts.into_iter().collect();
    for (word, count) in sorted {
        println!("{word}: {count}");
    }
}
