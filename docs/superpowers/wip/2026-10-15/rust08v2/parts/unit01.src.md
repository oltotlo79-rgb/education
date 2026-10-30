## UNIT 01: 認証・セッション・認可と脅威モデル

### この単元のゴール

- 認証（あなたは誰か）・セッション（さっきの人と同じか）・認可（それをしてよいか）の3つの確かめを区別し、RustNote でそれぞれが失敗したときの返事（303・404）を、テストで確かめられる
- ブラウザに渡すクッキー（トークン）と、DB に保存するもの（トークンのハッシュ）が違うことを、実際の値で説明できる
- ほかのサイトから送られてきたフォーム（CSRF）を、Topcoat が 403 で止めることを確かめられる
- 守るもの・攻める人・入口・信頼の境目を書き出した「脅威モデル」を作り、RustNote の守りと、まだ無い守りを表にできる

### たとえ話でつかむ

会員制のコンサート会場を考えます。

- **入口の受付**で、会員証と顔写真を見て「この人は本人だ」と確かめます。これが**認証**です。
- 本人だと分かったら、**リストバンド**を腕に付けてもらいます。会場の中では、毎回会員証を見せなくても、リストバンドを見せれば「さっき受付を通った人だ」と分かります。これが**セッション**です。
- 楽屋の前の係は、リストバンドを見て「会場に入ってよい人だ」と分かっても、それだけでは通しません。「この人は、この楽屋に入ってよいか」を、別に確かめます。これが**認可**です。

会場の警備の担当者は、開場の前に**警備の計画**を立てます。「守るもの（お客さんの財布・楽屋の機材）」「攻めてくる人（偽の会員証を作る人・落ちたリストバンドを拾った人・外から呼び出して入口を開けさせる人）」「入口（正面・裏口・搬入口）」を書き出してから、どこに係を立てるかを決めます。この計画が**脅威モデル**です。

| コンサート会場 | RustNote |
|---|---|
| 受付で会員証を確かめる | **認証**（メールアドレスとパスワードで、本人かを確かめる） |
| リストバンド | **セッション**（クッキーの `__Host-session` に入ったトークン） |
| リストバンドの台帳（番号の控え） | DB の `sessions` 表（トークンそのものではなく、そのハッシュを控える） |
| 楽屋の前の係 | **認可**（このノートの持ち主か。SQL の `user_id = ?`） |
| 警備の計画 | **脅威モデル**（守るもの・攻める人・入口・信頼の境目） |
| 外から呼び出して入口を開けさせる | **CSRF**（ほかのサイトから、ログイン中の人のブラウザにフォームを送らせる） |

> **用語:** 認証（にんしょう。authentication：相手が「名乗っているとおりの人か」を確かめること）

<!-- -->

> **用語:** 認可（にんか。authorization：確かめた相手に、「その操作をしてよいか」を決めること）

<!-- -->

> **用語:** セッション（session：一度認証した人を、次のリクエストからも「同じ人」と分かるようにする仕組み。RustNote ではクッキーのトークンで行う）

<!-- -->

> **用語:** 脅威モデル（きょういモデル。threat model：守るもの・攻める人・入口・信頼の境目を書き出し、どんな攻撃がありうるかを整理したもの）

### 図で理解する

