// わざと間違えた例（2 つ）。cargo build で、コンパイラがどう教えてくれるかを見る

#[derive(Clone, Copy)]
enum Status {
    Circulating,
    Returned,
    Done,
}

fn label(s: Status) -> &'static str {
    match s {
        Status::Circulating => "回覧中",
        Status::Returned => "差し戻し中",
        // 間違い 1: Status::Done の場合を書き忘れた
    }
}

fn main() {
    let title = String::from("型の改造");
    let moved = title;
    println!("{title}"); // 間違い 2: 所有権が moved に移った後で、title を使った
    println!("{moved} {}", label(Status::Done));
}
