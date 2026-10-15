fn main() {
    let mut v = vec![1, 2, 3];
    let mut extra = Vec::new();
    for n in &v {
        if *n == 2 {
            extra.push(20);
        }
    }
    v.extend(extra);
    println!("{v:?}");

    let mut w = vec![1, 2, 3];
    let len = w.len();
    for i in 0..len {
        if w[i] == 2 {
            w.push(20);
        }
    }
    println!("{w:?}");
}
