struct Note {
    title: String,
    done: bool,
}

// 検索条件を「木」の形で表す。And や Not の中に、また Filter が入る
enum Filter {
    TitleContains(String),
    Done(bool),
    And(Box<Filter>, Box<Filter>),
    Or(Box<Filter>, Box<Filter>),
    Not(Box<Filter>),
}

impl Filter {
    fn matches(&self, note: &Note) -> bool {
        match self {
            Filter::TitleContains(word) => note.title.contains(word.as_str()),
            Filter::Done(flag) => note.done == *flag,
            Filter::And(left, right) => left.matches(note) && right.matches(note),
            Filter::Or(left, right) => left.matches(note) || right.matches(note),
            Filter::Not(inner) => !inner.matches(note),
        }
    }
}

fn note(title: &str, done: bool) -> Note {
    Note {
        title: title.to_string(),
        done,
    }
}

fn main() {
    let notes = vec![
        note("牛乳を買う", false),
        note("牛乳パックを洗う", true),
        note("本を返す", false),
    ];

    // 「タイトルに『牛乳』を含む」かつ「完了していない」
    let filter = Filter::And(
        Box::new(Filter::TitleContains(String::from("牛乳"))),
        Box::new(Filter::Not(Box::new(Filter::Done(true)))),
    );
    for n in notes.iter().filter(|n| filter.matches(n)) {
        println!("一致: {}", n.title);
    }

    // 「完了済み」または「タイトルに『本』を含む」
    let filter2 = Filter::Or(
        Box::new(Filter::Done(true)),
        Box::new(Filter::TitleContains(String::from("本"))),
    );
    let count = notes.iter().filter(|n| filter2.matches(n)).count();
    println!("filter2 に一致: {count} 件");
}
