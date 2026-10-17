// 一覧の絞り込み条件。3つのうち「どれか1つ」の値をとる
#[derive(Debug, Clone, Copy, PartialEq)]
enum Filter {
    All,
    Open,
    Done,
}

// 絞り込み条件を、画面に出す日本語にする
fn label(filter: Filter) -> &'static str {
    match filter {
        Filter::All => "すべて",
        Filter::Open => "未完了",
        Filter::Done => "完了済み",
    }
}

fn main() {
    let current = Filter::Open;
    println!("{:?} → {}", current, label(current));
    println!("未完了を選んでいる? {}", current == Filter::Open);

    for f in [Filter::All, Filter::Open, Filter::Done] {
        println!("- {}", label(f));
    }
}
