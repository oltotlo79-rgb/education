## UNIT 09: OIDC・外の認証サービス・MFA・回復

### この単元のゴール

- 外の認証サービスでログインする OIDC の流れ（Authorization Code フローと PKCE）を説明でき、`code_verifier`・`code_challenge`・`state`・`nonce` を自分で作って、RFC の例と同じ値になることを確かめられる
- 二段階認証の TOTP（30 秒ごとに変わる6けたの数字）を自分で計算し、RFC 6238 の例と一致することと、時刻のずれ・同じ数字の使い回しの扱いを確かめられる
- パスワードを忘れた人のための、使い捨ての再設定のトークン（ハッシュで保存・1 時間・1 回だけ）を作り、違う・期限切れ・2 回目を断れる
- `state` を確かめないと、攻撃する人のアカウントに、利用者がログインさせられてしまうことを説明できる

### たとえ話でつかむ

大きな会社の、入館の受付を考えます。

- **外の認証サービス（OIDC）:** 自分の会社で入館証を作らずに、信頼できる「本人確認の窓口」（外の会社）で確かめてもらいます。窓口は、本人だと分かったら、**引換券**（`code`）を渡し、受付はその引換券を、窓口へ直接持って行って、「この人は○○さんです」という**証明書**（ID トークン）と交換します。
- 引換券を、途中で誰かに取られても困らないように、受付は最初に、**自分だけが知っている合言葉の印**（`code_challenge`）を窓口に預け、交換のときに**合言葉そのもの**（`code_verifier`）を見せます（PKCE）。
- 受付は、「この引換券は、**自分が案内した人の**ものか」を、案内のときに渡した番号札（`state`）で確かめます。番号札を確かめないと、ほかの人の引換券で、別の人として入館させられてしまいます。
- **二段階認証（MFA）:** 入館証に加えて、30 秒ごとに数字が変わる小さな機械を持たせます。入館証を盗まれても、機械が無ければ入れません。
- **回復:** 入館証を失くした人には、登録した住所に、**1 回しか使えない、1 時間だけ有効な**再発行の券を送ります。

| 入館の受付 | 認証の仕組み |
|---|---|
| 本人確認の窓口 | 外の認証サービス（OpenID Provider） |
| 引換券と、証明書への交換 | Authorization Code フロー（`code` → ID トークン） |
| 合言葉の印と合言葉 | PKCE（`code_challenge` と `code_verifier`） |
| 番号札 | `state`（と、ID トークンの中の `nonce`） |
| 30 秒ごとに変わる数字の機械 | TOTP（認証のアプリ） |
| 1 回だけ・1 時間の再発行の券 | パスワードの再設定のトークン |

> **用語:** OIDC（OpenID Connect：外の認証サービス（Google・会社の認証の仕組みなど）で本人を確かめてもらい、その結果を ID トークンとして受け取る決まり。パスワードを自分のアプリで預からずに済む）

<!-- -->

> **用語:** MFA（Multi-Factor Authentication：多要素認証。パスワード（知っている物）に、スマートフォンの数字（持っている物）などを加えて、2つ以上で本人を確かめること。二段階認証とも呼ぶ）

<!-- -->

> **用語:** TOTP（Time-based One-Time Password：時刻から計算する、使い捨ての数字。サーバーと認証のアプリが同じ秘密の鍵を持ち、30 秒ごとに同じ数字を計算する。RFC 6238）

### 図で理解する

