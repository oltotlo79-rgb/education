## UNIT 03: 登録・メールのそろえ方・重複

### この単元のゴール

- 新規登録（`POST /signup`）が、入力を全部検査してから（400）、パスワードをハッシュにして、`INSERT` し、`UNIQUE` に止められたら 409、通ったらそのままログインして 303、の順で動くことを確かめられる
- メールアドレスを「前後の空白を取って、小文字にそろえる」理由と、そろえないと同じ人が2つの登録を持てることを説明できる
- 同じメールの登録が**同時に2つ**届いても、DB の `UNIQUE` が1つだけにすることと、「先に探してから入れる」形では2つできてしまうことを、実際に確かめて説明できる
- `Email` 型で、検査していない文字列を `create` に渡せないことと、そのときの `E0308` を読める

### たとえ話でつかむ

図書館の、利用者カードの受付を考えます。

- 申込書の名前は、人によって「Alice」「alice」「 ALICE 」と書き方がばらばらです。受付の人は、名簿に書く前に、**書き方をそろえて**（空白を取り、小文字にして）から書きます。そろえないと、同じ人のカードが2枚できます。
- 受付の窓口が2つあって、同じ人が両方に同時に申し込んだとします。窓口の人が「名簿を見て、無いから作る」とすると、2人とも「無い」と思って、カードが2枚できます。そこで、**名簿の帳面そのものに**「同じ名前は2行書けない」という決まりを付けておきます。あとから書こうとした窓口は、帳面に断られます。
- 申込書の書き間違い（「＠」が無い、確認の欄と違う）は、**全部まとめて**付箋で返します。カードを作る手間は、全部そろってからかけます。

| 図書館の受付 | RustNote の新規登録 |
|---|---|
| 書き方をそろえてから名簿に書く | `Email::parse`（`trim` と `to_lowercase`） |
| 帳面に「同じ名前は2行書けない」 | `users.email` の `UNIQUE`（`migrations/0001_users.sql`） |
| あとから書こうとした窓口は断られる | `RepoError::UniqueViolation` → 409「既に登録されています」 |
| 書き間違いは全部まとめて返す | メール・パスワード・確認の問題を集めて 400 |

> **用語:** 正規化（せいきか：同じものを表す、いろいろな書き方を、1つの決まった書き方にそろえること。ここでは、メールアドレスの前後の空白を取り、小文字にする）

<!-- -->

> **用語:** UNIQUE 制約（表の列に「同じ値の行を2つ作ってはいけない」と決めておく決まり。破る `INSERT` は、DB がエラーにする。第7巻 UNIT 01）

<!-- -->

> **用語:** 409 Conflict（「今のデータの状態と、ぶつかっています」という返事。RustNote の登録では、メールアドレスが登録済みのときに使う）

### 図で理解する

