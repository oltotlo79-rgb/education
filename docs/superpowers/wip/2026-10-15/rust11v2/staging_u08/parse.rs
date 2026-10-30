//! 第11巻 UNIT 08: 検査済みの型の parse の速さを、criterion で測る。
//! 置き場所: benches/parse.rs（Cargo.toml に [[bench]] name = "parse"・harness = false）
//! 実行: cargo bench --bench parse

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use rustnote::domain::{Email, NoteTitle};

fn parse_titles(c: &mut Criterion) {
    let ascii = "a".repeat(80);
    let japanese = "あ".repeat(80);
    let emoji = "😀".repeat(80);
    let mut group = c.benchmark_group("NoteTitle::parse");
    for (name, input) in [("ascii80", &ascii), ("japanese80", &japanese), ("emoji80", &emoji)] {
        group.bench_function(name, |b| b.iter(|| NoteTitle::parse(black_box(input))));
    }
    group.finish();
}

fn parse_email(c: &mut Criterion) {
    c.bench_function("Email::parse", |b| {
        b.iter(|| Email::parse(black_box("  Alice@Example.COM ")))
    });
}

criterion_group!(benches, parse_titles, parse_email);
criterion_main!(benches);
