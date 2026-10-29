## UNIT 02: パスワード・Argon2id・salt

### この単元のゴール

- パスワードを「暗号化」ではなく「ハッシュ」にして保存する理由と、salt（毎回ちがう、ランダムな値）の役目を説明できる
- RustNote が DB に保存する PHC 文字列（`$argon2id$v=19$m=19456,t=2,p=1$…`）を、部分ごとに読める
- 速いハッシュと Argon2id の速さの違いを実際に測り、パスワードに「わざと重い」ハッシュを使う理由を説明できる
- 保存したハッシュが壊れていたとき・古い資料の書き方を写したときのエラーを読み、ログインの失敗の時間の差（DUMMY_HASH）を確かめられる

### たとえ話でつかむ

図書館の会員カードの、暗証番号を預かる場面を考えます。

- 暗証番号を紙に**そのまま**書いて金庫にしまうと、金庫を開けられた日に、全員の暗証番号が読まれてしまいます。
- そこで、暗証番号を**ミキサー**にかけます。ミキサーは、同じ材料なら必ず同じジュースになりますが、ジュースから材料に戻すことはできません。金庫には、ジュースだけをしまいます。会員が来たら、その場で暗証番号をミキサーにかけて、金庫のジュースと同じかを比べます。
- ただし、「1234 のジュースはこの色」という**早見表**を作られると、ジュースから暗証番号が分かってしまいます。そこで、ミキサーにかけるたびに、**会員ごとに違う隠し味**（salt）を入れ、隠し味もジュースの横に書いておきます。同じ暗証番号でも、会員ごとにジュースが違うので、早見表が使えません。
- さらに、ミキサーを**わざとゆっくり**回します。1杯作るのに時間がかかれば、金庫を盗んだ人が「0000 から 9999 まで全部試す」のに、とても長い時間がかかります。

| 図書館の暗証番号 | RustNote のパスワード |
|---|---|
| 紙にそのまま書く | 平文（へいぶん）で保存する。**RustNote はしない** |
| ミキサー（戻せない） | **ハッシュ**（Argon2id）。ハッシュから元のパスワードには戻せない |
| 会員ごとの隠し味 | **salt**。ハッシュにするたびに、ランダムに作る 16 バイトの値 |
| ジュースと隠し味を並べて書く | **PHC 文字列**（方式・設定・salt・ハッシュを、`$` で区切って1つの文字列にしたもの） |
| わざとゆっくり回す | Argon2id の**重さの設定**（`m=19456,t=2,p=1`） |
| 早見表 | 前もって計算したハッシュの一覧（レインボーテーブルなど）。salt で使えなくなる |

> **用語:** ハッシュ（hash：どんな長さのデータからでも、決まった長さの値を作る計算。同じ入力からは同じ値が出るが、値から入力には戻せない）

<!-- -->

> **用語:** salt（ソルト：ハッシュにする前に、パスワードに混ぜるランダムな値。人ごと・回ごとに違う。秘密にする必要はなく、ハッシュと一緒に保存する）

<!-- -->

> **用語:** 暗号化（あんごうか：鍵があれば元に戻せる変換。パスワードの保存には使わない。鍵を盗まれたら、全員のパスワードが戻されてしまうから）

### 図で理解する