`POST /signup` が届いてから、返事をするまでの流れです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-0301" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="20" width="200" height="120" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="110" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">① 入力を全部検査する</text>
<text x="24" y="68" font-size="11" fill="#0f172a">Email::parse（空白・小文字・形）</text>
<text x="24" y="90" font-size="11" fill="#0f172a">Password::parse（8〜128 文字）</text>
<text x="24" y="112" font-size="11" fill="#0f172a">確認の欄と同じか</text>
<text x="24" y="130" font-size="10" fill="#475569">途中で止めず、問題を全部集める</text>
<rect x="250" y="20" width="200" height="120" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="350" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">② ハッシュにして INSERT</text>
<text x="264" y="70" font-size="11" fill="#0f172a">hash_password（Argon2id）</text>
<text x="264" y="92" font-size="11" fill="#0f172a">INSERT INTO users …</text>
<text x="264" y="114" font-size="11" fill="#0f172a">email の UNIQUE が見張る</text>
<rect x="490" y="20" width="220" height="120" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="600" y="44" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">③ そのままログイン</text>
<text x="504" y="70" font-size="11" fill="#0f172a">log_in（セッションを作る）</text>
<text x="504" y="92" font-size="11" fill="#0f172a">お知らせ「登録しました」</text>
<text x="504" y="118" font-size="12" font-weight="700" fill="#047857">303 → /notes</text>
<line x1="210" y1="80" x2="246" y2="80" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0301)"/>
<line x1="450" y1="80" x2="486" y2="80" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0301)"/>
<line x1="110" y1="140" x2="110" y2="186" stroke="#f97316" stroke-width="2" marker-end="url(#ar-08-0301)"/>
<line x1="350" y1="140" x2="350" y2="186" stroke="#f97316" stroke-width="2" marker-end="url(#ar-08-0301)"/>
<rect x="10" y="190" width="200" height="86" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="110" y="214" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">400（すぐ返る）</text>
<text x="110" y="236" font-size="11" fill="#0f172a" text-anchor="middle">問題を入力欄の下に全部</text>
<text x="110" y="256" font-size="11" fill="#0f172a" text-anchor="middle">メールは残し、パスワードは消す</text>
<rect x="250" y="190" width="200" height="86" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="350" y="214" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">409（ハッシュのあと）</text>
<text x="350" y="236" font-size="11" fill="#0f172a" text-anchor="middle">UniqueViolation</text>
<text x="350" y="256" font-size="11" fill="#0f172a" text-anchor="middle">「既に登録されています」</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: 新規登録の流れ。入力の問題は全部集めてすぐ 400 で返し、そろったら重いハッシュを計算して INSERT する。登録済みかどうかは、DB の UNIQUE が INSERT のときに決める</div>
</div>

同じメールの登録が、同時に2つ届いたときです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="180" y="22" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">先に探してから入れる（エラー1）</text>
<rect x="20" y="34" width="320" height="214" rx="10" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="100" y="60" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">1つ目</text>
<text x="260" y="60" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">2つ目</text>
<text x="100" y="88" font-size="11" fill="#0f172a" text-anchor="middle">① 探す: 無い</text>
<text x="260" y="112" font-size="11" fill="#0f172a" text-anchor="middle">② 探す: 無い</text>
<text x="100" y="136" font-size="11" fill="#475569" text-anchor="middle">（ハッシュに約 0.4 秒）</text>
<text x="260" y="160" font-size="11" fill="#475569" text-anchor="middle">（ハッシュに約 0.4 秒）</text>
<text x="100" y="180" font-size="11" fill="#0f172a" text-anchor="middle">③ 入れる</text>
<text x="260" y="196" font-size="11" fill="#0f172a" text-anchor="middle">④ 入れる</text>
<text x="180" y="230" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">→ 同じメールの行が2つ</text>
<text x="540" y="22" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">UNIQUE に任せる（RustNote）</text>
<rect x="380" y="34" width="320" height="214" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="460" y="60" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">1つ目</text>
<text x="620" y="60" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">2つ目</text>
<text x="460" y="96" font-size="11" fill="#475569" text-anchor="middle">（ハッシュに約 0.4 秒）</text>
<text x="620" y="120" font-size="11" fill="#475569" text-anchor="middle">（ハッシュに約 0.4 秒）</text>
<text x="460" y="150" font-size="11" fill="#0f172a" text-anchor="middle">③ 入れる → 成功</text>
<text x="620" y="176" font-size="11" fill="#0f172a" text-anchor="middle">④ 入れる → UNIQUE で失敗</text>
<text x="540" y="214" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">→ 行は1つ。2つ目は 409</text>
<text x="540" y="234" font-size="11" fill="#475569" text-anchor="middle">「探す」と「入れる」の間が無い</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: 同時に2つの登録。先に探してから入れると、探す・入れるの間（ハッシュの 0.4 秒）に、もう1つが入り込む。UNIQUE は INSERT そのものが確かめるので、間が無い</div>
</div>

