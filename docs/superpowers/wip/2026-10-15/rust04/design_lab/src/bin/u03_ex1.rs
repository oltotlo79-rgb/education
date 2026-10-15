// == で比べられる型なら何でも数えられる
fn count_matching<T: PartialEq>(items: &[T], target: &T) -> usize {
    items.iter().filter(|x| *x == target).count()
}

fn main() {
    println!("{}", count_matching(&[1, 2, 1, 3, 1], &1));
    println!("{}", count_matching(&["済", "未", "済"], &"未"));
}