登録のときと、ログインのときに、RustNote の中で起きることです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-0201" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="20" y="24" font-size="13" font-weight="700" fill="#1e40af">登録（POST /signup）</text>
<rect x="20" y="36" width="150" height="50" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="95" y="58" font-size="12" fill="#0f172a" text-anchor="middle">パスワード</text>
<text x="95" y="76" font-size="11" fill="#475569" text-anchor="middle">correct horse battery</text>
<rect x="20" y="96" width="150" height="40" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="95" y="121" font-size="12" fill="#c2410c" text-anchor="middle">salt を新しく作る</text>
<rect x="220" y="56" width="170" height="60" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="305" y="82" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">Argon2id</text>
<text x="305" y="102" font-size="11" fill="#475569" text-anchor="middle">m=19456, t=2, p=1</text>
<rect x="440" y="56" width="260" height="60" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="570" y="80" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">users.password_hash</text>
<text x="570" y="100" font-size="11" fill="#0f172a" text-anchor="middle">$argon2id$v=19$m=…$salt$hash</text>
<line x1="170" y1="61" x2="216" y2="78" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0201)"/>
<line x1="170" y1="116" x2="216" y2="96" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0201)"/>
<line x1="390" y1="86" x2="436" y2="86" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0201)"/>
<text x="20" y="176" font-size="13" font-weight="700" fill="#047857">ログイン（POST /login）</text>
<rect x="20" y="188" width="150" height="50" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="95" y="210" font-size="12" fill="#0f172a" text-anchor="middle">入力されたパスワード</text>
<text x="95" y="228" font-size="11" fill="#475569" text-anchor="middle">correct horse battery</text>
<rect x="440" y="188" width="260" height="50" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="570" y="210" font-size="12" fill="#047857" text-anchor="middle">保存してある PHC 文字列</text>
<text x="570" y="228" font-size="11" fill="#0f172a" text-anchor="middle">設定と salt を、ここから読む</text>
<rect x="220" y="198" width="170" height="60" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="305" y="224" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">同じ設定・同じ salt</text>
<text x="305" y="244" font-size="11" fill="#475569" text-anchor="middle">で計算して比べる</text>
<line x1="170" y1="213" x2="216" y2="222" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0201)"/>
<line x1="440" y1="213" x2="394" y2="222" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0201)"/>
<text x="305" y="286" font-size="12" fill="#0f172a" text-anchor="middle">同じなら true（ログイン成功）、違えば false。パスワードそのものは、どこにも保存しない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: パスワードのハッシュの流れ。登録のときは新しい salt で計算して PHC 文字列を保存し、ログインのときは、保存した文字列から設定と salt を読んで、同じ計算をして比べる</div>
</div>

- **登録のとき**は、パスワードと、新しく作った salt を Argon2id に入れて、PHC 文字列を作り、`users.password_hash` に保存します。
- **ログインのとき**は、保存してある PHC 文字列から、設定（`m`・`t`・`p`）と salt を読み、入力されたパスワードを**同じ設定・同じ salt** で計算して、ハッシュが同じかを比べます。元のパスワードに戻して比べるのではありません。
- だから、DB の中にも、ログにも、パスワードそのものはどこにも残りません。

PHC 文字列の中身は、次のように並んでいます。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 170" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="30" width="90" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="55" y="55" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">argon2id</text>
<rect x="110" y="30" width="60" height="40" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="140" y="55" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">v=19</text>
<rect x="180" y="30" width="150" height="40" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="255" y="55" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">m=19456,t=2,p=1</text>
<rect x="340" y="30" width="160" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="420" y="55" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">salt（22 文字）</text>
<rect x="510" y="30" width="200" height="40" rx="6" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="610" y="55" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">ハッシュ（43 文字）</text>
<text x="55" y="96" font-size="11" fill="#1e40af" text-anchor="middle">方式</text>
<text x="140" y="96" font-size="11" fill="#475569" text-anchor="middle">版</text>
<text x="255" y="96" font-size="11" fill="#c2410c" text-anchor="middle">重さの設定</text>
<text x="420" y="96" font-size="11" fill="#047857" text-anchor="middle">16 バイトのランダムな値</text>
<text x="610" y="96" font-size="11" fill="#b91c1c" text-anchor="middle">32 バイトの計算の結果</text>
<text x="255" y="122" font-size="11" fill="#0f172a" text-anchor="middle">m: メモリ（KiB）</text>
<text x="255" y="140" font-size="11" fill="#0f172a" text-anchor="middle">t: くり返す回数</text>
<text x="255" y="158" font-size="11" fill="#0f172a" text-anchor="middle">p: 並べて動かす数</text>
<text x="420" y="130" font-size="11" fill="#475569" text-anchor="middle">どちらも Base64 で</text>
<text x="420" y="148" font-size="11" fill="#475569" text-anchor="middle">文字にしてある</text>
<text x="360" y="18" font-size="12" fill="#0f172a" text-anchor="middle">$ で区切った5つの部分（先頭にも $ が付く）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: PHC 文字列の部分。方式・版・重さの設定・salt・ハッシュが、1つの文字列に入っているので、照合に必要なものは、この文字列だけで足りる</div>
</div>

