//! 第11巻 UNIT 08: メモリの使い方を測る。
//! このテストのプログラムだけ、メモリを借りる係（global allocator）を「数える係」に替え、
//! 借りた回数・量と、同時に借りていた量のいちばん多い所（ピーク）を数える。
//!
//! 実行: cargo test --release --test u08_memory -- --nocapture --test-threads=1

mod common;

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use common::{body_text, test_app};
use rustnote::auth::{hash_password, verify_password};
use rustnote::domain::Password;

/// 本物の係（System）に任せつつ、数だけ数える係
struct Counting;

static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
static IN_USE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        let now = IN_USE.fetch_add(layout.size(), Ordering::Relaxed) + layout.size();
        PEAK.fetch_max(now, Ordering::Relaxed);
        // SAFETY: 受け取った layout を、そのまま本物の係に渡す
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        IN_USE.fetch_sub(layout.size(), Ordering::Relaxed);
        // SAFETY: alloc で本物の係から借りたものを、同じ layout で返す
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

/// 数え始め（ピークを、今の量にそろえる）
fn start() -> (usize, usize) {
    let in_use = IN_USE.load(Ordering::Relaxed);
    PEAK.store(in_use, Ordering::Relaxed);
    (ALLOCATIONS.load(Ordering::Relaxed), in_use)
}

/// 数え終わり: 借りた回数と、数え始めより何バイト多く借りていたか（ピーク）
fn finish(label: &str, (count, base): (usize, usize)) {
    let allocations = ALLOCATIONS.load(Ordering::Relaxed) - count;
    let peak = PEAK.load(Ordering::Relaxed) - base;
    println!("{label}: 借りた回数 {allocations} 回・ピーク +{:.2} MiB", peak as f64 / 1024.0 / 1024.0);
}

#[tokio::test]
async fn memory_of_main_operations() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", "correct horse battery").await;
    for i in 0..20 {
        app.create_note(&alice, &format!("メモ{i:02}"), &"本文".repeat(50)).await;
    }

    let mark = start();
    let html = body_text(app.get("/notes", Some(&alice)).await).await;
    finish(&format!("GET /notes（20 件・本文 {} バイト）", html.len()), mark);

    let password = Password::parse("correct horse battery").unwrap();
    let mark = start();
    let hash = hash_password(&password).await.unwrap();
    finish("パスワードのハッシュ（Argon2id）", mark);

    let mark = start();
    verify_password("correct horse battery", &hash).await.unwrap();
    finish("パスワードの照合（Argon2id）", mark);
}
