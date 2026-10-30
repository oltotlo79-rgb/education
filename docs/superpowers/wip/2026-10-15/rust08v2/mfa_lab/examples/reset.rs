//! 第8巻 UNIT 09: パスワードの再設定（アカウントの回復）のための、使い捨てのトークンの作り方を試す。
//! - メールで送るトークンは、でたらめな 32 バイト
//! - 保存するのは、トークンの SHA-256 と、期限（1 時間）と、使ったかどうか
//! - 違うトークン・期限切れ・2 回目は断る
//!
//! 実行: cargo run -q --example reset

use std::collections::HashMap;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngExt;
use sha2::{Digest, Sha256};

/// 保存する記録（トークンそのものは保存しない）
struct ResetRecord {
    email: String,
    expires_at: u64,
    used: bool,
}

struct ResetStore {
    records: HashMap<[u8; 32], ResetRecord>,
}

impl ResetStore {
    /// トークンを作って記録し、メールで送るトークンを返す
    fn issue(&mut self, email: &str, now: u64) -> String {
        let bytes: [u8; 32] = rand::rng().random();
        let token = URL_SAFE_NO_PAD.encode(bytes);
        let hash: [u8; 32] = Sha256::digest(token.as_bytes()).into();
        self.records.insert(
            hash,
            ResetRecord {
                email: email.to_string(),
                expires_at: now + 3600,
                used: false,
            },
        );
        token
    }

    /// 送られてきたトークンを確かめ、通れば「使った」にする
    fn redeem(&mut self, token: &str, now: u64) -> Result<String, &'static str> {
        let hash: [u8; 32] = Sha256::digest(token.as_bytes()).into();
        let record = self.records.get_mut(&hash).ok_or("違うトークン")?;
        if record.used {
            return Err("もう使ったトークン");
        }
        if now >= record.expires_at {
            return Err("期限切れのトークン");
        }
        record.used = true;
        Ok(record.email.clone())
    }
}

fn main() {
    let mut store = ResetStore {
        records: HashMap::new(),
    };
    let now = 1_793_000_000;
    let token = store.issue("alice@example.com", now);
    println!(
        "メールで送るトークン（{} 文字）: {}…",
        token.len(),
        &token[..8]
    );
    println!(
        "保存した記録: {} 件（トークンそのものは保存していない）",
        store.records.len()
    );

    let mut wrong = token.clone();
    wrong.replace_range(0..1, if wrong.starts_with('A') { "B" } else { "A" });
    println!("1文字違うトークン → {:?}", store.redeem(&wrong, now + 60));
    println!(
        "正しいトークン（1 分後）→ {:?}",
        store.redeem(&token, now + 60)
    );
    println!(
        "同じトークンをもう一度 → {:?}",
        store.redeem(&token, now + 120)
    );

    let late = store.issue("alice@example.com", now);
    println!(
        "新しいトークンを、2 時間後に使う → {:?}",
        store.redeem(&late, now + 7200)
    );
}
