// Copy を求めず、いちばん大きい要素への「参照」を返す
fn largest_ref<T>(items: &[T]) -> Option<&T>
where
    T: PartialOrd,
{
    let mut iter = items.iter();
    let mut max = iter.next()?;
    for item in iter {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

fn main() {
    let titles = vec!["牛乳".to_string(), "本".to_string(), "電気代".to_string()];
    println!("{:?}", largest_ref(&titles));
    println!("{:?}", largest_ref(&[3, 9, 4]));
}