### まず動かす最小コード

#### 例2-1: 同じパスワードを、2回ハッシュにする

> **▼ このコードがやること（先に日本語で）:** RustNote の `auth::hash_password` で、同じパスワードを2回ハッシュにして、2つの PHC 文字列を表示し、同じかどうかを比べます。1つ目を `$` で分けて、中身を1つずつ表示します。最後に `auth::verify_password` で、正しいパスワードと1文字違うパスワードを照合します。

```rust
@@FILE:rustnote/examples/u02_hash.rs@@
```

コピーした `rustnote` フォルダの `examples/u02_hash.rs` として保存して実行します。

```powershell
cargo run -q --example u02_hash
```

**▼ 実行結果**（salt とハッシュは、実行するたびに変わります）

```text
@@OUT:u02_hash@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 同じパスワードでも、ハッシュは毎回違う

- 1回目と2回目は、同じ `correct horse battery` なのに、違う文字列になりました（`2つは同じか: false`）。違うのは、後ろの2つの部分（salt とハッシュ）だけです。`hash_password` が、呼ばれるたびに新しい salt を作っているからです。
- それでも、照合はどちらも `true` です。照合のときは、それぞれの文字列に書いてある salt を使って計算し直すからです。

##### 解説2: 文字列の中に、照合に必要なものが全部ある

- `$` で分けると、方式（`argon2id`）・版（`v=19`）・重さの設定（`m=19456,t=2,p=1`）・salt（22 文字）・ハッシュ（43 文字）の5つになりました。salt は 16 バイト、ハッシュは 32 バイトの値を、Base64（英数字と `+`・`/` で表す書き方）で文字にしたものです。
- 設定まで一緒に保存しておくので、あとで設定を重くしても、古い設定で作ったハッシュを照合できます。古いハッシュは、その人が次にログインして照合に成功したときに、新しい設定で作り直せばよいのです。

##### 解説3: 1文字違えば、まったく違う結果になる

- 最後の `y` を `Y` にしただけで、照合は `false` になりました。ハッシュは、入力が少しでも違えば、まったく違う値になるように作られています。「だいたい合っている」はありません。

### もう一歩: 実務に近い形

#### 例2-2: 速いハッシュと Argon2id は、1秒に何回計算できるか

ハッシュなら何でもよいわけではありません。パスワードには、**わざと重い**ハッシュを使います。どれくらい違うのかを測ります。

> **▼ このコードがやること（先に日本語で）:** 1秒のあいだに、パスワードの候補（`password0`・`password1`・…）を作ってはハッシュにする、をくり返し、何回できたかを数えます。標準ライブラリの速いハッシュ（`DefaultHasher`）と、RustNote と同じ `Argon2::default()` で比べます。速さを測るので、release ビルドで動かします（第12巻 UNIT 01）。

```rust
@@FILE:rustnote/examples/u02_speed.rs@@
```

```powershell
cargo run -q --release --example u02_speed
```

**▼ 実行結果**

```text
@@OUT:u02_speed@@
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 170" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="20" y="40" font-size="12" font-weight="700" fill="#0f172a">速いハッシュ</text>
<rect x="160" y="24" width="540" height="26" rx="4" fill="#fed7aa"/>
<text x="690" y="42" font-size="12" font-weight="700" fill="#c2410c" text-anchor="end">1秒に 8,721,779 回</text>
<text x="20" y="96" font-size="12" font-weight="700" fill="#0f172a">Argon2id</text>
<rect x="160" y="80" width="3" height="26" rx="1" fill="#10b981"/>
<text x="172" y="98" font-size="12" font-weight="700" fill="#047857">1秒に 39 回（本当の棒の長さは上の 22 万分の 1 で、描けないほど短い）</text>
<text x="360" y="146" font-size="12" fill="#0f172a" text-anchor="middle">DB を盗んだ人が 1 億通りのパスワードを試すと: 速いハッシュは約 11 秒、Argon2id は約 30 日（PC 1台・この PC の速さで）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: 1秒に計算できる回数。この PC（Windows ARM64、release ビルド、1つのスレッド）で実際に測った数。わざと重い Argon2id は、総当たりを何十万倍も遅くする</div>
</div>