RustNote にリクエストが届くと、次の順番で確かめます。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-0101" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="40" width="120" height="70" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="70" y="68" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">リクエスト</text>
<text x="70" y="90" font-size="11" fill="#475569" text-anchor="middle">GET /notes/1</text>
<rect x="160" y="40" width="160" height="70" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="240" y="64" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">① セッション → 認証</text>
<text x="240" y="84" font-size="11" fill="#0f172a" text-anchor="middle">クッキーのトークンは</text>
<text x="240" y="100" font-size="11" fill="#0f172a" text-anchor="middle">sessions 表にあるか</text>
<rect x="350" y="40" width="160" height="70" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="430" y="64" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">② 認可</text>
<text x="430" y="84" font-size="11" fill="#0f172a" text-anchor="middle">ノート 1 の持ち主は</text>
<text x="430" y="100" font-size="11" fill="#0f172a" text-anchor="middle">この人か</text>
<rect x="540" y="40" width="170" height="70" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="625" y="68" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">③ 処理して返す</text>
<text x="625" y="90" font-size="11" fill="#0f172a" text-anchor="middle">200 OK</text>
<line x1="130" y1="75" x2="156" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0101)"/>
<line x1="320" y1="75" x2="346" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0101)"/>
<line x1="510" y1="75" x2="536" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0101)"/>
<text x="335" y="64" font-size="11" fill="#047857" text-anchor="middle">はい</text>
<text x="525" y="64" font-size="11" fill="#047857" text-anchor="middle">はい</text>
<line x1="240" y1="110" x2="240" y2="166" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0101)"/>
<line x1="430" y1="110" x2="430" y2="166" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0101)"/>
<text x="252" y="142" font-size="11" fill="#c2410c">いいえ</text>
<text x="442" y="142" font-size="11" fill="#c2410c">いいえ</text>
<rect x="160" y="170" width="160" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2" stroke-dasharray="6 4"/>
<text x="240" y="196" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">303 → /login</text>
<text x="240" y="218" font-size="11" fill="#475569" text-anchor="middle">「ログインしてください」</text>
<rect x="350" y="170" width="160" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2" stroke-dasharray="6 4"/>
<text x="430" y="196" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">404 Not Found</text>
<text x="430" y="218" font-size="11" fill="#475569" text-anchor="middle">ノートがあることも隠す</text>
<text x="360" y="276" font-size="12" fill="#0f172a" text-anchor="middle">① は auth.rs の require_user、② は repository の SQL の user_id = ? が受け持つ</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: RustNote の確かめの順番。セッションから「誰か」が分からなければ 303 でログイン画面へ、持ち主でなければ 404 にする</div>
</div>

- **① セッションと認証:** ブラウザが送ってきたクッキーのトークンから、ログイン中のユーザーを探します。見つからなければ、「ログインしてください」を出すために、303 でログイン画面へ移します（`auth.rs` の `require_user`）。
- **② 認可:** ユーザーが分かっても、そのノートの持ち主かどうかは別に確かめます。RustNote は、ノートを探す SQL に必ず `user_id = ?`（持ち主の条件）を入れるので、他人のノートは「見つからない」ことになり、404 を返します。「ある」ことを知らせる 403 にしないのは、他人のノートの番号を探り当てられないようにするためです（第9巻 UNIT 01）。

もう1つの図は、RustNote の脅威モデルです。誰が、どこから、何を狙うのかを描きます。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 360" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-0102" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#dc2626"/></marker>
</defs>
<rect x="250" y="20" width="460" height="320" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2" stroke-dasharray="6 4"/>
<text x="480" y="44" font-size="12" font-weight="700" fill="#475569" text-anchor="middle">信頼の境目の内側（サーバーと DB）</text>
<rect x="290" y="64" width="180" height="120" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="380" y="88" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">RustNote</text>
<text x="380" y="112" font-size="11" fill="#0f172a" text-anchor="middle">入口: フォーム（登録・ログイン）</text>
<text x="380" y="132" font-size="11" fill="#0f172a" text-anchor="middle">入口: クッキー（セッション）</text>
<text x="380" y="152" font-size="11" fill="#0f172a" text-anchor="middle">入口: URL の ID（/notes/1）</text>
<rect x="500" y="64" width="190" height="260" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="595" y="88" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">守るもの（DB）</text>
<text x="595" y="118" font-size="12" fill="#0f172a" text-anchor="middle">パスワード</text>
<text x="595" y="136" font-size="11" fill="#475569" text-anchor="middle">→ Argon2id のハッシュだけ</text>
<text x="595" y="168" font-size="12" fill="#0f172a" text-anchor="middle">セッションのトークン</text>
<text x="595" y="186" font-size="11" fill="#475569" text-anchor="middle">→ SHA-256 のハッシュだけ</text>
<text x="595" y="218" font-size="12" fill="#0f172a" text-anchor="middle">ノートの中身</text>
<text x="595" y="236" font-size="11" fill="#475569" text-anchor="middle">→ 持ち主だけが読める</text>
<text x="595" y="268" font-size="12" fill="#0f172a" text-anchor="middle">登録したメールアドレス</text>
<text x="595" y="286" font-size="11" fill="#475569" text-anchor="middle">→ 他人に知られない</text>
<rect x="290" y="206" width="180" height="118" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="380" y="230" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">RustNote の守り</text>
<text x="380" y="254" font-size="11" fill="#0f172a" text-anchor="middle">require_user（303）</text>
<text x="380" y="274" font-size="11" fill="#0f172a" text-anchor="middle">user_id = ?（404）</text>
<text x="380" y="294" font-size="11" fill="#0f172a" text-anchor="middle">Origin の検査（403）</text>
<text x="380" y="314" font-size="11" fill="#0f172a" text-anchor="middle">同じ文で断る（401）</text>
<rect x="10" y="40" width="200" height="60" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="110" y="64" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">パスワードを試す人</text>
<text x="110" y="84" font-size="11" fill="#0f172a" text-anchor="middle">何回もログインする</text>
<rect x="10" y="116" width="200" height="60" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="110" y="140" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">ほかのサイト</text>
<text x="110" y="160" font-size="11" fill="#0f172a" text-anchor="middle">ブラウザにフォームを送らせる</text>
<rect x="10" y="192" width="200" height="60" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="110" y="216" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">ほかの利用者（bob）</text>
<text x="110" y="236" font-size="11" fill="#0f172a" text-anchor="middle">URL の数字を変える</text>
<rect x="10" y="268" width="200" height="60" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="110" y="292" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">DB を盗んだ人</text>
<text x="110" y="312" font-size="11" fill="#0f172a" text-anchor="middle">中身をそのまま読む</text>
<line x1="210" y1="70" x2="286" y2="110" stroke="#dc2626" stroke-width="2" marker-end="url(#ar-08-0102)"/>
<line x1="210" y1="146" x2="286" y2="130" stroke="#dc2626" stroke-width="2" marker-end="url(#ar-08-0102)"/>
<line x1="210" y1="222" x2="286" y2="150" stroke="#dc2626" stroke-width="2" marker-end="url(#ar-08-0102)"/>
<path d="M210,298 H240 V336 H595 V328" fill="none" stroke="#dc2626" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-08-0102)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: RustNote の脅威モデル。左の4種類の「攻める人」が、3つの入口（フォーム・クッキー・URL の ID）から、DB の中の守るものを狙う。DB を盗んだ人は入口を通らないので、保存する形そのもので守る</div>
</div>

