## UNIT 08: Better Auth との比較と、おすすめの構成

### この単元のゴール

- TypeScript の認証のライブラリ Better Auth（1.7.14）を、本物の Node.js で動かし、RustNote で確かめてきたこと（登録・重複・ログインの失敗・回数の制限・ほかのサイトからの送信・DB の中身・ログアウト）を、同じ順番で確かめられる
- 2つの作りの、同じ所（同じ文で断る・SameSite=Lax・Origin の確かめ）と、違う所（DB に入るもの・パスワードのハッシュ・期限・回数の制限）を、測った結果から説明できる
- 外の仕組みを使っても、**設定を間違えれば守りが効かない**ことを、3つの設定の間違い（秘密の鍵・開発のときの回数の制限・送り元の IP）で確かめられる
- RustNote のような Rust のアプリで、「自分で作る」「Better Auth を使う」「外の認証サービスを使う」のどれを選ぶかを、理由とともに言える

### たとえ話でつかむ

家の玄関の鍵を考えます。

- **自分で作る鍵:** 錠前の中の仕組みを、全部自分で知っています。どこが弱いかも、自分で確かめられます。その代わり、足りない機能（指紋で開ける、外出先から鍵の具合を見る）は、自分で作り足すことになります。
- **市販の錠前:** たくさんの家で使われていて、指紋や暗証番号の機能も、部品を足すだけで使えます。でも、**取り付け方を間違える**と（扉の枠に合わない・説明書の「本番用の設定」を飛ばす）、どんなに良い錠前でも効きません。そして、その錠前が、自分の家の扉（使っている言葉や仕組み）に付けられるかどうかも、確かめなければなりません。
- **警備会社に任せる:** 鍵の管理を、まるごと外の会社に頼みます。家の中のことは家で、出入りの確かめは会社で、と分けます（UNIT 09）。

| 玄関の鍵 | 認証の作り方 |
|---|---|
| 自分で作る鍵 | RustNote（Topcoat のセッション・Argon2id・自分の SQL） |
| 市販の錠前 | Better Auth（TypeScript のライブラリ。プラグインで機能を足す） |
| 取り付け方を間違えると効かない | `secret`・`rateLimit`・送り元の IP の設定（エラー1〜3） |
| 自分の扉に付けられるか | Better Auth は JavaScript の実行環境で動く。Rust のアプリからは別のサーバーになる |
| 警備会社に任せる | OIDC で外の認証サービスに任せる（UNIT 09） |

> **用語:** Better Auth（ベター・オース：TypeScript で書かれた、認証のためのライブラリ。Node.js などの JavaScript の実行環境で動き、メールとパスワードのログイン・セッション・回数の制限などを持ち、二段階認証などをプラグインで足せる）

<!-- -->

> **用語:** scrypt（エスクリプト：Argon2id と同じく、わざと重く作った、パスワード用のハッシュの方法。たくさんのメモリを使う。Better Auth の既定）

<!-- -->

> **用語:** 署名付きのクッキー（クッキーの値に、サーバーだけが知る秘密の鍵で作った「署名」を付けたもの。値を書き換えると署名が合わなくなるので、作り変えを見つけられる）

### 図で理解する

