## UNIT 05: Topcoat のセッションの一生

### この単元のゴール

- セッションの一生（`session::start` で発行 → 毎回 `token_hash` で照合 → `session::stop` でログアウト）を、クッキーと `sessions` 表の**両方**で追いかけられる
- Topcoat が受け持つこと（トークンを作り、クッキーで運ぶ）と、RustNote が受け持つこと（ハッシュ・持ち主・期限を DB に記録し、照合・期限・取り消し・掃除をする）を、分けて言える
- ログアウトで DB の記録を消さないと、写し取られたクッキーが使い続けられることを、実際に確かめて説明できる
- 期限（30 日）が使っても延びないことと、期限切れの行がいつ消えるかを説明でき、`session::refresh` で延ばす作り方を言える

### たとえ話でつかむ

劇場の、途中で出入りできる公演を考えます。

- 入口で、係の人が**半券**を渡します。半券には、でたらめな番号が印刷されています。係の人は、その番号を、そのまま台帳に書かずに、**番号から作った印**（ハッシュ）と、「誰の席か」「いつまで有効か」を台帳に書きます。
- 出入りのたびに、半券を見せます。係の人は、半券の番号から印を作り直し、台帳で探します。有効期限が過ぎていれば、入れません。
- 帰るときは、半券を回収し、**台帳からも消します**。半券を回収しても台帳に残っていたら、誰かが半券をこっそり写し取っていた場合に、その写しで入れてしまいます。
- 台帳を持っているのは劇場なので、「この人の半券を、全部使えなくする」ことも、劇場の側だけでできます。

| 劇場の半券 | RustNote のセッション |
|---|---|
| でたらめな番号の半券を渡す | `session::start`（Topcoat がトークンを作り、`__Host-session` のクッキーで渡す） |
| 番号の印・誰の席か・いつまでを台帳に | `sessions` 表（`token_hash` 32 バイト・`user_id`・`expires_at`） |
| 出入りのたびに台帳で探す | `load_current_user`（`token_hash` → `find_user`、期限も見る） |
| 帰るとき、回収して台帳からも消す | `log_out`（`session::stop` と、`sessions.delete`） |
| 劇場の側だけで全部使えなくする | `DELETE FROM sessions WHERE user_id = ?`（例5-2） |

> **用語:** セッション（ログインしてからログアウトするまでの、「この人は、さっきログインした alice だ」とサーバーが覚えている状態。RustNote では、クッキーのトークンと、DB の記録の組で表す）

<!-- -->

> **用語:** サーバーの側のセッション（セッションの記録をサーバー（DB）に持つ作り方。サーバーの側で記録を消せば、すぐに使えなくできる。反対に、必要な情報を全部クッキーに入れて署名する作り方もある）

<!-- -->

> **用語:** 期限切れの掃除（有効期限が過ぎたセッションの行を、DB から消すこと。照合のときに期限を見ていれば、残っていても使えないが、行はたまっていく）

### 図で理解する

セッションの一生と、それぞれの時の、クッキーと `sessions` 表です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-0501" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="100" y="22" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">いつ</text>
<text x="330" y="22" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">クッキー（ブラウザ）</text>
<text x="585" y="22" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">sessions 表（DB）</text>
<rect x="10" y="34" width="180" height="56" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="100" y="58" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">① ログイン・登録</text>
<text x="100" y="78" font-size="11" fill="#475569" text-anchor="middle">session::start → log_in</text>
<text x="330" y="58" font-size="11" fill="#0f172a" text-anchor="middle">__Host-session=新しいトークン</text>
<text x="330" y="78" font-size="10" fill="#475569" text-anchor="middle">Max-Age=2592000（30 日）</text>
<text x="585" y="58" font-size="11" fill="#0f172a" text-anchor="middle">INSERT（token_hash・user_id・</text>
<text x="585" y="78" font-size="11" fill="#0f172a" text-anchor="middle">expires_at = 今 + 30 日）</text>
<rect x="10" y="104" width="180" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="100" y="128" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">② 毎回のリクエスト</text>
<text x="100" y="148" font-size="11" fill="#475569" text-anchor="middle">session::token_hash</text>
<text x="330" y="128" font-size="11" fill="#0f172a" text-anchor="middle">送るだけ（Set-Cookie は無い）</text>
<text x="585" y="128" font-size="11" fill="#0f172a" text-anchor="middle">SELECT … WHERE token_hash = ?</text>
<text x="585" y="148" font-size="11" fill="#0f172a" text-anchor="middle">AND expires_at &gt; 今（延ばさない）</text>
<rect x="10" y="174" width="180" height="56" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="100" y="198" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">③ ログアウト</text>
<text x="100" y="218" font-size="11" fill="#475569" text-anchor="middle">session::stop → log_out</text>
<text x="330" y="198" font-size="11" fill="#0f172a" text-anchor="middle">__Host-session=; Max-Age=0</text>
<text x="330" y="218" font-size="10" fill="#475569" text-anchor="middle">（ブラウザが消す）</text>
<text x="585" y="198" font-size="11" fill="#0f172a" text-anchor="middle">DELETE … WHERE token_hash = ?</text>
<text x="585" y="218" font-size="10" fill="#475569" text-anchor="middle">（写しも使えなくなる）</text>
<rect x="10" y="244" width="180" height="48" rx="8" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="100" y="266" font-size="12" font-weight="700" fill="#334155" text-anchor="middle">④ 期限切れの掃除</text>
<text x="100" y="284" font-size="10" fill="#475569" text-anchor="middle">誰かがログインしたとき</text>
<text x="585" y="272" font-size="11" fill="#0f172a" text-anchor="middle">DELETE … WHERE expires_at &lt;= 今</text>
<line x1="460" y1="30" x2="460" y2="292" stroke="#e2e8f0" stroke-width="2"/>
<line x1="200" y1="30" x2="200" y2="292" stroke="#e2e8f0" stroke-width="2"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: セッションの一生。ログインでクッキーと DB の記録を作り、毎回のリクエストでハッシュと期限を照らし、ログアウトでクッキーと記録の両方を消す。期限切れの行は、誰かがログインしたときに掃除する</div>
</div>