### まず動かす最小コード

#### 例3-1: いろいろな入力で、新規登録する

メールアドレスの検査とそろえ方は、`src/domain.rs` の `Email::parse` です（抜き出しています）。

```rust
pub fn parse(input: &str) -> Result<Email, EmailError> {
    // 同じ人を同じ値にするため、前後の空白を除いて小文字にそろえる
    let normalized = input.trim().to_lowercase();
    if normalized.len() > Self::MAX_LEN {
        return Err(EmailError::TooLong);
    }
    let Some((local, domain)) = normalized.split_once('@') else {
        return Err(EmailError::MissingAt);
    };
    // …（@ の前後が空・ドメインに . が無い・空白がある、などは Malformed）
    Ok(Email(normalized))
}
```

登録の本体は、`src/routes/signup.rs` の `register` です。入力の検査で止めずに、問題を全部集めてから、400 にするかどうかを決めます。

```rust
let (Some(email), Some(password), None) = (email, password, &form.password_confirm_error)
else {
    return Ok(SignupOutcome::Rejected(StatusCode::BAD_REQUEST, form));
};

// 2) パスワードをハッシュ化して保存する（生のパスワードは保存しない）
let password_hash = hash_password(&password).await?;
let user = match app_state(cx).users.create(&email, &password_hash).await {
    Ok(user) => user,
    // users.email の UNIQUE に引っかかった ＝ 登録済みのメール
    Err(RepoError::UniqueViolation(_)) => {
        form.email_error = Some(AppError::EmailTaken.to_string());
        return Ok(SignupOutcome::Rejected(StatusCode::CONFLICT, form));
    }
    Err(other) => return Err(other.into()),
};
```

> **▼ このコードがやること（先に日本語で）:** 8通りの入力（正しい登録・大文字と空白の混ざった同じメール・@ の無いメール・ドメインに . の無いメール・短いパスワード・確認の欄と違うパスワード・3つとも間違い・大文字の混ざった別のメール）で `POST /signup` を送ります。返事の状態コードと、400 や 409 のときは入力欄の下のエラーと、メールの入力欄に残った値を表示します。最後に、`users` 表の `email` を表示します。

```rust
//! 第8巻 UNIT 03: いろいろな入力で新規登録して、返事と、画面のエラーと、users 表に入った値を確かめる。
//! 実行: cargo test --test u03_signup -- --nocapture --test-threads=1

mod common;

use common::{TestApp, body_text, encode, test_app};

/// 画面の、入力欄の下のエラー（<p class="field-error">）を全部
fn field_errors(html: &str) -> Vec<String> {
    html.split("<p class=\"field-error\">")
        .skip(1)
        .map(|t| t.split("</p>").next().unwrap().to_string())
        .collect()
}

/// メールの入力欄に残っている値
fn email_value(html: &str) -> String {
    html.split("<input")
        .find(|t| t.contains("name=\"email\""))
        .and_then(|t| t.split("value=\"").nth(1))
        .map(|t| t.split('"').next().unwrap().to_string())
        .unwrap_or_else(|| "（なし）".to_string())
}

/// users 表の email を、id の順に
async fn emails(app: &TestApp) -> Vec<String> {
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query("SELECT email FROM users ORDER BY id", ())
        .await
        .unwrap();
    let mut found = Vec::new();
    while let Some(row) = rows.next().await.unwrap() {
        found.push(row.get::<String>(0).unwrap());
    }
    found
}

#[tokio::test]
async fn signup_cases() {
    let app = test_app().await;
    let good = "correct horse battery";
    let cases = [
        ("alice@example.com", good, good),
        ("  Alice@Example.COM  ", good, good),
        ("alice", good, good),
        ("alice@example", good, good),
        ("bob@example.com", "short", "short"),
        ("bob@example.com", good, "correct horse batterY"),
        ("bob", "short", "other"),
        ("Bob@Example.com", good, good),
    ];
    for (n, (email, password, confirm)) in cases.into_iter().enumerate() {
        let form = format!(
            "email={}&password={}&password_confirm={}",
            encode(email),
            encode(password),
            encode(confirm)
        );
        let response = app.post_form("/signup", &form, None).await;
        let status = response.status();
        let html = body_text(response).await;
        println!("{}. email={email:?} → {status}", n + 1);
        if !status.is_redirection() {
            println!("   エラー: {:?}", field_errors(&html));
            println!("   メールの入力欄: {:?}", email_value(&html));
        }
    }
    println!("users 表の email: {:?}", emails(&app).await);
}
```

