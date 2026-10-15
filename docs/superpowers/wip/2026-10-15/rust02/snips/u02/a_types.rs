fn main() {
    // 整数: 型を書かなければ i32 になる
    let temperature = -5;
    // 型を書いて決める。u8 は 0〜255 の整数
    let level: u8 = 200;
    // 数字の後ろに型名を付けて決める書き方もある
    let big = 3_000_000_000u64;
    // 小数: 型を書かなければ f64 になる
    let ratio = 0.75;
    // 真偽値: true か false のどちらか
    let done = false;
    // 文字: シングルクォート ' で1文字だけを囲む
    let mark = 'あ';

    println!("{temperature} {level} {big} {ratio} {done} {mark}");

    // 型ごとの最小値と最大値
    println!("i32: {} 〜 {}", i32::MIN, i32::MAX);
    println!("u8 : {} 〜 {}", u8::MIN, u8::MAX);
    println!("u64 の最大: {}", u64::MAX);
    // その型が何バイトを使うか
    println!("char は {} バイト", std::mem::size_of::<char>());
}
