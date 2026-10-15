fn double_all(values: &mut Vec<i32>) {
    for v in values.iter_mut() {
        *v *= 2;
    }
}

fn sum(values: &Vec<i32>) -> i32 {
    let mut total = 0;
    for v in values {
        total += v;
    }
    total
}

fn main() {
    let mut values = vec![1, 2, 3];
    double_all(&mut values);
    println!("{values:?} 合計 = {}", sum(&values));
}