```powershell
cargo test --test u03_signup -- --nocapture --test-threads=1
```

**▼ 実行結果**（`POST /signup -> …` のような行は、RustNote のアクセスログです）

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.64s
     Running tests\u03_signup.rs (target\debug\deps\u03_signup-6525422e8736d1b3.exe)

running 1 test
test signup_cases ... POST /signup -> 303 See Other (427.2562ms)
1. email="alice@example.com" → 303 See Other
POST /signup -> 409 Conflict (405.0977ms)
2. email="  Alice@Example.COM  " → 409 Conflict
   エラー: ["そのメールアドレスは既に登録されています"]
   メールの入力欄: "  Alice@Example.COM  "
POST /signup -> 400 Bad Request (274µs)
3. email="alice" → 400 Bad Request
   エラー: ["メールアドレスに @ がありません"]
   メールの入力欄: "alice"
POST /signup -> 400 Bad Request (80.3µs)
4. email="alice@example" → 400 Bad Request
   エラー: ["メールアドレスの形式が正しくありません"]
   メールの入力欄: "alice@example"
POST /signup -> 400 Bad Request (70µs)
5. email="bob@example.com" → 400 Bad Request
   エラー: ["パスワードは8文字以上にしてください"]
   メールの入力欄: "bob@example.com"
POST /signup -> 400 Bad Request (66.8µs)
6. email="bob@example.com" → 400 Bad Request
   エラー: ["確認用のパスワードが一致しません"]
   メールの入力欄: "bob@example.com"
POST /signup -> 400 Bad Request (727µs)
7. email="bob" → 400 Bad Request
   エラー: ["メールアドレスに @ がありません", "パスワードは8文字以上にしてください", "確認用のパスワードが一致しません"]
   メールの入力欄: "bob"
POST /signup -> 303 See Other (531.5354ms)
8. email="Bob@Example.com" → 303 See Other
users 表の email: ["alice@example.com", "bob@example.com"]
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.45s
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: そろえてから比べるので、書き方が違っても同じ人

- 2の `  Alice@Example.COM  ` は、1の `alice@example.com` と、**同じ人**として 409 になりました。`Email::parse` が、前後の空白を取り、小文字にした `alice@example.com` で `INSERT` しようとして、`UNIQUE` に止められたからです。
- 8の `Bob@Example.com` は、`bob@example.com` として登録されました。`users` 表には、`["alice@example.com", "bob@example.com"]` と、そろえた形だけが入っています。ログインでも同じ `Email::parse` を通すので、どの書き方でログインしても、同じ行が見つかります。
- 入力欄に残る値は、そろえる前の `  Alice@Example.COM  ` です。利用者が打った文字を、そのまま返して、直しやすくしています（パスワードの欄は、どの返事でも空です）。

##### 解説2: 問題は全部まとめて、すぐに返す

- 7の `bob`・`short`・`other` では、「@ がありません」「8文字以上にしてください」「確認用のパスワードが一致しません」の3つが、一度に出ました。1つ直すたびに、また別の問題を言われる、ということがありません（第10巻 UNIT 02 と同じ考えです）。
- アクセスログを見ると、400 の返事は 0.1 ミリ秒以下〜1 ミリ秒でした。1と8の 303 と、2の 409 は、約 0.4 秒です。**入力が全部そろってから**、重いパスワードのハッシュ（UNIT 02）を計算しているからです。409 も、ハッシュを計算したあとの `INSERT` で分かるので、同じくらいかかります。

