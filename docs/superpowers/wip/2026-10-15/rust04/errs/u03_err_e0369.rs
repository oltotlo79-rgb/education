// T に何の条件も付けずに、大小比較しようとする
fn largest<T: Copy>(items: &[T]) -> Option<T> {
    let mut iter = items.iter().copied();
    let mut max = iter.next()?;
    for item in iter {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

fn main() {
    println!("{:?}", largest(&[3, 9, 4]));
}
