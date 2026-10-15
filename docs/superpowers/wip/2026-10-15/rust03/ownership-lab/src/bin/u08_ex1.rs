#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn len(list: &List) -> usize {
    match list {
        Cons(_, rest) => 1 + len(rest),
        Nil => 0,
    }
}

fn max(list: &List) -> Option<i32> {
    match list {
        Cons(value, rest) => match max(rest) {
            Some(m) if m > *value => Some(m),
            _ => Some(*value),
        },
        Nil => None,
    }
}

fn main() {
    let list = Cons(4, Box::new(Cons(9, Box::new(Cons(2, Box::new(Nil))))));
    println!("len = {}, max = {:?}", len(&list), max(&list));
    println!("空: len = {}, max = {:?}", len(&Nil), max(&Nil));
}