### もう一歩: 実務に近い形

#### 例3-2: 同じメールの登録を、同時に2つ送る

登録のボタンをダブルクリックしたときや、2つのタブで同時に送ったときを、`tokio::join!` で再現します。

> **▼ このコードがやること（先に日本語で）:** 同じメールアドレス・同じパスワードの登録の POST を、`tokio::join!` で2つ同時に送ります。それぞれの返事の状態コードと、そのあとの `users` 表の `email` を表示します。

```rust
//! 第8巻 UNIT 03: 同じメールアドレスの新規登録を、2つ同時に送る（ダブルクリックや、2つのタブ）。
//! 実行: cargo test --test u03_race -- --nocapture

mod common;

use common::{TestApp, encode, test_app};

/// users 表の email を、id の順に
async fn emails(app: &TestApp) -> Vec<String> {
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query("SELECT email FROM users ORDER BY id", ())
        .await
        .unwrap();
    let mut found = Vec::new();
    while let Some(row) = rows.next().await.unwrap() {
        found.push(row.get::<String>(0).unwrap());
    }
    found
}

#[tokio::test]
async fn two_signups_at_once() {
    let app = test_app().await;
    let password = encode("correct horse battery");
    let form = format!(
        "email={}&password={password}&password_confirm={password}",
        encode("alice@example.com")
    );
    let (a, b) = tokio::join!(
        app.post_form("/signup", &form, None),
        app.post_form("/signup", &form, None),
    );
    println!("1つ目: {}", a.status());
    println!("2つ目: {}", b.status());
    println!("users 表の email: {:?}", emails(&app).await);
}
```

```powershell
cargo test --test u03_race -- --nocapture
```

```text
1つ目: 409 Conflict
2つ目: 303 See Other
users 表の email: ["alice@example.com"]
```

- 片方が `303`、もう片方が `409` で、`users` 表の行は1つでした。2つの登録は、どちらも「パスワードのハッシュ（約 0.4 秒）」を計算してから `INSERT` します。先に `INSERT` したほうが成功し、あとのほうは `UNIQUE` に止められました。
- どちらが先になるかは、その時しだいです（この実行では、2つ目が先でした）。大事なのは、**どちらが先でも、行が1つになる**ことです。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: UNIQUE を外して、先に探してから入れた

「登録済みかどうかは、先に `find_by_email` で探せば分かる」と考えて、`migrations/0001_users.sql` の `UNIQUE` を外し、`register` の中で、ハッシュの前に探す形にした場合です（練習用のコピーで書き換え、確かめたら元に戻します）。

```rust
// わざと間違えた版: 先に探して、無ければ登録する（DB の UNIQUE は外した）
if app_state(cx).users.find_by_email(&email).await?.is_some() {
    form.email_error = Some(AppError::EmailTaken.to_string());
    return Ok(SignupOutcome::Rejected(StatusCode::CONFLICT, form));
}
```

例3-2のテストを動かします。

```text
1つ目: 303 See Other
2つ目: 303 See Other
users 表の email: ["alice@example.com", "alice@example.com"]
```

- **何が起きたか:** 2つとも `303` で、`users` 表に `alice@example.com` が**2行**できました。2つの登録が、どちらも「探す: 無い」を通ってから、ハッシュの計算（約 0.4 秒）をしている間に、もう片方も「無い」を見たからです（図の左）。1つずつ順に試すテストでは、この間違いは見つかりません。
- **困ること:** ログインのときに `find_by_email` が、どちらの行を返すかで、パスワードが合ったり合わなかったりします。ノートも、どちらの行に付くかで、見えたり見えなかったりします。
- **学べること:** 「同じ物を2つ作らない」決まりは、**DB の制約**に書きます。アプリの「探してから入れる」は、使いやすさのための先回りの確かめとしてなら足してもよいですが、守りの本体にはなりません。第10巻 UNIT 05 の完了の切り替えと同じ、「読む・書くの間」の問題です。

