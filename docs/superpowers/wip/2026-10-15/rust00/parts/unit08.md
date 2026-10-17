## UNIT 08: 認証と認可

### この単元のゴール

- **認証**（あなたは誰？）と**認可**（それをしてよい？）の違いを説明できる
- ログインしてからページを見るまでの流れ（パスワードの確認 → **セッション** → **Cookie**）を説明できる
- パスワードをそのまま保存してはいけない理由と、**ハッシュ**の考え方が分かる
- 「他人のデータが見えてしまう」不具合と、それを防ぐ書き方が分かる

### たとえ話でつかむ

ログインのしくみは、**ホテル**とそっくりです。

- **認証はフロントでの本人確認**です。名前と身分証で「あなたが予約した Hanako さんですね」と確かめます。Web アプリでは、メールアドレスとパスワードで確かめます。
- **セッションとカードキー**。本人確認が済むと、フロントは**カードキー**を渡し、台帳に「このカードキーは Hanako さんのもの、明日の 11 時まで有効」と記録します。それ以降、毎回身分証を見せる必要はありません。Web アプリでは、このカードキーにあたる**合言葉（トークン）**をブラウザの **Cookie（クッキー）** にしまい、ブラウザはリクエストのたびにそれを添えて送ります。
- **認可は「どの部屋を開けられるか」**です。カードキーはホテルの客だという証明にはなりますが、**開けられるのは自分の部屋だけ**です。Web アプリでも、ログインしている人が**他の人のノート**を見ようとしたら断らなければなりません。
- **ログアウトはチェックアウト**です。台帳からカードキーの記録を消すので、そのカードキーではもう扉が開きません。

