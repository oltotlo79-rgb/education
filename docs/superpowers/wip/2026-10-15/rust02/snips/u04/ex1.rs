fn main() {
    for n in 1..=15 {
        // 3 で割った余りと 5 で割った余りの組で分ける
        let text = match (n % 3, n % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => n.to_string(),
        };
        print!("{text} ");
    }
    println!();
}