- **入口は3つです。** フォーム（登録・ログイン・ノートの作成）、クッキー（セッションのトークン）、URL の ID（`/notes/1` の `1`）。攻める人は、この3つのどれかに、変な値を入れてきます。
- **DB を盗んだ人**だけは、入口を通りません。だから、パスワードとトークンは、盗まれても元に戻せない形（ハッシュ）で保存します。「入口で止める」守りと、「盗まれても困らない形で持つ」守りの2種類が必要なのです。
- 点線の四角が**信頼の境目**です。境目の外（ブラウザ・ほかのサイト）から来た値は、どれも信じずに、内側で確かめ直します。

### まず動かす最小コード

#### 例1-1: 3つの確かめを、1つのテストで見る

> **▼ このコードがやること（先に日本語で）:** ログインしていない人が `/notes` を開く（①認証が無い）、alice が登録してクッキーを付けて開く（②セッションで本人と分かる）、alice が作ったノートを alice と bob がそれぞれ開く（③認可）の3つを、順に試します。それぞれの返事を表示します。

```rust
@@FILE:rustnote/tests/u01_three_checks.rs@@
```

コピーした `rustnote` フォルダの `tests/u01_three_checks.rs` として保存して実行します。`--nocapture` で `println!` を表示し、`--test-threads=1` で、表示が混ざらないようにします。

```powershell
cargo test --test u01_three_checks -- --nocapture --test-threads=1
```

**▼ 実行結果**（最初の1回は `Compiling` の行が続きます。ここでは省いています）