> **用語:** **認証（にんしょう、Authentication）** … 相手が誰なのかを確かめること。ログインがその代表です。
>
> **用語:** **認可（にんか、Authorization）** … 確かめた相手に、その操作をしてよいかを判断すること。
>
> **用語:** **Cookie（クッキー）** … サーバーがブラウザに預けておく小さなメモ。ブラウザは、同じサイトへのリクエストのたびに自動でそれを送り返します。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-19" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="14" width="332" height="232" rx="12" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="180" y="42" font-size="16" font-weight="700" fill="#1e40af" text-anchor="middle">認証: あなたは誰？</text>
<rect x="34" y="62" width="130" height="64" rx="8" fill="#ffffff" stroke="#93c5fd"/>
<text x="99" y="88" font-size="12" fill="#334155" text-anchor="middle">メールアドレス</text>
<text x="99" y="108" font-size="12" fill="#334155" text-anchor="middle">＋ パスワード</text>
<line x1="166" y1="94" x2="194" y2="94" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-19)"/>
<rect x="198" y="62" width="130" height="64" rx="8" fill="#ffffff" stroke="#93c5fd"/>
<text x="263" y="88" font-size="12" fill="#334155" text-anchor="middle">本人なら</text>
<text x="263" y="108" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">「Hanako です」</text>
<text x="34" y="156" font-size="12" fill="#334155">・ログイン画面で1回だけ行う</text>
<text x="34" y="178" font-size="12" fill="#334155">・失敗 → ログインできない</text>
<text x="34" y="200" font-size="12" fill="#334155">・ホテルならフロントの本人確認</text>
<text x="34" y="222" font-size="12" fill="#334155">・HTTP では 401（ログインが必要）</text>
<rect x="374" y="14" width="332" height="232" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="540" y="42" font-size="16" font-weight="700" fill="#c2410c" text-anchor="middle">認可: それをしてよい？</text>
<rect x="394" y="62" width="130" height="64" rx="8" fill="#ffffff" stroke="#fdba74"/>
<text x="459" y="88" font-size="12" fill="#334155" text-anchor="middle">Hanako が</text>
<text x="459" y="108" font-size="12" fill="#334155" text-anchor="middle">ノート 11 を見たい</text>
<line x1="526" y1="94" x2="554" y2="94" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-19)"/>
<rect x="558" y="62" width="130" height="64" rx="8" fill="#ffffff" stroke="#fdba74"/>
<text x="623" y="88" font-size="12" fill="#334155" text-anchor="middle">持ち主は Taro</text>
<text x="623" y="108" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">→ 断る</text>
<text x="394" y="156" font-size="12" fill="#334155">・操作のたびに毎回行う</text>
<text x="394" y="178" font-size="12" fill="#334155">・ログイン済みでも断ることがある</text>
<text x="394" y="200" font-size="12" fill="#334155">・ホテルならカードキーで開く部屋</text>
<text x="394" y="222" font-size="12" fill="#334155">・HTTP では 403 か 404</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-1: 認証と認可は別の質問。ログインできても、他人のノートは見せない</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 400" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-20" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="10" width="140" height="36" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="84" y="34" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">ブラウザ</text>
<rect x="290" y="10" width="140" height="36" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="34" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">サーバー</text>
<rect x="566" y="10" width="140" height="36" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="636" y="34" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">データベース</text>
<line x1="84" y1="48" x2="84" y2="390" stroke="#cbd5e1" stroke-width="2" stroke-dasharray="4 4"/>
<line x1="360" y1="48" x2="360" y2="390" stroke="#cbd5e1" stroke-width="2" stroke-dasharray="4 4"/>
<line x1="636" y1="48" x2="636" y2="390" stroke="#cbd5e1" stroke-width="2" stroke-dasharray="4 4"/>
<line x1="86" y1="76" x2="356" y2="76" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="220" y="70" font-size="12" fill="#1e40af" text-anchor="middle">① POST /login（メール・パスワード）</text>
<line x1="362" y1="102" x2="632" y2="102" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="498" y="96" font-size="12" fill="#047857" text-anchor="middle">② users からメールで探す</text>
<line x1="632" y1="124" x2="362" y2="124" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="498" y="140" font-size="12" fill="#047857" text-anchor="middle">password_hash を返す</text>
<rect x="250" y="150" width="220" height="30" rx="6" fill="#ffffff" stroke="#fdba74"/>
<text x="360" y="170" font-size="12" fill="#9a3412" text-anchor="middle">③ Argon2id で照合 → 本人と確認</text>
<line x1="362" y1="198" x2="632" y2="198" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="498" y="192" font-size="12" fill="#047857" text-anchor="middle">④ sessions に記録</text>
<line x1="356" y1="226" x2="86" y2="226" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="220" y="220" font-size="12" fill="#c2410c" text-anchor="middle">⑤ 303 → /notes ＋ Cookie を預ける</text>
<line x1="86" y1="270" x2="356" y2="270" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="220" y="264" font-size="12" fill="#1e40af" text-anchor="middle">⑥ GET /notes ＋ Cookie</text>
<line x1="362" y1="296" x2="632" y2="296" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="498" y="290" font-size="12" fill="#047857" text-anchor="middle">⑦ sessions で誰かを調べる</text>
<line x1="362" y1="324" x2="632" y2="324" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="498" y="318" font-size="12" fill="#047857" text-anchor="middle">⑧ その人のノートだけ探す</text>
<line x1="356" y1="364" x2="86" y2="364" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="220" y="358" font-size="12" fill="#c2410c" text-anchor="middle">⑨ 200 OK ＋ 一覧の HTML</text>
<text x="20" y="150" font-size="12" font-weight="700" fill="#1e40af">認証</text>
<text x="20" y="320" font-size="12" font-weight="700" fill="#c2410c">認可</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-2: ログインしてから一覧を見るまで。①〜⑤が認証、⑥〜⑨で毎回「誰か」を調べて認可する</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 222" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-21" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
<marker id="ar-00-22" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#dc2626"/></marker>
</defs>
<rect x="14" y="40" width="170" height="64" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="99" y="66" font-size="12" fill="#334155" text-anchor="middle">パスワード</text>
<text x="99" y="88" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle" font-family="Cascadia Mono, Consolas, monospace">correct-horse</text>
<rect x="256" y="30" width="196" height="84" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="354" y="58" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">ハッシュ関数</text>
<text x="354" y="80" font-size="12" fill="#334155" text-anchor="middle">Argon2id（わざと遅い）</text>
<text x="354" y="100" font-size="12" fill="#334155" text-anchor="middle">＋ ソルト（人ごとの乱数）</text>
<rect x="524" y="40" width="182" height="64" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="615" y="66" font-size="12" fill="#334155" text-anchor="middle">DB に保存するもの</text>
<text x="615" y="88" font-size="13" font-weight="700" fill="#047857" text-anchor="middle" font-family="Cascadia Mono, Consolas, monospace">$argon2id$v=19$…</text>
<line x1="186" y1="72" x2="252" y2="72" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-21)"/>
<line x1="454" y1="72" x2="520" y2="72" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-21)"/>
<path d="M615,108 C615,170 99,170 99,110" fill="none" stroke="#dc2626" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-00-22)"/>
<text x="360" y="184" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">× 逆向きには戻せない（一方向）</text>
<text x="360" y="208" font-size="12" fill="#334155" text-anchor="middle">ログインのときは、入力されたパスワードを同じ手順で変換して、保存された値と照らし合わせる</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-3: パスワードはハッシュにして保存する。DB が盗まれても、元のパスワードは分からない</div>
</div>

