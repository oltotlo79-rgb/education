// 「数と、次の箱」または「終わり」のどちらか
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

// 再帰的にたどって合計する
fn sum(list: &List) -> i32 {
    match list {
        Cons(value, rest) => value + sum(rest),
        Nil => 0,
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{list:?}");
    println!("合計 = {}", sum(&list));
    println!("List の大きさ: {} バイト", std::mem::size_of::<List>());
}
