fn main() {
    let numbers = vec![1, 2, 3, 4];
    let evens = numbers.iter().filter(|n| **n % 2 == 0).collect();
    println!("{evens:?}");
}