確かめた結果を、並べたものです（例8-1と、RustNote の UNIT 01〜07）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="180" height="34" rx="6" fill="#0f172a"/>
<text x="100" y="32" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">確かめたこと</text>
<rect x="200" y="10" width="250" height="34" rx="6" fill="#0f172a"/>
<text x="325" y="32" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">RustNote（Rust・Topcoat）</text>
<rect x="460" y="10" width="250" height="34" rx="6" fill="#0f172a"/>
<text x="585" y="32" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">Better Auth 1.7.14（Node.js）</text>
<text x="20" y="70" font-size="11" fill="#0f172a">パスワードのハッシュ</text>
<text x="325" y="70" font-size="11" fill="#0f172a" text-anchor="middle">Argon2id（m=19456, t=2, p=1）</text>
<text x="585" y="70" font-size="11" fill="#0f172a" text-anchor="middle">scrypt（N=16384, r=16, p=1）</text>
<text x="20" y="102" font-size="11" fill="#0f172a">DB に入るトークン</text>
<text x="325" y="102" font-size="11" fill="#047857" text-anchor="middle">SHA-256 のハッシュだけ（32 バイト）</text>
<text x="585" y="102" font-size="11" fill="#c2410c" text-anchor="middle">トークンそのもの（32 文字）</text>
<text x="20" y="134" font-size="11" fill="#0f172a">クッキーの値</text>
<text x="325" y="134" font-size="11" fill="#0f172a" text-anchor="middle">トークン（__Host-session）</text>
<text x="585" y="134" font-size="11" fill="#0f172a" text-anchor="middle">トークン ＋「.」＋ 署名</text>
<text x="20" y="166" font-size="11" fill="#0f172a">セッションの期限</text>
<text x="325" y="166" font-size="11" fill="#0f172a" text-anchor="middle">30 日</text>
<text x="585" y="166" font-size="11" fill="#0f172a" text-anchor="middle">7 日</text>
<text x="20" y="198" font-size="11" fill="#0f172a">登録済みのメール</text>
<text x="325" y="198" font-size="11" fill="#0f172a" text-anchor="middle">409（大文字も空白もそろえる）</text>
<text x="585" y="198" font-size="11" fill="#0f172a" text-anchor="middle">422（大文字はそろえ、空白は 400）</text>
<text x="20" y="230" font-size="11" fill="#0f172a">ログインの失敗</text>
<text x="325" y="230" font-size="11" fill="#047857" text-anchor="middle">同じ 401・同じ文・同じ時間</text>
<text x="585" y="230" font-size="11" fill="#047857" text-anchor="middle">同じ 401・同じ code・ほぼ同じ時間</text>
<text x="20" y="262" font-size="11" fill="#0f172a">回数の制限</text>
<text x="325" y="262" font-size="11" fill="#b91c1c" text-anchor="middle">無い（UNIT 04 の改善点）</text>
<text x="585" y="262" font-size="11" fill="#047857" text-anchor="middle">送り元ごとに 10 秒で 3 回（4 回目 429）</text>
<text x="20" y="294" font-size="11" fill="#0f172a">ほかのサイトから</text>
<text x="325" y="294" font-size="11" fill="#047857" text-anchor="middle">403（OriginPolicy）</text>
<text x="585" y="294" font-size="11" fill="#047857" text-anchor="middle">403（INVALID_ORIGIN）</text>
<text x="20" y="320" font-size="11" fill="#0f172a">ログアウト</text>
<text x="325" y="320" font-size="11" fill="#047857" text-anchor="middle">DB の行を消す</text>
<text x="585" y="320" font-size="11" fill="#047857" text-anchor="middle">DB の行を消す</text>
<line x1="10" y1="84" x2="710" y2="84" stroke="#e2e8f0" stroke-width="1"/>
<line x1="10" y1="116" x2="710" y2="116" stroke="#e2e8f0" stroke-width="1"/>
<line x1="10" y1="148" x2="710" y2="148" stroke="#e2e8f0" stroke-width="1"/>
<line x1="10" y1="180" x2="710" y2="180" stroke="#e2e8f0" stroke-width="1"/>
<line x1="10" y1="212" x2="710" y2="212" stroke="#e2e8f0" stroke-width="1"/>
<line x1="10" y1="244" x2="710" y2="244" stroke="#e2e8f0" stroke-width="1"/>
<line x1="10" y1="276" x2="710" y2="276" stroke="#e2e8f0" stroke-width="1"/>
<line x1="10" y1="304" x2="710" y2="304" stroke="#e2e8f0" stroke-width="1"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: RustNote と Better Auth の比べ。失敗の文をそろえる・SameSite=Lax・Origin の確かめ・ログアウトで行を消す、は同じ。違うのは、DB に入るもの・ハッシュの方法・期限・回数の制限の有無</div>
</div>

