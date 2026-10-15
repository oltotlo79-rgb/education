fn main() {
    let x: u8 = 200;
    println!("{:?}", x.checked_add(100));
    println!("{}", x.wrapping_add(100));
    println!("{}", x.saturating_add(100));
}
