fn main() {
    let a = 10;
    let b = {
        let t = a * 2;
        t + 1
    };
    let c = if b > 20 { b - 20 } else { 0 };
    println!("{a} {b} {c}");
}
