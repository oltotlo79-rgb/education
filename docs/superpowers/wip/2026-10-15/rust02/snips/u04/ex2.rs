fn main() {
    let mut n = 0;
    let mut sum = 0;
    let answer = loop {
        n += 1;
        sum += n;
        if sum >= 100 {
            break n;
        }
    };
    println!("1 から {answer} まで足すと {sum} で、初めて 100 以上になる");
}