```text
@@OUTGREP:u01_three:^(?!   Compiling)@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: ① 認証が無いと、303 でログイン画面へ

- クッキーを付けずに `/notes` を開くと、`303 See Other`（移動先 `/login`）でした。RustNote のノートの画面は、どれも最初に `require_user(cx)` を呼び、ログイン中のユーザーがいなければ、ログイン画面へ移します。
- 1行目の `test three_checks ... GET /notes -> 303 See Other (1.2911ms)` は、テストの名前の表示と、RustNote のアクセスログ（標準エラー出力）が、同じ行に並んだものです。

##### 解説2: ② クッキーを見せると、本人と分かる

- `app.sign_up(...)` は、登録のフォームを送り、返事の `Set-Cookie` から `__Host-session=...` を取り出して返す、テスト用の道具です（`tests/common/mod.rs`）。そのクッキーを付けて `/notes` を開くと、`200 OK` でした。
- 2回目からは、パスワードを送っていません。クッキーのトークンだけで「alice だ」と分かったのです。これがセッションの役目です。

##### 解説3: ③ ログインしていても、他人のノートは 404

- alice のノート（`/notes/1`）を alice が開くと `200 OK`、bob が開くと `404 Not Found` でした。bob はログインしている（認証は通っている）のに、認可で断られたのです。
- 認証と認可は、別の確かめです。「ログインしているから、何でも見てよい」わけではありません。

### もう一歩: 実務に近い形

#### 例1-2: クッキーに入るものと、DB に保存されるものを比べる

> **▼ このコードがやること（先に日本語で）:** 登録のフォームを送り、返事の `Set-Cookie` から、セッションのクッキーの名前・値（トークン）の先頭・属性を表示します。続いて、DB の `sessions` 表に入った行（トークンのハッシュの長さ・持ち主・期限）を表示し、トークンそのものと同じ値が DB に入っていないことを確かめます。

```rust
@@FILE:rustnote/tests/u01_session.rs@@
```

```powershell
cargo test --test u01_session -- --nocapture
```

**▼ 実行結果**

```text
@@OUT:u01_session@@
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 220" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-0103" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="30" width="250" height="160" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="145" y="56" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">ブラウザ（クッキー）</text>
<text x="145" y="86" font-size="12" fill="#0f172a" text-anchor="middle">__Host-session = トークン</text>
<text x="145" y="106" font-size="11" fill="#475569" text-anchor="middle">46 文字のランダムな値</text>
<text x="145" y="136" font-size="11" fill="#0f172a" text-anchor="middle">HttpOnly・Secure・SameSite=Lax</text>
<text x="145" y="156" font-size="11" fill="#0f172a" text-anchor="middle">Path=/・30 日</text>
<rect x="450" y="30" width="250" height="160" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="575" y="56" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">DB（sessions 表）</text>
<text x="575" y="86" font-size="12" fill="#0f172a" text-anchor="middle">token_hash（32 バイト）</text>
<text x="575" y="106" font-size="11" fill="#475569" text-anchor="middle">トークンの SHA-256</text>
<text x="575" y="136" font-size="11" fill="#0f172a" text-anchor="middle">user_id・expires_at</text>
<text x="575" y="156" font-size="11" fill="#0f172a" text-anchor="middle">トークンそのものは無い</text>
<line x1="270" y1="100" x2="446" y2="100" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0103)"/>
<text x="358" y="90" font-size="11" fill="#475569" text-anchor="middle">ハッシュにして照合</text>
<text x="358" y="124" font-size="11" fill="#475569" text-anchor="middle">（逆には戻せない）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: クッキーに入るトークンと、DB に保存されるハッシュ。DB にはトークンそのものが無いので、DB を盗まれても、そのままではログイン中の人になりすませない</div>
</div>

#### ▼ コードを1つずつ分解して解説

##### 解説1: クッキーの名前と属性が、トークンを守っている

- `__Host-session` の `__Host-` は、ブラウザに「`Secure` が付いていて、`Path=/` で、ほかのドメインと共有しないクッキーだけを受け付けよ」と求める、特別な名前の付け方です。名前の決まりを守らないクッキーは、ブラウザが受け付けません。
- `HttpOnly` は「ページの JavaScript からは読めない」、`Secure` は「HTTPS のときだけ送る」、`SameSite=Lax` は「ほかのサイトからの POST には付けて送らない」という意味です。`Max-Age=2592000` は 30 日（2,592,000 秒）で消えるという意味です。属性の1つずつは、UNIT 06 でくわしく扱います。
- これらの属性は、Topcoat の `SessionConfig::default()` が付けています（RustNote の `routes.rs` の `.sessions(SessionConfig::default())`）。

##### 解説2: DB には、トークンではなくハッシュを保存する

- `sessions` 表の `token_hash` は 32 バイトでした。Topcoat が、トークンから SHA-256（1方向にしか計算できない変換）で作った値です。クッキーの値（46 文字）とは、長さも中身も違います。最後の行で、トークンと同じ値の行が 0 件であることも確かめました。
- RustNote の `auth.rs` の `log_in` は、`session::start(cx)` でトークンを発行してクッキーに入れ、返ってきた `session.token_hash` だけを `sessions` 表に保存します。次のリクエストでは、クッキーのトークンを同じようにハッシュにして、表の中を探します（`load_current_user`）。
- こうしておくと、DB のファイルが盗まれても、表に入っているのはハッシュだけなので、それをクッキーに入れてもログインできません（問3）。

