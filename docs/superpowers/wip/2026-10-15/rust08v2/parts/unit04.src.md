## UNIT 04: ログイン・同じ文で断る・回数の制限

### この単元のゴール

- ログイン（`POST /login`）が、メールのそろえ方 → 探す → パスワードの照合 → 新しいセッション → 303、の順で動き、失敗はどれも同じ 401「メールアドレスまたはパスワードが違います」になることを確かめられる
- 失敗の画面の**文**と**時間**の両方をそろえる理由と、ダミーのハッシュ（`DUMMY_HASH`）の役目を、測った数で説明できる
- ログインのたびにセッションのトークンが新しくなり、前のトークンが使えなくなることを確かめられる
- RustNote に無い**回数の制限**を、「何を鍵に数えるか」から設計でき、メールごと・送り元ごとの制限の得意・不得意を言える

### たとえ話でつかむ

マンションの、暗証番号で開く入口を考えます。

- 間違った番号を押したとき、機械が「その部屋番号の人は住んでいません」「部屋番号は合っていますが、番号が違います」と**言い分ける**と、泥棒は、どの部屋に人が住んでいるかを知ることができます。だから、どちらでも「開きません」とだけ言います。
- 言葉をそろえても、住んでいない部屋のときだけ**すぐに**「開きません」と言い、住んでいる部屋のときは確かめに3秒かかるなら、時間で分かってしまいます。だから、どちらも同じくらい待たせます。
- 入るたびに、新しい**入館証**を渡し、前の入館証は使えなくします。誰かに前の入館証を拾われても、入れません。
- 何度も間違える人がいたら、しばらく押せなくします。ただし、「部屋番号ごと」に止めると、泥棒は部屋を変えながら押し続けます。「その人ごと（入口の前に立っている人ごと）」に数えることも考えます。

| マンションの入口 | RustNote のログイン |
|---|---|
| どちらでも「開きません」 | 401「メールアドレスまたはパスワードが違います」（`LOGIN_FAILED`） |
| どちらも同じくらい待たせる | 登録が無くても `DUMMY_HASH` で照合する（約 0.4 秒、debug ビルド） |
| 入るたびに新しい入館証 | `log_in` が前のセッションを消し、新しいトークンを出す |
| しばらく押せなくする | 回数の制限。**RustNote には無い**（例4-3で形を作る） |

> **用語:** アカウントの列挙（画面の文や返事の時間の違いから、あるメールアドレスが登録されているかどうかを調べること。登録されていると分かったアドレスは、パスワードの総当たりや、だましのメールの的になる）

<!-- -->

> **用語:** セッションの固定（攻撃する人が、自分の知っているトークンを利用者に使わせてから、利用者にログインさせ、そのトークンで成りすますこと。ログインのたびにトークンを新しくすると防げる）

<!-- -->

> **用語:** パスワードスプレー（よく使われるパスワードを1つ決めて、たくさんの人のメールアドレスで1回ずつ試す攻撃。1人あたりの失敗は少ないので、メールごとの回数の制限では見つけにくい）

### 図で理解する

