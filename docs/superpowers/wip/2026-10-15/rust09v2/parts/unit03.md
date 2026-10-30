## UNIT 03: ドメインモデル・スキーマ・不変条件

### この単元のゴール

- RustNote が扱う「もの」（ユーザー・ノート・セッション）と、そのつながり（1人のユーザーが、ノートとセッションを何件でも持つ）を、ER 図で表せる
- ドメインの型（`UserId`・`Email`・`NoteTitle` など）と、表の列の対応を説明できる
- 「いつでも必ず成り立つ決まり（不変条件）」を書き出し、それを**型の検査・アプリの処理・DB の制約**の3か所のどこで守るかを、表にできる
- 型の検査と DB の制約の答えが食い違う入力を実験で見つけ、どちらの守りを何のために置くのかを説明できる

### たとえ話でつかむ

図書館の本の貸し出しを考えます。図書館には、守らなければならない決まりがいくつもあります。「貸出カードは1人1枚」「1人が借りられるのは 10 冊まで」「ほかの人の貸出記録は見せない」。

この決まりは、1か所だけで守っているわけではありません。

- **受付の人**は、申込書を受け取ったとき、名前の欄が空いていないか、電話番号の桁が正しいかを確かめます。書き方が正しくなければ、その場で書き直してもらいます。
- **貸出の係**は、「ほかの人の貸出記録を見せてください」と言われても、本人でなければ見せません。これは申込書の書き方の問題ではなく、**仕事の決まり**です。
- **記録の台帳**には、最初から「同じカード番号は2行書けない」欄が印刷されています。受付の人がうっかり見落としても、台帳そのものが二重の登録を受け付けません。

受付の人・貸出の係・台帳は、守っている決まりが少しずつ違い、少しずつ重なっています。どれか1つが見落としても、ほかが止めてくれます。

アプリも同じ3か所で決まりを守ります。

| 図書館 | RustNote |
|---|---|
| 守るべき決まり（カードは1人1枚） | **不変条件**（メールアドレスは1人1つ） |
| 受付の人（書き方を確かめる） | **型の検査**（`NoteTitle::parse` など。外から来た値を、検査に合格したときだけ型にする） |
| 貸出の係（仕事の決まり） | **アプリの処理**（SQL に必ず持ち主の条件を入れる、版の番号を比べる） |
| 印刷された台帳の欄 | **DB の制約**（`UNIQUE`・`CHECK`・外部キー） |
| 申込書・カード・貸出記録の関係 | **ドメインモデル**と **ER 図** |

> **用語:** ドメインモデル（domain model：アプリが扱う「もの」と、その性質とつながりを整理したもの。RustNote ならユーザー・ノート・セッション）

<!-- -->

> **用語:** スキーマ（schema：DB の表・列・型・制約の決まりの全体。RustNote では `migrations/` の SQL が作る）

<!-- -->

> **用語:** 不変条件（ふへんじょうけん。invariant：プログラムが動いている間、いつでも必ず成り立っていなければならない決まり。例:「ノートのタイトルは1〜80文字」「ノートには必ず持ち主がいる」）

<!-- -->

> **用語:** ER 図（イーアールず。Entity-Relationship diagram：表（もの）と、表どうしのつながりを描いた図）

### 図で理解する

RustNote の DB には、4つの表があります。次の図は、その ER 図です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 400" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-0301" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="40" width="230" height="146" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<rect x="21" y="41" width="228" height="30" rx="7" fill="#eff6ff"/>
<text x="135" y="61" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">users（ユーザー）</text>
<text x="34" y="94" font-size="12" fill="#0f172a">id（番号・主キー）</text>
<text x="34" y="118" font-size="12" fill="#0f172a">email（UNIQUE）</text>
<text x="34" y="142" font-size="12" fill="#0f172a">password_hash</text>
<text x="34" y="166" font-size="12" fill="#0f172a">created_at</text>
<rect x="440" y="20" width="260" height="214" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<rect x="441" y="21" width="258" height="30" rx="7" fill="#ecfdf5"/>
<text x="570" y="41" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">notes（ノート）</text>
<text x="454" y="74" font-size="12" fill="#0f172a">id（番号・主キー）</text>
<text x="454" y="97" font-size="12" fill="#c2410c">user_id → users.id</text>
<text x="454" y="120" font-size="12" fill="#0f172a">title（1〜80 文字）</text>
<text x="454" y="143" font-size="12" fill="#0f172a">body</text>
<text x="454" y="166" font-size="12" fill="#0f172a">done（0 か 1）</text>
<text x="454" y="189" font-size="12" fill="#0f172a">version（版の番号）</text>
<text x="454" y="212" font-size="12" fill="#0f172a">created_at・updated_at</text>
<rect x="440" y="250" width="260" height="140" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<rect x="441" y="251" width="258" height="30" rx="7" fill="#fff7ed"/>
<text x="570" y="271" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">sessions（ログイン中）</text>
<text x="454" y="304" font-size="12" fill="#0f172a">token_hash（主キー）</text>
<text x="454" y="327" font-size="12" fill="#c2410c">user_id → users.id</text>
<text x="454" y="350" font-size="12" fill="#0f172a">expires_at（UNIX 秒）</text>
<text x="454" y="373" font-size="12" fill="#0f172a">created_at</text>
<rect x="20" y="256" width="230" height="96" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2" stroke-dasharray="5 4"/>
<text x="135" y="280" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">schema_migrations</text>
<text x="34" y="306" font-size="12" fill="#0f172a">version（主キー）</text>
<text x="34" y="330" font-size="12" fill="#0f172a">applied_at</text>
<line x1="250" y1="110" x2="436" y2="94" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0301)"/>
<text x="262" y="100" font-size="12" font-weight="700" fill="#1e40af">1</text>
<text x="412" y="84" font-size="12" font-weight="700" fill="#047857">多</text>
<text x="343" y="122" font-size="11" fill="#475569" text-anchor="middle">1人が何件でも書く</text>
<path d="M250,160 H340 V327 H436" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0301)"/>
<text x="262" y="152" font-size="12" font-weight="700" fill="#1e40af">1</text>
<text x="412" y="319" font-size="12" font-weight="700" fill="#c2410c">多</text>
<text x="348" y="236" font-size="11" fill="#475569">1人が何か所でも</text>
<text x="348" y="252" font-size="11" fill="#475569">ログインできる</text>
<text x="135" y="374" font-size="11" fill="#475569" text-anchor="middle">どの表ともつながらない（適用の記録）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: RustNote の ER 図。ノートとセッションは user_id で持ち主のユーザーを指す。ユーザーの行が消えると、指していたノートとセッションも一緒に消える（ON DELETE CASCADE）</div>
</div>

- **users** が中心です。ノート（**notes**）もセッション（**sessions**）も、`user_id` の列で「誰のものか」を指しています。矢印の根元の「1」と先の「多」は、「1人のユーザーに、ノートが何件でもつながる」という意味です。0件でもかまいません。
- `user_id` は**外部キー**（`REFERENCES users(id)`）です。存在しないユーザーを指すノートは作れません。さらに `ON DELETE CASCADE` が付いているので、ユーザーの行が消えると、そのユーザーのノートとセッションも自動で消えます（第7巻 UNIT 01）。
- **schema_migrations** は、どのマイグレーションを適用したかの記録で、ほかの表とはつながっていません。アプリが起動するときに、コード（`src/db.rs` の `migrate`）が作ります。