#### 例1-3: ほかのサイトから送られてきたフォームを、Topcoat が止めるか

ログイン中の人のブラウザには、RustNote のクッキーが入っています。悪いサイトが、そのブラウザに「RustNote の `/notes` へ POST するフォーム」を自動で送らせたら、どうなるでしょう。これが **CSRF**（クロスサイト・リクエスト・フォージェリ）という攻撃です。

> **▼ このコードがやること（先に日本語で）:** alice のクッキーを付けて、ノートを作る POST を4通り送ります。ヘッダーが何も無い場合、同じサイトのフォームから送った場合（`Origin: http://127.0.0.1:3000`）、ほかのサイトのフォームから送った場合（`Origin: https://evil.example`）、ブラウザが「ほかのサイトから」と印を付けた場合（`Sec-Fetch-Site: cross-site`）です。それぞれの返事と、最後に保存されたノートの数を表示します。

```rust
@@FILE:rustnote/tests/u01_cross_site.rs@@
```

```powershell
cargo test --test u01_cross_site -- --nocapture
```

**▼ 実行結果**

```text
@@OUT:u01_cross@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: ほかのサイトからの POST は、403 で止まる

- `Origin: https://evil.example` と `Sec-Fetch-Site: cross-site` の2つは、`403 Forbidden`（本文は `forbidden`）でした。クッキーは正しいのに、断られています。保存されたノートは 2 件で、断られた2回の分は保存されていません。
- ブラウザは、フォームを送るとき、`Origin`（どのサイトのページから送ったか）と `Sec-Fetch-Site`（同じサイトからか、ほかのサイトからか）を、自動で付けます。ページの JavaScript が勝手に書き換えることはできません。Topcoat は、この2つを見て、ほかのサイトから来た「状態を変えるリクエスト」（GET・HEAD・OPTIONS 以外）を止めます。この守りは Topcoat に最初から入っていて、RustNote は何も書いていません。

##### 解説2: ヘッダーが無いリクエストは、通る

- `Origin` も `Sec-Fetch-Site` も無いリクエスト（curl やテストの道具など）は、303 で通りました。CSRF は「ログイン中の人のブラウザに、勝手に送らせる」攻撃です。ブラウザは必ずこのヘッダーを付けるので、ヘッダーが無いリクエストは、ブラウザから来たものではありません。そうしたリクエストを送れるのは、クッキーを自分で持っている人だけです。
- 同じサイトのフォームから送った場合（`Origin: http://127.0.0.1:3000`）も通りました。`Origin` と、リクエストの `Host`（`127.0.0.1:3000`）が一致したからです。

##### 解説3: 403 のときは、アクセスログに行が出ない

- 出力をよく見ると、303 の2回には `POST /notes -> 303 See Other (...)` のアクセスログがありますが、403 の2回にはありません。Topcoat の Origin の検査は、RustNote のアクセスログの層（`routes.rs` の `access_log`）よりも**前**で動くからです。攻撃を止めた記録を残したいときは、この点に気をつけます（UNIT 10）。

### RustNote の脅威モデルの表

ここまでの実験をもとに、RustNote の脅威モデルを表にします。この表が、この巻の地図です。

| 攻める人と攻め方 | 狙う入口 | RustNote の守り | 確かめた所・くわしい単元 |
|---|---|---|---|
| ほかの利用者が、URL の数字を変える（IDOR） | URL の ID | すべての SQL に `user_id = ?`、他人のノートは 404 | 例1-1・エラー3・UNIT 07 |
| ログインしていない人が、ノートの画面を開く | URL | `require_user` で 303、ログイン画面へ | 例1-1・UNIT 05 |
| ほかのサイトが、ブラウザにフォームを送らせる（CSRF） | フォーム・クッキー | Topcoat の Origin の検査で 403、`SameSite=Lax`、状態を変える操作は POST だけ | 例1-3・UNIT 06 |
| ページに入れた悪い JavaScript が、クッキーを盗む | クッキー | `HttpOnly`（JavaScript から読めない）、`view!` の自動エスケープ | 例1-2・UNIT 06 |
| DB を盗んだ人が、パスワードとトークンを読む | （入口を通らない） | パスワードは Argon2id、トークンは SHA-256 のハッシュだけを保存 | 例1-2・UNIT 02・05 |
| 失敗の文の違いから、登録済みのメールを探る | ログインのフォーム | ログインは「メールアドレスまたはパスワードが違います」の同じ文、未登録でもダミーのハッシュを計算 | UNIT 04 |
| パスワードを何回も試す（総当たり） | ログインのフォーム | **守りが無い**（回数の制限が無い） | 問1・UNIT 04 |
| ログアウトした後の古いクッキーを使う | クッキー | ログアウトで `sessions` の行を消す | 問2・UNIT 05 |