OIDC の Authorization Code フローと PKCE です（RustNote に足すとした場合）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 360" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-0901" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="10" width="130" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="85" y="32" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">ブラウザ</text>
<rect x="295" y="10" width="130" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="32" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">RustNote</text>
<rect x="560" y="10" width="140" height="34" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="630" y="32" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">認証サービス</text>
<line x1="85" y1="44" x2="85" y2="350" stroke="#94a3b8" stroke-width="2" stroke-dasharray="4 4"/>
<line x1="360" y1="44" x2="360" y2="350" stroke="#94a3b8" stroke-width="2" stroke-dasharray="4 4"/>
<line x1="630" y1="44" x2="630" y2="350" stroke="#94a3b8" stroke-width="2" stroke-dasharray="4 4"/>
<text x="372" y="70" font-size="11" fill="#0f172a">① verifier・state・nonce を作り、セッションに覚える</text>
<line x1="356" y1="92" x2="89" y2="92" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0901)"/>
<text x="222" y="86" font-size="10" fill="#0f172a" text-anchor="middle">303 → /authorize?…challenge・state・nonce</text>
<line x1="89" y1="122" x2="626" y2="122" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0901)"/>
<text x="360" y="116" font-size="10" fill="#0f172a" text-anchor="middle">② 認証サービスの画面で、パスワード・MFA（RustNote は見ない）</text>
<line x1="626" y1="152" x2="89" y2="152" stroke="#f97316" stroke-width="2" marker-end="url(#ar-08-0901)"/>
<text x="360" y="146" font-size="10" fill="#c2410c" text-anchor="middle">③ 303 → RustNote の /auth/callback?code=…&amp;state=…</text>
<line x1="89" y1="182" x2="356" y2="182" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0901)"/>
<text x="222" y="176" font-size="10" fill="#0f172a" text-anchor="middle">GET /auth/callback</text>
<text x="372" y="206" font-size="11" fill="#b91c1c">④ state がセッションのものと同じか（違えば断る）</text>
<line x1="364" y1="232" x2="626" y2="232" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0901)"/>
<text x="495" y="226" font-size="10" fill="#0f172a" text-anchor="middle">⑤ code ＋ verifier（サーバーどうし）</text>
<line x1="626" y1="262" x2="364" y2="262" stroke="#10b981" stroke-width="2" marker-end="url(#ar-08-0901)"/>
<text x="495" y="256" font-size="10" fill="#047857" text-anchor="middle">ID トークン（署名付き）</text>
<text x="372" y="290" font-size="11" fill="#0f172a">⑥ 署名・iss・aud・exp・nonce を確かめ、sub で利用者を探す</text>
<text x="372" y="312" font-size="11" fill="#047857">⑦ log_in（ここからは今の RustNote と同じセッション）</text>
<line x1="356" y1="336" x2="89" y2="336" stroke="#10b981" stroke-width="2" marker-end="url(#ar-08-0901)"/>
<text x="222" y="330" font-size="10" fill="#047857" text-anchor="middle">303 → /notes（__Host-session）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: OIDC の Authorization Code フローと PKCE。パスワードは認証サービスだけが見て、RustNote は code と verifier を交換して ID トークンを受け取る。state で自分が始めたログインかを確かめ、最後は今の RustNote と同じセッションを作る</div>
</div>

TOTP の数字の作り方と、確かめ方です（例9-2）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 200" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-0902" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="20" width="150" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="85" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">秘密の鍵</text>
<text x="85" y="64" font-size="10" fill="#475569" text-anchor="middle">登録のときに共有</text>
<rect x="10" y="100" width="150" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="85" y="124" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">今の時刻 ÷ 30</text>
<text x="85" y="144" font-size="10" fill="#475569" text-anchor="middle">30 秒ごとに 1 進む数</text>
<rect x="200" y="56" width="150" height="64" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="275" y="84" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">HMAC-SHA1</text>
<text x="275" y="104" font-size="10" fill="#475569" text-anchor="middle">20 バイトの結果</text>
<rect x="390" y="56" width="150" height="64" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="465" y="84" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">4 バイト取り出す</text>
<text x="465" y="104" font-size="10" fill="#475569" text-anchor="middle">最後の下 4 ビットの場所から</text>
<rect x="580" y="56" width="130" height="64" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="645" y="84" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">÷ 10⁶ の余り</text>
<text x="645" y="104" font-size="10" fill="#475569" text-anchor="middle">6 けたの数字</text>
<line x1="160" y1="48" x2="196" y2="80" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0902)"/>
<line x1="160" y1="128" x2="196" y2="96" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0902)"/>
<line x1="350" y1="88" x2="386" y2="88" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0902)"/>
<line x1="540" y1="88" x2="576" y2="88" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0902)"/>
<text x="360" y="176" font-size="11" fill="#0f172a" text-anchor="middle">確かめるとき: 今の 30 秒と、前後 1 つ分まで受け付ける（時計のずれ）・使った 30 秒は覚えて、2 回目は断る</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: TOTP の数字の作り方。サーバーと認証のアプリが同じ秘密の鍵と時刻から、同じ6けたを計算する。確かめるときは、少しの時計のずれを許し、同じ数字の使い回しは断る</div>
</div>