Rust のアプリで、認証をどう組み立てるかの、3つの選び方です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="226" height="230" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="123" y="36" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">A. 自分で作る</text>
<text x="123" y="56" font-size="11" fill="#475569" text-anchor="middle">RustNote の今の作り</text>
<text x="26" y="86" font-size="11" fill="#0f172a">・1つの Rust のサーバーで済む</text>
<text x="26" y="108" font-size="11" fill="#0f172a">・中身を全部、テストで確かめられる</text>
<text x="26" y="130" font-size="11" fill="#0f172a">・回数の制限・二段階認証・</text>
<text x="34" y="148" font-size="11" fill="#0f172a">パスワードの再設定は、自分で足す</text>
<text x="26" y="190" font-size="11" font-weight="700" fill="#047857">向く: 機能が少なく、</text>
<text x="26" y="208" font-size="11" font-weight="700" fill="#047857">中身を学びたい・握りたい</text>
<rect x="247" y="10" width="226" height="230" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="36" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">B. Better Auth</text>
<text x="360" y="56" font-size="11" fill="#475569" text-anchor="middle">JavaScript の実行環境で動く</text>
<text x="263" y="86" font-size="11" fill="#0f172a">・回数の制限などが最初からある</text>
<text x="263" y="108" font-size="11" fill="#0f172a">・二段階認証などはプラグイン</text>
<text x="263" y="130" font-size="11" fill="#0f172a">・Rust のアプリからは、別の</text>
<text x="271" y="148" font-size="11" fill="#0f172a">サーバー（Node.js）になる</text>
<text x="263" y="190" font-size="11" font-weight="700" fill="#1e40af">向く: 画面の側も</text>
<text x="263" y="208" font-size="11" font-weight="700" fill="#1e40af">TypeScript で作るアプリ</text>
<rect x="484" y="10" width="226" height="230" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="597" y="36" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">C. 外の認証サービス</text>
<text x="597" y="56" font-size="11" fill="#475569" text-anchor="middle">OIDC（UNIT 09）</text>
<text x="500" y="86" font-size="11" fill="#0f172a">・パスワードを預からない</text>
<text x="500" y="108" font-size="11" fill="#0f172a">・二段階認証も向こうで</text>
<text x="500" y="130" font-size="11" fill="#0f172a">・ログイン後のセッションは</text>
<text x="508" y="148" font-size="11" fill="#0f172a">自分のアプリで持つ（A と同じ）</text>
<text x="500" y="190" font-size="11" font-weight="700" fill="#c2410c">向く: 会社の人だけが使う・</text>
<text x="500" y="208" font-size="11" font-weight="700" fill="#c2410c">守る物が大きいアプリ</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: Rust のアプリの認証の3つの選び方。Rust だけで完結させるなら A、画面の側も TypeScript なら B、パスワードを預かりたくないなら C。C でも、ログインしたあとのセッションは A と同じ作りになる</div>
</div>

### まず動かす最小コード

#### 例8-1: Better Auth に、RustNote と同じ確かめをする

Better Auth は、`npm install better-auth@1.7.14` で入れました（Node.js v25.9.0。DB は、Node.js に入っている `node:sqlite` を使います）。RustNote のテストと同じように、HTTP のサーバーは立てずに、`auth.handler` に `Request` を直接渡します。

> **▼ このコードがやること（先に日本語で）:** ファイルの SQLite で Better Auth を作り、表を作ります。送り元の IP を `X-Forwarded-For` から読む設定にして、ふつうは毎回ちがう送り元から送ります。1. 登録（正しい・同じメール・大文字の同じメール・前後に空白の同じメール・短いパスワード）、2. ログインの失敗（登録していないメールと、パスワードの違いを、交互に4回ずつ。時間も測る）、3. 1つの送り元から続けて5回失敗し、別の送り元から正しいパスワードでログイン、4. ほかのサイトの `Origin` での登録、5. DB のセッションとパスワードの中身、6. ログアウトのあとのセッションの行の数、を表示します。

```javascript
@@FILE:ba_demo/probe.mjs@@
```

```powershell
node probe.mjs
```