`POST /login` の流れです（`src/auth.rs` の `authenticate` と、`src/routes/login.rs` の `login`）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-0401" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="30" width="150" height="60" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="85" y="56" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">Email::parse</text>
<text x="85" y="76" font-size="11" fill="#475569" text-anchor="middle">そろえる・形を見る</text>
<rect x="190" y="30" width="150" height="60" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="265" y="56" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">find_by_email</text>
<text x="265" y="76" font-size="11" fill="#475569" text-anchor="middle">登録を探す</text>
<rect x="370" y="30" width="160" height="60" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="450" y="56" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">verify_password</text>
<text x="450" y="76" font-size="11" fill="#475569" text-anchor="middle">保存したハッシュで照合</text>
<rect x="560" y="30" width="150" height="60" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="635" y="52" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">log_in → 303</text>
<text x="635" y="70" font-size="10" fill="#475569" text-anchor="middle">前のセッションを消し</text>
<text x="635" y="84" font-size="10" fill="#475569" text-anchor="middle">新しいトークンを出す</text>
<line x1="160" y1="60" x2="186" y2="60" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0401)"/>
<line x1="340" y1="60" x2="366" y2="60" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0401)"/>
<line x1="530" y1="60" x2="556" y2="60" stroke="#10b981" stroke-width="2" marker-end="url(#ar-08-0401)"/>
<text x="543" y="52" font-size="10" fill="#047857" text-anchor="middle">合う</text>
<line x1="85" y1="90" x2="85" y2="146" stroke="#f97316" stroke-width="2" marker-end="url(#ar-08-0401)"/>
<text x="95" y="122" font-size="10" fill="#c2410c">形が違う</text>
<line x1="265" y1="90" x2="265" y2="146" stroke="#f97316" stroke-width="2" marker-end="url(#ar-08-0401)"/>
<text x="275" y="122" font-size="10" fill="#c2410c">無い</text>
<line x1="450" y1="90" x2="450" y2="196" stroke="#f97316" stroke-width="2" marker-end="url(#ar-08-0401)"/>
<text x="460" y="122" font-size="10" fill="#c2410c">合わない</text>
<rect x="10" y="150" width="330" height="40" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="175" y="175" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">DUMMY_HASH で照合する（同じくらい時間をかける）</text>
<line x1="175" y1="190" x2="400" y2="214" stroke="#f97316" stroke-width="2" marker-end="url(#ar-08-0401)"/>
<rect x="250" y="200" width="400" height="66" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="450" y="226" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">401「メールアドレスまたはパスワードが違います」</text>
<text x="450" y="250" font-size="11" fill="#475569" text-anchor="middle">3つの失敗は、文も、画面も、時間も同じ</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: ログインの流れ。メールの形が違う・登録が無い・パスワードが合わない、の3つの失敗は、どれも照合の計算をしてから、同じ 401 の画面を返す。合ったときだけ、新しいセッションで 303 へ移る</div>
</div>

回数の制限を、何を鍵に数えるかの比べです（例4-3で、実際に試した結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="240" height="36" rx="6" fill="#0f172a"/>
<text x="130" y="33" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">攻め方</text>
<rect x="260" y="10" width="220" height="36" rx="6" fill="#0f172a"/>
<text x="370" y="33" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">メールごとに数える</text>
<rect x="490" y="10" width="220" height="36" rx="6" fill="#0f172a"/>
<text x="600" y="33" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">送り元ごとに数える</text>
<text x="20" y="80" font-size="12" fill="#0f172a">1人のメールに総当たり</text>
<text x="370" y="80" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">止まる（6回目から）</text>
<text x="600" y="80" font-size="12" fill="#047857" text-anchor="middle">止まる（同じ送り元なら）</text>
<text x="20" y="130" font-size="12" fill="#0f172a">パスワードスプレー</text>
<text x="20" y="146" font-size="10" fill="#475569">（1つのパスワードを7人に）</text>
<text x="370" y="138" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">止まらない（1人1回）</text>
<text x="600" y="138" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">止まる（6人目から）</text>
<text x="20" y="190" font-size="12" fill="#0f172a">わざと本人を締め出す</text>
<text x="370" y="190" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">本人も入れない（窓の間）</text>
<text x="600" y="190" font-size="12" fill="#047857" text-anchor="middle">別の送り元の本人は入れる</text>
<line x1="10" y1="102" x2="710" y2="102" stroke="#e2e8f0" stroke-width="1"/>
<line x1="10" y1="162" x2="710" y2="162" stroke="#e2e8f0" stroke-width="1"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: 回数の制限の鍵の比べ（10 分に 5 回まで）。メールごとだけでは、パスワードスプレーを止められず、本人の締め出しに使われる。送り元ごとの制限と組み合わせる</div>
</div>

### まず動かす最小コード

#### 例4-1: いろいろな入力でログインして、失敗の画面を比べる

ログインの画面（`src/routes/login.rs` の `login`）です。

```rust
/// ログインに失敗したときの文章
const LOGIN_FAILED: &str = "メールアドレスまたはパスワードが違います";

#[page(POST)]
pub async fn login(cx: &Cx, Form(input): Form<LoginForm>) -> Result<impl View> {
    if let Some(user) = authenticate(cx, &input.email, &input.password).await? {
        log_in(cx, &user).await?;
        flash::set(cx, Flash::LoggedIn);
        return Err(see_other(href!(crate::routes::notes::index).resolve(cx)).into());
    }
    // メールが無いのか、パスワードが違うのかは区別しない
    let error = Some(LOGIN_FAILED.to_string());
    Ok(view! {
        (StatusCode::UNAUTHORIZED)
        login_form(email: input.email, error: error)
    })
}
```