Topcoat と RustNote の、受け持ちの分け方です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 220" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="340" height="200" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="180" y="36" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">Topcoat（topcoat-session）</text>
<text x="30" y="66" font-size="11" fill="#0f172a">・でたらめなトークン（32 バイト）を作る</text>
<text x="30" y="90" font-size="11" fill="#0f172a">・クッキー（__Host-・Secure・HttpOnly・Lax）で運ぶ</text>
<text x="30" y="114" font-size="11" fill="#0f172a">・トークンから SHA-256 の token_hash を作る</text>
<text x="30" y="138" font-size="11" fill="#0f172a">・アプリには token_hash と expires_at だけ渡す</text>
<text x="30" y="162" font-size="11" fill="#475569">・使えるが RustNote は使っていない:</text>
<text x="40" y="182" font-size="11" fill="#475569">refresh（期限を延ばす）・rotate（取り替える）</text>
<rect x="370" y="10" width="340" height="200" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="540" y="36" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">RustNote（src/auth.rs と sessions 表）</text>
<text x="390" y="66" font-size="11" fill="#0f172a">・token_hash・user_id・expires_at を記録する</text>
<text x="390" y="90" font-size="11" fill="#0f172a">・照合で、記録があるか・期限の中かを見る</text>
<text x="390" y="114" font-size="11" fill="#0f172a">・ログアウトで、記録を消す</text>
<text x="390" y="138" font-size="11" fill="#0f172a">・ログインのたびに、前の記録を消す（UNIT 04）</text>
<text x="390" y="162" font-size="11" fill="#0f172a">・期限切れの行を掃除する（delete_expired）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: セッションの受け持ち。Topcoat はトークンを作って運び、ハッシュを渡すだけで、記録は持たない。誰のセッションか・いつまでか・消すか、はアプリが DB で決める</div>
</div>

### まず動かす最小コード

#### 例5-1: 発行・使う・ログアウトを、クッキーと DB で見る

ログインとログアウトの関数（`src/auth.rs`）です。

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

照合の SQL（`src/repository/sessions.rs` の `find_user`）は、ハッシュと**期限**の両方で探します。

```sql
SELECT u.id, u.email
  FROM sessions s
  JOIN users u ON u.id = s.user_id
 WHERE s.token_hash = ?1 AND s.expires_at > ?2
```

> **▼ このコードがやること（先に日本語で）:** alice を登録して（このときログインもします）、`sessions` 表の行（ハッシュの先頭8文字・長さ・ユーザー・期限・作ってから期限までの秒）を表示します。1秒待ってから一覧を開き、返事の `Set-Cookie` と、行が変わったかを表示します。最後に、ログアウトしたときの `Set-Cookie` と行の数、前のクッキーのまま一覧を開いたときの返事を表示します。

```rust
@@FILE:rustnote/tests/u05_life.rs@@
```

```powershell
cargo test --test u05_life -- --nocapture
```

```text
@@OUTGREP:u05_life:^([0-9]\. |   (token|Set|sessions))@@
```

（結果の行だけを抜き出しています。`expires_at` と Expires の日時は、実行した日によって変わります）

#### ▼ コードを1つずつ分解して解説

##### 解説1: DB にあるのは、ハッシュ・持ち主・期限

