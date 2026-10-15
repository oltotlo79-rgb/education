fn main() {
    let score = 72;
    // 条件を上から順に調べ、最初に当てはまった枝だけを実行する
    if score >= 80 {
        println!("よくできました");
    } else if score >= 60 {
        println!("合格です");
    } else {
        println!("もう一歩です");
    }

    // if は式なので、結果を変数に入れられる（どの枝も同じ型を返す）
    let parity = if score % 2 == 0 { "偶数" } else { "奇数" };
    println!("{score} は{parity}");
}
