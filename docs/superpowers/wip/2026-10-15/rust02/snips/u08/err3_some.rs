// 偶数なら半分を返す。奇数なら None
fn half(n: u32) -> Option<u32> {
    if n % 2 == 0 { n / 2 } else { None }
}

fn main() {
    println!("{:?}", half(10));
}
