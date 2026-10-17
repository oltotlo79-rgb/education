use std::collections::{BTreeMap, HashMap};

fn main() {
    let tags = ["rust", "web", "rust", "db", "rust", "web"];

    let mut counts: HashMap<&str, u32> = HashMap::new();
    for tag in tags {
        // entry(キー).or_insert(0) は「なければ 0 を入れて」、その値の場所を返す
        *counts.entry(tag).or_insert(0) += 1;
    }
    println!("rust の件数: {}", counts["rust"]);

    // BTreeMap に入れ直すと、キーの順に並ぶ
    let sorted: BTreeMap<&str, u32> = counts.into_iter().collect();
    println!("{sorted:?}");
}
