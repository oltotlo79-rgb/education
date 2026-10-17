fn find_title(titles: &[&str], keyword: &str) -> Option<usize> {
    for (i, title) in titles.iter().enumerate() {
        if title.contains(keyword) {
            return Some(i);
        }
    }
    None
}

fn main() {
    let titles = ["牛乳を買う", "Rust の勉強"];
    let i = find_title(&titles, "旅行").unwrap();
    println!("{}", titles[i]);
}