- 速いハッシュは1秒に約 870 万回、Argon2id は 39 回でした。約 22 万倍の差です。
- 1回のログインで 1 回計算するだけなら、Argon2id の 1/39 秒（約 26 ミリ秒）は、利用者が待つほどの時間ではありません。ところが、DB を盗んだ人が、よく使われるパスワードを 1 億通り試すとすると、速いハッシュなら約 11 秒、Argon2id なら約 30 日かかります（この PC 1台で、1つのスレッドで計算した場合の見積もりです）。**正しい人には気にならず、総当たりする人にはとても重い**のが、パスワード用のハッシュの役目です。
- Argon2id は、計算に毎回 19 MiB（`m=19456` KiB）のメモリを使います。メモリを多く使う計算は、たくさんの計算を同時に進める専用の機械（GPU など）でも、速くしにくいように作られています。
- `DefaultHasher` は、`HashMap` の表を作るためのハッシュで、結果も 64 ビットと短く、パスワードのような秘密を守るためのものではありません。ここでは「速いハッシュ」の例として使っています。SHA-256 のような安全なハッシュでも、速すぎるので、パスワードの保存には向きません。

> **注意:** 速さは、PC や設定によって大きく変わります。この例の数は、この PC で測った目安です。重さの設定は、OWASP（Web の安全についての資料をまとめている団体）が勧める Argon2id の最低の設定（m=19456, t=2, p=1）と同じで、argon2 0.6 の `Argon2::default()` が使う値です。

#### 例2-3: DB には、何が保存されているか

> **▼ このコードがやること（先に日本語で）:** 1つ目のテストでは、alice と bob を**同じパスワード**で登録し、`users` 表の `password_hash` の先頭と長さ、パスワードそのものが入っていないか、2人のハッシュが同じかを表示します。2つ目のテストでは、alice のハッシュを、読めない文字列（`broken`）に壊してから、正しいパスワードでログインし、どうなるかを表示します。

```rust
@@FILE:rustnote/tests/u02_stored.rs@@
```

```powershell
cargo test --test u02_stored -- --nocapture --test-threads=1
```

**▼ 実行結果**

```text
@@OUTGREP:u02_stored:^(?!   Compiling)@@
```

- 2人とも、`$argon2id$v=19$m=19456,t=2,p=1$` で始まる 97 文字の文字列で、パスワードそのものは入っていません。同じパスワードなのに、2人のハッシュは違います（salt が違うから）。DB を見ても、「alice と bob は同じパスワードだ」とは分かりません。
- ハッシュを壊すと、正しいパスワードでもログインは `500 Internal Server Error` になり、画面には「時間をおいてもう一度お試しください」だけが出ました。サーバーのログには `[error] サーバーの中でエラーが起きました <- invalid encoding`（文字列の形が正しくない）と、原因が残ります。「パスワードが違います」（401）ではなく 500 にしているのは、これが利用者のまちがいではなく、**DB の中身が壊れている**というサーバー側の問題だからです（`src/auth.rs` の `verify_password` は、保存した文字列を読めないときを、内部のエラーとして扱います）。

### よくあるエラーと読み方

#### エラー1: 古い資料（argon2 0.5）の書き方を写した（E0432・E0061）

インターネットの記事には、argon2 の古い版（0.5）の書き方がたくさん残っています。それを、0.6 を使う RustNote にそのまま写した場合です（`examples/e02_old_api.rs`）。

```rust
@@FILE:errs/e02_old_api.rs@@
```

```powershell
cargo build --example e02_old_api
```

```text
@@OUT:u02_err_old_api@@
```

