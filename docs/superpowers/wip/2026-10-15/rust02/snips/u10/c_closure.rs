// f は「i32 を受け取って i32 を返すもの」なら何でもよい
fn apply_twice(f: impl Fn(i32) -> i32, value: i32) -> i32 {
    f(f(value))
}

fn main() {
    // クロージャ: |引数| 式 の形で、その場で作る小さな関数
    let double = |x: i32| x * 2;
    println!("{}", double(21));

    // 周りの変数（tax_rate）を取り込んで使える
    let tax_rate = 10;
    let with_tax = |price: u32| price * (100 + tax_rate) / 100;
    println!("{}", with_tax(1_000));

    // 関数に「処理そのもの」を渡す
    println!("{}", apply_twice(double, 5));
    println!("{}", apply_twice(|x| x + 100, 5));
}