#### エラー2（エラーが出ない間違い）: メールアドレスを、小文字にそろえなかった

`Email::parse` の `input.trim().to_lowercase()` を `input.trim().to_string()` にした場合です。例3-1のテストを動かします（結果の行だけを抜き出しています）。

```text
1. email="alice@example.com" → 303 See Other
2. email="  Alice@Example.COM  " → 303 See Other
3. email="alice" → 400 Bad Request
4. email="alice@example" → 400 Bad Request
5. email="bob@example.com" → 400 Bad Request
6. email="bob@example.com" → 400 Bad Request
7. email="bob" → 400 Bad Request
8. email="Bob@Example.com" → 303 See Other
users 表の email: ["alice@example.com", "Alice@Example.COM", "Bob@Example.com"]
```

- **何が起きたか:** 2の `  Alice@Example.COM  ` が `303` で登録でき、`users` 表に `alice@example.com` と `Alice@Example.COM` の2人ができました。8の `Bob@Example.com` も、大文字のまま入っています。
- **困ること:** メールアドレスの `@` の後ろ（ドメイン）は、大文字と小文字を区別しない決まりです。前（ローカル部）は、決まりの上では区別してよいのですが、実際のメールの仕組みのほとんどは区別しません。利用者は「同じアドレス」だと思っているので、`Alice@…` で登録した人が `alice@…` でログインすると、「違います」と言われます。
- **学べること:** そろえるのは、**保存するときと、探すとき**の両方です。RustNote は、どちらも `Email::parse` を通るので、1か所を直せば両方に効きます。

#### エラー3: 検査していない文字列を、そのまま渡した（E0308）

`users.create(&email, …)` を、フォームの文字列そのまま `users.create(&input.email, …)` と書いた場合です。

```text
   Compiling rustnote v0.1.0 (...\rustnote)
error[E0308]: mismatched types
   --> src\routes\signup.rs:149:49
    |
149 |     let user = match app_state(cx).users.create(&input.email, &password_hash).await {
    |                                          ------ ^^^^^^^^^^^^ expected `&Email`, found `&String`
    |                                          |
    |                                          arguments to this method are incorrect
    |
    = note: expected reference `&Email`
               found reference `&std::string::String`
note: method defined here
   --> src\repository\users.rs:23:14
    |
 23 |     async fn create(&self, email: &Email, password_hash: &str) -> Result<User, RepoError>;
    |              ^^^^^^        -----
```

- **何が起きたか:** 「`&Email` を期待しましたが、`&String` でした」です。`create` の引数は `email: &Email` で、`Email` は `Email::parse` を通らないと作れない型です（第9巻 UNIT 03）。
- **学べること:** 検査とそろえ方を忘れた文字列は、型が違うので、DB に届く前にコンパイルで止まります。エラー2のような「そろえ忘れ」も、`Email::parse` の中を直さない限り、どこか1か所だけで起きることはありません。

### RustNote ではここで使う

- 新規登録は `src/routes/signup.rs` です。ファイルの先頭のコメントに、成功（303）・形式が違う（400）・登録済み（409）の表があります。登録できたら、そのまま `log_in` でセッションを作り、お知らせ「登録しました」を予約します。
- `users.email` の `UNIQUE` は `migrations/0001_users.sql` にあり、`UNIQUE constraint failed` の文は、`src/error.rs` の `From<libsql::Error> for RepoError` が `UniqueViolation` に変えています（第9巻 UNIT 07）。
- フォームの入力欄には、`autocomplete="email"` と `autocomplete="new-password"` が付いています。ブラウザやパスワードの管理の道具が、「ここは新しいパスワードの欄」と分かるので、強いパスワードを作って覚えてくれます。
- パスワードは 8〜128 文字（`MIN_PASSWORD_CHARS`・`MAX_PASSWORD_CHARS`）で、前後の空白も含めて、そのまま使います（問3）。

