fn main() {
    let s1 = String::from("hello");
    // as_ptr() は、ヒープにある本体の場所（アドレス）を返す
    println!("s1 の本体の場所: {:p}", s1.as_ptr());

    // s1 の持ち手が s2 へ移る（move）。本体は動かない
    let s2 = s1;
    println!("s2 の本体の場所: {:p}", s2.as_ptr());

    // clone は本体をまるごと複製する。場所が別になる
    let s3 = s2.clone();
    println!("s3 の本体の場所: {:p}", s3.as_ptr());
    println!("s2 = {s2}, s3 = {s3}");

    // i32 は Copy 型。代入しても x はそのまま使える
    let x = 42;
    let y = x;
    println!("x = {x}, y = {y}");
}
