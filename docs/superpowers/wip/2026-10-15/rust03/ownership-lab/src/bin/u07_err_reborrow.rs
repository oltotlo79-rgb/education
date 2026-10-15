fn main() {
    let mut v = vec![1];
    let r = &mut v;
    let r2 = &mut *r;
    r.push(2);
    r2.push(3);
    println!("{v:?}");
}
