//! Rust の基本の練習: 所有権・借用・Option・Result・イテレーター
use status::{Action, Request, Status};

/// 借用（&）: 中身を見るだけ。持ち主は変わらない
fn print_request(r: &Request) {
    println!("{} {}（{}）", r.id, r.title, r.status.label());
}

/// 見つからないかもしれないときは Option
fn find(list: &[Request], id: u32) -> Option<&Request> {
    list.iter().find(|r| r.id == id)
}

fn main() {
    let mut list = vec![Request::new(1, "治具の製作"), Request::new(2, "部品の追加工"), Request::new(3, "棚の取り付け")];

    // 1. 借用して表示する（list は、このあとも使える）
    for r in &list {
        print_request(r);
    }

    // 2. 書きかえるときは &mut（同時に 1 か所だけ）
    list[0].apply(Action::Return).unwrap();
    list[0].comments.push("【11/02 09:00 回覧者1 佐藤】寸法を追記してください".to_string());
    list[1].apply(Action::ApproveAll).unwrap();

    // 3. Result: できない操作は Err で返ってくるので、match で分ける
    match list[2].apply(Action::HandOver) {
        Ok(()) => println!("引き渡しました"),
        Err(e) => println!("エラー: {e}"),
    }

    // 4. Option: 見つかったときだけ使う
    if let Some(r) = find(&list, 1) {
        println!("1 番のコメント: {:?}", r.comments);
    }
    println!("99 番はあるか: {}", find(&list, 99).is_some());

    // 5. イテレーター（TypeScript の filter・map と同じ考え方）
    let circulating: Vec<&str> =
        list.iter().filter(|r| r.status == Status::Circulating).map(|r| r.title.as_str()).collect();
    println!("回覧中: {circulating:?}");

    // 6. 所有権の移動（ムーブ）: title を別の変数に渡すと、元の変数は使えなくなる
    let title = String::from("型の改造");
    let moved = title; // ここで持ち主が moved に移る
    // println!("{title}"); // ← これはコンパイルエラーになる（所有権が移ったため）
    println!("移った先: {moved}");
}
