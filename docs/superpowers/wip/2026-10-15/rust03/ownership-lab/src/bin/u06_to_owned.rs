// リクエスト処理の間だけ使う、借用版の検索条件
struct SearchQuery<'a> {
    q: &'a str,
    page: u32,
}

// 後まで保存しておく、所有版の検索条件
struct SavedSearch {
    q: String,
    page: u32,
}

// "q=milk&page=2" のような文字列を、コピーせずに区切って読む
fn parse_query(raw: &str) -> SearchQuery<'_> {
    let mut query = SearchQuery { q: "", page: 1 };
    for pair in raw.split('&') {
        match pair.split_once('=') {
            Some(("q", value)) => query.q = value,
            Some(("page", value)) => query.page = value.parse().unwrap_or(1),
            _ => {}
        }
    }
    query
}

impl SearchQuery<'_> {
    // 借用版から所有版を作る。ここで初めて文字列がコピーされる
    fn to_saved(&self) -> SavedSearch {
        SavedSearch {
            q: self.q.to_string(),
            page: self.page,
        }
    }
}

fn main() {
    let mut history: Vec<SavedSearch> = Vec::new();
    for raw in ["q=milk&page=2", "q=book", "page=3"] {
        // 届いたリクエストの文字列（処理が終わると片付けられる）
        let url = String::from(raw);
        let query = parse_query(&url);
        println!("検索: q=[{}] page={}", query.q, query.page);
        history.push(query.to_saved());
    }
    // url はもう片付けられたが、history は自分の String を持っているので読める
    for saved in &history {
        println!("履歴: q=[{}] page={}", saved.q, saved.page);
    }
}
