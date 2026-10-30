## UNIT 06: ログイン・お知らせ・共有の状態

### この単元のゴール

- `src/auth.rs` が、パスワードを Argon2id でハッシュにし（別のスレッドで）、照合し、登録されていないメールでも同じくらい時間をかける仕組みを、本物の時間の計測で説明できる
- ログイン・ログイン中のユーザーの確認・ログアウトで、クッキー（`__Host-session`）と `sessions` の表がどう変わるかを、本物のサーバーへのリクエストで確かめられる
- `src/flash.rs` の「次の画面で一度だけ出すお知らせ」が、クッキーに決まったコードだけを入れて、書き換えに強くしている仕組みを説明できる
- `src/state.rs` の `AppState` が `Arc` と `Send + Sync` で共有される理由を、`Rc` にしたときのエラーで説明できる

### たとえ話でつかむ

会員制のジムの受付を考えます。

- 入会のとき、合言葉（パスワード）は、そのまま帳簿に書かず、**混ぜ物（塩）を足して、すりつぶした粉**（ハッシュ）にして保管します。同じ合言葉でも、混ぜ物が毎回違うので、粉は毎回違います。
- すりつぶすのは、わざと**時間のかかる作業**です。受付の人が手を止めないよう、**奥の作業場**（別のスレッド）に頼みます。
- 会員でない人が来たときも、**会員のときと同じくらい時間をかけて**から「違います」と答えます。すぐ答えると、待ち時間で「この人は会員ではない」と分かってしまいます。
- 入館が認められたら、**その日だけのリストバンド**（トークン）を渡し、帳簿には、リストバンドの番号そのものではなく、**番号を変換した印**（SHA-256）を書きます。前のリストバンドを持っていたら、その印は帳簿から消します。
- 受付の「ようこそ！」の札（お知らせ）は、**1回見せたら片付けます**。札には、決まった種類の番号しか書けないので、だれかが札を書き換えても、変な文は出ません。

| ジムの受付 | RustNote（`src/auth.rs`・`src/flash.rs`・`src/state.rs`） |
|---|---|
| 混ぜ物を足してすりつぶす | `hash_password`（Argon2id・塩は毎回ランダム） |
| 奥の作業場に頼む | `tokio::task::spawn_blocking` |
| 会員でなくても同じだけ時間をかける | `DUMMY_HASH` で照合してから `None` を返す |
| その日だけのリストバンド | `session::start` が発行するトークン（`__Host-session` クッキー） |
| 帳簿には番号を変換した印 | `sessions.token_hash`（トークンの SHA-256） |
| 前のリストバンドの印を消す | `log_in` の最初の `sessions.delete(&old_hash)` |
| 1回見せたら片付ける札 | `flash::set` と `flash::take`（クッキー `flash`） |
| 受付の全員が見る帳簿 | `AppState`（`Arc` で共有） |

> **用語:** Argon2id（アルゴン・ツー・アイディー：パスワードをハッシュにするための方式。わざと時間とメモリを使って、総当たりで合言葉を探すのを遅くする）

<!-- -->

> **用語:** 塩（しお、salt：ハッシュにする前にパスワードに足す、ランダムな値。同じパスワードでも、ハッシュが毎回違う値になる）

<!-- -->

> **用語:** フラッシュメッセージ（「ノートを作成しました」のように、次の画面で一度だけ出すお知らせ）

### 図で理解する

