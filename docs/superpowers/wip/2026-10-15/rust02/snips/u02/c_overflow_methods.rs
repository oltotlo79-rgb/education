fn main() {
    let stock: u8 = 250;
    // wrapping_add: はみ出したら 0 に戻って数え直す（時計の針のように一周する）
    println!("wrapping_add:    {}", stock.wrapping_add(10));
    // checked_add: はみ出したら None（計算できなかった）を返す
    println!("checked_add:     {:?}", stock.checked_add(10));
    println!("checked_add(5):  {:?}", stock.checked_add(5));
    // saturating_add: はみ出したら最大値で止める
    println!("saturating_add:  {}", stock.saturating_add(10));
    // overflowing_add: 一周した結果と「はみ出したかどうか」の両方を返す
    println!("overflowing_add: {:?}", stock.overflowing_add(10));
}