### まず動かす最小コード

この単元の例は、RustNote のコピーとは別の、小さな練習用のクレート（`mfa_lab`）で動かします。外の認証サービスのアカウントは使いません。使ったクレートと版です。

```toml
[package]
name = "mfa_lab"
version = "0.1.0"
edition = "2024"

[dependencies]
base64 = "0.23.1"
hmac = "0.13.0"
rand = "0.10.3"
sha1 = "0.11.0"
sha2 = "0.11.1"
```

#### 例9-1: PKCE・state・nonce を作る

> **▼ このコードがやること（先に日本語で）:** `code_challenge` を「`code_verifier` の SHA-256 を、URL に入れられる base64 にしたもの」（S256 の方法）として作る関数を書き、RFC 7636（PKCE の決まり）の付録 B の例と同じ値になるかを確かめます。次に、ログインのたびに作る `code_verifier`・`state`・`nonce`（どれも、でたらめな 32 バイト）を作り、認証サービスへ移る URL を組み立てて、項目ごとに表示します（URL は送りません）。

```rust
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
```

```powershell
cargo run -q --example pkce
```

```text
### cargo run -q --example pkce
RFC 7636 の例: E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM
RFC の値と同じ: true
code_verifier（43 文字）: VVf6-B34…
code_challenge: KHgXPO4I…
移る URL の長さ: 345 文字
  response_type = code
  client_id = rustnote
  redirect_uri = https%3A…
  scope = openid%20email
  state = 9nozLdW3…
  nonce = oaTAYD-o…
  code_challenge = KHgXPO4I…
  code_challenge_method = S256
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: PKCE は、引換券を盗まれても使えなくする

- RFC の例の `code_verifier` から作った値は、`E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM` で、RFC に書かれた値と同じでした（`true`）。計算の方法が、決まりどおりであることが分かります。
- 認証サービスへ移るときに渡すのは `code_challenge`（ハッシュ）だけで、`code_verifier` は RustNote の中に残します。③で戻ってくる `code` を誰かが途中で盗んでも、⑤の交換には `code_verifier` が要るので、盗んだ人は ID トークンを受け取れません。SHA-256 から `code_verifier` を作り直すこともできません。

##### 解説2: state と nonce は、ログインのたびに新しく

- `state` は、③で戻ってきたときに「このブラウザが始めたログインか」を確かめるための値です（エラー1）。`nonce` は、⑥で受け取った ID トークンの中に、同じ値が入っているかを確かめて、古い ID トークンの使い回しを断るための値です。
- どちらも、推測できない、でたらめな値（32 バイト・43 文字）にし、ブラウザのセッション（RustNote なら Topcoat のセッションか、署名付きのクッキー）に覚えておきます。

> **注意:** 本物の OIDC では、⑥で ID トークンの**署名**を、認証サービスが公開している鍵（JWKS）で確かめ、`iss`（誰が作ったか）・`aud`（RustNote 宛てか）・`exp`（期限）・`nonce` を確かめます。この確かめは、自分で書かずに、決まりをよく知るライブラリ（Rust なら `openidconnect` クレートなど）に任せます。この例では、自分で書いても安全な、値を作る部分だけを試しています。

### もう一歩: 実務に近い形

#### 例9-2: TOTP の数字を、自分で計算する

> **▼ このコードがやること（先に日本語で）:** TOTP の計算（時刻を 30 秒ごとに数え、その数を HMAC-SHA1 にかけ、決まった場所から 4 バイトを取り出して、けた数で割った余りにする）を書き、RFC 6238 の付録 B の6つの例（8 けた）と同じになるかを確かめます。次に、6 けたで、今の数字・同じ数字の2回目・30 秒前の数字・90 秒前の数字を確かめ、最後に、時計が 40 秒遅れたスマートフォンの数字を、今の 30 秒だけで比べる場合と、前後 1 つ分まで比べる場合で確かめます。

```rust
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
```

```powershell
cargo run -q --example totp
```

```text
### cargo run -q --example totp
時刻          59: 94287082（RFC と同じ: true）
時刻  1111111109: 07081804（RFC と同じ: true）
時刻  1111111111: 14050471（RFC と同じ: true）
時刻  1234567890: 89005924（RFC と同じ: true）
時刻  2000000000: 69279037（RFC と同じ: true）
時刻 20000000000: 65353130（RFC と同じ: true）
--- 6 けた（今の時刻 1793000000）
今の数字 909607 → 通す（今の数字）
同じ数字をもう一度 → 断る（同じ数字は、もう使った）
30 秒前の数字 428770 → 通す（30 秒前の数字）
90 秒前の数字 341302 → 断る（合わない）
--- スマートフォンの時計が 40 秒遅れている
表示された数字 428770: 今の 30 秒だけで比べる → 断る
表示された数字 428770: 前後 1 つ分まで比べる → 通す（30 秒前の数字）
```

- RFC 6238 の6つの例が、全部同じになりました（`true`）。時刻 `20000000000`（西暦 2603 年ごろ）まで、計算が合っています。数字は、秘密の鍵と時刻だけから決まるので、サーバーと認証のアプリが、通信をせずに同じ数字を出せます。
- 今の数字は通り、**同じ数字の2回目は断り**ました。同じ 30 秒の間に、盗み見た数字を使い回されないためです。30 秒前の数字は「前後 1 つ分」の決まりで通り、90 秒前の数字は断りました。
- **時計のずれ:** 40 秒遅れたスマートフォンの数字は、今の 30 秒だけで比べると断られ、前後 1 つ分まで比べると通りました。スマートフォンの時計は、少しずれることがあるので、多くのサービスは、前後 1 つ分（±30 秒）を受け付けます。広げすぎると、当てずっぽうで当たりやすくなります（問1）。

#### 例9-3: パスワードの再設定のトークン

> **▼ このコードがやること（先に日本語で）:** パスワードの再設定のトークン（でたらめな 32 バイト）を作り、その SHA-256・メールアドレス・期限（1 時間後）・使ったかどうかを記録します。1文字違うトークン・正しいトークン（1 分後）・同じトークンの2回目・2 時間後の新しいトークン、の4つを確かめます。

```rust
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
```

```powershell
cargo run -q --example reset
```

```text
### cargo run -q --example reset
メールで送るトークン（43 文字）: MfqGlpUU…
保存した記録: 1 件（トークンそのものは保存していない）
1文字違うトークン → Err("違うトークン")
正しいトークン（1 分後）→ Ok("alice@example.com")
同じトークンをもう一度 → Err("もう使ったトークン")
新しいトークンを、2 時間後に使う → Err("期限切れのトークン")
```

- 正しいトークンは1回だけ通り、違うトークン・2回目・期限切れは断りました。トークンは、RustNote のセッションと同じく、**SHA-256 だけを保存**しています（UNIT 05）。DB が漏れても、メールの中のトークンを作り直せません。
- 再設定のトークンは、メールを読める人なら誰でも使えるので、パスワードと同じくらい大事な物です。だから、短い期限（1 時間）・1 回だけ・使ったら、そのユーザーの全部のセッションを消す（UNIT 05 の例5-2）、を組み合わせます。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: コールバックで、state を確かめなかった

③のコールバックで、`state` を確かめる版と、確かめない版を比べます。攻撃する人が、**自分のアカウント**でログインを始めて受け取った `code` と `state` を、alice のブラウザに開かせた場合です（判断の部分だけを取り出した、練習用の例です）。

```rust
//! 第8巻 UNIT 09: 外の認証サービスから戻ってくる（コールバック）とき、state を確かめるかどうかで、
//! 何が変わるかを、判断の部分だけ取り出して試す（本物の認証サービスには送らない）。
//! 攻撃する人は、自分のアカウントでログインを始めて受け取った code と state を、
//! alice のブラウザに開かせる（ログインの CSRF）。
//!
//! 実行: cargo run -q --example callback