照合は `src/auth.rs` の `authenticate` です。登録が無いときも、形が違うときも、ダミーのハッシュで照合してから `None` を返します（抜き出しています）。

```rust
// 形式の違うメールは「見つからない」と同じ扱いにする
let Ok(email) = Email::parse(email_input) else {
    verify_password(password, &DUMMY_HASH).await?;
    return Ok(None);
};
match app_state(cx).users.find_by_email(&email).await? {
    Some(record) => {
        if verify_password(password, &record.password_hash).await? {
            Ok(Some(record.user))
        } else {
            Ok(None)
        }
    }
    None => {
        verify_password(password, &DUMMY_HASH).await?;
        Ok(None)
    }
}
```

> **▼ このコードがやること（先に日本語で）:** alice を登録してから、6通り（正しい・大文字のメール・違うパスワード・登録していないメール・形の違うメール・空）でログインの POST を送ります。返事の状態コードと、成功なら新しいセッションのクッキーがあるか、失敗なら画面の文を表示します。最後に、失敗の画面どうしを、入力欄に残った自分のメールだけを伏せて、1文字残らず同じかで比べます。

```rust
@@FILE:rustnote/tests/u04_login.rs@@
```

```powershell
cargo test --test u04_login -- --nocapture --test-threads=1
```

```text
@@OUTGREP:u04_login:^([0-9]\. |   (新しい|文)|---|[0-9] と)@@
```

（結果の行だけを抜き出しています）

#### ▼ コードを1つずつ分解して解説

##### 解説1: 4つの失敗は、画面が1文字も違わない

- 3〜6の失敗は、どれも `401 Unauthorized` で、文も「メールアドレスまたはパスワードが違います」でした。入力欄に残る「自分で打ったメール」を伏せると、画面は1文字残らず同じ（`まったく同じ = true`）です。画面から分かるのは「ログインできなかった」ことだけです。
- 2の `ALICE@example.com` は、`Email::parse` で `alice@example.com` にそろえてから探すので、ログインできました（UNIT 03）。
- 401 は「あなたが誰かを確かめられませんでした」という返事です。404（ページが無い）や 400（送り方が違う）にしないのは、送り方は正しく、確かめに失敗した、という意味を伝えるためです。

##### 解説2: 時間もそろえる

> **▼ このコードがやること（先に日本語で）:** 先に1回ログインに失敗させて、ダミーのハッシュを作っておきます（UNIT 02 の問2のとおり、最初の1回だけ遅いため）。そのあと、「登録していないメール」と「パスワードが違う」の失敗を、交互に4回ずつ送り、それぞれにかかった時間を表示します。

```rust
@@FILE:rustnote/tests/u04_timing.rs@@
```

```powershell
cargo test --test u04_timing -- --nocapture
```

```text
@@OUTGREP:u04_timing:^(登録|パスワード)@@
```

- 「登録していないメール」と「パスワードが違う」は、どの回も約 370〜410 ms で、ほぼ同じ時間でした。どちらも Argon2id の照合を1回しているからです。
- このテストは、2つを**交互に**送っています。続けて計算すると、PC の都合で後半の回が遅くなることがあり（UNIT 01 の問1・UNIT 02 の問2では、後半が 600〜800 ms 台になりました）、片方をまとめて先に送ると、その遅れを「登録の有無の差」と見間違えてしまうからです。測って比べるときは、比べる物を同じ条件で交互に並べます。

### もう一歩: 実務に近い形

#### 例4-2: ログインのたびに、トークンが新しくなる

> **▼ このコードがやること（先に日本語で）:** alice を登録して、そのときのトークン A を持ったまま、もう一度ログインして、トークン B を受け取ります。A と B の先頭6文字と、`sessions` 表の行の数、A と B が同じかを表示します。最後に、A と B のそれぞれで一覧を開きます。

```rust
@@FILE:rustnote/tests/u04_renew.rs@@
```

```powershell
cargo test --test u04_renew -- --nocapture
```

```text
@@OUTGREP:u04_renew:^([0-9]\. |   A)@@
```

- ログインのあとのトークン B は、A と違い（`false`）、`sessions` 表は1行のままでした。`log_in`（`src/auth.rs`）は、前のセッションのクッキーがあれば、その記録を先に消してから、`session::start` で新しいトークンを出します。
- A で一覧を開くと `303 → /login`（もう使えない）、B なら `200` です。もし攻撃する人が、何かの方法で A を利用者のブラウザに入れておいても、利用者がログインした時点で A は消えるので、成りすませません（セッションの固定を防ぐ）。