- 表の最後から2行目のように、**守りが無い**ことも、はっきり書きます。脅威モデルの目的は、「全部守れている」と言うことではなく、「どこが弱いかを、みんなが知っている」状態にすることです。
- 新規登録では、すでに登録されたメールアドレスを入れると「そのメールアドレスは既に登録されています」（409）と表示されます。使いやすさのためですが、登録済みかどうかが他人に分かる、という弱さでもあります。どちらを選ぶかは、UNIT 03 で考えます。

### よくあるエラーと読み方

#### エラー1: 403 Forbidden（本文は forbidden だけ）

例1-3で、ほかのサイトから送ったときの返事です。自分でフォームを作ってブラウザで試したのに 403 になった、という場面でも出会います。

```text
ほかのサイトのフォーム（Origin: https://evil.example）
    → 403 Forbidden　本文: "forbidden"
```

- **何が起きたか:** Topcoat の Origin の検査が、「このリクエストは、ほかのサイトのページから来た」と判断して止めました。本文は `forbidden` の1語だけで、RustNote のエラー画面も、アクセスログの行も出ません。
- **自分のサイトなのに 403 になるとき:** フォームのページを表示したサイトと、送り先の RustNote が、別のサイトとして扱われている場合です。たとえば、フォームだけを書いた HTML ファイルを別の開発用のサーバー（別のポート）で開いて、そこから RustNote へ送ると、`Origin` と `Host` のポートが違うので止まります。また、RustNote の手前に置いた中継のサーバー（リバースプロキシ）が `Host` を書き換える設定だと、同じサイトからでも食い違います。フォームは RustNote 自身の画面から送り、中継のサーバーは `Host` をそのまま渡す設定にします。ほかのサイトからの送信をわざと受け付けたいときは、Topcoat の `OriginPolicy` で、信じてよい送り元を指定できます。守りそのものを外す設定もありますが、使わないでください。

#### エラー2: 持ち主を渡さずにノートを探そうとした（E0308・E0061）

RustNote のリポジトリを使って、ノートを ID だけで探そうとした場合です（`examples/e01_find_without_owner.rs` に保存して試します）。

```rust
@@FILE:errs/e01_find_without_owner.rs@@
```

```powershell
cargo build --example e01_find_without_owner
```

```text
@@OUT:u01_err_owner@@
```

- **何が起きたか:** エラーが2つ出ました。1つ目の `E0308` は「`UserId` が来るはずの場所に、`NoteId` が来ました」です。`find` の1つ目の引数は持ち主（`owner: UserId`）なので、ノートの ID を渡すと型が合いません。2つ目の `E0061` は「このメソッドは引数を2つとるのに、1つしか渡されていません」です。`note:` が、`src/repository/notes.rs` の 21 行目の `async fn find(&self, owner: UserId, id: NoteId)` を指しています。
- **これはエラーで正しい:** RustNote は、ノートを読む・変えるメソッドに、必ず持ち主を渡すように作っています。持ち主の確かめを忘れると、コンパイルが通りません。しかも `UserId` と `NoteId` は別の型（newtype）なので、2つを入れ替えてもエラーになります（第4巻 UNIT 01）。
- **help の案内に注意:** `help: provide the argument` は、`find(NoteId::new(1)?, /* NoteId */)` のように、足りない2つ目を足す形を案内しています。コンパイラーは「並び順のまちがい」までは分からないので、案内のとおりに直すと、まだ1つ目の型が合いません。正しくは、ログイン中のユーザーの ID を1つ目に渡して `find(user.id, note_id)` とします。確かめたら、このファイルは消しておきます。

#### エラー3（エラーが出ない間違い）: SQL に持ち主の条件を書き忘れた