ログインして、ノートの一覧を開くまでの流れです（例6-5の結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 340" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-0601" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="10" width="120" height="32" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="80" y="31" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">ブラウザ</text>
<rect x="290" y="10" width="140" height="32" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="31" font-size="13" font-weight="700" fill="#1d4ed8" text-anchor="middle">RustNote</text>
<rect x="580" y="10" width="120" height="32" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="640" y="31" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">DB</text>
<line x1="80" y1="42" x2="80" y2="300" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 4"/>
<line x1="360" y1="42" x2="360" y2="300" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 4"/>
<line x1="640" y1="42" x2="640" y2="300" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 4"/>
<line x1="80" y1="68" x2="356" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0601)"/>
<text x="90" y="62" font-size="11" fill="#0f172a">POST /login（メール・パスワード）</text>
<line x1="360" y1="96" x2="636" y2="96" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0601)"/>
<text x="370" y="90" font-size="11" fill="#0f172a">find_by_email → Argon2id で照合</text>
<line x1="360" y1="124" x2="636" y2="124" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0601)"/>
<text x="370" y="118" font-size="11" fill="#0f172a">前のトークンの行を消し、新しい行を保存</text>
<line x1="360" y1="160" x2="84" y2="160" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0601)"/>
<text x="90" y="146" font-size="11" fill="#c2410c">303 /notes・Set-Cookie: __Host-session=トークン</text>
<text x="90" y="174" font-size="11" fill="#c2410c">　　　　　　Set-Cookie: flash=logged_in</text>
<line x1="80" y1="204" x2="356" y2="204" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0601)"/>
<text x="90" y="198" font-size="11" fill="#0f172a">GET /notes（Cookie: __Host-session=…）</text>
<line x1="360" y1="232" x2="636" y2="232" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0601)"/>
<text x="370" y="226" font-size="11" fill="#0f172a">require_user → find_user(SHA-256, 今)</text>
<line x1="360" y1="262" x2="84" y2="262" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0601)"/>
<text x="90" y="256" font-size="11" fill="#047857">200（ノートの一覧）</text>
<text x="20" y="324" font-size="11" fill="#b91c1c">ログインしていない・トークンが古い・期限切れ → flash=login_required を付けて 303 /login</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: ログインと、ログイン中のユーザーの確認。DB にはトークンそのものではなく SHA-256 を保存し、リクエストのたびに、クッキーのトークンから探す</div>
</div>

お知らせ（フラッシュメッセージ）が、一度だけ出るしくみです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 200" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-0602" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="20" width="200" height="64" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="110" y="44" font-size="12" font-weight="700" fill="#1d4ed8" text-anchor="middle">POST の処理</text>
<text x="110" y="64" font-size="11" fill="#1d4ed8" text-anchor="middle">flash::set → flash=note_created</text>
<text x="110" y="78" font-size="10" fill="#1d4ed8" text-anchor="middle">（Max-Age=300 の 5 分）</text>
<rect x="260" y="20" width="200" height="64" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="44" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">次の GET の処理</text>
<text x="360" y="64" font-size="11" fill="#047857" text-anchor="middle">flash::take → 読んで消す</text>
<text x="360" y="78" font-size="10" fill="#047857" text-anchor="middle">（Max-Age=0 で消す）</text>
<rect x="510" y="20" width="200" height="64" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="610" y="44" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">もう一度読み込む</text>
<text x="610" y="64" font-size="11" fill="#0f172a" text-anchor="middle">クッキーが無いので</text>
<text x="610" y="78" font-size="11" fill="#0f172a" text-anchor="middle">お知らせは出ない</text>
<line x1="210" y1="52" x2="256" y2="52" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0602)"/>
<text x="233" y="44" font-size="10" fill="#475569" text-anchor="middle">303</text>
<line x1="460" y1="52" x2="506" y2="52" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0602)"/>
<rect x="10" y="110" width="700" height="70" rx="6" fill="#fef2f2" stroke="#ef4444" stroke-width="1.5"/>
<text x="24" y="134" font-size="12" fill="#b91c1c">クッキーに入れるのは、決まったコード（note_created など）だけ。画面の文は、サーバーの中の表から引く</text>
<text x="24" y="158" font-size="12" fill="#b91c1c">利用者が flash=&lt;script&gt; に書き換えても、知らないコードは None になり、何も出さずに消すだけ（例6-5）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: フラッシュメッセージ。予約（set）と取り出し（take）をクッキーでつなぎ、取り出したら消すので、一度だけ出る</div>
</div>

### まず動かす最小コード

