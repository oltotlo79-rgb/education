// 税率（%）、送料無料になる金額、送料
const TAX_RATE_PERCENT: u32 = 10;
const FREE_SHIPPING_THRESHOLD: u32 = 5_000;
const SHIPPING_FEE: u32 = 500;

// 税込みの金額を返す
fn with_tax(price: u32) -> u32 {
    price * (100 + TAX_RATE_PERCENT) / 100
}

// 合計金額に応じた送料を返す。if の結果がそのまま戻り値になる
fn shipping_fee(subtotal: u32) -> u32 {
    if subtotal >= FREE_SHIPPING_THRESHOLD {
        0
    } else {
        SHIPPING_FEE
    }
}

// 小さな関数を組み合わせて、支払い総額を計算する
fn total(price: u32, quantity: u32) -> u32 {
    let subtotal = with_tax(price * quantity);
    subtotal + shipping_fee(subtotal)
}

fn main() {
    println!("1,000円 × 2個 = {} 円", total(1_000, 2));
    println!("1,000円 × 5個 = {} 円", total(1_000, 5));
}