#### 例4-3: 回数の制限の、いちばん小さな形

RustNote には、ログインの回数の制限がありません（UNIT 01 の問1で、10 回まちがえても止まらないことを確かめました）。ここでは、RustNote に足すときの**考え方**を、小さな例で確かめます（RustNote のコードは変えていません）。

> **▼ このコードがやること（先に日本語で）:** 「10 分の窓の中で、同じ鍵の失敗が 5 回を超えたら断る」制限を、鍵ごとに覚える `HashMap` で作ります。時計は、始めの時刻に分を足して、自分で進めます。前半は、alice のメールに1分ごとに7回失敗し、8分目と12分目に本人が試します。後半は、1つのパスワードを7人のメールで1回ずつ試し、「メールごと」と「送り元（`203.0.113.7`）ごと」の2つの制限が、それぞれ許すかを表示します。

```rust
@@FILE:rustnote/examples/u04_limiter.rs@@
```

```powershell
cargo run -q --example u04_limiter
```

```text
@@OUT:u04_limiter@@
```

- **1人への総当たり:** 5回までは試せて、6回目から断りました。12分目には、1分目から 10 分の窓が過ぎたので、また試せます。
- **本人の締め出し:** 8分目に、本人が正しいパスワードで入ろうとしても「断る」でした。メールごとの制限は、**攻撃する人が、本人をわざと締め出す**のにも使えてしまいます。
- **パスワードスプレー:** 7人のメールに1回ずつなら、メールごとの制限は1人1回なので、全部「試せる」でした。送り元ごとの制限なら、6人目から断れます。ただし、送り元を変えながら攻める人もいるので、送り元ごとの制限も、それだけでは足りません。
- 実際には、両方の制限を組み合わせ、断るときも、ログインの画面と同じく「違います」と同じ文・同じ時間にします（数えていることを、教えすぎないため）。数える場所は、サーバーが1台なら、この例のようにメモリでもよく、何台もあるなら DB（Turso）や、共有の置き場が要ります（第12巻 UNIT 09（準備中））。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: 失敗の理由で、文を変えた

「何が違うのか教えたほうが親切だ」と考えて、ログインに失敗したとき、登録があるかを探し直し、「パスワードが違います」「そのメールアドレスは登録されていません」と言い分けた場合です（練習用のコピーの `src/routes/login.rs` を書き換え、確かめたら元に戻します）。例4-1のテストを動かします。

```text
@@OUTGREP:u04_err_message:^([0-9]\. |   文|[0-9] と|warning: constant)@@
```

- **何が起きたか:** alice のメールでは「パスワードが違います」、dave・形の違うメール・空では「登録されていません」と、文が分かれました。画面は、もう同じではありません（`false`）。
- **困ること:** 攻撃する人は、メールアドレスの一覧を1つずつ送るだけで、どれが RustNote に登録されているかを知れます（アカウントの列挙）。パスワードの総当たりは、登録が分かったアドレスに絞れます。
- コンパイラーの `warning: constant LOGIN_FAILED is never used`（使われていない定数）が、唯一の手がかりでした。

#### エラー2（エラーが出ない間違い）: 登録が無ければ、照合せずにすぐ返した

`authenticate` の `None =>`（登録が無い）のところで、`DUMMY_HASH` の照合を外し、すぐ `Ok(None)` を返した場合です。例4-1の解説2のテストを動かします。

```text
@@OUTGREP:u04_err_dummy:^(登録|パスワード)@@
```

- **何が起きたか:** 文と画面は同じままですが、「登録していないメール」は **2〜4 ms**、「パスワードが違う」は約 400 ms と、**百倍以上**の差がつきました。時間を測るだけで、登録されているかが分かります。
- **学べること:** 失敗の理由を隠すときは、**文・画面・状態コード・時間**のすべてをそろえます。どれか1つでも違えば、そこから漏れます。ダミーのハッシュで照合するのは、時間をそろえるためです。

#### エラー3: .await を忘れて、? を付けた（E0277）

照合の行を、`if verify_password(password, &record.password_hash)? {` と書いた場合です（`.await` を忘れた）。

```text
@@OUTHEAD:u04_err_await:13@@
```

