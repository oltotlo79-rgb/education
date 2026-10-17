use std::collections::BTreeMap;

struct Note {
    title: String,
    tag: String,
    done: bool,
}

impl Note {
    fn new(title: &str, tag: &str, done: bool) -> Self {
        Self {
            title: title.to_string(),
            tag: tag.to_string(),
            done,
        }
    }
}

fn main() {
    let notes = vec![
        Note::new("牛乳を買う", "買い物", true),
        Note::new("Rust の勉強", "勉強", false),
        Note::new("卵を買う", "買い物", false),
        Note::new("Topcoat を調べる", "勉強", false),
        Note::new("部屋の掃除", "家事", true),
    ];

    // 未完了のノートのタイトルだけを集める
    let open: Vec<&str> = notes
        .iter()
        .filter(|n| !n.done)
        .map(|n| n.title.as_str())
        .collect();
    println!("未完了: {open:?}");

    // タグごとの件数（BTreeMap なのでタグ名の順に並ぶ）
    let mut per_tag: BTreeMap<&str, usize> = BTreeMap::new();
    for note in &notes {
        *per_tag.entry(note.tag.as_str()).or_insert(0) += 1;
    }
    println!("タグ別: {per_tag:?}");

    // 完了率（%）
    let done = notes.iter().filter(|n| n.done).count();
    let rate = done as f64 / notes.len() as f64 * 100.0;
    println!("完了率: {rate:.0}%（{done}/{} 件）", notes.len());

    // 「買」を含むタイトルを、番号付きの文字列にする
    let lines: Vec<String> = notes
        .iter()
        .filter(|n| n.title.contains("買"))
        .enumerate()
        .map(|(i, n)| format!("{}. {}", i + 1, n.title))
        .collect();
    println!("{}", lines.join(" / "));
}