- 1の行は、`token_hash` が 32 バイト（SHA-256）、`user_id=1`、`expires_at` は 1970 年からの秒数で、**作ってから 2,592,000 秒**（30 日）でした。クッキーの `Max-Age=2592000` と同じ長さです（UNIT 01）。
- トークンそのものは、どこにも保存されていません。Topcoat の `session::start` は、アプリに `Session { token_hash, expires_at }` だけを返すので、RustNote は保存しようにも、トークンを持っていないのです（エラー3）。

##### 解説2: 使っても、期限は延びない

- 2の、1秒後の一覧は `200 OK` で、`Set-Cookie` は無く、行の `expires_at` も変わりませんでした。RustNote のセッションは「**ログインしてから 30 日**」で切れ、毎日使っていても、30 日目には、もう一度ログインが要ります（問2）。
- 3のログアウトでは、`Max-Age=0` のクッキー（すぐ消える）が来て、`sessions` の行は 0 になりました。4で前のクッキーを送っても、行が無いので `303`（ログイン画面へ）です。

### もう一歩: 実務に近い形

#### 例5-2: サーバーの側だけで、セッションを終わらせる

セッションの記録が DB にあるので、利用者のブラウザに何もしなくても、サーバーの側でセッションを終わらせられます。2つの場合を試します。

> **▼ このコードがやること（先に日本語で）:** 1つ目のテストでは、alice と bob を登録してから、alice のセッションの `expires_at` を 1 秒前にします。alice のクッキーで一覧を開き、そのあと bob がログインします。それぞれのあとで、ユーザーごとの `sessions` の行の数を表示します。2つ目のテストでは、alice が PC（登録）とスマートフォン（ログイン）の2つでログインしたあと、`DELETE FROM sessions WHERE user_id = 1` で alice の行を全部消し、2つのクッキーで一覧を開きます。

```rust
@@FILE:rustnote/tests/u05_server.rs@@
```

```powershell
cargo test --test u05_server -- --nocapture --test-threads=1
```

```text
@@OUTGREP:u05_server:^[0-9]\. @@
```

- **期限切れ:** 期限を過ぎた alice のクッキーは `303` で、もう使えません。でも、行は `alice=1` のまま残っていました。照合の SQL が `expires_at > 今` で見ないだけで、消してはいないからです。bob がログインしたときに、`log_in` の中の `delete_expired` が、alice の行を消しました（`alice=0`）。
- **全部の端末からログアウト:** alice の行を2つとも消すと、PC とスマートフォンのどちらのクッキーも、すぐに `303` になりました。パスワードが漏れたかもしれないとき、パスワードを変えるのと一緒に、こうして全部のセッションを終わらせます。RustNote には、この操作の画面はありません（足すなら、`DELETE FROM sessions WHERE user_id = ?` を、ログイン中の本人だけが動かせる POST にします）。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: ログアウトで、DB の記録を消さなかった

`log_out` を、`session::stop` でクッキーを消すだけにした場合です（練習用のコピーの `src/auth.rs` を書き換え、確かめたら元に戻します）。alice のクッキーを写し取った人がいて、alice がログアウトする、というテスト（`tests/u05_stolen.rs`）で確かめます。

```rust
@@FILE:rustnote/tests/u05_stolen.rs@@
```

まず、RustNote のコードです。

```text
@@OUTGREP:u05_stolen:^[0-9]\. @@
```

次に、DB の記録を消さない版です。

```text
@@OUTGREP:u05_err_logout:^[0-9]\. @@
```

- **何が起きたか:** alice のブラウザでは、クッキーが消え、ログアウトしたように見えます。ところが、写し取られたクッキーは、ログアウトのあとも `200 OK` で使えました。DB の記録が残っているので、照合が通るのです。期限の 30 日まで、そのまま使えます。
- **困ること:** 「共用の PC で使ったあと、ログアウトしたから大丈夫」は、DB の記録を消して初めて成り立ちます。クッキーは、ブラウザの中の写しにすぎません。
- **学べること:** サーバーの側のセッションでは、**ログアウトの本体は DB の記録を消すこと**です。クッキーを消すのは、ブラウザに後片付けを頼むだけです。

#### エラー2（エラーが出ない間違い）: 照合で、期限を見なかった

`find_user` の SQL を、`WHERE s.token_hash = ?1` だけにした場合です。例5-2の1つ目のテストを動かします。

```text
@@OUTGREP:u05_err_expiry:^[0-9]\. @@
```

- **何が起きたか:** 期限を過ぎた alice のクッキーが `200 OK` で使えました。ふつうのブラウザは、クッキーの `Max-Age` が過ぎれば送らなくなりますが、写し取られたクッキーを使う人は、期限を守るとは限りません。
- 3で bob がログインすると、`delete_expired` が alice の行を消すので、そのあとは使えなくなります。でも、それまでの間は、期限切れのセッションが生きています。
- **学べること:** 期限の確かめは、クッキーの `Max-Age`（ブラウザの側）に任せず、**サーバーの照合で**行います。

