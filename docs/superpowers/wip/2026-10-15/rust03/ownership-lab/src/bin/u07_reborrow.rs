fn push_twice(v: &mut Vec<i32>, x: i32) {
    v.push(x);
    v.push(x);
}

fn main() {
    let mut v = Vec::new();
    let r = &mut v;

    // 関数に r を渡すと、自動で「r からの一時的な借用」（再借用）になる
    push_twice(r, 1);
    // だから r は渡した後も使える
    push_twice(r, 2);

    // 明示的に再借用する書き方: &mut *r
    let r2 = &mut *r;
    r2.push(3);
    // r2 の出番が終わったので、r に戻ってこられる
    r.push(4);

    println!("{v:?}");
}