```text
@@OUT:u08_probe@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 同じ所 — 失敗の文・クッキー・Origin・ログアウト

- **ログインの失敗:** 登録していないメールも、パスワードの違いも、`401` と `INVALID_EMAIL_OR_PASSWORD`（メールかパスワードが違う）の1つの code でした。時間も、63〜67 ms と 63〜66 ms で、ほぼ同じです。RustNote と同じく、文と時間の両方をそろえています（UNIT 04）。
- **クッキー:** `HttpOnly; SameSite=Lax` は RustNote と同じです（この例は `baseURL` が `http://` なので、`Secure` は付いていません）。
- **ほかのサイトから:** `Origin: https://evil.example` の登録は `403`（`INVALID_ORIGIN`）で、RustNote の OriginPolicy と同じ働きです（UNIT 06）。
- **ログアウト:** `sign-out` で、セッションの行が1つ減りました（2 行から 1 行。もう1行は、3で別の送り元からログインした分）。DB の行を消すのは、RustNote と同じです（UNIT 05）。

##### 解説2: 違う所 — DB に入るもの・ハッシュ・期限・回数の制限

- **DB のトークン:** `session.token` には、**トークンそのもの**（32 文字）が入っていて、クッキーの値は「トークン ＋ `.` ＋ 署名（44 文字）」でした。RustNote は、トークンの SHA-256 だけを DB に入れます（UNIT 01・05）。Better Auth は、クッキーに秘密の鍵（`secret`）の署名を付けるので、DB のトークンだけでは、正しいクッキーを作れません。でも、**DB と秘密の鍵の両方**が漏れたら、成りすませます。RustNote は、DB が漏れても、トークンを作り直せません。守り方の考えが違います。
- **パスワード:** `account.password` は「salt（32 文字）：ハッシュ（128 文字）」で、方法は scrypt（`N=16384, r=16, p=1`、64 バイト）です（`@better-auth/utils` の `password.node.mjs` で確かめました）。パスワードを NFKC（全角と半角などをそろえる正規化）にしてから計算するので、全角で打っても半角で打っても同じパスワードになります。RustNote は、打ったとおりに扱います（UNIT 03 の問3）。
- **期限:** `Max-Age=604800`（7 日）で、RustNote の 30 日より短いです。
- **登録:** 登録済みは `422`、大文字の同じメールも `422`（小文字にそろえている）、前後に空白のメールは `400`（`VALIDATION_ERROR`、空白を取らずに、形が違うと断る）でした。RustNote は、空白も取ってから比べ、登録済みは `409` です（UNIT 03）。パスワードの長さの決まり（8〜128 文字）は、同じです。
- **回数の制限:** 1つの送り元から続けて失敗すると、**4回目から `429`**（Too many requests）になり、別の送り元からは、正しいパスワードでログインできました。Better Auth は、ログインと登録に「送り元ごとに 10 秒で 3 回」の決まりを、最初から持っています。RustNote には無い守りです（UNIT 04 の例4-3）。

### もう一歩: 実務に近い形

#### おすすめの構成 — RustNote なら、どれを選ぶか

図の3つの選び方を、RustNote に当てはめます。

- **RustNote は A（自分で作る）のままがよい、と考えます。** RustNote は、Rust と Topcoat だけで、1つのサーバーとして動きます。Better Auth を使うと、Node.js のサーバーをもう1つ動かし、RustNote の各ページで「このクッキーは、Better Auth のどのセッションか」を、別のサーバーに問い合わせる（または、同じ DB を読む）作りになります。動かす物が2つになり、片方が止まるとログインできなくなります。
- そのうえで、A に足りない物を、優先する順に足します。
  1. **回数の制限**（UNIT 04 の例4-3。メールごとと送り元ごと）。Better Auth の「送り元ごとに 10 秒で 3 回」は、決め方の目安になります。
  2. **パスワードの再設定**（メールを送る仕組みが要る）と、**二段階認証**。
  3. 守る物が大きくなったら、C（外の認証サービス）に切り替え、パスワードそのものを預からないようにします（UNIT 09）。