#### エラー3: 生のトークンを保存しようとした（E0609）

「トークンそのものを保存すれば、ハッシュにしなくて済む」と考えて、`create(&session.token, …)` と書いた場合です。

```text
@@OUT:u05_err_token@@
```

- **何が起きたか:** 「`topcoat::session::Session` に `token` という項目はありません」というエラーで、`note:` が「使える項目は `token_hash` と `expires_at`」と教えています。
- **学べること:** Topcoat は、トークンそのものを、アプリに**渡さない**作りです。DB が盗まれても、ハッシュからはトークンを作れないので、クッキーを作って成りすますことはできません（UNIT 01 の脅威モデル）。間違った使い方が、型のおかげで、最初からできないのです。

### RustNote ではここで使う

- セッションの設定は、`src/routes.rs` の `.sessions(SessionConfig::default())` です。`SessionConfig::default()` の期限は 30 日（Topcoat の `DEFAULT_LIFETIME`）で、`SessionConfig::builder().lifetime(…)` で変えられます。
- 記録の表は `migrations/0003_sessions.sql` で、`token_hash BLOB PRIMARY KEY`・`user_id`（`ON DELETE CASCADE`）・`expires_at INTEGER`（1970 年からの秒）と、`user_id` と `expires_at` の索引があります。
- 照合は `src/auth.rs` の `load_current_user` で、`#[memoize(as_ref)]` により、1つのリクエストの中では DB に1回しか問い合わせません（第6巻 UNIT 07）。
- RustNote が使っていない Topcoat の関数に、`session::refresh`（同じトークンのまま期限を延ばす）と `session::rotate`（トークンを取り替え、前のハッシュを返す）があります。

### 練習問題

**問1:** alice が PC とスマートフォンでログインしているときに、PC でだけログアウトすると、スマートフォンはどうなるでしょうか。

**問2:** 「最後に使ってから 30 日」で切れるようにするには、何をすればよいでしょうか。

**問3:** 期限切れの行は、いつ消えるでしょうか。誰もログインしないと、どうなるでしょうか。

### 解答と解説

**問1の解答:**

```rust
@@FILE:rustnote/tests/u05_q.rs@@
```

```powershell
cargo test --test u05_q -- --nocapture
```

```text
@@OUTGREP:u05_q:^問@@
```

- PC のログアウトで、`sessions` は2行から1行になり、PC のクッキーは `303`、スマートフォンのクッキーは `200 OK` のままでした。ログアウトは、**そのトークンの行だけ**を消します（`DELETE … WHERE token_hash = ?`）。
- 全部の端末から一度にログアウトしたいときは、例5-2のように、`user_id` で全部の行を消します。

**問2の解答:**

- 照合が通ったあとで、Topcoat の `session::refresh(cx)` を呼びます。`refresh` は、同じトークンのまま、クッキーの期限を延ばし、新しい `expires_at` を返すので、それを `UPDATE sessions SET expires_at = ? WHERE token_hash = ?` で DB にも書きます。
- ただし、毎回のリクエストで書き込むと、DB の書き込みが増えます。「残りが半分を切ったときだけ延ばす」ようにすると、書き込みを減らせます。延ばし続けると、盗まれたトークンも生き続けるので、「ログインから最長 90 日」のような上限も一緒に決めます。

**問3の解答:**

- 期限切れの行は、**誰かがログインしたとき**に、`log_in` の中の `delete_expired` で消えます（例5-2の1つ目で、bob のログインのときに消えました）。
- 誰もログインしなければ、期限切れの行は残り続けます。照合で期限を見ているので、使われる心配はありませんが、行はたまっていきます。決まった時間ごとに掃除する仕事（第5巻 UNIT 10 のバックグラウンドジョブ）にするか、`idx_sessions_expires` の索引を使う `DELETE` を、起動のときにも動かす、などの方法があります。

### まとめ

- Topcoat は、トークンを作ってクッキーで運び、アプリには `token_hash` と `expires_at` だけを渡す。RustNote は、ハッシュ・持ち主・期限を `sessions` 表に記録し、毎回、ハッシュと期限で照合する。期限は「ログインから 30 日」で、使っても延びない。
- ログアウトの本体は、DB の記録を消すこと。クッキーを消すだけでは、写し取られたクッキーが使い続けられる。照合で期限を見ないと、期限切れのセッションが生きる。サーバーの側で行を消せば、全部の端末のセッションを、すぐに終わらせられる。
- 期限切れの行は、ログインのときの `delete_expired` で消える。延ばしたいときは `session::refresh` と、DB の `expires_at` の更新を組にする。
