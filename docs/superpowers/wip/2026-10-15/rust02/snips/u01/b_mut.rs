fn main() {
    // mut を付けると、あとから中身を入れ替えられる変数になる
    let mut done_count = 0;
    println!("はじめ: {done_count}");

    // 今の値に 1 を足して、同じ変数へ入れ直す
    done_count = done_count + 1;
    println!("1件完了: {done_count}");

    // += は「自分に足して入れ直す」の短い書き方
    done_count += 2;
    println!("さらに2件完了: {done_count}");
}
