// 空でない並びから、いちばん大きい値を返す
fn largest(values: &[i32]) -> i32 {
    let mut max = values[0];
    for v in values {
        // v は &i32（借りた値）なので、*v で中の値を取り出して比べる
        if *v > max {
            max = *v;
        }
    }
    max
}

fn main() {
    println!("{}", largest(&[3, 9, -2, 7]));
    println!("{}", largest(&vec![-5, -1, -8]));
}