- **何が起きたか:** エラーが2つ出ました。1つ目の `E0432` は「`SaltString` と `rand_core` を読み込めません」です。argon2 0.6 では、salt を自分で作る必要がなくなり、`SaltString` は、この場所（`password_hash` の直下）にはありません。`rand_core` も、`note:` のとおり、`rand_core` の機能を有効にしたときだけ使えるようになっています。2つ目の `E0061` は「`hash_password` は引数を1つとるのに、2つ渡されました」です。0.6 の `hash_password` は、パスワードだけを受け取り、salt は中で作ります（`note:` が、`fn hash_password(&self, password: &[u8])` を指しています）。
- **直し方:** 0.6 の書き方は、`Argon2::default().hash_password(b"…")` だけです。RustNote の `auth.rs` と、例2-1の形を参考にします。確かめたら、このファイルは消しておきます。
- **学べること:** 外のクレートの使い方は、版によって変わります。記事やサンプルを写すときは、その記事が使っている版と、自分の `Cargo.toml` の版（第13巻 UNIT 01）が同じかを、まず確かめます。

#### エラー2（エラーが出ない間違い）: パスワードを、そのまま保存する

もし `users` 表に、パスワードをそのまま（平文で）保存していても、登録もログインも、問題なく動きます。エラーは1つも出ません。困るのは、DB のファイル・バックアップ・ログのどれかが**漏れた日**です。

- 全員のパスワードが、その日のうちに読まれます。多くの人は、同じパスワードをほかのサービスでも使っているので、被害は RustNote の外にも広がります。
- 「暗号化して保存すれば大丈夫」とも考えがちですが、暗号化は鍵で元に戻せる変換です。鍵はサーバーの中のどこかにあるので、サーバーごと盗まれれば、戻されてしまいます。パスワードは、**戻せない**ハッシュにして保存します。
- 例2-3の1つ目のテストは、「`password_hash` にパスワードそのものが入っていない」ことを確かめています。このような**守りのテスト**を残しておくと、誰かがうっかり平文の保存に変えたときに気づけます（第11巻 UNIT 06（準備中））。

### RustNote ではここで使う

パスワードをハッシュにする関数と、照合する関数は、`src/auth.rs` にあります（抜き出しています）。

```rust
/// パスワードを Argon2id でハッシュ化し、PHC 文字列（$argon2id$v=19$...）にする。
/// 塩（salt）は argon2 クレートが毎回ランダムに作る。
pub async fn hash_password(password: &Password) -> Result<String, AppError> {
    let password = password.clone();
    // 重い計算なので、非同期の処理を止めないよう別スレッドで行う
    let joined = tokio::task::spawn_blocking(move || {
        Argon2::default()
            .hash_password(password.as_str().as_bytes())
            .map(|hash| hash.to_string())
    })
    .await;
    match joined {
        Ok(Ok(phc)) => Ok(phc),
        Ok(Err(error)) => Err(AppError::internal(error)),
        Err(error) => Err(AppError::internal(error)),
    }
}
```

- 例2-2で見たとおり、Argon2id は1回に約 26 ミリ秒（debug ビルドでは約 400 ミリ秒）かかる重い計算です。async の処理の中でそのまま計算すると、そのあいだ、同じスレッドのほかのリクエストが止まってしまいます。そこで `spawn_blocking` で、重い計算用の別のスレッドに任せています（第5巻 UNIT 07）。
- `hash_password` は、検査済みの `Password` 型しか受け取りません。8〜128 文字の決まりを通ったパスワードだけが、ハッシュになります（第11巻 UNIT 01 の問3）。

登録していないメールアドレスでログインしようとしたときのために、ダミーのハッシュも用意してあります（抜き出しています）。

```rust
/// 登録されていないメールでも同じくらい時間をかけるための、ダミーのハッシュ。
/// （すぐ返すと「このメールは未登録だ」と応答時間から推測されてしまう）
static DUMMY_HASH: LazyLock<String> = LazyLock::new(|| {
    Argon2::default()
        .hash_password(b"rustnote-dummy-password")
        .expect("ダミーのハッシュを作れません")
        .to_string()
});
```

- メールアドレスが登録されていないときも、このダミーのハッシュで照合の計算を1回して、時間をそろえます。すぐに「違います」と返すと、応答の速さから「このメールは登録されていない」と分かってしまうからです。問2で、本当に時間がそろっているかを測ります。

### 練習問題

