//! 第8巻 UNIT 09: OIDC の Authorization Code フローで使う PKCE・state・nonce を作る。
//! 1. RFC 7636 の付録 B の例で、code_challenge の作り方が正しいかを確かめる
//! 2. 新しい code_verifier・state・nonce を作り、認証サービスへ移る URL を組み立てる（送りはしない）
//!
//! 実行: cargo run -q --example pkce

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngExt;
use sha2::{Digest, Sha256};

/// code_challenge = BASE64URL(SHA-256(code_verifier))（S256 の方法）
fn challenge(verifier: &str) -> String {
    URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()))
}

/// でたらめな 32 バイトを、URL に入れられる文字にする
fn random_token() -> String {
    let bytes: [u8; 32] = rand::rng().random();
    URL_SAFE_NO_PAD.encode(bytes)
}

fn main() {
    // RFC 7636 付録 B の例
    let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
    let expected = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";
    println!("RFC 7636 の例: {}", challenge(verifier));
    println!("RFC の値と同じ: {}", challenge(verifier) == expected);

    // ログインのたびに、新しく作る
    let verifier = random_token();
    let state = random_token();
    let nonce = random_token();
    println!(
        "code_verifier（{} 文字）: {}…",
        verifier.len(),
        &verifier[..8]
    );
    println!("code_challenge: {}…", &challenge(&verifier)[..8]);
    let url = format!(
        "https://accounts.example.com/authorize?response_type=code&client_id=rustnote\
         &redirect_uri=https%3A%2F%2Frustnote.example%2Fauth%2Fcallback\
         &scope=openid%20email&state={state}&nonce={nonce}\
         &code_challenge={}&code_challenge_method=S256",
        challenge(&verifier)
    );
    println!("移る URL の長さ: {} 文字", url.len());
    for part in url.split(['?', '&']).skip(1) {
        let (name, value) = part.split_once('=').unwrap();
        let shown = if value.len() > 20 {
            format!("{}…", &value[..8])
        } else {
            value.to_string()
        };
        println!("  {name} = {shown}");
    }
}
