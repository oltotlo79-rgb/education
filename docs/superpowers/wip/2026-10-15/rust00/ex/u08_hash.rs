use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// 文字列から「指紋」のような数値を作る（しくみを見るためのおもちゃ）
fn toy_hash(text: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

fn main() {
    let inputs = ["correct-horse", "correct-horse", "correct-horsf", "a"];
    for input in inputs {
        // {:016x} は「16 けたの 16 進数で表示する」という指定
        println!("{input:<14} -> {:016x}", toy_hash(input));
    }
}