**問1:** 例2-1の PHC 文字列の `m=19456,t=2,p=1` は、それぞれ何を表していますか。`m=19456` は、何 MiB のメモリでしょうか。

**問2:** alice を登録してから、「登録していないメールアドレス」と「alice のメールアドレスで、違うパスワード」で、それぞれ3回ずつログインに失敗し、1回ごとにかかった時間を表示してください。2つの時間は、そろっているでしょうか。

**問3:** 7 文字のパスワード（短すぎる）で登録しようとしたとき、返ってくる画面に、入力したパスワードが残っているかを確かめてください。メールアドレスはどうでしょうか。

### 解答と解説

**問1の解答:**

- `m` は、計算に使うメモリの大きさ（KiB）です。19456 KiB ÷ 1024 = **19 MiB** です。
- `t` は、メモリ全体を何回くり返して計算するか（2 回）、`p` は、計算をいくつ並べて動かすか（1 つ）です。
- この3つは、OWASP が Argon2id の最低の設定として勧めている値と同じで、argon2 0.6 の `Argon2::default()` が使う値です。重くするほど総当たりに強くなりますが、ログインのたびの計算も重くなり、同時にたくさんログインされたときのメモリも増えます（1回 19 MiB なので、同時に 10 人なら 190 MiB）。サーバーの大きさと相談して決めます（第12巻 UNIT 09（準備中））。

**問2の解答:**

`tests/u02_q2.rs` です。

```rust
@@FILE:rustnote/tests/u02_q2.rs@@
```

```powershell
cargo test --test u02_q2 -- --nocapture
```

```text
@@OUTGREP:u02_q2:^(?!   Compiling)@@
```

- 「登録していないメール」の2回目・3回目と、「パスワードが違う」の3回は、どれも約 380〜410 ミリ秒（debug ビルド）で、そろっています。ダミーのハッシュのおかげで、時間から「登録されているかどうか」は分かりません。
- ところが、**「登録していないメール」の1回目だけ、804 ミリ秒**と、約2倍かかりました。`DUMMY_HASH` は `LazyLock`（最初に使われたときに一度だけ作る）なので、最初の1回は「ダミーのハッシュを作る計算」と「照合の計算」の2回分の時間がかかるのです。サーバーを起動してから最初の1回だけですが、「登録していないメールの1回目は遅い」という差が残ります。
- これは、RustNote に残っている改善点です。起動するときに、先に一度 `DUMMY_HASH` を作っておけば（`LazyLock::force(&DUMMY_HASH)` を呼ぶなど）、この差は無くなります（RustNote のコードは変えていません）。**守りの仕組みも、測って確かめる**と、こうした抜けが見つかります。

**問3の解答:**

`tests/u02_q3.rs` です。

```rust
@@FILE:rustnote/tests/u02_q3.rs@@
```

```powershell
cargo test --test u02_q3 -- --nocapture
```

```text
@@OUTGREP:u02_q3:^(?!   Compiling)@@
```

- 400 で、「パスワードは8文字以上にしてください」が出ました。メールアドレスは入力欄（`value="alice@example.com"`）に残りますが、パスワード（`short77`）は、画面のどこにもありません。
- パスワードを入力欄に入れ直して返すと、HTML の中にパスワードがそのまま入ります。HTML は、ブラウザの履歴や、途中の仕組みに残ることがあるので、パスワードは**返さない**のが決まりです。利用者は、パスワードだけ打ち直すことになりますが、安全のための手間です。

### まとめ

- パスワードは、暗号化（戻せる）ではなく、ハッシュ（戻せない）にして保存する。RustNote は Argon2id を使い、salt を毎回ランダムに作るので、同じパスワードでも人ごとにハッシュが違う。
- PHC 文字列には、方式・版・重さの設定・salt・ハッシュが入っていて、照合に必要なものはこれだけで足りる。重さの設定（m=19456, t=2, p=1）は、総当たりを何十万倍も遅くする。
- 保存したハッシュが読めないときは 500 にして、原因はログにだけ残す。登録していないメールでもダミーのハッシュで時間をそろえるが、最初の1回だけは遅いという抜けがあることを、測って確かめた。