#### 例6-1: src/auth.rs の全文

> **▼ このファイルがやること（先に日本語で）:** ログインまわりの関数を集めたファイルです。パスワードを Argon2id でハッシュにする（`hash_password`）・照合する（`verify_password`）、メールとパスワードを確かめる（`authenticate`）、ログインとログアウト（`log_in`・`log_out`）、ログイン中のユーザーを探す（`current_user`）、ログインが必要なページの最初に呼ぶ（`require_user`）、の6つが中心です。

```rust
@@FILE:rustnote/src/auth.rs@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: Argon2id は、別のスレッドで

- `hash_password` は、`Argon2::default()` で、パスワードを PHC 文字列（`$argon2id$v=19$m=…$塩$ハッシュ`）にします。塩は、argon2 クレートが毎回ランダムに作るので、同じパスワードでも、毎回違う文字列になります（例6-4）。
- Argon2id は、わざと時間とメモリを使う計算です。`async fn` の中でそのまま計算すると、その間、同じスレッドのほかのリクエストが止まります。だから `tokio::task::spawn_blocking` で、重い計算用のスレッドに任せ、`.await` で終わるのを待ちます。
- `spawn_blocking` に渡す処理（クロージャ）は、別のスレッドで、いつまで動くか分かりません。だから、借りた値（`&Password`）は渡せず、`password.clone()` で自分の物にしてから、`move` で渡します（エラー1）。
- `spawn_blocking` の結果は、`Result<Result<…>, JoinError>` の2重です。外側は「別のスレッドがパニックした」など、内側は「argon2 の失敗」です。どちらも想定外なので、`AppError::internal` にします。

##### 解説2: 登録されていないメールでも、同じだけ時間をかける

- `authenticate` は、メールの形式が違うときも、登録されていないときも、`DUMMY_HASH`（ダミーのハッシュ）で `verify_password` をしてから `None` を返します。登録済みでパスワードが違うときと、かかる時間をそろえるためです。すぐ返すと、応答の時間で「このメールは登録されていない」と分かってしまいます（例6-5・問3）。
- `DUMMY_HASH` は `static` ですが、Argon2id の計算はプログラムを動かしてからでないとできないので、`LazyLock` で「最初に使うときに1回だけ作る」ようにしています（エラー2）。
- ルート（`src/routes/login.rs`）は、`None` なら、メールが無いのかパスワードが違うのかを区別せず、「メールアドレスまたはパスワードが違います」と 401 で返します（UNIT 07）。

##### 解説3: ログインとログアウト

- `log_in` は、①前のセッションのクッキーがあれば、その記録を DB から消し、②`session::start` で**新しいトークン**を発行して `__Host-session` クッキーに入れ、③そのハッシュを `sessions` に保存し、④ついでに期限切れの行を掃除します。毎回新しいトークンにするのは、ログインの前から知られていたトークンが、ログインのあとも使われる（セッション固定攻撃）のを防ぐためです（第8巻 UNIT 05）。
- `log_out` は、`session::stop` でクッキーを消し、返ってきたトークンのハッシュの行を DB から消します。

##### 解説4: ログイン中のユーザーは、1つのリクエストで1回だけ探す

- `load_current_user` は、クッキーのトークンのハッシュで、`sessions.find_user` を呼びます。`#[memoize(as_ref)]` が付いているので、1つのリクエストの中で何度呼ばれても、DB に問い合わせるのは最初の1回だけで、2回目からは、覚えておいた結果への参照（借り物）を返します。画面の枠（ヘッダーのログイン中の表示）と、ページの処理の両方から呼ばれても、問い合わせは1回です（UNIT 07）。
- `current_user` は、その借り物を `Option<&User>` にします。覚えておいたエラーは借り物で、そのまま返せないので、文にして新しい `Internal` を作ります。
- `require_user` は、ログインが必要なページの**最初**に呼ぶ関数です。ログインしていなければ、`Flash::LoginRequired` を予約し、`see_other`（303）でログインの画面へ送ります。Topcoat は「ミドルウェアではなく関数」という考え方なので、ログインが要るかどうかが、ページの処理の1行目を見れば分かります。

