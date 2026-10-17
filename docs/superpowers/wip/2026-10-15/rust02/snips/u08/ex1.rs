enum Filter {
    All,
    Open,
    Done,
    Archived,
}

fn label(filter: Filter) -> &'static str {
    match filter {
        Filter::All => "すべて",
        Filter::Open => "未完了",
        Filter::Done => "完了済み",
        Filter::Archived => "アーカイブ",
    }
}

fn main() {
    for f in [Filter::All, Filter::Open, Filter::Done, Filter::Archived] {
        println!("{}", label(f));
    }
}
