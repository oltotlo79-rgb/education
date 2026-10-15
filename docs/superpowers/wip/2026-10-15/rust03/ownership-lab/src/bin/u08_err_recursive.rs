enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    let _list = List::Cons(1, List::Nil);
}