### 練習問題

**問1:** alice が登録済みのときに、alice のメールで登録しようとすると、どう表示されるでしょうか。ログインの画面では、alice のメールで違うパスワードを入れたときと、登録していない dave のメールのときとで、表示は違うでしょうか。

**問2:** `ＡＬＩＣＥ@example.com`（全角の英字）で登録すると、どうなるでしょうか。

**問3:** パスワードを `  correct horse battery  `（前後に空白）で登録しました。空白なしの `correct horse battery` でログインできるでしょうか。

### 解答と解説

```rust
//! 第8巻 UNIT 03 練習問題。
//! 実行: cargo test --test u03_q -- --nocapture --test-threads=1

mod common;

use common::{TestApp, body_text, encode, test_app};

const PASSWORD: &str = "correct horse battery";

/// 画面の、入力欄の下のエラー・フォームの上のお知らせ
fn messages(html: &str) -> Vec<String> {
    let mut found = Vec::new();
    for start in ["<p class=\"field-error\">", "<p class=\"form-notice\">"] {
        for t in html.split(start).skip(1) {
            found.push(t.split("</p>").next().unwrap().to_string());
        }
    }
    found
}

async fn signup(app: &TestApp, email: &str, password: &str) -> String {
    let form = format!(
        "email={}&password={}&password_confirm={}",
        encode(email),
        encode(password),
        encode(password)
    );
    let response = app.post_form("/signup", &form, None).await;
    let status = response.status();
    let html = body_text(response).await;
    format!("{status} {:?}", messages(&html))
}

async fn login(app: &TestApp, email: &str, password: &str) -> String {
    let form = format!("email={}&password={}", encode(email), encode(password));
    let response = app.post_form("/login", &form, None).await;
    let status = response.status();
    let html = body_text(response).await;
    format!("{status} {:?}", messages(&html))
}

/// 問1: 登録の画面とログインの画面は、alice がいることを、それぞれどう伝えるか
#[tokio::test]
async fn q1_signup_tells_more_than_login() {
    let app = test_app().await;
    app.sign_up("alice@example.com", PASSWORD).await;
    println!(
        "問1 登録（alice）: {}",
        signup(&app, "alice@example.com", PASSWORD).await
    );
    println!(
        "問1 登録（carol）: {}",
        signup(&app, "carol@example.com", PASSWORD).await
    );
    println!(
        "問1 ログイン（alice・違うパスワード）: {}",
        login(&app, "alice@example.com", "wrong password").await
    );
    println!(
        "問1 ログイン（dave・登録していない）: {}",
        login(&app, "dave@example.com", "wrong password").await
    );
}

/// 問2: 全角の英字で書いたメールアドレス
#[tokio::test]
async fn q2_full_width_letters() {
    let app = test_app().await;
    app.sign_up("alice@example.com", PASSWORD).await;
    println!(
        "問2 登録（ＡＬＩＣＥ@example.com）: {}",
        signup(&app, "ＡＬＩＣＥ@example.com", PASSWORD).await
    );
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query("SELECT email FROM users ORDER BY id", ())
        .await
        .unwrap();
    let mut found = Vec::new();
    while let Some(row) = rows.next().await.unwrap() {
        found.push(row.get::<String>(0).unwrap());
    }
    println!("問2 users 表の email: {found:?}");
}

/// 問3: パスワードの前後の空白
#[tokio::test]
async fn q3_spaces_in_password() {
    let app = test_app().await;
    let spaced = "  correct horse battery  ";
    println!(
        "問3 登録（前後に空白）: {}",
        signup(&app, "alice@example.com", spaced).await
    );
    println!(
        "問3 ログイン（空白なし）: {}",
        login(&app, "alice@example.com", PASSWORD).await
    );
    println!(
        "問3 ログイン（前後に空白）: {}",
        login(&app, "alice@example.com", spaced).await
    );
}
```

