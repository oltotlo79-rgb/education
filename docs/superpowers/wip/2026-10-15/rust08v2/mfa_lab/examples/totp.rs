//! 第8巻 UNIT 09: 二段階認証の TOTP（30 秒ごとに変わる数字）を、自分で計算する。
//! 1. RFC 6238 の付録 B の例（SHA-1・8 けた）で、計算が正しいかを確かめる
//! 2. 6 けたの数字で、時刻のずれ（前後 1 つ分）と、同じ数字の使い回しを確かめる
//!
//! 実行: cargo run -q --example totp

use std::collections::HashSet;

use hmac::{Hmac, KeyInit, Mac};
use sha1::Sha1;

/// TOTP（RFC 6238）: 時刻を 30 秒ごとに数え、その数を HMAC-SHA1 にかけて、digits けたの数字にする
fn totp(secret: &[u8], unix_seconds: u64, digits: u32) -> u32 {
    let step = unix_seconds / 30;
    let mut mac = Hmac::<Sha1>::new_from_slice(secret).expect("どんな長さの鍵でもよい");
    mac.update(&step.to_be_bytes());
    let hash = mac.finalize().into_bytes();
    // 最後のバイトの下 4 ビットで、取り出す場所を決める（dynamic truncation）
    let offset = (hash[hash.len() - 1] & 0x0f) as usize;
    let number = u32::from_be_bytes([
        hash[offset] & 0x7f,
        hash[offset + 1],
        hash[offset + 2],
        hash[offset + 3],
    ]);
    number % 10u32.pow(digits)
}

/// 入力された数字を確かめる。今の 30 秒と、前後 1 つ分まで受け付け、使った回は覚えておく
fn verify(secret: &[u8], code: u32, now: u64, used: &mut HashSet<u64>) -> &'static str {
    for drift in [0i64, -1, 1] {
        let step = (now / 30) as i64 + drift;
        if totp(secret, step as u64 * 30, 6) == code {
            if !used.insert(step as u64) {
                return "断る（同じ数字は、もう使った）";
            }
            return match drift {
                0 => "通す（今の数字）",
                -1 => "通す（30 秒前の数字）",
                _ => "通す（30 秒先の数字）",
            };
        }
    }
    "断る（合わない）"
}

fn main() {
    // RFC 6238 付録 B の例（SHA-1 の鍵は "12345678901234567890"）
    let secret = b"12345678901234567890";
    for (time, expected) in [
        (59, 94287082),
        (1111111109, 7081804),
        (1111111111, 14050471),
        (1234567890, 89005924),
        (2000000000, 69279037),
        (20000000000, 65353130),
    ] {
        let code = totp(secret, time, 8);
        println!(
            "時刻 {time:>11}: {code:08}（RFC と同じ: {}）",
            code == expected
        );
    }

    // 6 けたで、ずれと使い回しを確かめる
    let now = 1_793_000_000;
    let mut used = HashSet::new();
    let current = totp(secret, now, 6);
    let before = totp(secret, now - 30, 6);
    let old = totp(secret, now - 90, 6);
    println!("--- 6 けた（今の時刻 {now}）");
    println!(
        "今の数字 {current:06} → {}",
        verify(secret, current, now, &mut used)
    );
    println!(
        "同じ数字をもう一度 → {}",
        verify(secret, current, now, &mut used)
    );
    println!(
        "30 秒前の数字 {before:06} → {}",
        verify(secret, before, now, &mut used)
    );
    println!(
        "90 秒前の数字 {old:06} → {}",
        verify(secret, old, now, &mut used)
    );

    // 時計が 40 秒遅れているスマートフォンで表示した数字
    let phone = totp(secret, now - 40, 6);
    let strict = if totp(secret, now, 6) == phone {
        "通す"
    } else {
        "断る"
    };
    println!("--- スマートフォンの時計が 40 秒遅れている");
    println!("表示された数字 {phone:06}: 今の 30 秒だけで比べる → {strict}");
    println!(
        "表示された数字 {phone:06}: 前後 1 つ分まで比べる → {}",
        verify(secret, phone, now, &mut HashSet::new())
    );
}