/// alice のブラウザのセッションに、ログインを始めたときに覚えておく値
struct Pending {
    state: String,
}

/// コールバックに来た値
struct Callback {
    code: &'static str,
    state: &'static str,
}

/// state を確かめる版
fn handle_checked(pending: Option<&Pending>, callback: &Callback) -> String {
    match pending {
        Some(pending) if pending.state == callback.state => {
            format!("code {} で、ログインを続ける", callback.code)
        }
        _ => "断る（このブラウザが始めたログインではない）".to_string(),
    }
}

/// state を確かめない版（わざと間違えた版）
fn handle_unchecked(_pending: Option<&Pending>, callback: &Callback) -> String {
    format!("code {} で、ログインを続ける", callback.code)
}

fn main() {
    let alice = Pending {
        state: "alice-no-state-PcI4617p".to_string(),
    };
    let own = Callback {
        code: "alice-no-code",
        state: "alice-no-state-PcI4617p",
    };
    let attack = Callback {
        code: "kougeki-suru-hito-no-code",
        state: "kougeki-suru-hito-no-state",
    };
    println!("--- state を確かめる版");
    println!(
        "alice 本人のコールバック → {}",
        handle_checked(Some(&alice), &own)
    );
    println!(
        "攻撃する人が開かせたコールバック → {}",
        handle_checked(Some(&alice), &attack)
    );
    println!(
        "ログインを始めていないブラウザ → {}",
        handle_checked(None, &attack)
    );
    println!("--- state を確かめない版");
    println!(
        "攻撃する人が開かせたコールバック → {}",
        handle_unchecked(Some(&alice), &attack)
    );
    println!(
        "ログインを始めていないブラウザ → {}",
        handle_unchecked(None, &attack)
    );
}
```

```powershell
cargo run -q --example callback
```

```text
### cargo run -q --example callback
--- state を確かめる版
alice 本人のコールバック → code alice-no-code で、ログインを続ける
攻撃する人が開かせたコールバック → 断る（このブラウザが始めたログインではない）
ログインを始めていないブラウザ → 断る（このブラウザが始めたログインではない）
--- state を確かめない版
攻撃する人が開かせたコールバック → code kougeki-suru-hito-no-code で、ログインを続ける
ログインを始めていないブラウザ → code kougeki-suru-hito-no-code で、ログインを続ける
```

- **何が起きたか:** 確かめる版は、攻撃する人のコールバックを「このブラウザが始めたログインではない」と断りました。確かめない版は、攻撃する人の `code` で、ログインを続けました。alice は、**攻撃する人のアカウントに**ログインした状態になります（ログインの CSRF）。
- **困ること:** alice が気づかずに、そのアカウントでノートを書くと、その中身は、攻撃する人のアカウントに残り、あとで攻撃する人が読めます。ログインを始めていないブラウザ（`None`）のコールバックも、通ってしまいます。
- **学べること:** ③のコールバックは、ほかのサイトからでも開かせられる GET です（UNIT 06）。だから、`state` で「自分が始めたログインの続きか」を必ず確かめます。

#### エラー2（エラーが出ない間違い）: TOTP を、今の 30 秒だけで比べた

例9-2の最後の2行のとおり、今の 30 秒だけで比べると、時計が 40 秒遅れたスマートフォンの、正しい数字を断ります。

```text
--- スマートフォンの時計が 40 秒遅れている
表示された数字 428770: 今の 30 秒だけで比べる → 断る
表示された数字 428770: 前後 1 つ分まで比べる → 通す（30 秒前の数字）
```

- **何が起きたか:** 利用者は、正しい数字を打ったのに、何度やっても断られます。スマートフォンの時計が直るまで、ログインできません。
- **直し方:** 前後 1 つ分（±30 秒）まで受け付け、その代わり、同じ数字の2回目は断ります（例9-2の `verify`）。サーバーの時計も、正しく合わせておきます（NTP）。

#### エラー3: 古い資料（base64 0.13 の頃）の書き方を写した（E0425）

`code_challenge` を作る行を、古い資料のとおり `base64::encode_config(…, base64::URL_SAFE_NO_PAD)` と書いた場合です（今の base64 は 0.23.1）。

```rust
//! 第8巻 UNIT 09 エラー3: 古い資料（base64 0.13 の頃）の書き方を写した
use sha2::{Digest, Sha256};