> **用語:** 外部キー（がいぶきー。foreign key：ほかの表の行を指す列。指している先が存在しない値は入れられない）

決まりを守る3か所は、次の図のように並んでいます。外から来た値は、左から順に関所を通ってから、DB に保存されます。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-0302" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="40" width="112" height="60" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="66" y="66" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">外から来た値</text>
<text x="66" y="86" font-size="11" fill="#475569" text-anchor="middle">フォーム・URL</text>
<rect x="150" y="40" width="170" height="60" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="235" y="66" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">① 型の検査</text>
<text x="235" y="86" font-size="11" fill="#475569" text-anchor="middle">NoteTitle::parse など</text>
<rect x="350" y="40" width="170" height="60" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="435" y="66" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">② アプリの処理</text>
<text x="435" y="86" font-size="11" fill="#475569" text-anchor="middle">routes/・repository/</text>
<rect x="550" y="40" width="160" height="60" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="630" y="66" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">③ DB の制約</text>
<text x="630" y="86" font-size="11" fill="#475569" text-anchor="middle">migrations/*.sql</text>
<line x1="122" y1="70" x2="146" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0302)"/>
<line x1="320" y1="70" x2="346" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0302)"/>
<line x1="520" y1="70" x2="546" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0302)"/>
<text x="235" y="128" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">止めるもの</text>
<text x="235" y="150" font-size="12" fill="#0f172a" text-anchor="middle">空・81 文字のタイトル</text>
<text x="235" y="172" font-size="12" fill="#0f172a" text-anchor="middle">@ の無いメール</text>
<text x="235" y="194" font-size="12" fill="#0f172a" text-anchor="middle">0・abc のような ID</text>
<text x="435" y="128" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">止めるもの</text>
<text x="435" y="150" font-size="12" fill="#0f172a" text-anchor="middle">他人のノート → 404</text>
<text x="435" y="172" font-size="12" fill="#0f172a" text-anchor="middle">古い版の保存 → 409</text>
<text x="435" y="194" font-size="12" fill="#0f172a" text-anchor="middle">確認用パスワードの違い</text>
<text x="630" y="128" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">止めるもの</text>
<text x="630" y="150" font-size="12" fill="#0f172a" text-anchor="middle">同じメール（UNIQUE）</text>
<text x="630" y="172" font-size="12" fill="#0f172a" text-anchor="middle">持ち主なし（外部キー）</text>
<text x="630" y="194" font-size="12" fill="#0f172a" text-anchor="middle">done に 2（CHECK）</text>
<rect x="350" y="242" width="170" height="44" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2" stroke-dasharray="5 4"/>
<text x="435" y="261" font-size="12" fill="#0f172a" text-anchor="middle">見本のデータ・</text>
<text x="435" y="278" font-size="12" fill="#0f172a" text-anchor="middle">手で打った SQL</text>
<path d="M520,264 H630 V206" fill="none" stroke="#f97316" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#ar-09-0302)"/>
<text x="600" y="306" font-size="12" fill="#c2410c" text-anchor="middle">型を通らない書き込みも、③ は止める</text>
<text x="190" y="262" font-size="12" fill="#475569" text-anchor="middle">① は早く・やさしく断る</text>
<text x="190" y="282" font-size="12" fill="#475569" text-anchor="middle">③ は最後の砦</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: 不変条件を守る3か所。型の検査が入口で早く断り、アプリの処理が仕事の決まりを守り、DB の制約が最後に止める。型を通らない書き込みも、DB の制約は止める</div>
</div>

- **① 型の検査**は、入口でいちばん早く断ります。「タイトルを入力してください」のような、利用者に分かる言葉でエラーを返せるのも、ここだけです。
- **② アプリの処理**は、値の形だけでは決まらない決まりを守ります。「このノートの持ち主は、今ログインしている人か」は、タイトルの文字数を調べても分かりません。
- **③ DB の制約**は、最後の砦です。テスト用の見本のデータを入れるプログラムや、手で打った SQL のように、①と②を通らない書き込みがあっても、決まりを破る行は保存させません。
- 同じ決まりを2か所で守ることもあります（タイトルの長さは①と③）。それぞれの役目が違うからです。答えが食い違わないように、決まりの中身をそろえておくことが大切です（例3-3で確かめます）。

### まず動かす最小コード

まず、ドメインの型と、表の列の対応を表にします。

| ドメインの型（`src/domain.rs`） | 中身の決まり | 表と列 | DB の型と制約 |
|---|---|---|---|
| `UserId` | `i64` で 1 以上 | `users.id`・`notes.user_id`・`sessions.user_id` | `INTEGER PRIMARY KEY AUTOINCREMENT` ／ `REFERENCES users(id) ON DELETE CASCADE` |
| `Email` | 前後の空白を除いて小文字にそろえた `String`。`@` の前後が空でなく、ドメインに `.` がある | `users.email` | `TEXT NOT NULL UNIQUE` |
| （パスワードのハッシュ） | Argon2id の文字列（`$argon2id$v=19$...`） | `users.password_hash` | `TEXT NOT NULL` |
| `NoteId` | `i64` で 1 以上 | `notes.id` | `INTEGER PRIMARY KEY AUTOINCREMENT` |
| `NoteTitle` | 前後の空白を除いて 1〜80 文字 | `notes.title` | `TEXT NOT NULL CHECK (length(title) BETWEEN 1 AND 80)` |
| `NoteBody` | 改行を `\n` にそろえた 10,000 文字までの文字列（空でもよい） | `notes.body` | `TEXT NOT NULL DEFAULT ''` |
| `bool`（`Note.done`） | 完了か未完了か | `notes.done` | `INTEGER NOT NULL DEFAULT 0 CHECK (done IN (0, 1))` |
| `i64`（`Note.version`） | 1 から始まり、更新のたびに 1 増える | `notes.version` | `INTEGER NOT NULL DEFAULT 1` |
| `String`（日時） | UTC の `YYYY-MM-DD HH:MM:SS` | `created_at`・`updated_at` | `TEXT NOT NULL DEFAULT (datetime('now'))` |
| `TokenHash`（Topcoat） | トークンの SHA-256（32 バイト） | `sessions.token_hash` | `BLOB PRIMARY KEY` |
| `SystemTime` | セッションの期限 | `sessions.expires_at` | `INTEGER NOT NULL`（1970-01-01 からの秒数） |

- Rust の `bool` は、SQLite には無い型です。そこで DB では `0` と `1` の整数にして、`CHECK (done IN (0, 1))` で、ほかの数を入れられないようにしています。
- セッションの期限は、日時の文字列ではなく「1970-01-01 からの秒数」（UNIX 時刻）の整数にしています。「今より前か後か」を、数の大小で簡単に比べられるからです（`src/repository.rs` の `unix_seconds`）。

次に、不変条件を書き出し、3か所のどこで守るかを表にします。この表が、この単元の「設計書」です。

| 番号 | 不変条件 | ① 型の検査 | ② アプリの処理 | ③ DB の制約 |
|---|---|---|---|---|
| I-1 | メールアドレスは1人1つ（大文字と小文字の違いは同じとみなす） | `Email::parse` で小文字にそろえる | 二重の登録は 409 にする | `UNIQUE` |
| I-2 | タイトルは、前後の空白を除いて 1〜80 文字 | `NoteTitle::parse` | 400 でフォームを出し直す | `CHECK (length(title) BETWEEN 1 AND 80)` |
| I-3 | 本文は 10,000 文字まで | `NoteBody::parse` | 400 でフォームを出し直す | （なし） |
| I-4 | ノートには、必ず実在する持ち主がいる | `UserId` | ログイン中の人の ID だけを渡す | `NOT NULL` と外部キー |
| I-5 | 他人のノートは、見ることも変えることもできない | （なし） | すべての SQL に `user_id = ?` を入れる | （なし） |
| I-6 | 完了の状態は「完了」と「未完了」の2つだけ | `bool` | （なし） | `CHECK (done IN (0, 1))` |
| I-7 | 古い画面からの保存で、新しい内容を上書きしない | （なし） | `version` が一致したときだけ更新し、違えば 409 | （なし） |
| I-8 | ユーザーの行が消えたら、そのノートとセッションも残らない | （なし） | （なし） | `ON DELETE CASCADE` |
| I-9 | ID は 1 以上 | `UserId::new`・`NoteId::new` | URL の `{id}` が 0・負の数・数字以外なら 404 | `AUTOINCREMENT`（1 から振る） |

- 表を作ると、守っている場所が**1か所しか無い**決まりが見えてきます。I-5（他人のノート）と I-7（古い版）は、②アプリの処理だけが頼りです。DB の制約では、「今ログインしているのは誰か」も「利用者がどの版の画面を見ていたか」も分からないからです。だから、この2つは必ずテストで確かめます（UNIT 01 の AC-05、第7巻 UNIT 05）。
- I-8 について: RustNote には、退会の画面はありません（UNIT 01 のスコープの外）。それでも、運用する人がユーザーの行を消したときに、持ち主のいないノートが残らないように、DB の側で決めてあります。

では、①型の検査と③DB の制約を、実際に動かして確かめます。

#### 例3-1: ドメインの型に、いろいろな入力を渡す

> **▼ このコードがやること（先に日本語で）:** RustNote の `src/domain.rs` にある型（`NoteTitle`・`Email`・`NoteId`・`Password`）の検査の関数に、正しい入力と、決まりを破る入力を渡します。合格したら `OK` と、型の中に入った値を、不合格なら `NG` と、エラーの文を表示します。

```rust
//! RustNote のドメインの型に、いろいろな入力を渡して、合格・不合格を確かめる。
//! 実行: cargo run -q --example u03_domain

use rustnote::domain::{Email, NoteId, NoteTitle, Password};

// 長い文字列は、先頭の 5 文字と文字数だけを表示する
fn short(text: &str) -> String {
    let count = text.chars().count();
    if count <= 10 {
        format!("{text:?}")
    } else {
        let head: String = text.chars().take(5).collect();
        format!("\"{head}…\"（{count} 文字）")
    }
}

fn main() {
    println!("--- NoteTitle::parse（前後の空白を除いて 1〜80 文字）");
    let eighty = "あ".repeat(80);
    let eighty_one = "あ".repeat(81);
    let titles = [
        ("牛乳を買う", "牛乳を買う"),
        ("前後に空白", "  買い物  "),
        ("空", ""),
        ("空白3つ", "   "),
        ("「あ」80個", eighty.as_str()),
        ("「あ」81個", eighty_one.as_str()),
    ];
    for (label, input) in titles {
        match NoteTitle::parse(input) {
            Ok(title) => println!("{label}: OK {}", short(title.as_str())),
            Err(error) => println!("{label}: NG {error}"),
        }
    }

    println!("--- Email::parse（小文字にそろえる）");
    for input in ["  Alice@Example.COM ", "alice", "alice@example"] {
        match Email::parse(input) {
            Ok(email) => println!("{input:?}: OK {:?}", email.as_str()),
            Err(error) => println!("{input:?}: NG {error}"),
        }
    }

    println!("--- NoteId（URL の /notes/{{id}} の部分）");
    for input in ["42", "0", "abc"] {
        match input.parse::<NoteId>() {
            Ok(id) => println!("{input:?}: OK {}", id.get()),
            Err(error) => println!("{input:?}: NG {error}"),
        }
    }

    println!("--- Password::parse（8〜128 文字。{{:?}} でも中身は出ない）");
    for input in ["short", "correct horse battery"] {
        match Password::parse(input) {
            Ok(password) => println!("{input:?}: OK {password:?}"),
            Err(error) => println!("{input:?}: NG {error}"),
        }
    }
}
```

コピーした `rustnote` フォルダの `examples/u03_domain.rs` として保存して実行します（`examples` フォルダが無ければ作ります）。

```powershell
cargo run -q --example u03_domain
```

**▼ 実行結果**

```text
--- NoteTitle::parse（前後の空白を除いて 1〜80 文字）
牛乳を買う: OK "牛乳を買う"
前後に空白: OK "買い物"
空: NG タイトルを入力してください
空白3つ: NG タイトルを入力してください
「あ」80個: OK "あああああ…"（80 文字）
「あ」81個: NG タイトルは80文字以内にしてください（現在81文字）
--- Email::parse（小文字にそろえる）
"  Alice@Example.COM ": OK "alice@example.com"
"alice": NG メールアドレスに @ がありません
"alice@example": NG メールアドレスの形式が正しくありません
--- NoteId（URL の /notes/{id} の部分）
"42": OK 42
"0": NG ID は 1 以上の数です
"abc": NG ID は数字で指定してください
--- Password::parse（8〜128 文字。{:?} でも中身は出ない）
"short": NG パスワードは8文字以上にしてください
"correct horse battery": OK Password(***)
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 合格した値は、きれいな形に直されて型に入る

- `"  買い物  "` は、前後の空白が取られて `"買い物"` として `NoteTitle` に入りました。`"  Alice@Example.COM "` も、空白が取られ、小文字にそろえられて `"alice@example.com"` になりました。
- つまり型の検査は、合格・不合格を決めるだけでなく、値を**決まった形にそろえる**役目も持っています。そろえてから保存するので、DB には `"alice@example.com"` の形しか入りません。この「そろえる」が、I-1（メールは1人1つ）を守る鍵になります（練習問題の問2で確かめます）。
- `"   "`（空白3つ）は、空白を取ると空になるので、空のタイトルと同じ「タイトルを入力してください」で断られます。第0巻（学習ガイド）で「空白だけのタイトルを認めるかどうかは、アプリの決めごと」と書きました。RustNote は「認めない」と決めて、`NoteTitle::parse` でそう作っています。

##### 解説2: 文字数は「文字」で数える

- 「あ」を 80 個並べたタイトルは合格、81 個は不合格です。「あ」は UTF-8 で 3 バイトなので、80 個は 240 バイトですが、`NoteTitle::parse` は `chars().count()` で**文字の数**を数えています（第2巻 UNIT 05）。
- DB の `length(title)` も、文字列の**文字の数**を数えます。Rust 側と DB 側で数え方をそろえてあるので、「Rust では 80 文字なのに、DB では長すぎる」という食い違いが起きません（エラー3で、そろっていない例を見ます）。

##### 解説3: エラーの文は、そのまま画面に出せる言葉

- `NG` の後ろの文（「メールアドレスに @ がありません」など）は、`src/domain.rs` の `TitleError`・`EmailError`・`IdError` に、`thiserror` の `#[error("...")]` で書いてある文です（第4巻 UNIT 05）。`{error}` で表示すると、この文が出ます。
- 型の検査のエラーの文は、利用者が読む言葉で書いてあります。RustNote は、この文をそのままフォームの項目の下に出しています。図の「① は早く・やさしく断る」の「やさしく」は、このことです。

##### 解説4: パスワードは、`{:?}` で表示しても中身が出ない

- `Password` を `{password:?}` で表示すると、`Password(***)` と出ました。`src/domain.rs` では、`Password` の `Debug` を自動で作らずに、手で書いて `***` を表示するようにしています。ログに設定や入力を `{:?}` で書き出したとき、パスワードが漏れないようにするためです（第4巻 UNIT 09 の `Secret` と同じ考え方です）。
- これも「パスワードはログに出さない」という不変条件を、**型で**守っている例です。

#### 例3-2: DB の制約に、決まりを破るデータを入れてみる

> **▼ このコードがやること（先に日本語で）:** 一時フォルダに DB を作り、RustNote の起動時と同じ関数（`db::migrate`）で、3つのマイグレーションを適用します。そこに、型の検査を通さずに、SQL で直接「決まりを破るデータ」を入れて、DB がどう断るかを1行ずつ表示します。最後に、ユーザーを消すと、そのノートとセッションも消えることを確かめます。

```rust
//! RustNote と同じマイグレーションで表を作り、わざと決まりを破るデータを入れて、
//! DB がどう断るかを確かめる。
//! 実行: cargo run -q --example u03_constraints

use libsql::Connection;
use rustnote::db::{self, Db};

// SQL を1つ実行し、成功か失敗かを1行で表示する
async fn try_sql(conn: &Connection, label: &str, sql: &str) {
    match conn.execute(sql, ()).await {
        Ok(rows) => println!("OK  {label}（{rows} 行）"),
        Err(error) => println!("NG  {label} → {error}"),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 使い捨ての一時フォルダに DB を作る（プログラムが終わるとフォルダごと消える）
    let dir = tempfile::tempdir()?;
    let db = Db::open_local(dir.path().join("u03.db")).await?;
    // RustNote の起動時と同じ関数で、3つのマイグレーションを適用する
    let applied = db::migrate(&db).await?;
    println!("適用したマイグレーション: {applied:?}");

    // RustNote と同じ接続の作り方（外部キーの確認を ON にした接続）
    let conn = db.connect().await?;
    try_sql(
        &conn,
        "alice を登録",
        "INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'x')",
    )
    .await;
    try_sql(
        &conn,
        "同じメールでもう一度登録",
        "INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'y')",
    )
    .await;
    try_sql(
        &conn,
        "空のタイトル",
        "INSERT INTO notes (user_id, title) VALUES (1, '')",
    )
    .await;
    try_sql(
        &conn,
        "done に 2 を入れる",
        "INSERT INTO notes (user_id, title, done) VALUES (1, '宿題', 2)",
    )
    .await;
    try_sql(
        &conn,
        "いないユーザー 999 のノート",
        "INSERT INTO notes (user_id, title) VALUES (999, '宿題')",
    )
    .await;
    try_sql(
        &conn,
        "alice のノート",
        "INSERT INTO notes (user_id, title) VALUES (1, '宿題')",
    )
    .await;
    try_sql(
        &conn,
        "alice のセッション",
        "INSERT INTO sessions (token_hash, user_id, expires_at) VALUES (x'00', 1, 0)",
    )
    .await;

    // ユーザーを消すと、そのユーザーのノートとセッションも一緒に消える（ON DELETE CASCADE）
    try_sql(&conn, "alice を削除", "DELETE FROM users WHERE id = 1").await;
    for table in ["notes", "sessions"] {
        let sql = format!("SELECT COUNT(*) FROM {table}");
        let mut rows = conn.query(&sql, ()).await?;
        if let Some(row) = rows.next().await? {
            println!("残っている {table}: {} 行", row.get::<i64>(0)?);
        }
    }
    Ok(())
}
```

`examples/u03_constraints.rs` として保存して実行します。

```powershell
cargo run -q --example u03_constraints
```

**▼ 実行結果**

```text
適用したマイグレーション: ["0001_users", "0002_notes", "0003_sessions"]
OK  alice を登録（1 行）
NG  同じメールでもう一度登録 → SQLite failure: `UNIQUE constraint failed: users.email`
NG  空のタイトル → SQLite failure: `CHECK constraint failed: length(title) BETWEEN 1 AND 80`
NG  done に 2 を入れる → SQLite failure: `CHECK constraint failed: done IN (0, 1)`
NG  いないユーザー 999 のノート → SQLite failure: `FOREIGN KEY constraint failed`
OK  alice のノート（1 行）
OK  alice のセッション（1 行）
OK  alice を削除（1 行）
残っている notes: 0 行
残っている sessions: 0 行
```

決まりを破る4つの `INSERT` は、すべて DB が断りました。alice を消すと、alice のノートとセッションも消えています。

#### ▼ コードを1つずつ分解して解説

##### 解説1: RustNote と同じ「表の作り方」と「接続の作り方」を使う

```rust
    let db = Db::open_local(dir.path().join("u03.db")).await?;
    // RustNote の起動時と同じ関数で、3つのマイグレーションを適用する
    let applied = db::migrate(&db).await?;
```

- `Db::open_local` と `db::migrate` は、RustNote の `src/db.rs` にある関数です。`migrate` は、`migrations/` の3つの SQL を、まだ適用していないものだけ、1つずつ transaction の中で適用し、適用した版の名前を返します。表示された `["0001_users", "0002_notes", "0003_sessions"]` がそれです。
- 練習用に表を自分で作り直すと、本物の表との食い違いに気づけません。**本物と同じ SQL で作った表**で確かめるのが大事です。
- `tempfile::tempdir()` は、使い捨ての一時フォルダを作ります。`dir` が片付けられるとき（`main` の終わり）に、フォルダごと消えます。RustNote のテストと同じやり方です。

##### 解説2: DB のエラーの文は、どの制約に引っかかったかを教えてくれる

| エラーの文 | 意味 | 不変条件 |
|---|---|---|
| `UNIQUE constraint failed: users.email` | `users` 表の `email` 列に、同じ値が既にある | I-1 |
| `CHECK constraint failed: length(title) BETWEEN 1 AND 80` | `CHECK` の条件（タイトルが1〜80文字）を満たさない | I-2 |
| `CHECK constraint failed: done IN (0, 1)` | `done` が 0 でも 1 でもない | I-6 |
| `FOREIGN KEY constraint failed` | 外部キーの指す先（`users` の 999 番）が無い | I-4 |

- `UNIQUE` と `CHECK` のエラーは、どの列・どの条件かまで教えてくれます。外部キーのエラーは、どの列かを教えてくれないので、表に外部キーが複数あるときは、入れた値を1つずつ確かめます。
- RustNote は、このうち `UNIQUE constraint failed` という文を手がかりに、「登録済みのメールアドレス」というエラーに分類しています（「RustNote ではここで使う」で見ます）。

##### 解説3: 外部キーの確認は、接続ごとに ON にしている

```rust
    // RustNote と同じ接続の作り方（外部キーの確認を ON にした接続）
    let conn = db.connect().await?;
```

- RustNote の `Db::connect` は、ローカルのファイルの DB に接続するたびに、`PRAGMA foreign_keys = ON;` を実行しています。SQLite 本来の既定では外部キーの確認が OFF で、接続ごとに ON にしないと、存在しないユーザーのノートが入ってしまうからです（第7巻 UNIT 01）。
- 本書で使う libsql 0.10 のローカルの DB では、何もしなくても ON でした（第7巻 UNIT 01 で確かめました）。それでも、既定の値に頼らずに明示しておくのが、RustNote の決めごとです。

##### 解説4: ON DELETE CASCADE で、持ち主のいない行が残らない

- `DELETE FROM users WHERE id = 1` は1行（alice）を消しただけですが、残っているノートとセッションは 0 行になりました。`notes.user_id` と `sessions.user_id` の外部キーに付けた `ON DELETE CASCADE` が、alice を指していた行を一緒に消したのです。
- もし `CASCADE` が無ければ、alice のノートが残っている間は、外部キーの決まりにより、alice を消すことができません。外部キーに `ON DELETE` を付けない表で試すと、子の行が残っている親の行の `DELETE` は `FOREIGN KEY constraint failed` で断られました。「ユーザーを消したら、ノートをどうするか」は、表を作るときに決めておくことです。

### もう一歩: 実務に近い形

型の検査と DB の制約は、同じ決まり（I-2: タイトルは1〜80文字）を2か所で守っています。では、2つの答えは**いつでも同じ**でしょうか。実験で確かめます。

#### 例3-3: 型の検査と DB の制約に、同じ入力を渡して比べる

> **▼ このコードがやること（先に日本語で）:** 6つのタイトルの候補を、`NoteTitle::parse`（型の検査）と、`notes` 表への直接の `INSERT`（DB の制約）の両方に渡します。それぞれの答え（OK か NG か）を並べて表示し、2つの答えが食い違ったときは「答えが食い違う」と表示します。

```rust
//! 同じタイトルを「型の検査（NoteTitle::parse）」と「DB の制約（CHECK）」の両方に渡し、
//! 2つの守りの答えが食い違う入力を探す。
//! 実行: cargo run -q --example u03_layers

use rustnote::db::{self, Db};
use rustnote::domain::NoteTitle;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let db = Db::open_local(dir.path().join("u03.db")).await?;
    db::migrate(&db).await?;
    let conn = db.connect().await?;
    conn.execute(
        "INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'x')",
        (),
    )
    .await?;

    let eighty_one = "あ".repeat(81);
    let inputs = [
        ("牛乳を買う", "牛乳を買う"),
        ("空", ""),
        ("空白3つ", "   "),
        ("全角の空白1つ", "\u{3000}"),
        ("ゼロ幅スペース1つ", "\u{200B}"),
        ("「あ」81個", eighty_one.as_str()),
    ];
    for (label, input) in inputs {
        // 型の検査
        let by_type = match NoteTitle::parse(input) {
            Ok(_) => "OK".to_string(),
            Err(error) => format!("NG: {error}"),
        };
        // DB の制約（検査を通さず、入力をそのまま INSERT する）
        let by_db = match conn
            .execute("INSERT INTO notes (user_id, title) VALUES (1, ?1)", [input])
            .await
        {
            Ok(_) => "OK".to_string(),
            Err(error) => format!("NG: {error}"),
        };
        let same = by_type.starts_with("OK") == by_db.starts_with("OK");
        println!("[{label}]");
        println!("    型: {by_type}");
        println!("    DB: {by_db}");
        if !same {
            println!("    → 答えが食い違う");
        }
    }
    Ok(())
}
```

`examples/u03_layers.rs` として保存して実行します。

```powershell
cargo run -q --example u03_layers
```

**▼ 実行結果**

```text
[牛乳を買う]
    型: OK
    DB: OK
[空]
    型: NG: タイトルを入力してください
    DB: NG: SQLite failure: `CHECK constraint failed: length(title) BETWEEN 1 AND 80`
[空白3つ]
    型: NG: タイトルを入力してください
    DB: OK
    → 答えが食い違う
[全角の空白1つ]
    型: NG: タイトルを入力してください
    DB: OK
    → 答えが食い違う
[ゼロ幅スペース1つ]
    型: OK
    DB: OK
[「あ」81個]
    型: NG: タイトルは80文字以内にしてください（現在81文字）
    DB: NG: SQLite failure: `CHECK constraint failed: length(title) BETWEEN 1 AND 80`
```

6つのうち2つで、型と DB の答えが食い違いました。また、食い違ってはいないものの、気になる結果が1つあります。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 空白だけのタイトルは、DB の制約では止まらない

- `"   "`（半角の空白3つ）と `"\u{3000}"`（全角の空白1つ）は、型の検査では NG、DB では OK でした。DB の `length(title)` は、空白も1文字として数えるので、「3文字」「1文字」として 1〜80 文字の条件を満たしてしまうのです。
- 型の検査の `trim()` は、半角の空白だけでなく、全角の空白（U+3000）も「空白」として取り除きます。Rust の `trim` は、Unicode で空白と決められている文字をまとめて扱うからです。
- では、空白だけのタイトルが DB に入ってしまうのでしょうか。RustNote のアプリを通る限り、入りません。保存するのは、型の検査に合格して、前後の空白を取った後の `NoteTitle` だけだからです。DB の制約は、アプリを通らない書き込み（図の破線の矢印）に対する、最後の守りです。
- もし「空白だけのタイトル」を DB でも止めたいなら、`CHECK (length(trim(title)) BETWEEN 1 AND 80)` のように書く方法があります。ただし、SQLite の `trim` は、何も指定しなければ半角の空白だけを取り除きます。この PC の libsql 0.10 で試すと、`length(trim('   '))`（半角の空白3つ）は 0 でしたが、`length(trim('　'))`（全角の空白1つ）は 1 のままでした。これでは、全角の空白だけのタイトルは止められません。**決まりの中身を、本当に2か所でそろえるのは難しい**、ということも覚えておきましょう。だからこそ、利用者が通る入口（①）を正とし、DB（③）は「明らかにおかしいもの」を止める役にしています。

##### 解説2: 目に見えない文字は、どちらの守りも通り抜ける

- `"\u{200B}"`（ゼロ幅スペース）は、画面に何も表示されない文字です。型の検査も DB も OK でした。Rust の `trim()` は、ゼロ幅スペースを空白として扱わないので、取り除きません。DB の `length` では1文字です。
- その結果、RustNote では、画面上は**タイトルが空に見えるノート**を作れてしまいます。これは今の RustNote に残っている穴です。
- 防ぐかどうかは、要件として決めることです。たとえば「タイトルには、目に見える文字が1つ以上ないといけない」という不変条件を足し、`NoteTitle::parse` で目に見えない文字を除いてから数える、という作り方が考えられます。このように、不変条件の表と実験を突き合わせると、受入条件（UNIT 01）に書き足すべきことが見つかります。

##### 解説3: bind（`?1`）で値を渡す

```rust
            .execute("INSERT INTO notes (user_id, title) VALUES (1, ?1)", [input])
```

- タイトルは、SQL の文字列に `format!` で埋め込まずに、`?1` の場所へ **bind**（値を別に渡す）しています。入力に `'` が含まれていても、SQL の文の一部として読まれることはありません（第7巻 UNIT 07）。
- 例3-2で `format!` を使っているのは、表の名前（`notes`・`sessions`）のように、プログラムの中で決めた値だけです。利用者の入力を `format!` で SQL に入れてはいけません。

### よくあるエラーと読み方

#### エラー1: 検査を通さずに、型を直接作ろうとした（E0423）

`NoteTitle::parse` を通さずに、空のタイトルを持つ `NoteTitle` を直接作ろうとした場合です（`examples/e03_private.rs` に保存して試します）。

```rust
use rustnote::domain::NoteTitle;

fn main() {
    // 検査（parse）を通さずに、空のタイトルを直接作ろうとした
    let title = NoteTitle(String::new());
    println!("{title}");
}
```

```powershell
cargo build --example e03_private
```

```text
   Compiling rustnote v0.1.0 (...\rustnote)
error[E0423]: cannot initialize a tuple struct which contains private fields
   --> examples\e03_private.rs:5:17
    |
  5 |     let title = NoteTitle(String::new());
    |                 ^^^^^^^^^
    |
    = note: a struct named `NoteTitle` exists in another namespace
note: constructor is not visible here due to private fields
   --> src\domain.rs:221:22
    |
221 | pub struct NoteTitle(String);
    |                      ^^^^^^ private field

For more information about this error, try `rustc --explain E0423`.
error: could not compile `rustnote` (example "e03_private") due to 1 previous error
```

- **何が起きたか:** 「外から見えない（private な）項目を持つタプル構造体は、作れません」というエラーです。`note:` が `src/domain.rs` の 221 行目 `pub struct NoteTitle(String);` を指し、中の `String` に `private field` と印を付けています。型そのものは `pub`（公開）ですが、中身の `String` には `pub` が付いていないので、`domain` モジュールの外からは中身を指定して作れません。
- **これはエラーで正しい:** このエラーこそが、不変条件を守る仕組みです。`domain` の外で `NoteTitle` を手に入れる方法は `NoteTitle::parse` だけなので、「`NoteTitle` の値がある」ことが「検査に合格した」ことを意味します（第4巻 UNIT 01）。
- **直し方:** `NoteTitle::parse("...")` で作り、`Result` を受け取ります。確かめ終わったら、`examples/e03_private.rs` は消しておきます。

#### エラー2: include_str! のパスの書き間違い

RustNote は、マイグレーションの SQL を `include_str!` でプログラムの中に埋め込んでいます（`src/db.rs`）。同じことを `examples/` のプログラムでしようとして、場所の書き方を間違えた場合です（`examples/e03_path.rs`）。

```rust
// RustNote の migrations/ にある SQL を読み込むつもりで、場所の書き方を間違えた
const USERS_SQL: &str = include_str!("migrations/0001_users.sql");

fn main() {
    println!("{} 文字", USERS_SQL.chars().count());
}
```

```powershell
cargo build --example e03_path
```

```text
   Compiling rustnote v0.1.0 (...\rustnote)
error: couldn't read `examples\migrations/0001_users.sql`: 指定されたパスが見つかりません。 (os error 3)
 --> examples\e03_path.rs:2:25
  |
2 | const USERS_SQL: &str = include_str!("migrations/0001_users.sql");
  |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: there is a file with the same name in a different directory
  |
2 | const USERS_SQL: &str = include_str!("../migrations/0001_users.sql");
  |                                       +++
```

- **何が起きたか:** `include_str!` のパスは、プロジェクトのフォルダからではなく、**`include_str!` を書いたファイルがあるフォルダ**から数えます。`examples/e03_path.rs` から見た `migrations/0001_users.sql` は `examples\migrations/0001_users.sql` なので、そんなファイルはありません。
- **直し方:** `help:` が「同じ名前のファイルが別の場所にあります」と教えてくれています。1つ上のフォルダを表す `../` を足して、`"../migrations/0001_users.sql"` にします。`src/db.rs` も、`src/` から1つ上がって `include_str!("../migrations/0001_users.sql")` と書いています。確かめ終わったら、`examples/e03_path.rs` は消しておきます。
- このエラーは**コンパイルのとき**に出ます。`include_str!` はファイルの中身をプログラムに埋め込むので、できあがった実行ファイルは、`migrations/` フォルダが無い場所でも動きます。本番のサーバーに SQL のファイルを置き忘れる心配がありません。

#### エラー3（エラーが出ない間違い）: 文字数を len() で数える

タイトルの長さを、`chars().count()` ではなく `len()` で数えてしまった場合です。コンパイルは通り、動きもします。

```rust
//! 文字数を len()（バイト数）で数えてしまう間違いを、chars().count() と比べる。
//! 実行: cargo run -q --example u03_len_bug

// 間違い: len() はバイト数を返す
fn is_valid_by_bytes(title: &str) -> bool {
    let len = title.trim().len();
    (1..=80).contains(&len)
}

// 正しい: chars().count() は文字数を返す（SQLite の length() と同じ数え方）
fn is_valid_by_chars(title: &str) -> bool {
    let len = title.trim().chars().count();
    (1..=80).contains(&len)
}

fn main() {
    let title = "あ".repeat(27);
    println!("「あ」27 個のタイトル");
    println!(
        "    len()           = {} → {}",
        title.len(),
        is_valid_by_bytes(&title)
    );
    println!(
        "    chars().count() = {} → {}",
        title.chars().count(),
        is_valid_by_chars(&title)
    );
}
```

```powershell
cargo run -q --example u03_len_bug
```

```text
「あ」27 個のタイトル
    len()           = 81 → false
    chars().count() = 27 → true
```

- **何が起きたか:** 27 文字のタイトルが、`len()` では 81（バイト）と数えられ、「長すぎる」と断られました。英数字だけで試していると、1文字が1バイトなので、この間違いに気づきません。
- **食い違いの向き:** DB の `length(title)` は文字数（27）なので OK です。つまり「アプリは断るのに、DB は受け付ける」という、例3-3と逆向きの食い違いになります。日本語で 27 文字を超えるタイトルが、全部書けなくなります。
- **見落とさない工夫:** 境界の値のテストは、**日本語で**書きます。RustNote のユニットテスト `note_title_counts_characters_not_bytes` は、「あ」を 80 個と 81 個並べて確かめています。UNIT 01 の受入条件 AC-04 にも「80 文字（日本語でも）」と書いたのは、このためです。

### RustNote ではここで使う

`src/domain.rs` の先頭には、この単元の考え方が、そのままコメントで書いてあります。

```rust
//! ドメイン（アプリが扱う「もの」）の型。
//!
//! 外から来た文字列（フォーム・URL・DB の値）は、ここの `parse` / `new` を通して
//! 検査済みの型に変える。中身は private なので、検査を通らずに作る方法はない。
//! 「`NoteTitle` 型の値がある」＝「1〜80 文字の検査に合格した」が必ず成り立つ。
```

保存するときと、読み出すときで、使う型が違うことにも注目してください（`src/domain.rs` から抜き出しています）。

```rust
/// DB から読んだノート
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    pub id: NoteId,
    /// 持ち主（notes.user_id）
    pub owner: UserId,
    pub title: String,
    pub body: String,
    pub done: bool,
    /// 楽観的ロック用の番号。更新のたびに 1 増える
    pub version: i64,
    /// 作成日時（UTC、'YYYY-MM-DD HH:MM:SS'）
    pub created_at: String,
    /// 最終更新日時（UTC、'YYYY-MM-DD HH:MM:SS'）
    pub updated_at: String,
}

/// 作成・編集フォームの入力を検査したもの（タイトルと本文のセット）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewNote {
    pub title: NoteTitle,
    pub body: NoteBody,
}
```

- **書くとき**の `NewNote` は、`title: NoteTitle` と `body: NoteBody` を持ちます。リポジトリの `insert` と `update` は `NewNote` しか受け取らないので、検査していないタイトルを保存する道は、コードの上に存在しません。
- **読むとき**の `Note` は、`title: String` です。DB に入っている値は、書くときに検査済みで、さらに DB の制約も通っているので、読むたびに検査し直すことはしていません。ここは「検査を何回するか」と「速さ・書きやすさ」の釣り合いで決めたところです。
- 一方で、ID は読むときにも検査しています。DB の行を `Note` に変える `row_to_note`（`src/repository/notes.rs`）の一部です。

```rust
/// 行を Note に変える（列の順番は NOTE_COLUMNS と同じ）
fn row_to_note(row: &Row) -> Result<Note, RepoError> {
    let id = NoteId::new(row.get::<i64>(0)?).map_err(|e| invalid_data("notes.id", e))?;
    let owner = UserId::new(row.get::<i64>(1)?).map_err(|e| invalid_data("notes.user_id", e))?;
    Ok(Note {
        id,
        owner,
        title: row.get::<String>(2)?,
        body: row.get::<String>(3)?,
        done: row.get::<i64>(4)? != 0,
        version: row.get::<i64>(5)?,
        created_at: row.get::<String>(6)?,
        updated_at: row.get::<String>(7)?,
    })
}
```

- `NoteId::new` と `UserId::new` を通すので、もし DB に 0 以下の ID が入っていたら、`RepoError::InvalidData`（DB の値を読み取れない）になります。`NoteId` や `UserId` の型を持つ値は、どこで作られても「1 以上」です。
- `done: row.get::<i64>(4)? != 0` が、DB の `0` / `1` を Rust の `bool` に戻しているところです。型と列の対応表の `bool` の行です。

最後に、DB の制約のエラーを、利用者への返事に変えているところです。`src/error.rs` は、libsql のエラーの文に `UNIQUE constraint failed` が含まれていたら、`RepoError::UniqueViolation` に分類します（抜き出しています）。

```rust
// libsql のエラーを RepoError に変換する。
// 「UNIQUE constraint failed」はローカル（SQLite）でも Turso でも同じ文言なので、文言で見分ける。
impl From<libsql::Error> for RepoError {
    fn from(error: libsql::Error) -> Self {
        let message = error.to_string();
        if message.contains("UNIQUE constraint failed") {
            RepoError::UniqueViolation(message)
        } else {
            RepoError::Database(error)
        }
    }
}
```

登録の処理（`src/routes/signup.rs` の `register`）は、それを受けて、メールの欄に「そのメールアドレスは既に登録されています」を出し、409 で返します（抜き出しています）。

```rust
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

- I-1（メールは1人1つ）は、「先に同じメールがあるか SELECT で調べてから INSERT する」のではなく、**INSERT して、DB の `UNIQUE` に断られたら登録済みとみなす**形で守っています。調べてから入れる形だと、2人がほぼ同時に同じメールで登録したとき、2人とも「まだ無い」と判断して、両方が INSERT へ進むかもしれません。最後の判断を DB に任せれば、そうした隙間がありません。

### 練習問題

**問1:** ノートに「優先度」を足すことになりました。優先度は「高・普通・低」の3つで、DB には 1・2・3 の数で入れ、何も指定しなければ「普通」です。

1. この不変条件を、不変条件の表の形（① 型・② アプリ・③ DB）で書いてください。
2. Rust の型（`Priority`）と、DB の数との変換の関数を書いてください。
3. 列と制約を足す SQL を書き、0・2・4 を入れたときに DB がどう答えるかを確かめてください。

**問2:** もし `Email::parse` が小文字にそろえる処理をしていなかったら、`"Alice@Example.com"` と `"alice@example.com"` の2つの登録は、DB の `UNIQUE` で止まるでしょうか。予想してから、実際に確かめてください。

**問3:** 不変条件 I-5「他人のノートは、見ることも変えることもできない」は、3か所のうち②アプリの処理だけで守っています。

1. なぜ③DB の制約では守れないのでしょうか。
2. ②だけが頼りの決まりを、うっかり破らないために、RustNote ではどんな工夫とテストをしていますか。`src/repository/notes.rs` の先頭のコメントと、`tests/` のテストの名前を手がかりに答えてください。

### 解答と解説

**問1の解答:**

1. 不変条件の表に、次の1行を足します。

| 番号 | 不変条件 | ① 型の検査 | ② アプリの処理 | ③ DB の制約 |
|---|---|---|---|---|
| I-10 | 優先度は「高・普通・低」の3つだけ。指定がなければ「普通」 | `enum Priority` と、DB の数から作る関数 | フォームの値から `Priority` を作れなければ 400 | `INTEGER NOT NULL DEFAULT 2 CHECK (priority BETWEEN 1 AND 3)` |

2・3. 型と変換の関数、DB の制約の確かめを1つにしたプログラムです（`examples/u03_q1.rs`）。

```rust
//! 問1の解答: ノートに「優先度」（1 = 高・2 = 普通・3 = 低）を足すときの、型と DB の守り。
//! 実行: cargo run -q --example u03_q1

use rustnote::db::{self, Db};

/// 優先度。DB には 1〜3 の数で入れる
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Priority {
    High,
    Normal,
    Low,
}

impl Priority {
    /// DB の数から作る。1〜3 以外なら Err
    fn from_db(value: i64) -> Result<Priority, String> {
        match value {
            1 => Ok(Priority::High),
            2 => Ok(Priority::Normal),
            3 => Ok(Priority::Low),
            other => Err(format!("優先度は 1〜3 です（{other} が来ました）")),
        }
    }

    /// DB に入れる数
    fn to_db(self) -> i64 {
        match self {
            Priority::High => 1,
            Priority::Normal => 2,
            Priority::Low => 3,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 型の守り: 1〜3 以外は Priority にならない
    for value in [1, 2, 4] {
        println!(
            "Priority::from_db({value}) = {:?}",
            Priority::from_db(value)
        );
    }
    println!("Priority::Low.to_db() = {}", Priority::Low.to_db());

    // DB の守り: マイグレーションに書く SQL で、列と CHECK を足す（ここでは直接実行する）
    let dir = tempfile::tempdir()?;
    let db = Db::open_local(dir.path().join("u03.db")).await?;
    db::migrate(&db).await?;
    let conn = db.connect().await?;
    conn.execute_batch(
        "ALTER TABLE notes ADD COLUMN priority INTEGER NOT NULL DEFAULT 2
             CHECK (priority BETWEEN 1 AND 3);
         INSERT INTO users (email, password_hash) VALUES ('alice@example.com', 'x');",
    )
    .await?;
    for value in [0, 2, 4] {
        let result = conn
            .execute(
                "INSERT INTO notes (user_id, title, priority) VALUES (1, '宿題', ?1)",
                [value],
            )
            .await;
        match result {
            Ok(_) => println!("priority = {value}: OK"),
            Err(error) => println!("priority = {value}: NG {error}"),
        }
    }
    Ok(())
}
```

```powershell
cargo run -q --example u03_q1
```

```text
Priority::from_db(1) = Ok(High)
Priority::from_db(2) = Ok(Normal)
Priority::from_db(4) = Err("優先度は 1〜3 です（4 が来ました）")
Priority::Low.to_db() = 3
priority = 0: NG SQLite failure: `CHECK constraint failed: priority BETWEEN 1 AND 3`
priority = 2: OK
priority = 4: NG SQLite failure: `CHECK constraint failed: priority BETWEEN 1 AND 3`
```

- Rust の側は `enum` にしたので、`Priority` の値は3つのどれかでしかありえません。数の 4 のような「ありえない優先度」は、`Priority` として存在できないのです（第2巻 UNIT 08）。
- DB の側は、`CHECK (priority BETWEEN 1 AND 3)` で 0 と 4 を断りました。`DEFAULT 2` があるので、優先度を指定しない古い INSERT の書き方も、そのまま動きます。
- 実際に RustNote へ足すときは、この `ALTER TABLE` を新しいマイグレーションのファイル（たとえば `migrations/0004_note_priority.sql`）にし、`src/db.rs` の `MIGRATIONS` に足します。マイグレーションの手順は、UNIT 07 で扱います。

**問2の解答:**

予想: 止まらない。SQLite の `UNIQUE` は、何も指定しなければ、大文字と小文字を**区別して**比べるからです。確かめるプログラムです（`examples/u03_q2.rs`）。

```rust
//! 問2の解答: 大文字・小文字だけが違うメールアドレスは、DB の UNIQUE で止まるか。
//! 実行: cargo run -q --example u03_q2

use rustnote::db::{self, Db};
use rustnote::domain::Email;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let db = Db::open_local(dir.path().join("u03.db")).await?;
    db::migrate(&db).await?;
    let conn = db.connect().await?;
    let sql = "INSERT INTO users (email, password_hash) VALUES (?1, 'x')";

    println!("--- そろえずに、そのまま入れる");
    for input in ["Alice@Example.com", "alice@example.com"] {
        match conn.execute(sql, [input]).await {
            Ok(_) => println!("{input}: OK"),
            Err(error) => println!("{input}: NG {error}"),
        }
    }

    println!("--- Email::parse でそろえてから入れる");
    conn.execute("DELETE FROM users", ()).await?;
    for input in ["Alice@Example.com", "alice@example.com"] {
        let email = Email::parse(input)?;
        match conn.execute(sql, [email.as_str()]).await {
            Ok(_) => println!("{input} → {email}: OK"),
            Err(error) => println!("{input} → {email}: NG {error}"),
        }
    }
    Ok(())
}
```

```powershell
cargo run -q --example u03_q2
```

```text
--- そろえずに、そのまま入れる
Alice@Example.com: OK
alice@example.com: OK
--- Email::parse でそろえてから入れる
Alice@Example.com → alice@example.com: OK
alice@example.com → alice@example.com: NG SQLite failure: `UNIQUE constraint failed: users.email`
```

- そろえずに入れると、2つとも OK でした。DB から見ると、`Alice@Example.com` と `alice@example.com` は**別の文字列**だからです。同じ人が、2つのアカウントを作れてしまいます。
- `Email::parse` でそろえてから入れると、2つ目は `UNIQUE` で止まりました。I-1 は、①型の検査（そろえる）と③DB の制約（`UNIQUE`）の**2つがそろって初めて**守れる決まりです。どちらか片方だけでは守れません。

**問3の解答:**

1. DB の制約は、「この行の中の値」や「ほかの表の行があるか」は調べられますが、「**今、誰がログインしてこの SQL を送っているか**」は知りません。ログインしているのが alice か bob かは、アプリだけが知っている情報なので、②で守るしかありません。
2. 工夫とテストは次のとおりです。

- **工夫:** `src/repository/notes.rs` の先頭のコメントに「どのメソッドも『持ち主（owner）』を受け取り、SQL の WHERE に必ず `user_id = 持ち主` を入れる」と書いてあります。ノートを読む・変えるメソッドは、すべて引数に `owner: UserId` を取るので、持ち主を渡し忘れるとコンパイルエラーになります。持ち主の条件を「各画面の処理で毎回思い出して書く」のではなく、リポジトリの**形**で強制しているのです。
- **テスト:** `tests/repository.rs` の `notes_are_isolated_per_owner`（DB の読み書きの段階で、bob から alice のノートが見えない・変えられない）と、`tests/http.rs` の `other_users_notes_are_404`（画面の段階で、5つの操作がすべて 404）の2段構えで確かめています。

### まとめ

- RustNote のドメインは、ユーザー・ノート・セッションの3つで、ノートとセッションは `user_id` の外部キーで持ち主を指す。ドメインの型と表の列の対応を表にし、`bool` や日時のように、Rust と DB で形が違うものの変換を決めておく。
- 不変条件は、①型の検査（入口で早く、分かる言葉で断る）・②アプリの処理（ログイン中の人や版の番号のような、値だけでは決まらない決まり）・③DB の制約（アプリを通らない書き込みも止める最後の砦）の3か所で守る。1か所でしか守っていない決まりは、必ずテストで確かめる。
- 同じ決まりを2か所で守るときは、数え方（文字数とバイト数）や空白の扱いがずれやすい。実験で食い違いを探し、どちらを正とするかを決めておく。