- **B（Better Auth）が向くのは**、画面の側を TypeScript（たとえば React で作る画面）で作り、そのサーバーも JavaScript の実行環境で動かすアプリです。その場合は、Better Auth を、その JavaScript のサーバーの中で使い、二段階認証などはプラグイン（1.7.14 の `two-factor`・`magic-link`・`email-otp`・`captcha`・`haveibeenpwned` など）で足します。
- どれを選んでも、**この巻で書いた確かめのテスト**（失敗の文と時間・セッションの一生・CSRF・認可の表）は、そのまま使えます。外の仕組みに任せた所こそ、自分のテストで確かめます。

### よくあるエラーと読み方

#### エラー1: secret を設定しないまま、本番で動かした

`secret` を書かず、環境変数 `BETTER_AUTH_SECRET` も無いまま、本番（`NODE_ENV=production`）と、開発（`NODE_ENV` 無し）で動かした場合です。

```javascript
@@FILE:ba_demo/e1_no_secret.mjs@@
```

```text
@@OUT:u08_err_secret@@
```

- **何が起きたか:** 本番では、最初のリクエストのときに `BetterAuthError: You are using the default secret`（決まりの秘密の鍵を使っています。`BETTER_AUTH_SECRET` を設定してください）で止まりました。開発では、そのまま `200` で動きます。
- **困ること:** 開発のときは、決まりの秘密の鍵で黙って動くので、気づきません。本番で `NODE_ENV=production` を付け忘れると、誰でも知っている鍵でクッキーに署名することになり、クッキーを作られてしまうかもしれません。Better Auth は、本番では止めて知らせてくれますが、「本番である」ことは、こちらが正しく伝える必要があります。RustNote の、秘密の値を設定から読む考え方（第9巻 UNIT 06）と同じく、秘密の鍵は環境変数で渡します。

#### エラー2（エラーが出ない間違い）: 開発のときは、回数の制限が効いていない

`rateLimit` の設定を書かずに、開発のとき（`NODE_ENV` が production でない）に、1つの送り元から6回失敗した場合です。

```javascript
@@FILE:ba_demo/e2_dev_limit.mjs@@
```

```text
@@OUT:u08_err_dev@@
```

- **何が起きたか:** 6回とも `401` で、`429` になりませんでした。Better Auth の回数の制限は、開発のときは、決まりで**切ってあります**（例8-1では `rateLimit: { enabled: true }` で、わざと入れました）。
- **困ること:** 手元で「回数の制限が効くか」を試すと、効かないように見えます。逆に、効く前提で書いたテストが、開発では通らないこともあります。本番の設定と、開発の設定の違いを、知っておく必要があります。

#### エラー3（エラーが出ない間違い）: 送り元の IP が分からないまま、回数の制限を入れた

回数の制限を入れたが、手前の中継のサーバー（リバースプロキシ）が、送り元の IP を伝えていない場合です。4人が、1回ずつログインに失敗します。

```javascript
@@FILE:ba_demo/e3_no_ip.mjs@@
```

```text
@@OUTGREP:u08_err_ip:^(user|.*WARN.*client IP)@@
```

- **何が起きたか:** `Rate limiting could not determine a client IP and is falling back to a single shared per-path bucket`（送り元の IP が分からないので、全員で1つの数えの箱を使います）という注意が出て、**4人目の user4 が、1回目なのに `429`** になりました。4人の失敗が、まとめて数えられたからです。
- **困ること:** 本当の利用者が、ほかの人（攻撃する人も含む）の失敗のせいで、ログインできなくなります。攻撃する人は、ときどき失敗を送るだけで、全員を締め出せます（UNIT 04 の問2の「本人の締め出し」が、全員に起きる形です）。
- **直し方:** 注意の文のとおり、中継のサーバーが送り元の IP を伝えるヘッダー（`X-Forwarded-For` など）を、信じてよい物として `advanced.ipAddress.ipAddressHeaders` か `trustedProxies` で指定します（例8-1の設定）。ただし、中継のサーバーを通らずに届くリクエストでは、このヘッダーは誰でも書けるので、「中継のサーバーが付けたもの」だけを信じる設定にします。

### RustNote ではここで使う