fn main() {
    let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
    // 古い資料の書き方
    let challenge = base64::encode_config(Sha256::digest(verifier.as_bytes()), base64::URL_SAFE_NO_PAD);
    println!("{challenge}");
}
```

```text
### cargo build --example e09_old_base64
   Compiling mfa_lab v0.1.0 (...\mfa_lab)
error[E0425]: cannot find function `encode_config` in crate `base64`
 --> examples\e09_old_base64.rs:7:29
  |
7 |     let challenge = base64::encode_config(Sha256::digest(verifier.as_bytes()), base64::URL_SAFE_NO_PAD);
  |                             ^^^^^^^^^^^^^ not found in `base64`

error[E0425]: cannot find value `URL_SAFE_NO_PAD` in crate `base64`
 --> examples\e09_old_base64.rs:7:88
  |
7 |     let challenge = base64::encode_config(Sha256::digest(verifier.as_bytes()), base64::URL_SAFE_NO_PAD);
  |                                                                                        ^^^^^^^^^^^^^^^ not found in `base64`

For more information about this error, try `rustc --explain E0425`.
error: could not compile `mfa_lab` (example "e09_old_base64") due to 2 previous errors
```

- **何が起きたか:** 「`encode_config` という関数も、`URL_SAFE_NO_PAD` という値も、`base64` クレートの直下にはありません」という2つのエラーです。base64 は、0.21 から、エンコードの方法を「エンジン」で選ぶ書き方に変わりました。古い関数は、今の 0.23.1 にはありません。
- **直し方:** 例9-1のとおり、`use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};` として、`URL_SAFE_NO_PAD.encode(…)` と書きます。UNIT 02 の argon2 0.5 のエラーと同じく、資料を写すときは、`Cargo.toml` の版と、その版の説明書（docs.rs）を確かめます。

### RustNote ではここで使う

- RustNote には、外の認証サービスでのログインも、二段階認証も、パスワードの再設定もありません（UNIT 08 のおすすめの構成で、足していく順番を考えました）。
- 足すときに使える、RustNote の今の部品: ③のコールバックは `#[page]`（GET）で作り、④の `state` は Topcoat のセッションか、署名付きのクッキー（第6巻 UNIT 10）に入れます。⑦は、今の `log_in`（UNIT 04・05）を、そのまま使います。再設定のトークンは、`sessions` 表と同じ形（トークンのハッシュ・ユーザー・期限）の表を足して作れます。
- 外の認証サービスの利用者を、RustNote の `users` 表と結び付けるときは、メールアドレスではなく、ID トークンの `iss`（認証サービス）と `sub`（その中での利用者の番号）の組を使います（問3）。
- 二段階認証の秘密の鍵は、パスワードのハッシュとは違い、サーバーがそのまま使う値なので、DB に入れるときは暗号化します。

