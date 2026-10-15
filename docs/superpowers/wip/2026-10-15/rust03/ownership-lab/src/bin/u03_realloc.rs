fn main() {
    let mut v = vec![1, 2, 3];
    let mut place = v.as_ptr();
    println!("最初    : cap = {:>3}, 本体の場所 = {:p}", v.capacity(), place);
    for i in 4..=100 {
        v.push(i);
        // 本体の場所が変わったときだけ表示する
        if v.as_ptr() != place {
            place = v.as_ptr();
            println!("引っ越し: cap = {:>3}, 本体の場所 = {:p}", v.capacity(), place);
        }
    }
}
