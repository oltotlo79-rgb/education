#[derive(Debug)]
enum Filter {
    All,
    Open,
    Done,
}

fn parse_filter(text: &str) -> Option<Filter> {
    match text {
        "all" => Some(Filter::All),
        "open" => Some(Filter::Open),
        "done" => Some(Filter::Done),
        _ => None,
    }
}

fn main() {
    for text in ["open", "done", "everything"] {
        println!("{text} → {:?}", parse_filter(text));
    }
}
