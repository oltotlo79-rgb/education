use std::hint::black_box;
use std::time::Instant;

fn main() {
    // 計算を始めた時刻を覚えておく
    let start = Instant::now();

    // 2 億回のくり返しで、1 から順に「7 で割った余り」を足していく
    let mut total: u64 = 0;
    for n in 1..=200_000_000u64 {
        // black_box は「この計算を省略しないで」とコンパイラに伝えるための道具
        total += black_box(n) % 7;
    }

    // かかった時間を表示する
    println!("合計: {total}");
    println!("かかった時間: {:?}", start.elapsed());
}
