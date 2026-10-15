fn main() {
    // 金額は小数ではなく「円（整数）」で持つ
    let unit_price: u32 = 1_250;
    // 個数はフォームから u16 で届いたとする
    let quantity: u16 = 3;

    // u16 → u32 は必ず成功するので From を使う
    let quantity = u32::from(quantity);
    // 掛け算がはみ出したら None になる checked_mul を使う
    match unit_price.checked_mul(quantity) {
        Some(total) => println!("合計: {total} 円"),
        None => println!("金額が大きすぎます"),
    }

    // 極端に大きな個数も試す
    let huge: u32 = 4_000_000;
    match unit_price.checked_mul(huge) {
        Some(total) => println!("合計: {total} 円"),
        None => println!("金額が大きすぎます"),
    }

    // データベースの ID は i64 で届くことが多い。負の値は ID として不正
    for raw_id in [42_i64, -7] {
        match u64::try_from(raw_id) {
            Ok(id) => println!("ID {id} を使います"),
            Err(e) => println!("ID {raw_id} は使えません: {e}"),
        }
    }
}