リポジトリを通さずに、SQL を自分で書いたときに起きやすい間違いです。alice のノートを、bob の立場で探します（`examples/u01_idor.rs`）。

```rust
@@FILE:rustnote/examples/u01_idor.rs@@
```

```powershell
cargo run -q --example u01_idor
```

```text
@@OUT:u01_idor@@
```

- **何が起きたか:** ID だけで探すと、bob に alice のノートが見つかってしまいました。コンパイルも通り、エラーも出ません。画面の処理がこの SQL を使っていたら、bob は URL の数字を 1, 2, 3, … と変えていくだけで、全員のノートを読めてしまいます。これが **IDOR**（Insecure Direct Object Reference：他人のデータを、ID を変えるだけで直接見られてしまう弱さ）です。
- **直し方:** 持ち主の条件 `AND user_id = ?2` を付けます。RustNote のリポジトリは、すべての SQL にこの条件を入れています。そして、エラー2のとおり、リポジトリのメソッドは持ち主を受け取らないと呼べません。**SQL を書く場所をリポジトリに集め、持ち主を必ず受け取る形にする**のが、IDOR を防ぐいちばん確実な方法です（第9巻 UNIT 04）。

### RustNote ではここで使う

ログインとログアウトは、`src/auth.rs` の2つの関数です（抜き出しています）。

```rust
/// ログインさせる: 新しいトークンを発行してクッキーに入れ、そのハッシュを DB に記録する
pub async fn log_in(cx: &Cx, user: &User) -> Result<(), AppError> {
    let state = app_state(cx);
    // 前のセッションのクッキーが残っていれば、その記録は先に消しておく
    if let Some(old_hash) = session::token_hash(cx).await.map_err(AppError::internal)? {
        state.sessions.delete(&old_hash).await?;
    }
    // 毎回新しいトークンを作る（セッション固定攻撃を防ぐ）
    let session = session::start(cx).await.map_err(AppError::internal)?;
    state
        .sessions
        .create(&session.token_hash, user.id, session.expires_at)
        .await?;
    // ついでに、期限切れのセッションを掃除しておく
    state.sessions.delete_expired(SystemTime::now()).await?;
    Ok(())
}

/// ログアウトさせる: クッキーを消し、DB の記録も消す
pub async fn log_out(cx: &Cx) -> Result<(), AppError> {
    if let Some(token_hash) = session::stop(cx).await.map_err(AppError::internal)? {
        app_state(cx).sessions.delete(&token_hash).await?;
    }
    Ok(())
}
```

- `session::start(cx)` は、Topcoat が新しいトークンを作ってクッキーに入れ、そのハッシュ（`token_hash`）と期限（`expires_at`）を返す関数です。Topcoat は、トークンを**発行するだけ**で、保存はしません。保存するのはアプリ（RustNote の `sessions` 表）の役目です。
- ログインのたびに**新しい**トークンを作るのは、「攻める人が先に用意したトークンを、本人にログインさせて使わせる」攻撃（セッション固定攻撃）を防ぐためです。くわしくは UNIT 05 で扱います。
- `log_out` は、`session::stop(cx)` でクッキーを消し、返ってきたハッシュで `sessions` の行も消します。どちらか片方だけでは足りない理由は、問2で確かめます。

ログインの失敗は、`src/routes/login.rs` で、理由を区別せずに同じ文で返しています（抜き出しています）。

```rust
const LOGIN_FAILED: &str = "メールアドレスまたはパスワードが違います";
```

```rust
    // メールが無いのか、パスワードが違うのかは区別しない
    let error = Some(LOGIN_FAILED.to_string());
    Ok(view! {
        (StatusCode::UNAUTHORIZED)
        login_form(email: input.email, error: error)
    })
```

### 練習問題

**問1:** 脅威モデルの表で「守りが無い」とした、パスワードの総当たりを確かめます。alice を登録してから、まちがったパスワードで 10 回続けてログインし、11 回目に正しいパスワードを送るテストを書いてください。RustNote は、何回目かで止めるでしょうか。

**問2:** ログアウトしたとき、DB の `sessions` の行と、ブラウザのクッキーはどうなるでしょうか。テストで、ログアウトの前と後の `sessions` の行の数、ログアウトの返事の `Set-Cookie`、ログアウト前のクッキーでもう一度 `/notes` を開いた結果を表示してください。

