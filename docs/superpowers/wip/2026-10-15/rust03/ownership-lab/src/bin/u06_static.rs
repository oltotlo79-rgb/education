use std::sync::atomic::{AtomicU64, Ordering};

// static: プログラムが動いている間ずっと、1つだけ存在する値
static APP_NAME: &str = "RustNote";
// 書き換えたい static は、スレッドから安全に触れる型（ここでは原子的な整数）にする
static REQUEST_COUNT: AtomicU64 = AtomicU64::new(0);

// 文字列リテラルは 'static なので、関数の外へそのまま返せる
fn status_text(code: u16) -> &'static str {
    match code {
        200 => "OK",
        303 => "See Other",
        404 => "Not Found",
        _ => "Unknown",
    }
}

fn handle(code: u16) {
    REQUEST_COUNT.fetch_add(1, Ordering::Relaxed);
    println!("[{APP_NAME}] {code} {}", status_text(code));
}

fn main() {
    handle(200);
    handle(404);
    let literal: &'static str = "リテラルはプログラム全体で有効";
    println!("{literal}");
    println!("処理したリクエスト: {}", REQUEST_COUNT.load(Ordering::Relaxed));
}