### まず動かす最小コード

#### ログインと「持ち主チェック」のミニ版

本物のログイン機能は第8巻で作ります。ここでは、認証と認可の**考え方だけ**を、メモリの中のデータで確かめます。

> **▼ このコードがやること（先に日本語で）:** 2人のユーザー（Hanako と Taro）と、それぞれのノートを1つずつ用意します。まず Hanako としてログインを試し（間違ったパスワードと正しいパスワード）、次にログインした Hanako が「自分のノート」と「Taro のノート」を見ようとしたときの結果を表示します。

```rust
// 登録した人（ユーザー）
struct User {
    id: u32,
    email: String,
    // 注意: 学習用にパスワードをそのまま持っている。本物はハッシュだけを保存する
    password: String,
}

// ノート（誰のものかを user_id で覚えている）
struct Note {
    id: u32,
    user_id: u32,
    title: String,
}

// 認証: メールアドレスとパスワードが合っていれば、その人の id を返す
fn login(users: &[User], email: &str, password: &str) -> Option<u32> {
    for user in users {
        if user.email == email && user.password == password {
            return Some(user.id);
        }
    }
    None
}

// 認可: ログイン中の人（current_user）が持ち主であるノートだけを返す
fn find_note(notes: &[Note], current_user: u32, note_id: u32) -> Option<&Note> {
    notes
        .iter()
        .find(|note| note.id == note_id && note.user_id == current_user)
}

fn main() {
    let users = vec![
        User {
            id: 1,
            email: String::from("hanako@example.com"),
            password: String::from("correct-horse"),
        },
        User {
            id: 2,
            email: String::from("taro@example.com"),
            password: String::from("battery-staple"),
        },
    ];
    let notes = vec![
        Note {
            id: 10,
            user_id: 1,
            title: String::from("牛乳を買う"),
        },
        Note {
            id: 11,
            user_id: 2,
            title: String::from("歯医者の予約"),
        },
    ];

    // 1. 認証（あなたは誰？）
    let wrong = login(&users, "hanako@example.com", "wrong-password");
    println!("[認証] 間違ったパスワード -> {wrong:?}");
    let hanako = login(&users, "hanako@example.com", "correct-horse");
    println!("[認証] 正しいパスワード   -> {hanako:?}");

    // 2. 認可（そのノートを見てよい？）
    let Some(current_user) = hanako else {
        println!("ログインできなかったので、ここで終わり");
        return;
    };
    for note_id in [10, 11] {
        match find_note(&notes, current_user, note_id) {
            Some(note) => println!("[認可] ノート {note_id} -> 200 OK「{}」", note.title),
            None => println!("[認可] ノート {note_id} -> 404 Not Found"),
        }
    }
}
```

**▼ 実行結果**

```text
[認証] 間違ったパスワード -> None
[認証] 正しいパスワード   -> Some(1)
[認可] ノート 10 -> 200 OK「牛乳を買う」
[認可] ノート 11 -> 404 Not Found
```

ログインに成功した Hanako でも、Taro のノート 11 は「見つからない（404）」になりました。

#### ▼ コードを1つずつ分解して解説

##### 解説1: データの形を決める `struct`

