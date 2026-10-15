fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
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
    let titles = vec!["牛乳".to_string(), "本".to_string()];
    // String は Copy ではない
    println!("{:?}", largest(&titles));
}
