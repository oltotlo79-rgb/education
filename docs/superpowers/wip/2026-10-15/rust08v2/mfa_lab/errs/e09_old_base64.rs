//! 第8巻 UNIT 09 エラー3: 古い資料（base64 0.13 の頃）の書き方を写した
use sha2::{Digest, Sha256};

fn main() {
    let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
    // 古い資料の書き方
    let challenge = base64::encode_config(Sha256::digest(verifier.as_bytes()), base64::URL_SAFE_NO_PAD);
    println!("{challenge}");
}