- RustNote の認証の部品は、`src/auth.rs`（ハッシュ・照合・ログイン・ログアウト・ログイン中のユーザー）・`src/repository/users.rs` と `sessions.rs`・`src/routes/signup.rs`・`login.rs`・`logout.rs` です。全部を合わせても、Rust のファイルが数個で、この巻で全部の動きを確かめました。
- RustNote に無くて Better Auth に有ったもの: 回数の制限（ログイン・登録）、二段階認証などのプラグイン。RustNote に有って Better Auth の例に無かったもの: DB にトークンのハッシュだけを入れる作り、`__Host-` の付いたセッションのクッキー。
- この巻の練習用の比較は、`ba_demo/`（`probe.mjs`・`lab.mjs`・`e1〜e3`）にあります。RustNote のコードは、変えていません。

### 練習問題

**問1:** Better Auth の DB の `session.token` が漏れたとします。それだけで、alice として成りすませるでしょうか。RustNote の `sessions.token_hash` が漏れた場合と、比べてください。

**問2:** 例8-1の3で、4回目からは `429` になりました。攻撃する人が、送り元を毎回変えられるとすると、この制限はどこまで役に立つでしょうか。

**問3:** あなたのチームが、画面を React（TypeScript）で作り、その画面のためのサーバーも Node.js で動かす新しいアプリを作ります。認証は、A・B・C のどれから考えるとよいでしょうか。

### 解答と解説

**問1の解答:**

- `session.token` だけでは、成りすませません。クッキーの値は「トークン ＋ `.` ＋ 署名」で、署名を作るには、サーバーの秘密の鍵（`secret`）が要るからです（例8-1の5）。ただし、秘密の鍵も一緒に漏れれば（設定のファイルや環境変数が見られた、など）、正しいクッキーを作れます。
- RustNote の `sessions.token_hash` は、トークンの SHA-256 です。SHA-256 は、元に戻せないので、DB と、サーバーの設定の両方が漏れても、トークンを作れません（UNIT 01 の脅威モデル）。「DB が漏れたとき」の守りは、RustNote のほうが一段厚い作りです。

**問2の解答:**

- 1つの送り元からの総当たりは、10 秒に3回まで、1日に約 2 万 6 千回（86,400 ÷ 10 × 3 = 25,920）に抑えられます。止めはしませんが、遅くできます。
- 送り元を毎回変えられる攻撃（たくさんの機械を使うなど）には、送り元ごとの制限だけでは足りません。UNIT 04 の例4-3のように、**メールごとの制限**も組み合わせ、さらに二段階認証で、パスワードが当たっても入れないようにします。Better Auth なら、`two-factor` や `captcha` のプラグインが、その役目です。

**問3の解答:**

- **B（Better Auth）から**考えるのがよいでしょう。画面もサーバーも TypeScript なら、Better Auth を、そのサーバーの中で使えるので、別のサーバーを足す必要がありません。回数の制限や、二段階認証などのプラグインも、すぐに使えます。
- ただし、エラー1〜3のような設定の確かめは、自分で行います。この巻の確かめのテスト（失敗の文と時間・セッションの一生・CSRF・認可の表）を、そのアプリにも書き直して残します。会社の人だけが使うアプリなら、C（外の認証サービス）も候補です。

### まとめ

- Better Auth 1.7.14 で、RustNote と同じ確かめをすると、失敗の文と時間をそろえる・SameSite=Lax・Origin の確かめ・ログアウトで行を消す、は同じだった。違うのは、DB にトークンそのものを入れて、クッキーに署名を付けること、scrypt、7 日の期限、そして、送り元ごとに 10 秒で 3 回の回数の制限が最初からあること。
- 外の仕組みでも、設定を間違えれば効かない。秘密の鍵は環境変数で渡し、開発のときは回数の制限が切ってあることを知り、送り元の IP を正しく読ませる（読めないと、全員で1つの数えになり、4人目が締め出される）。
- Rust だけで動く RustNote は、自分で作る作り（A）のまま、回数の制限・パスワードの再設定・二段階認証を足していくのがよい。画面もサーバーも TypeScript なら Better Auth（B）、パスワードを預かりたくなければ外の認証サービス（C）を考える。
