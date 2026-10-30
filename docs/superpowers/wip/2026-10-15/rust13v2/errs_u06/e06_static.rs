//! 第13巻 UNIT 06 エラー2: static の値を、ふつうの関数で作ろうとした。
//! 実行: cargo run -q --example e06_static

fn make_dummy_hash() -> String {
    "$argon2id$v=19$dummy".to_string()
}

static DUMMY_HASH: String = make_dummy_hash();

fn main() {
    println!("{DUMMY_HASH}");
}
