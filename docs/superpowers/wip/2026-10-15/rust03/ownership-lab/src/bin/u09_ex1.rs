use std::rc::Rc;

fn main() {
    let a = Rc::new(vec![1, 2, 3]);
    let b = Rc::clone(&a);
    let c = Rc::clone(&b);
    println!("{}", Rc::strong_count(&a));
    drop(b);
    println!("{}", Rc::strong_count(&c));
}
