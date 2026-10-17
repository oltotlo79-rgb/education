enum Filter {
    All,
    Open,
    Done,
}

fn label(filter: Filter) -> &'static str {
    match filter {
        Filter::All => "すべて",
        Filter::Open => "未完了",
    }
}

fn main() {
    println!("{}", label(Filter::Done));
}