### 練習問題

**問1:** TOTP の6けたの数字を、当てずっぽうで当てようとします。前後 1 つ分（3つの数字）を受け付けるとき、1回で当たる見込みは、どのくらいでしょうか。UNIT 08 の Better Auth のように「10 秒で 3 回」に制限すると、1日でどのくらい当たりやすくなるでしょうか。

**問2:** パスワードの再設定の画面で、メールアドレスを入れたときの返事を、「登録されていないメールアドレスです」と「メールを送りました」で言い分けると、何が困るでしょうか。

**問3:** 外の認証サービスでログインした人を、RustNote の `users` 表と、メールアドレスで結び付けると、何が困るでしょうか。

### 解答と解説

**問1の解答:**

- 6けたの数字は 1,000,000 通りで、3つを受け付けるので、1回で当たる見込みは 3 ÷ 1,000,000 ＝ 0.0003%（33 万回に 1 回くらい）です。
- 10 秒で 3 回なら、1日に 86,400 ÷ 10 × 3 ＝ 25,920 回試せるので、1日で1回でも当たる見込みは、1 − (1 − 0.000003)<sup>25,920</sup> ≒ **7.5%** です（ざっと 25,920 × 3 ÷ 1,000,000 ≒ 7.8% と見積もっても、ほぼ同じです）。何日も続ければ、当たってしまいます。だから、二段階認証の数字の入力にも、パスワードより厳しい回数の制限（たとえば、5回まちがえたら、しばらく待たせる）を付けます。
- パスワードと数字の**両方**が要るので、数字だけを当てても入れません。それでも、パスワードが漏れた人を守るための仕組みなので、数字の総当たりにも備えます。