- **何が起きたか:** 「`?` は `Try` を実装した値にしか使えません」というエラーで、`?` を付けた値の型は `impl Future<Output = Result<bool, AppError>>` でした。`verify_password` は `async fn` なので、呼んだだけでは、まだ動いていない「これから結果が出る物」（`Future`）が返ります（第5巻 UNIT 01）。
- **直し方:** `help:` のとおり、`.await` で結果を待ってから `?` を付けます（`.await?`）。

### RustNote ではここで使う

- ログインは `src/routes/login.rs` です。ファイルの先頭のコメントに「メールが無いのかパスワードが違うのかを区別せず、401 で返す（登録の有無を推測させないため）」と書いてあります。
- `src/auth.rs` の `authenticate` は、3つの失敗のどれでも Argon2id の照合を1回します。`DUMMY_HASH` は `LazyLock` なので、起動して最初の1回だけ遅いことは、UNIT 02 の問2で見つけた改善点です。
- `log_in` は、前のセッションの記録を消してから、新しいトークンを出し、ついでに期限切れのセッションを掃除します（UNIT 05）。
- 回数の制限は、RustNote にありません（UNIT 01 の脅威モデルで「守りが無い」とした所）。足すなら、例4-3のように、メールごとと送り元ごとを組み合わせます。

### 練習問題

**問1:** alice でログインしたまま、ログインの画面（`GET /login`）を開くと、どうなるでしょうか。また、alice のクッキーのまま、bob のメールとパスワードでログインすると、alice のセッションはどうなるでしょうか。

**問2:** 例4-3で、メールごとの制限だけを使うと困る攻め方を、2つ挙げてください。

**問3:** 「5回まちがえたら、そのアカウントを止めて、本人がメールで解除するまで入れない」作りにすると、何が困るでしょうか。

### 解答と解説

**問1の解答:**

```rust
@@FILE:rustnote/tests/u04_q.rs@@
```

```powershell
cargo test --test u04_q -- --nocapture
```

```text
@@OUTGREP:u04_q:^問@@
```

- ログイン済みで `GET /login` を開くと、`303 See Other → /notes` でした。`login_page` が、最初に `current_user` で確かめて、ログイン済みなら一覧へ送ります。
- alice のクッキーのまま bob でログインすると、`303` で、セッションは `alice@example.com=0 bob@example.com=2` になりました。`log_in` が、送られてきたクッキー（alice のトークン）の記録を消してから、bob の新しいセッションを作ったからです（bob の2つは、登録したときと、今のログインの分）。右上の表示は `bob@example.com` で、alice の古いクッキーでは、もう一覧を開けません（`303`）。

**問2の解答:**

- **パスワードスプレー:** 1つのパスワードを、たくさんの人のメールで1回ずつ試されると、1人あたりの失敗は1回なので、止まりません（例4-3の後半で、7人とも「試せる」）。
- **本人の締め出し:** 攻撃する人が、alice のメールでわざと5回失敗すれば、本人の alice も、窓の間は入れなくなります（例4-3の8分目）。どちらも、送り元ごとの制限と組み合わせて防ぎます。

**問3の解答:**

- 問2の「本人の締め出し」が、窓の時間ではなく、**本人が解除するまで**続きます。攻撃する人は、狙った人のメールアドレスで5回失敗するだけで、その人を RustNote から追い出せます。
- 解除のメールを送る仕組み（と、そのメールが届くこと）も要ります。だから、多くのサービスは、止めるのではなく「しばらく待たせる」「待つ時間を少しずつ長くする」「確かめの手続き（画像の文字を読むなど）を足す」といった、弱めの方法を組み合わせます。二段階の認証（UNIT 09）があれば、パスワードが当てられても入れないので、回数の制限は弱めにできます。

### まとめ

- ログインは、メールをそろえて探し、パスワードを照合し、合えば前のセッションを消して新しいトークンで 303。失敗は、メールの形が違う・登録が無い・パスワードが違う、のどれでも同じ 401 の画面で、時間もダミーのハッシュでそろえる。
- 文を言い分けたり、登録が無いときだけすぐ返したりすると、画面や時間からアカウントの列挙ができる。そろえるのは、文・画面・状態コード・時間のすべて。
- RustNote には回数の制限が無い。足すなら、メールごとと送り元ごとを組み合わせる。メールごとだけでは、パスワードスプレーを止められず、本人の締め出しに使われる。
