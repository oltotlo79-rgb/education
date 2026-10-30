//! 第11巻 UNIT 07 エラー3: 数える入れ物を、HashMap の static で作ろうとした。
//! 実行: cargo test --test e07_static

use std::collections::HashMap;
use std::sync::Mutex;

/// 状態コードごとの数
static COUNTS: Mutex<HashMap<u16, u64>> = Mutex::new(HashMap::new());

#[test]
fn count_statuses() {
    for status in [200, 200, 404] {
        *COUNTS.lock().unwrap().entry(status).or_insert(0) += 1;
    }
    println!("{:?}", COUNTS.lock().unwrap());
}
