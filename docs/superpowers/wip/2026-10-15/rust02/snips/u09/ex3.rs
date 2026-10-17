use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let count: u32 = "12".parse()?;
    println!("{count}");
    Ok(())
}