```powershell
cargo test --test u03_q -- --nocapture --test-threads=1
```

```text
問1 登録（alice）: 409 Conflict ["そのメールアドレスは既に登録されています"]
問1 登録（carol）: 303 See Other []
問1 ログイン（alice・違うパスワード）: 401 Unauthorized ["メールアドレスまたはパスワードが違います"]
問1 ログイン（dave・登録していない）: 401 Unauthorized ["メールアドレスまたはパスワードが違います"]
問2 登録（ＡＬＩＣＥ@example.com）: 303 See Other []
問2 users 表の email: ["alice@example.com", "ａｌｉｃｅ@example.com"]
問3 登録（前後に空白）: 303 See Other []
問3 ログイン（空白なし）: 401 Unauthorized ["メールアドレスまたはパスワードが違います"]
問3 ログイン（前後に空白）: 303 See Other []
```

（問題の答えの行だけを抜き出しています）

**問1の解答:**

- 登録の画面は、alice のメールで `409`「そのメールアドレスは既に登録されています」でした。ログインの画面は、alice のときも dave のときも、同じ `401`「メールアドレスまたはパスワードが違います」です。
- つまり、**登録の画面からは、そのメールが登録済みかどうかが分かります**。ログインの画面で隠していても、登録の画面で分かってしまうのです。
- これを避ける方法もあります。登録の画面でも「確認のメールを送りました」と、いつも同じ返事をして、登録済みなら「すでに登録があります」というメールを本人に送る、という作り方です。ただし、メールを送る仕組みが要ります。RustNote は、使いやすさを選んで、409 で知らせています。どちらを選ぶかは、守る物の大きさで決めます（UNIT 01 の脅威モデル）。

**問2の解答:**

- `303` で、**別の人として**登録されました。`users` 表には `alice@example.com` と `ａｌｉｃｅ@example.com` が入っています。`to_lowercase` は、全角の `Ａ` を全角の `ａ` にしますが、半角の `a` にはしません。
- 全角の英字のメールアドレスは、ふつうのメールの仕組みでは届きません。直すなら、`Email::parse` で ASCII 以外の文字を断るか、Unicode の正規化（NFKC：全角の英数字を半角にそろえる方法）をしてから検査します。これは RustNote に残る改善点です（RustNote のコードは変えていません）。

**問3の解答:**

- 前後に空白のあるパスワードで登録すると、**空白なしでは `401`**、同じ空白付きなら `303` でした。`Password::parse` は、`trim` をしません（`src/domain.rs` のコメント「パスワードは空白も含めて利用者が決めたものなので、trim しない」）。
- メールアドレスはそろえて、パスワードはそろえないのは、目的が違うからです。メールアドレスは「同じ人を見分ける」ためのもので、書き方の違いを消したほうがよく、パスワードは「本人しか知らない文字の並び」なので、利用者が打ったとおりに扱います。パスワードの管理の道具が作るパスワードには、空白や記号が入ることもあります。

### まとめ

- 新規登録は、入力を全部検査して問題があればすぐ 400、そろったらハッシュにして `INSERT` し、`UNIQUE` に止められたら 409、通ったらそのままログインして 303。
- メールアドレスは `Email::parse` で前後の空白を取り、小文字にそろえてから、保存にも検索にも使う。そろえないと、書き方ごとに別の人ができる。全角の英字は、そろえても別の人になる（改善点）。パスワードは、そろえない。
- 「同じメールは1人だけ」は、DB の `UNIQUE` に任せる。先に探してから入れる形は、同時の2つの登録で2行になる。`Email` 型のおかげで、検査していない文字列は `create` に渡せない（E0308）。