#### 例6-2: src/flash.rs の全文

> **▼ このファイルがやること（先に日本語で）:** 「ノートを作成しました」のような、次の画面で一度だけ出すお知らせです。フォームの処理が成功したら `set` でクッキー `flash` に種類のコードを入れ、次の画面の処理で `take` を呼んで、読んで消します。クッキーに入れるのは決まったコードだけで、画面の文は、ファイルの中の表から引きます。

```rust
@@FILE:rustnote/src/flash.rs@@
```

##### 解説5: クッキーには、決まったコードだけ

- `set` は、`flash=note_created` のようなコードだけを入れます。画面の文（「ノートを作成しました」）は、`message` の表から引きます。利用者がクッキーを `flash=<script>` に書き換えても、`from_code` が `None` を返すので、何も表示されません（例6-5）。
- `HttpOnly`（JavaScript から読めない）・`SameSite = Lax`（ほかのサイトからの POST では送らない）・`MaxAge` 5 分を付けています。中身は秘密ではないので、`Secure` は付けていません（例6-5の `Set-Cookie` のとおり）。
- `take` は、クッキーを読んでから、同じ `Path = "/"` で消します。`Path` が違うと、別のクッキーとして扱われ、消えません。
- クッキーの書き込みは、「ページの処理が値を返す前」にする決まりです（Topcoat）。だから、`set` は 303 を返す直前に、`take` はページの処理の最初に呼びます。

#### 例6-3: src/state.rs の全文

> **▼ このファイルがやること（先に日本語で）:** アプリ全体で共有する状態 `AppState`（DB と、3つのリポジトリ）を定義します。ルーターに登録しておくと、どのページの処理からでも `app_state(cx)` で借りられます。

```rust
@@FILE:rustnote/src/state.rs@@
```

##### 解説6: Arc と Send + Sync

- `AppState` は `#[derive(Clone)]` ですが、中身は `Db`（中は `Arc`）と3つの `Arc` なので、clone は、参照の数を1つ増やすだけで、DB を開き直したりはしません。
- `app_context::<AppState>(cx)` は、登録した値を借ります。Topcoat の `app_context` は、リクエストを複数のスレッドで同時に処理するので、`T: Any + Send + Sync` を求めます。`Arc` は Send と Sync ですが、`Rc` はどちらでもないので、使えません（エラー3）。
- 3つのリポジトリを `Arc<dyn … + Send + Sync>` で持つので、テストで、別の実装（たとえば、わざと失敗する実装）に差し替えられます（UNIT 05）。

### もう一歩: 実務に近い形

#### 例6-4: Argon2id のハッシュを作って照合する

> **▼ このコードがやること（先に日本語で）:** 同じパスワードで `hash_password` を2回呼び、PHC 文字列の中身（方式・版・設定・塩・ハッシュ）と、2回が違う文字列になることを確かめます。それから、正しいパスワード・1文字違う・前に空白、の3つを照合して、時間を測ります。最後に、壊れた PHC 文字列で照合したときのエラーを表示します。時間を測るので、`--release` で動かします。

```rust
@@FILE:rustnote/examples/u06_auth.rs@@
```

```powershell
cargo run -q --release --example u06_auth
```

```text
@@OUT:u06_auth@@
```

- 設定の `m=19456,t=2,p=1` は、メモリ 19,456 KiB（約 19 MiB）・2回くり返す・並列 1、という意味です。`Argon2::default()` の設定です。
- 2回のハッシュは、塩が違うので、別の文字列になりました。照合のときは、PHC 文字列の中の塩と設定を読んで、同じ計算をします。
- 照合は、1文字違いでも、前に空白があっても `false` でした（パスワードは trim しない。UNIT 04）。どれも 20 ミリ秒前後です。
- 壊れた PHC 文字列は、画面の文は「時間をおいてもう一度お試しください」、原因は `invalid encoding`（読み取れない形）でした。DB のハッシュが壊れていれば、500 になり、原因はログにだけ残ります。

