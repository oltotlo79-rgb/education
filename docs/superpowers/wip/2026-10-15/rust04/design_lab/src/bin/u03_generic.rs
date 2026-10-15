use std::any::type_name;

// T は「呼ばれるときに決まる型」。
// PartialOrd（大小を比べられる）と Copy（コピーできる）を T に求める
fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let mut iter = items.iter().copied();
    // 空なら None を返して終わり
    let mut max = iter.next()?;
    for item in iter {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

// どの型で呼ばれたのかを表示する
fn show_type<T>(_value: &T) {
    println!("この呼び出しの T = {}", type_name::<T>());
}

fn main() {
    println!("{:?}", largest(&[3, 9, 4]));
    println!("{:?}", largest(&[1.5, 0.2]));
    println!("{:?}", largest(&['あ', 'ん', 'か']));
    let empty: [i32; 0] = [];
    println!("{:?}", largest(&empty));

    show_type(&42);
    show_type(&1.5);
    show_type(&"文字列");
}
