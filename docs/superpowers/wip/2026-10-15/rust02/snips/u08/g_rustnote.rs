// URL の ?q= の値。付いていないこともあるので Option で受け取る
fn search_query(raw: Option<&str>) -> Option<String> {
    let Some(text) = raw else {
        return None;
    };
    let text = text.trim();
    // 空白だけの検索語は「検索なし」とみなす
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

fn main() {
    let titles = ["牛乳を買う", "Rust の勉強", "Rust で CLI を作る"];
    for raw in [None, Some("   "), Some(" Rust ")] {
        match search_query(raw) {
            None => println!("{raw:?} → 検索なし: {} 件すべて表示", titles.len()),
            Some(q) => {
                let mut hits = 0;
                for title in titles {
                    if title.contains(q.as_str()) {
                        hits += 1;
                    }
                }
                println!("{raw:?} → 「{q}」で検索: {hits} 件");
            }
        }
    }
}