```rust
struct Note {
    id: u32,
    user_id: u32,
    title: String,
}
```

- `struct`（ストラクト、**構造体**）は、関係のある値をひとまとめにした「自分だけの型」を作る書き方です。データベースのテーブルの1行と同じ形にしています。
- `String` は、あとから長さが変わってもよい文字列の型です（第2巻）。
- `user_id` が「このノートの持ち主」です。認可のかなめになる列です。

##### 解説2: 認証する関数 `login`

```rust
fn login(users: &[User], email: &str, password: &str) -> Option<u32> {
    for user in users {
        if user.email == email && user.password == password {
            return Some(user.id);
        }
    }
    None
}
```

- `users: &[User]` は「ユーザーの並びを借りて受け取る」、`-> Option<u32>` は「見つかれば `Some(id)`、見つからなければ `None` を返す」という意味です。
- `&&` は「かつ」です。メールアドレスとパスワードの**両方**が合ったときだけ、その人の `id` を返します。
- 最後の `None` は「全員調べても合わなかった」ときの返り値です。Rust では、最後に書いた値（`;` を付けない）がそのまま関数の返り値になります。

> **注意:** この例はパスワードをそのまま比べていますが、**本物のアプリでは絶対にこう書きません**。パスワードをそのまま保存すると、データベースが盗まれたとき、全員のパスワードがそのまま漏れてしまいます。正しいやり方は「もう一歩」と第8巻で扱います。

##### 解説3: 認可する関数 `find_note`

```rust
fn find_note(notes: &[Note], current_user: u32, note_id: u32) -> Option<&Note> {
    notes
        .iter()
        .find(|note| note.id == note_id && note.user_id == current_user)
}
```

- `.iter().find(...)` は、並びの中から条件に合う最初の1つを探します。見つからなければ `None` です。
- `|note| ...` は **クロージャ**（その場で作る小さな関数）で、「1件ずつ `note` として受け取り、条件を満たすか」を書いています。
- 条件は「番号が一致する **かつ** 持ち主がログイン中の人」です。他人のノートは「存在しない」のと同じ扱いになります。これが認可です。

##### 解説4: ログインできなかったら先へ進まない

```rust
let Some(current_user) = hanako else {
    println!("ログインできなかったので、ここで終わり");
    return;
};
```

- `let Some(x) = 値 else { ... };` は、「値が `Some` なら中身を `x` に取り出して先へ進む。`None` なら `else` の中を実行する」という書き方です。
- `else` の中は必ず `return` などで抜けなければなりません。「ログインしていないのに、ノートを探す処理へ進んでしまう」ことが、文法の上で起こりえない形になっています。

### もう一歩: 実務に近い形

#### ハッシュの性質を確かめる（しくみを見るためのおもちゃ）

パスワードの代わりに保存する**ハッシュ**の性質を、Rust の標準ライブラリにある `DefaultHasher` で確かめます。