#### 例6-5: 本物のサーバーで、ログイン・お知らせ・セッションの流れを確かめる

`topcoat asset bundle --release` で作った実行ファイルを、一時フォルダで `PORT=3014`・`DATABASE_URL=u06.db` で起動し、`curl` でリクエストを送りました。表示は、状態の行・`location`・`set-cookie`（トークンは先頭の 6 文字だけ）と、画面のお知らせの部分です。`sessions` の表は、例のプログラム（`examples/u06_sessions.rs`、DB を別に開いて読むだけ）で表示しました。

**▼ ① ログインせずに `/notes` を開く**

```text
@@OUT:u06_notes_anon@@
```

**▼ ② その `flash=login_required` を付けて `/login` を開く**

```text
@@OUT:u06_login_flash@@
```

**▼ ③ 書き換えたクッキー `flash=<script>`（URL の形で `%3Cscript%3E`）で `/login` を開く**

```text
@@OUT:u06_login_bad_flash@@
```

**▼ ④ 登録（`POST /signup`）と、そのあとの `sessions` の表**

```text
@@OUT:u06_signup@@
```

```text
@@OUT:u06_sess1@@
```

**▼ ⑤ そのクッキーで `/notes` を開く**

```text
@@OUT:u06_notes_auth@@
```

**▼ ⑥ 同じクッキーを付けたまま、もう一度ログインする（`POST /login`）**

```text
@@OUT:u06_login_again@@
```

```text
@@OUT:u06_sess2@@
```

**▼ ⑦ ⑥の前のクッキー（古いトークン）で `/notes` を開く**

```text
@@OUT:u06_old_cookie@@
```

**▼ ⑧ ログアウト（`POST /logout`）と、そのあと**

```text
@@OUT:u06_logout@@
```

```text
@@OUT:u06_sess3@@
```

```text
@@OUT:u06_after_logout@@
```

- ①は、`require_user` が、`flash=login_required`（5 分）を付けて、303 で `/login` へ送りました。②で `/login` を開くと「ログインしてください」が出て、同時に `flash=; Max-Age=0`（と 1 年前の `Expires`）でクッキーが消されました。
- ③の書き換えたクッキーでは、お知らせは出ず、クッキーは消されました。
- ④の登録で、`__Host-session` クッキー（`Secure`・`HttpOnly`・`SameSite=Lax`・30 日の `Max-Age=2592000`）が来ました。`sessions` には、トークンの SHA-256 が 32 バイトの `blob` で入り、期限は 2,592,000 秒（30 日）後でした。
- ⑥でログインし直すと、新しいトークンが来て、`sessions` は1件のままでした。前の行は消され、⑦の古いトークンは使えず、ログインの画面へ送られました。
- ⑧のログアウトで、`__Host-session` が `Max-Age=0` で消され、`sessions` は 0 件になり、同じトークンではもう入れません。
- `set-cookie` の順番は、④（セッションが先）と⑥（お知らせが先）で違いました。Topcoat の中で使っている cookie クレートが、クッキーを順番の決まらない入れ物（HashSet）に入れているからです（第11巻 UNIT 03）。

最後に、`POST /login` の失敗を、3通りで7回ずつ送り、時間の中央値を比べました。

```text
@@OUT:u06_timing@@
```

- 3つとも 401 で、時間も 36〜41 ミリ秒とそろいました。登録されていないメール（B）でも、形式の違うメール（C）でも、ダミーのハッシュで照合しているからです。

サーバーのログ（標準エラー出力）の先頭です。

```text
@@OUT:u06_server_log@@
```

- 登録（`POST /signup`）とログイン（`POST /login`、失敗も）は 35〜50 ミリ秒、ほかは数ミリ秒以下でした。時間のほとんどは Argon2id です。

### よくあるエラーと読み方