**問2の解答:**

- 「登録されていないメールアドレスです」と言うと、その画面から、メールアドレスが登録されているかどうかが分かります（UNIT 04 のアカウントの列挙）。ログインの画面で同じ文に揃えていても、ここから漏れてしまいます。
- いつも「登録されていれば、メールを送りました」と同じ文を返し、時間もそろえます（メールを送る仕事を、裏で動かすなど）。登録されていないメールには、何も送らない（か、「登録がありません」というメールを、その人だけに送る）ようにします。

**問3の解答:**

- メールアドレスは、変わることがあり、認証サービスによっては、**本人が確かめていない**メールアドレスが入っていることもあります。攻撃する人が、別の認証サービスで、alice のメールアドレスを名乗るアカウントを作れれば、alice の RustNote のアカウントに入れてしまいます。
- 結び付けには、ID トークンの `iss` と `sub` の組（その認証サービスの中で変わらない利用者の番号）を使い、`users` 表とは別の表（たとえば `identities`：`iss`・`sub`・`user_id`）に記録します。メールアドレスは、表示や連絡のためにだけ使います。

### まとめ

- OIDC の Authorization Code フローでは、パスワードは認証サービスだけが見て、RustNote は `code` と `code_verifier` を交換して ID トークンを受け取る。PKCE の `code_challenge` は SHA-256 の base64 で、RFC 7636 の例と一致した。`state` で自分が始めたログインかを、`nonce` で ID トークンの使い回しを確かめる。ID トークンの署名の確かめは、ライブラリに任せる。
- TOTP は、秘密の鍵と時刻から HMAC-SHA1 で6けたを作り、RFC 6238 の例と一致した。前後 1 つ分の時計のずれを許し、同じ数字の2回目は断り、数字の入力にも回数の制限を付ける。
- パスワードの再設定のトークンは、ハッシュで保存し、1 時間・1 回だけ。`state` を確かめないと、ログインの CSRF で、攻撃する人のアカウントにログインさせられる。外の認証サービスの利用者は、メールではなく `iss` と `sub` で結び付ける。