> **▼ このコードがやること（先に日本語で）:** いくつかの文字列から、それぞれの「指紋」のような数値（ハッシュ値）を作って表示します。同じ文字列なら同じ値になること、1文字違うだけでまったく違う値になることを確かめます。

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// 文字列から「指紋」のような数値を作る（しくみを見るためのおもちゃ）
fn toy_hash(text: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

fn main() {
    let inputs = ["correct-horse", "correct-horse", "correct-horsf", "a"];
    for input in inputs {
        // {:016x} は「16 けたの 16 進数で表示する」という指定
        println!("{input:<14} -> {:016x}", toy_hash(input));
    }
}
```

**▼ 実行結果**

```text
correct-horse  -> f96723216a3e141a
correct-horse  -> f96723216a3e141a
correct-horsf  -> db8dd628cfe17851
a              -> 719b50b9a4f0e9f3
```

（数値そのものは、Rust の版によって変わることがあります。）

#### ▼ コードを1つずつ分解して解説

##### 解説1: ハッシュ値の作り方

```rust
let mut hasher = DefaultHasher::new();
text.hash(&mut hasher);
hasher.finish()
```

- `DefaultHasher` は、データから決まった長さの数値（ハッシュ値）を作る道具です。
- `text.hash(&mut hasher)` で文字列を流し込み、`finish()` で結果の数値を取り出します。

##### 解説2: 結果から分かる性質

- **同じ入力からは、いつも同じ値**ができます（1行目と2行目）。だから、ログインのときに入力されたパスワードを同じ手順で変換すれば、保存しておいた値と照らし合わせられます。
- **1文字違うだけで、まったく違う値**になります（`horse` と `horsf`）。似ているかどうかも分かりません。
- 入力が1文字でも長い文でも、**結果は同じ長さ**です。結果から元の文字列を計算で戻すことはできません（**一方向**）。

##### 解説3: なぜ本物には Argon2id を使うのか

> **注意:** `DefaultHasher` は、データを素早く仕分けるための道具で、**パスワードの保存には絶対に使ってはいけません**。速すぎるので、盗んだ人が「よくあるパスワード」を1秒に何億通りも試して、元のパスワードを当ててしまえるからです。

パスワード専用のハッシュ関数 **Argon2id（アルゴンツー・アイディー）** は、次の工夫で当てられにくくしてあります。

- **わざと遅く、わざとメモリをたくさん使う**ので、総当たりで試すのにとても時間がかかる
- 人ごとに違う乱数（**ソルト**）を混ぜるので、同じパスワードの人がいても保存される値は別々になる

RustNote は `users.password_hash` 列に Argon2id の結果だけを保存し、元のパスワードはどこにも保存しません（第8巻）。

### よくあるエラーと読み方

#### 持ち主チェックを書き忘れた（コンパイラの警告がヒント）

最小コードの `find_note` で、持ち主の条件を消してしまったとします（抜粋。他の部分は同じです）。

```rust
// 認可: ログイン中の人（current_user）が持ち主であるノートだけを返す
fn find_note(notes: &[Note], current_user: u32, note_id: u32) -> Option<&Note> {
    notes.iter().find(|note| note.id == note_id)
}
```

**▼ 実行結果**（コンパイル時の警告と、実行結果）

```text
   Compiling practice v0.1.0 (C:\Users\you\rust-study\practice)
warning: unused variable: `current_user`
  --> src\main.rs:27:30
   |
27 | fn find_note(notes: &[Note], current_user: u32, note_id: u32) -> Option<&Note> {
   |                              ^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_current_user`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: field `user_id` is never read
  --> src\main.rs:12:5
   |
10 | struct Note {
   |        ---- field in this struct
11 |     id: u32,
12 |     user_id: u32,
   |     ^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `practice` (bin "practice") generated 2 warnings (run `cargo fix --bin "practice" -p practice` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target\debug\practice.exe`
[認証] 間違ったパスワード -> None
[認証] 正しいパスワード   -> Some(1)
[認可] ノート 10 -> 200 OK「牛乳を買う」
[認可] ノート 11 -> 200 OK「歯医者の予約」
```

**何が起きたか:**

- 実行結果の最後の行で、Hanako が **Taro のノート「歯医者の予約」を見られてしまいました**。URL の番号を書き換えるだけで他人のデータが見えてしまうこの種の不具合は **IDOR（アイドア）** と呼ばれ、実際の Web サービスでもよく起きる、深刻な問題です。
- コンパイルは成功しましたが、コンパイラは **warning（警告）** を2つ出しています。
  - `unused variable: current_user` …「引数 `current_user` を受け取ったのに、一度も使っていない」
  - `field user_id is never read` …「`Note` の `user_id` を、どこからも読んでいない」
- どちらも「持ち主を調べる処理が無い」ことを、遠回しに教えてくれています。

**直し方:** 条件に `&& note.user_id == current_user` を戻します。警告は「エラーではないから無視してよいもの」ではなく、**バグの手がかり**です。警告が出たら、必ず理由を確かめましょう。

> **ヒント:** `help: if this is intentional, prefix it with an underscore` は「わざと使わないのなら、名前の先頭に `_` を付けてください（そうすれば警告を消せます）」という助言です。今回は**わざとではない**ので、`_` を付けて黙らせるのは間違った直し方です。

#### 401・403・404 の使い分け

| 状況 | 返すもの | 理由 |
|---|---|---|
| ログインしていない人が `/notes` を開いた | ログイン画面へ移動させる（または 401） | まず認証が必要 |
| ログインしているが、権限のない操作をした | 403 Forbidden | 認可で断った |
| ログインしているが、**他人のノート**を開こうとした | **404 Not Found** | 「そのノートは存在する」ことすら教えないため |

他人のノートに 403 を返すと、「その番号のノートは存在する（が、あなたのではない）」と相手に分かってしまいます。404 なら、存在しないノートと区別がつきません。RustNote はこの理由で 404 を返します。

### RustNote ではここで使う

- 登録・ログイン・ログアウトは `GET|POST /signup`・`GET|POST /login`・`POST /logout` です。パスワードは **Argon2id** のハッシュにして `users.password_hash` に保存します。
- ログインに成功すると、推測できない長い乱数（**セッショントークン**）を作り、ブラウザには Cookie として渡し、データベースの `sessions` テーブルには**トークンのハッシュ**（`token_hash`）と有効期限を保存します。トークンそのものを保存しないのは、パスワードと同じく、DB が漏れても悪用されないようにするためです。
- Topcoat のセッション機能が使う Cookie には、`Secure`（https のときだけ送る）・`HttpOnly`（ページ内のプログラムから読めない）・`SameSite=Lax`（他のサイトからの送信に勝手に付かない）といった安全のための印が付きます（第8巻）。
- ノートを探す SQL には、必ず `WHERE user_id = ?`（ログイン中の人）を付けます。他人のノートは 404 です。

### 練習問題

**問題1:** 最小コードの `main` を書き換えて、Taro（`taro@example.com`・`battery-staple`）でログインし、ノート 10 と 11 を見ようとした結果を表示してください。

**問題2:** 次の a〜d は、認証と認可のどちらでしょうか。

- a. ログイン画面で、入力されたパスワードが正しいか確かめる
- b. 編集ボタンが押されたとき、そのノートの持ち主がログイン中の人か確かめる
- c. Cookie のセッショントークンから、ログイン中の人が誰かを調べる
- d. 管理者だけが、全ユーザーの一覧ページを開ける

**問題3:** パスワードを「そのまま」ではなく「ハッシュにして」保存する理由を、1〜2文で説明してください。

### 解答と解説

**問題1の解答:** `main` 関数の「1. 認証」から後ろを、次のように置き換えます（`struct` と関数、`users`・`notes` の用意は同じです）。

```rust
    // Taro（taro@example.com）でログインする
    let taro = login(&users, "taro@example.com", "battery-staple");
    println!("[認証] Taro -> {taro:?}");

    let Some(current_user) = taro else {
        println!("ログインできなかったので、ここで終わり");
        return;
    };
    for note_id in [10, 11] {
        match find_note(&notes, current_user, note_id) {
            Some(note) => println!("[認可] ノート {note_id} -> 200 OK「{}」", note.title),
            None => println!("[認可] ノート {note_id} -> 404 Not Found"),
        }
    }
}
```

**▼ 実行結果**

```text
[認証] Taro -> Some(2)
[認可] ノート 10 -> 404 Not Found
[認可] ノート 11 -> 200 OK「歯医者の予約」
```

Hanako のときとちょうど逆になりました。同じ `find_note` でも、ログイン中の人によって結果が変わるのが認可です。

**問題2の解答:** a は**認証**、b は**認可**、c は**認証**（セッションで「誰か」を確かめている）、d は**認可**（誰かは分かったうえで、してよいかを判断している）。

**問題3の解答:** データベースが盗まれたり、内部の人がのぞいたりしても、元のパスワードが分からないようにするためです。多くの人は同じパスワードを他のサービスでも使っているので、漏れると被害がそのサービスだけでは済みません。

### まとめ

- **認証**は「あなたは誰？」（ログイン）、**認可**は「それをしてよい？」（持ち主チェック）で、別々に毎回考えます。
- ログイン後はセッショントークンを Cookie で受け渡し、サーバーは毎回それで「誰か」を調べます。パスワードもトークンも、DB には**ハッシュ**で保存します。
- 他人のデータには 404 を返します。コンパイラの「使っていない」という警告は、持ち主チェック漏れの手がかりになります。