**問3:** もし RustNote が、`sessions` 表に、トークンのハッシュではなく**トークンそのもの**を保存していたら、DB のファイルが盗まれたときに何が起きるでしょうか。例1-2の結果をもとに説明してください。

### 解答と解説

**問1の解答:**

`tests/u01_q1.rs` です。

```rust
@@FILE:rustnote/tests/u01_q1.rs@@
```

```powershell
cargo test --test u01_q1 -- --nocapture
```

```text
@@OUT:u01_q1@@
```

- 10 回とも `401 Unauthorized` で、11 回目の正しいパスワードは `303 See Other`（ログイン成功）でした。RustNote は、何回まちがえても止めません。脅威モデルの表のとおり、**回数の制限が無い**ことが確かめられました。
- 1回のまちがいに、約 0.4〜0.8 秒（10 回で 5.2 秒）かかっています。これは debug ビルドで Argon2id の照合をしている時間で、release ビルドでは約 20 ms です（第12巻 UNIT 01）。続けて計算すると、後半の回が 0.6〜0.8 秒と遅くなりましたが、これは PC の状態によるもので、RustNote が回数に応じてわざと遅くしているわけではありません。重い照合は総当たりを遅くしますが、止めはしません。たとえば 1 秒に 50 回なら、1 日で 400 万回以上試せてしまいます。
- 直すには、「同じメールアドレスへの失敗が、10 分に 5 回を超えたら、しばらく断る」「同じ送り元からの回数を数える」といった**回数の制限**が必要です。どこに、どの単位で数えるかは、UNIT 04 で扱います（RustNote のコードは、この巻では変えません）。

**問2の解答:**

`tests/u01_q2.rs` です。

```rust
@@FILE:rustnote/tests/u01_q2.rs@@
```

```powershell
cargo test --test u01_q2 -- --nocapture
```

```text
@@OUT:u01_q2@@
```

- ログアウトで、`sessions` の行は 1 件から 0 件になりました。返事の `Set-Cookie` は、値が空の `__Host-session=` と、`Max-Age=0`・過去の `Expires`（1年前）で、ブラウザに「このクッキーを今すぐ消せ」と伝えています。
- ログアウト前のクッキーをもう一度見せると、303 でログイン画面へ移りました。クッキーのトークンをハッシュにしても、`sessions` 表に同じ行が無いからです。
- **片方だけでは足りない理由:** ブラウザのクッキーを消すだけ（DB の行を残す）だと、ログアウトの前にトークンを盗まれていた場合、盗んだ人は、そのトークンで 30 日間ログインしたままでいられます。DB の行を消すことで、**サーバーの側で**そのトークンを使えなくするのです。ブラウザはクッキーを消すよう頼まれても、従わないこと（壊れたブラウザや、攻める人の道具）もあります。

**問3の解答:**

- 例1-2で見たとおり、クッキーに入るのはトークンそのもの（46 文字）です。もし `sessions` 表にトークンそのものが入っていたら、DB を盗んだ人は、表から値を取り出して、自分のブラウザのクッキー `__Host-session` に入れるだけで、**期限の切れていない全員に、パスワードを知らないまま、なりすませます**。
- 実際の RustNote は、トークンの SHA-256（`token_hash`）だけを保存しています。SHA-256 は1方向にしか計算できないので、ハッシュからトークンを作り直すことはできません。トークンは十分に長いランダムな値なので、総当たりで同じハッシュになる値を探すこともできません。
- パスワードを Argon2id で、トークンを SHA-256 で、と使い分けているのは、パスワードは人が覚えられる短い値なので総当たりされやすく、わざと重い計算が必要だからです。トークンはもともと長いランダムな値なので、軽い SHA-256 で十分です（UNIT 02・05）。

### まとめ

- 認証（誰か）・セッション（同じ人か）・認可（してよいか）は別々の確かめ。RustNote は、認証が無ければ 303 でログイン画面へ、持ち主でなければ 404 にする。
- ブラウザにはトークン、DB にはそのハッシュだけを置く。クッキーは `__Host-` の名前と `HttpOnly`・`Secure`・`SameSite=Lax` で守り、ほかのサイトからの POST は Topcoat の Origin の検査が 403 で止める。
- 脅威モデルで、守るもの・攻める人・入口・信頼の境目を書き出し、守りが無い所（RustNote ではログインの回数の制限）も、はっきり表に残す。