#### エラー1: `spawn_blocking` に、借りたパスワードをそのまま渡した（E0521）

```rust
@@FILE:errs_u06/e06_borrow.rs@@
```

```text
@@OUT:u06_err_borrow@@
```

- **何が起きたか:** 「借りたデータが、関数の外へ逃げます」というエラーです。`spawn_blocking` に渡す処理は、別のスレッドで、関数が終わったあとも動くかもしれないので、`'static`（ずっと有効）でなければなりません。`password` は、関数の中だけで有効な借り物です。
- **直し方:** RustNote のように、`let password = password.clone();` で自分の物にしてから、`move ||` で処理に渡します。

#### エラー2: `static` の値を、ふつうの関数で作ろうとした（E0015）

```rust
@@FILE:errs_u06/e06_static.rs@@
```

```text
@@OUT:u06_err_static@@
```

- **何が起きたか:** 「static の中では、const でない関数を呼べません」というエラーです。`static` の値は、コンパイルのときに決まっていなければなりません。
- **直し方:** `note:` のとおり、`LazyLock::new(|| …)` で包み、最初に使うときに1回だけ作ります。RustNote の `DUMMY_HASH` は、この形です。

#### エラー3: 共有の状態に、`Arc` ではなく `Rc` を使った（E0277）

```rust
@@FILE:errs_u06/e06_rc.rs@@
```

```text
@@OUTHEAD:u06_err_rc:15@@
  …（Sync についても同じエラー）…
@@OUTGREP:u06_err_rc:^error: could not@@
```

- **何が起きたか:** 「`Rc<u32>` は、スレッドの間で安全に送れません」というエラーです。`note:` が、`app_context` の `T: Any + Send + Sync` を指しています。同じ形のエラーが、`Sync`（「安全に共有できません」）についても出ます。
- **直し方:** `Rc` を `Arc` に変えます。`Rc` は、参照の数を数えるときに、ほかのスレッドとの取り合いを考えていないので、1つのスレッドの中でしか使えません。

### RustNote ではここで使う

- **登録とログインのルート:** `POST /signup` は `hash_password` と `log_in`、`POST /login` は `authenticate` と `log_in`、`POST /logout` は `log_out` を呼びます（UNIT 07）。
- **ログインが必要なページ:** ノートの一覧・作成・詳細・編集・切り替え・削除は、どれも処理の最初で `require_user(cx)` を呼びます（UNIT 08）。
- **画面の枠とお知らせ:** 画面の枠のヘッダー（ログイン中の表示）が `current_user` を使います。お知らせは、それぞれのページ（トップ・ログイン・ノートの一覧など）が `flash::take` で取り出し、`flash_banner` で表示します（UNIT 07・08）。
- **テスト:** `flash.rs` のユニットテストが1つあります。ログインの流れは、`tests/http.rs` の HTTP のテストで確かめます（UNIT 10）。

```text
@@OUTGREP:u06_lib_tests:^(running|test |test result)@@
```

- **改善点（この単元で見つかったこと）:**
  - `Flash::from_code` の `all` は、enum の種類の一覧を、もう一度書いています。種類を足して `all` に足し忘れても、コンパイルも今のテストも通ります（問1）。
  - クッキーの名前 `"flash"` は、`COOKIE_NAME` の定数と、`set` の `cookie!` の中の2か所に書いてあります。
  - Argon2id の計算を同時にいくつまでするかの上限がありません。第11巻 UNIT 09 では、48 のログインを同時に送ると、メモリが 654 MiB まで増えました。

### 練習問題

**問1:** `Flash` に新しい種類 `NoteArchived` を足し、`code` と `message` の腕も足しましたが、`from_code` の `all` には足し忘れました。コンパイルと、今のテストは、どうなるでしょうか。何が困るでしょうか。

**問2:** `log_in` の最初の「前のセッションの記録を消す」3行を消すと、何が変わるでしょうか。例6-5の⑥⑦と同じ手順で確かめてください。

