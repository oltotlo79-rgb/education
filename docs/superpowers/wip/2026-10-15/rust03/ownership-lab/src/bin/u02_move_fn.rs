// String を受け取る関数。呼んだ側から所有権が移ってくる
fn print_and_drop(s: String) {
    println!("受け取った: {s}");
    // ここで s のスコープが終わり、本体が片付けられる
}

// 作った String を呼び出し元へ返す。所有権も一緒に返る
fn make_greeting(name: &str) -> String {
    let greeting = format!("こんにちは、{name}さん");
    greeting
}

// 受け取って、手を加えて、返す
fn add_mark(mut s: String) -> String {
    s.push('!');
    s
}

// i32 は Copy 型なので、渡しても呼んだ側の値は残る
fn show_number(n: i32) {
    println!("数値: {n}");
}

fn main() {
    let memo = String::from("メモ");
    print_and_drop(memo);

    let greeting = make_greeting("佐藤");
    let greeting = add_mark(greeting);
    println!("{greeting}");

    let n = 7;
    show_number(n);
    println!("n はまだ使える: {n}");
}
