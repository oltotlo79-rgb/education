// &[u32] は「u32 の並びを借りて読む」型。配列からも Vec からも受け取れる
fn total(values: &[u32]) -> u32 {
    let mut sum = 0;
    for v in values {
        sum += v;
    }
    sum
}

fn main() {
    let scores = [72, 88, 95, 60, 81];
    // 配列全体を貸す
    println!("全員の合計: {}", total(&scores));

    // 1番目から3番目の手前まで（88 と 95）を貸す
    let middle = &scores[1..3];
    println!("middle = {middle:?}");
    println!("middle の合計: {}", total(middle));
    // 先頭から2つ、3番目から最後まで
    println!("{:?} {:?}", &scores[..2], &scores[3..]);

    // Vec も同じ関数に渡せる
    let dynamic = vec![10, 20, 30];
    println!("Vec の合計: {}", total(&dynamic));
}