**問3:** `authenticate` で、登録されていないメールのときに、ダミーの照合をせずにすぐ `None` を返すと、何が困るでしょうか。例6-5の時間の比べ方で確かめてください。

### 解答と解説

**問1の解答:**

教材のコードのコピーで、`NoteArchived` を、enum・`code`・`message` に足し（`all` には足さない）、`cargo test --lib -- flash::` を動かしました。

```text
@@OUTGREP:u06_q1_old:^(running|test |test result)@@
```

- コンパイルも、今のテスト（3つの種類だけを確かめる）も通りました。`code` と `message` の `match` には `_` が無いので、足し忘れればコンパイルエラーになりますが、`all` は、ただの配列なので、コンパイラは気づけません。

次に、足した種類のコードを読めるかを確かめるテストを足しました。

```text
@@OUTGREP:u06_q1_new:^(running|test |thread|assertion|  left|  right|test result)@@
```

- `from_code("note_archived")` は `None` でした。`flash::set(cx, Flash::NoteArchived)` で予約しても、次の画面の `take` が読めないので、**お知らせが、エラーも無く出ない**ことになります。
- **直し方:** テストは、全部の種類を確かめる形にします（`all` と同じ一覧を回して、`code` から戻せるかを見る）。種類の一覧を書く場所を、`all` の1か所にまとめ、`code` と `message` の `match` の近くに置くと、足し忘れにくくなります。

**問2の解答:**

教材のコードのコピーで、3行を消して release で作り直し、登録 → 同じクッキーでログインし直す → `sessions` を見る → 古いクッキーで `/notes` を開く、を試しました。

```text
@@OUT:u06_q2_sess@@
```

```text
@@OUT:u06_q2_old_cookie@@
```

- `sessions` が2件になり、ログインし直す前の古いトークンでも、`/notes` が 200 で開けました（例6-5の⑦では、ログインの画面へ送られました）。
- 古いトークンは、30 日の期限まで使えるままです。共用のパソコンで、ほかの人のあとにログインし直した場合や、古いトークンが漏れていた場合に、そのトークンで入られてしまいます。ログインし直したら、前のトークンは使えなくするのが安全です。

**問3の解答:**

教材のコードのコピーで、登録されていないメールのときは `None => Ok(None),` とすぐ返すようにして、例6-5と同じ3通りの時間を比べました。

```text
@@OUT:u06_q3_timing@@
```

- 登録されていないメール（B）だけ、4 ミリ秒と、ほかの約 10 分の 1 になりました。どれも同じ 401・同じ文でも、**時間を測れば**、そのメールが登録されているかどうかが分かってしまいます（ユーザーの列挙）。
- 形式の違うメール（C）は、ダミーの照合が残っているので、遅いままでした。教材のコードは、3つの道すべてで照合をして、時間をそろえています（例6-5）。

### まとめ

- `src/auth.rs` は、パスワードを Argon2id（`m=19456,t=2,p=1`・塩は毎回ランダム）でハッシュにし、`spawn_blocking` で別のスレッドに任せる（借り物を渡すと E0521）。登録されていないメールでも `DUMMY_HASH`（`LazyLock`）で照合し、時間をそろえる（しないと 4 ミリ秒で分かる）。
- `log_in` は、前のトークンの行を消し、新しいトークンを `__Host-session`（Secure・HttpOnly・SameSite=Lax・30 日）で渡して、SHA-256 を `sessions` に保存する。`require_user` は、ログインしていなければ `flash=login_required` と 303。`current_user` は `#[memoize(as_ref)]` で1リクエスト1回。
- `src/flash.rs` は、決まったコードだけをクッキーに入れ、次の画面で読んで消す。書き換えられた `flash=<script>` は何も出さずに消える。種類を足して `all` に足し忘れると、お知らせが黙って消える（改善点）。
- `src/state.rs` の `AppState` は、clone が安い `Arc` の集まり。`app_context` は `Send + Sync` を求めるので、`Rc` は使えない（E0277）。
