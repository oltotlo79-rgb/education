use std::mem::size_of;

fn main() {
    // 5 という値をヒープに置き、その場所を指す Box をスタックに置く
    let b = Box::new(5);
    // 中身を使うときは、ふつうの値と同じように書ける（必要なら * で取り出す）
    println!("b = {b}, b + 1 = {}", *b + 1);

    // 大きな配列を Box に入れると、持ち歩くのは「場所」だけになる
    let big = Box::new([0u8; 4096]);
    println!("big[0] = {}, 要素数 = {}", big[0], big.len());
    println!("[u8; 4096] の大きさ: {} バイト", size_of::<[u8; 4096]>());
    println!("Box<[u8; 4096]> の大きさ: {} バイト", size_of::<Box<[u8; 4096]>>());

    // Box を別の変数へ move しても、動くのは 8 バイトの持ち手だけ
    let moved = big;
    println!("moved.len() = {}", moved.len());
}
