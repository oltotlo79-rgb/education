fn check(raw: &str) -> Result<u32, std::num::ParseIntError> {
    raw.parse()
}

fn main() {
    check("abc");
    println!("チェックしました");
}
