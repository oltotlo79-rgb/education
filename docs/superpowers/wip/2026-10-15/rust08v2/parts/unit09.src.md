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
@@FILE:mfa_lab/Cargo.toml@@
```

#### 例9-1: PKCE・state・nonce を作る

> **▼ このコードがやること（先に日本語で）:** `code_challenge` を「`code_verifier` の SHA-256 を、URL に入れられる base64 にしたもの」（S256 の方法）として作る関数を書き、RFC 7636（PKCE の決まり）の付録 B の例と同じ値になるかを確かめます。次に、ログインのたびに作る `code_verifier`・`state`・`nonce`（どれも、でたらめな 32 バイト）を作り、認証サービスへ移る URL を組み立てて、項目ごとに表示します（URL は送りません）。

```rust
@@FILE:mfa_lab/examples/pkce.rs@@
```

```powershell
cargo run -q --example pkce
```

```text
@@OUT:u09_pkce@@
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
@@FILE:mfa_lab/examples/totp.rs@@
```

```powershell
cargo run -q --example totp
```

```text
@@OUT:u09_totp@@
```

- RFC 6238 の6つの例が、全部同じになりました（`true`）。時刻 `20000000000`（西暦 2603 年ごろ）まで、計算が合っています。数字は、秘密の鍵と時刻だけから決まるので、サーバーと認証のアプリが、通信をせずに同じ数字を出せます。
- 今の数字は通り、**同じ数字の2回目は断り**ました。同じ 30 秒の間に、盗み見た数字を使い回されないためです。30 秒前の数字は「前後 1 つ分」の決まりで通り、90 秒前の数字は断りました。
- **時計のずれ:** 40 秒遅れたスマートフォンの数字は、今の 30 秒だけで比べると断られ、前後 1 つ分まで比べると通りました。スマートフォンの時計は、少しずれることがあるので、多くのサービスは、前後 1 つ分（±30 秒）を受け付けます。広げすぎると、当てずっぽうで当たりやすくなります（問1）。

#### 例9-3: パスワードの再設定のトークン

> **▼ このコードがやること（先に日本語で）:** パスワードの再設定のトークン（でたらめな 32 バイト）を作り、その SHA-256・メールアドレス・期限（1 時間後）・使ったかどうかを記録します。1文字違うトークン・正しいトークン（1 分後）・同じトークンの2回目・2 時間後の新しいトークン、の4つを確かめます。

```rust
@@FILE:mfa_lab/examples/reset.rs@@
```

```powershell
cargo run -q --example reset
```

```text
@@OUT:u09_reset@@
```

- 正しいトークンは1回だけ通り、違うトークン・2回目・期限切れは断りました。トークンは、RustNote のセッションと同じく、**SHA-256 だけを保存**しています（UNIT 05）。DB が漏れても、メールの中のトークンを作り直せません。
- 再設定のトークンは、メールを読める人なら誰でも使えるので、パスワードと同じくらい大事な物です。だから、短い期限（1 時間）・1 回だけ・使ったら、そのユーザーの全部のセッションを消す（UNIT 05 の例5-2）、を組み合わせます。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: コールバックで、state を確かめなかった

③のコールバックで、`state` を確かめる版と、確かめない版を比べます。攻撃する人が、**自分のアカウント**でログインを始めて受け取った `code` と `state` を、alice のブラウザに開かせた場合です（判断の部分だけを取り出した、練習用の例です）。

```rust
@@FILE:mfa_lab/examples/callback.rs@@
```

```powershell
cargo run -q --example callback
```

```text
@@OUT:u09_callback@@
```

- **何が起きたか:** 確かめる版は、攻撃する人のコールバックを「このブラウザが始めたログインではない」と断りました。確かめない版は、攻撃する人の `code` で、ログインを続けました。alice は、**攻撃する人のアカウントに**ログインした状態になります（ログインの CSRF）。
- **困ること:** alice が気づかずに、そのアカウントでノートを書くと、その中身は、攻撃する人のアカウントに残り、あとで攻撃する人が読めます。ログインを始めていないブラウザ（`None`）のコールバックも、通ってしまいます。
- **学べること:** ③のコールバックは、ほかのサイトからでも開かせられる GET です（UNIT 06）。だから、`state` で「自分が始めたログインの続きか」を必ず確かめます。

#### エラー2（エラーが出ない間違い）: TOTP を、今の 30 秒だけで比べた

例9-2の最後の2行のとおり、今の 30 秒だけで比べると、時計が 40 秒遅れたスマートフォンの、正しい数字を断ります。

```text
@@OUTTAIL:u09_totp:3@@
```

- **何が起きたか:** 利用者は、正しい数字を打ったのに、何度やっても断られます。スマートフォンの時計が直るまで、ログインできません。
- **直し方:** 前後 1 つ分（±30 秒）まで受け付け、その代わり、同じ数字の2回目は断ります（例9-2の `verify`）。サーバーの時計も、正しく合わせておきます（NTP）。

#### エラー3: 古い資料（base64 0.13 の頃）の書き方を写した（E0425）

`code_challenge` を作る行を、古い資料のとおり `base64::encode_config(…, base64::URL_SAFE_NO_PAD)` と書いた場合です（今の base64 は 0.23.1）。

```rust
@@FILE:mfa_lab/errs/e09_old_base64.rs@@
```

```text
@@OUT:u09_err_base64@@
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
