struct SearchQuery<'a> {
    q: &'a str,
}

fn parse_query(raw: &str) -> SearchQuery<'_> {
    SearchQuery {
        q: raw.strip_prefix("q=").unwrap_or(""),
    }
}

fn main() {
    let mut history: Vec<SearchQuery> = Vec::new();
    for raw in ["q=milk", "q=book"] {
        let url = String::from(raw);
        history.push(parse_query(&url));
    }
    println!("{}", history.len());
    println!("{}", history[0].q);
}
