fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // 1つずつ取り出す → 偶数だけ残す → 10 倍する → Vec に集める
    let result: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|n| *n % 2 == 0)
        .map(|n| n * 10)
        .collect();
    println!("{result:?}");

    // 合計・個数
    let sum: i32 = numbers.iter().sum();
    let big = numbers.iter().copied().filter(|n| *n > 3).count();
    println!("合計 {sum}, 3 より大きいのは {big} 個");

    // find は条件に合う最初の1つ（Option）を返す
    println!("{:?}", numbers.iter().copied().find(|n| *n > 4));

    // any: 1つでも合うか / all: 全部合うか
    let any_big = numbers.iter().copied().any(|n| n > 5);
    let all_positive = numbers.iter().copied().all(|n| n > 0);
    println!("{any_big} {all_positive}");

    // 元の Vec は変わらない
    println!("{numbers:?}");
}
