//! 第11巻 UNIT 08 エラー1: 速さを debug ビルドで測ってしまう。
//! 同じ計算（題 80 文字の parse を 10 万回）を、debug と release で測って比べる。
//!
//! 実行: cargo test --test u08_debug_vs_release -- --nocapture
//!       cargo test --release --test u08_debug_vs_release -- --nocapture

use std::hint::black_box;
use std::time::Instant;

use rustnote::domain::NoteTitle;

#[test]
fn parse_100k_titles() {
    let input = "あ".repeat(80);
    let started = Instant::now();
    for _ in 0..100_000 {
        let _ = black_box(NoteTitle::parse(black_box(&input)));
    }
    let elapsed = started.elapsed();
    let profile = if cfg!(debug_assertions) { "debug" } else { "release" };
    println!(
        "{profile}: 10 万回で {elapsed:.2?}（1回あたり {:.0} ns）",
        elapsed.as_nanos() as f64 / 100_000.0
    );
}
