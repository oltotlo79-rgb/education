## UNIT 07: 認可・IDOR・持ち主の境目

### この単元のゴール

- ノートの5つの入口（詳細・編集の画面・編集の保存・完了・削除）の全部で、他人には 404、ログインしていない人には 303 になり、データが変わらないことを、**1つの表のテスト**で確かめられる
- 認可の守りが、ログインの確かめ・ID の形・SQL の持ち主の条件の3か所にあり、RustNote ではリポジトリの全部の関数が `owner: UserId` を受け取る作りになっている理由を説明できる
- 持ち主の条件を1つの SQL から外す・`AND` と `OR` のかっこを忘れる、の2つの間違いが、表のテストや検索のテストで見つかることを確かめられる
- ノートの ID が全員で1つの連番であることから、他人のノートの数が分かってしまうことと、その直し方を言える

### たとえ話でつかむ

マンションの、宅配の受け取りの棚を考えます。

- 棚には、部屋の番号の札が付いた箱が並んでいます。住人は、管理人さんに部屋の番号を言って、荷物を受け取ります。
- 管理人さんが確かめるのは、「この人は、ここの住人か」（ログイン）だけではありません。「この人は、**その部屋の**住人か」（持ち主）も確かめます。301 号室の人が「305 号室の荷物をください」と言っても、渡しません。
- 受け取りだけでなく、「荷物を返送して」「中身を入れ替えて」のような**どの頼みでも**、同じ確かめをします。1つの頼みだけ確かめを忘れると、そこから、ほかの部屋の荷物に手が届きます。
- 部屋番号が 101・102・103…と並んでいると、空き部屋の札から、何部屋が埋まっているかが分かります。番号の付け方からも、知られたくないことが分かることがあります。

| 宅配の棚 | RustNote の認可 |
|---|---|
| ここの住人か | `require_user`（ログインしていなければ 303） |
| その部屋の住人か | SQL の `AND user_id = ?`（他人なら 0 行 → 404） |
| どの頼みでも同じ確かめ | 5つの入口と、リポジトリの全部の関数に `owner` |
| 部屋番号の並びから分かること | ノートの ID は全員で1つの連番（例7-2） |

> **用語:** 認可（にんか：ログインした人が、その操作をしてよいかを決めること。RustNote では「自分のノートだけを読み書きしてよい」。認証（誰か）とは別。UNIT 01）

<!-- -->

> **用語:** IDOR（アイドア。Insecure Direct Object Reference：URL やフォームの ID を書き換えるだけで、他人のデータに届いてしまう弱さ。持ち主の確かめを忘れると起きる）

<!-- -->

> **用語:** 横の権限と縦の権限（同じ立場の他人のデータに届くのが「横」の問題、一般の利用者が管理者の操作に届くのが「縦」の問題。RustNote には管理者がいないので、横だけを考えればよい）

### 図で理解する

認可の守りが、どこにあるかです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-0701" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="80" y="22" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">5つの入口</text>
<rect x="10" y="34" width="140" height="30" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="80" y="54" font-size="11" fill="#0f172a" text-anchor="middle">GET /notes/{id}</text>
<rect x="10" y="70" width="140" height="30" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="80" y="90" font-size="11" fill="#0f172a" text-anchor="middle">GET …/edit</text>
<rect x="10" y="106" width="140" height="30" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="80" y="126" font-size="11" fill="#0f172a" text-anchor="middle">POST …/edit</text>
<rect x="10" y="142" width="140" height="30" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="80" y="162" font-size="11" fill="#0f172a" text-anchor="middle">POST …/toggle</text>
<rect x="10" y="178" width="140" height="30" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="80" y="198" font-size="11" fill="#0f172a" text-anchor="middle">POST …/delete</text>
<rect x="190" y="70" width="150" height="66" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="265" y="96" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">① require_user</text>
<text x="265" y="118" font-size="11" fill="#475569" text-anchor="middle">無ければ 303</text>
<rect x="370" y="70" width="150" height="66" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="445" y="96" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">② NoteId</text>
<text x="445" y="118" font-size="11" fill="#475569" text-anchor="middle">形が違えば 404</text>
<rect x="550" y="50" width="160" height="106" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="630" y="76" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">③ リポジトリ</text>
<text x="630" y="98" font-size="11" fill="#0f172a" text-anchor="middle">全部の関数が owner</text>
<text x="630" y="116" font-size="11" fill="#0f172a" text-anchor="middle">全部の SQL に</text>
<text x="630" y="134" font-size="11" fill="#0f172a" text-anchor="middle">user_id = ?（0 行 → 404）</text>
<line x1="150" y1="103" x2="186" y2="103" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0701)"/>
<line x1="340" y1="103" x2="366" y2="103" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0701)"/>
<line x1="520" y1="103" x2="546" y2="103" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-0701)"/>
<rect x="190" y="200" width="520" height="50" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="450" y="222" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">表のテスト（例7-1）: 5つの入口 × bob・guest を、1回で全部確かめる</text>
<text x="450" y="240" font-size="11" fill="#475569" text-anchor="middle">1つの入口の確かめ忘れが、表の1つのマスとして見える</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: 認可の守りの3か所。どの入口も、ログインの確かめ・ID の形・持ち主の条件付きの SQL を通る。持ち主の条件は、リポジトリの関数の引数として要るので、渡し忘れはコンパイルで止まる</div>
</div>

`AND` と `OR` の、かっこの違いです（エラー2）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 200" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="340" height="180" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="180" y="36" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">かっこあり（RustNote）</text>
<text x="30" y="66" font-size="11" fill="#0f172a" font-family="Consolas, monospace">user_id = ?1 AND (title LIKE ?2</text>
<text x="30" y="84" font-size="11" fill="#0f172a" font-family="Consolas, monospace">               OR body LIKE ?2)</text>
<text x="30" y="116" font-size="12" fill="#0f172a">＝ 自分のノート、で、しかも</text>
<text x="30" y="136" font-size="12" fill="#0f172a">　 タイトルか本文に合う</text>
<text x="30" y="170" font-size="12" font-weight="700" fill="#047857">→ 他人のノートは、どの言葉でも出ない</text>
<rect x="370" y="10" width="340" height="180" rx="10" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="540" y="36" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">かっこなし（エラー2）</text>
<text x="390" y="66" font-size="11" fill="#0f172a" font-family="Consolas, monospace">user_id = ?1 AND title LIKE ?2</text>
<text x="390" y="84" font-size="11" fill="#0f172a" font-family="Consolas, monospace">            OR body LIKE ?2</text>
<text x="390" y="116" font-size="12" fill="#0f172a">＝（自分のノートで、タイトルに合う）</text>
<text x="390" y="136" font-size="12" fill="#0f172a">　 または（<tspan font-weight="700">誰のでも</tspan>、本文に合う）</text>
<text x="390" y="170" font-size="12" font-weight="700" fill="#b91c1c">→ 他人のノートが、本文の言葉で出る</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図8-?: SQL では AND が OR より先に結び付く。かっこが無いと、持ち主の条件はタイトルの側にだけ掛かり、本文の側は誰のノートでも合ってしまう</div>
</div>

### まず動かす最小コード

#### 例7-1: 5つの入口を、表で確かめる

ノートのリポジトリ（`src/repository/notes.rs`）の、ファイルの先頭のコメントと約束（trait）です（抜き出しています）。ノートを扱う関数は、どれも最初に `owner: UserId` を受け取ります。

```rust
//! どのメソッドも「持ち主（owner）」を受け取り、SQL の WHERE に必ず
//! `user_id = 持ち主` を入れる。だから他人のノートは「見つからない」扱いになる。

/// ノートの保存先に求める約束
#[async_trait]
pub trait NoteRepository {
    /// ノートを追加し、保存された内容（id や日時つき）を返す
    async fn insert(&self, owner: UserId, note: &NewNote) -> Result<Note, RepoError>;

    /// 自分のノートを1件探す（他人のノート・存在しないノートは None）
    async fn find(&self, owner: UserId, id: NoteId) -> Result<Option<Note>, RepoError>;

    /// 自分のノートを、キーワードとページ番号で検索する（新しい順）
    async fn search(&self, owner: UserId, query: &NoteQuery) -> Result<NotePage, RepoError>;

    // …（update・toggle_done・delete も、最初の引数は owner: UserId）
}
```

> **▼ このコードがやること（先に日本語で）:** alice と bob を登録し、alice のノートを1件作って、その中身（タイトル・完了・version）を表示します。次に、ノートの5つの入口（詳細・編集の画面・編集の保存・完了・削除）に、bob のクッキーと、クッキー無し（guest）で、リクエストを送り、状態コードを表にします。そのあとで、alice のノートが変わっていないかを表示し、最後に alice 本人なら詳細と編集の画面が開けることを確かめます。

```rust
//! 第8巻 UNIT 07: alice のノート1件に、ノートの5つの入口から、alice・bob・ログインしていない人（guest）が
//! 近づいたときの返事を、表にして確かめる。最後に、alice のノートが変わっていないかを見る。
//! 実行: cargo test --test u07_matrix -- --nocapture

mod common;

use common::{TestApp, encode, test_app};

/// alice のノート（id=1）の、タイトル・完了・version
async fn note(app: &TestApp) -> String {
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query("SELECT title, done, version FROM notes WHERE id = 1", ())
        .await
        .unwrap();
    match rows.next().await.unwrap() {
        Some(row) => format!(
            "title={:?} done={} version={}",
            row.get::<String>(0).unwrap(),
            row.get::<i64>(1).unwrap(),
            row.get::<i64>(2).unwrap()
        ),
        None => "（消えた）".to_string(),
    }
}

#[tokio::test]
async fn authorization_matrix() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let bob = app
        .sign_up("bob@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "牛乳を買う", "").await;
    println!("はじめ: {}", note(&app).await);

    let edit = format!("version=1&title={}&body=", encode("書き換えた"));
    let entrances: [(&str, &str, Option<&str>); 5] = [
        ("GET ", "/notes/1", None),
        ("GET ", "/notes/1/edit", None),
        ("POST", "/notes/1/edit", Some(edit.as_str())),
        ("POST", "/notes/1/toggle", Some("")),
        ("POST", "/notes/1/delete", Some("")),
    ];
    println!("・{:<21} {:>8} {:>8}", "入口", "bob", "guest");
    for (method, path, form) in entrances {
        let mut cells = Vec::new();
        for cookie in [Some(bob.as_str()), None] {
            let response = match form {
                Some(form) => app.post_form(path, form, cookie).await,
                None => app.get(path, cookie).await,
            };
            cells.push(response.status().as_u16().to_string());
        }
        println!("・{method} {path:<17} {:>8} {:>8}", cells[0], cells[1]);
    }
    println!("bob と guest のあと: {}", note(&app).await);

    // 最後に、alice 本人なら、同じ入口が使えることを確かめる（GET の2つだけ）
    for path in ["/notes/1", "/notes/1/edit"] {
        let status = app.get(path, Some(&alice)).await.status().as_u16();
        println!("alice GET {path} → {status}");
    }
}
```

```powershell
cargo test --test u07_matrix -- --nocapture
```

```text
はじめ: title="牛乳を買う" done=0 version=1
・入口                         bob    guest
・GET  /notes/1               404      303
・GET  /notes/1/edit          404      303
・POST /notes/1/edit          404      303
・POST /notes/1/toggle        404      303
・POST /notes/1/delete        404      303
bob と guest のあと: title="牛乳を買う" done=0 version=1
alice GET /notes/1 → 200
alice GET /notes/1/edit → 200
```

（結果の行だけを抜き出しています）

#### ▼ コードを1つずつ分解して解説

##### 解説1: どの入口も、bob は 404、guest は 303

- 表の10のマスは、bob が全部 `404`、guest が全部 `303`（ログイン画面へ）でした。表のあとの alice のノートは、`title="牛乳を買う" done=0 version=1` のままで、bob の保存・完了・削除は、1つも効いていません。
- guest が 404 ではなく 303 なのは、①の `require_user` が、どの入口でも最初に動くからです。ログインしていない人には、ノートがあるかどうかを調べる前に、ログイン画面へ送ります（第10巻 UNIT 03 の問1）。

##### 解説2: 持ち主は、関数の引数として要る

- リポジトリの関数は、全部が `owner: UserId` を最初に受け取り、全部の SQL に `user_id = ?` の条件があります（第10巻 UNIT 06 の問2で、SQL を一覧にしました）。持ち主を渡すのは**呼ぶ側の義務**で、渡し忘れると、コンパイルで止まります（エラー3）。
- 表のテストは、「入口を1つ足したら、表に1行足す」だけで、新しい入口も、同じ確かめに入れられます。1つずつ名前を付けたテストを書くより、抜けが目に見えやすいのです。

### もう一歩: 実務に近い形

#### 例7-2: ノートの ID の連番から、分かること

> **▼ このコードがやること（先に日本語で）:** alice と bob を登録し、bob がノートを1件作ります。そのあとで alice が5件作り、bob がもう1件作ります。bob の2件のノートの URL（ID）を表示し、bob が、自分のではない ID を開いてみます。

```rust
//! 第8巻 UNIT 07: ノートの ID は、全員で1つの連番。bob が自分のノートを作ったときの ID から、何が分かるか。
//! 実行: cargo test --test u07_ids -- --nocapture

mod common;

use common::test_app;

#[tokio::test]
async fn ids_are_shared() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let bob = app
        .sign_up("bob@example.com", "correct horse battery")
        .await;
    let first = app.create_note(&bob, "bob の1件目", "").await;
    println!("bob の1件目: {first}");
    for n in 1..=5 {
        app.create_note(&alice, &format!("alice の{n}件目"), "")
            .await;
    }
    println!("（この間に、alice が5件作った）");
    let second = app.create_note(&bob, "bob の2件目", "").await;
    println!("bob の2件目: {second}");
    for path in ["/notes/2", "/notes/6"] {
        let status = app.get(path, Some(&bob)).await.status();
        println!("bob GET {path} → {status}");
    }
}
```

```powershell
cargo test --test u07_ids -- --nocapture
```

```text
bob の1件目: /notes/1
（この間に、alice が5件作った）
bob の2件目: /notes/7
bob GET /notes/2 → 404 Not Found
bob GET /notes/6 → 404 Not Found
```

- bob の1件目は `/notes/1`、2件目は `/notes/7` でした。ノートの ID は、`notes` 表の `INTEGER PRIMARY KEY AUTOINCREMENT` で、**全員で1つの連番**です。bob は、自分の2件の間に、**ほかの人が5件のノートを作った**ことを知りました。
- 中身は見えません（`/notes/2` も `/notes/6` も `404`）。それでも、「RustNote 全体で、1日に何件のノートが作られているか」のような、運営する側が知られたくない数が、少しずつ分かります。
- 直すなら、ID を、推測できない値（ランダムな UUID など）にするか、URL には人ごとの番号（その人の何件目か）を使います。どちらも、表の作りを変えることになるので、RustNote は、連番のまま、持ち主の条件と 404 で中身を守っています（改善点）。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: 1つの SQL だけ、持ち主の条件を外した

完了の切り替え（`toggle_done`）の SQL を `WHERE id = ?1` だけにした場合です（練習用のコピーの `src/repository/notes.rs` を書き換え、確かめたら元に戻します）。ほかの4つの入口は、そのままです。例7-1のテストを動かします。

```text
はじめ: title="牛乳を買う" done=0 version=1
・入口                         bob    guest
・GET  /notes/1               404      303
・GET  /notes/1/edit          404      303
・POST /notes/1/edit          404      303
・POST /notes/1/toggle        303      303
・POST /notes/1/delete        404      303
bob と guest のあと: title="牛乳を買う" done=1 version=2
```

- **何が起きたか:** 表の中で、`POST /notes/1/toggle` の bob のマスだけが `303` になり、alice のノートが `done=1 version=2` に変わりました。bob は、alice のノートの完了を、勝手に切り替えたのです。ほかの9つのマスは、正しいままです。
- **気づき方:** 1つずつの入口を、alice だけで確かめるテストでは、見つかりません。**他人として、全部の入口を**確かめる表のテストなら、マス1つの違いとして見えます。`params!` は持ち主の値を渡したままですが、libsql は、使われない値があってもエラーにしません（第10巻 UNIT 03 のエラー1）。

#### エラー2（エラーが出ない間違い）: 検索の条件の、かっこを忘れた

検索の条件を `user_id = ?1 AND title LIKE ?2 … OR body LIKE ?2 …` と、かっこ無しで書いた場合です。alice が「銀行のメモ」（本文に「暗証番号は 1234」）、bob が「bob の買い物」を持っているときに、3通りで検索するテスト（`tests/u07_search.rs`）を動かします。

```rust
//! 第8巻 UNIT 07: bob が、alice のノートの本文にだけある言葉で検索する。
//! 実行: cargo test --test u07_search -- --nocapture

mod common;

use common::{body_text, encode, test_app};

/// 一覧の HTML から、ノートのタイトルを、上から順に読む
fn titles(html: &str) -> Vec<String> {
    html.split("<li class=\"note-item")
        .skip(1)
        .filter_map(|item| item.split("<a href=\"/notes/").nth(1))
        .filter_map(|t| t.split_once('>'))
        .map(|(_, rest)| rest.split('<').next().unwrap().to_string())
        .collect()
}

#[tokio::test]
async fn search_stays_inside_own_notes() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let bob = app
        .sign_up("bob@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "銀行のメモ", "暗証番号は 1234")
        .await;
    app.create_note(&bob, "bob の買い物", "牛乳").await;
    for (who, cookie, q) in [
        ("bob", &bob, "暗証番号"),
        ("bob", &bob, "買い物"),
        ("alice", &alice, "暗証番号"),
    ] {
        let html = body_text(
            app.get(&format!("/notes?q={}", encode(q)), Some(cookie))
                .await,
        )
        .await;
        println!("{who} が q={q} で検索 → {:?}", titles(&html));
    }
}
```

まず、RustNote のコードです。

```text
bob が q=暗証番号 で検索 → []
bob が q=買い物 で検索 → ["bob の買い物"]
alice が q=暗証番号 で検索 → ["銀行のメモ"]
```

次に、かっこを忘れた版です。

```text
bob が q=暗証番号 で検索 → ["銀行のメモ"]
bob が q=買い物 で検索 → ["bob の買い物"]
alice が q=暗証番号 で検索 → ["銀行のメモ"]
```

- **何が起きたか:** bob が「暗証番号」で検索すると、alice の「銀行のメモ」が、bob の一覧に出ました。SQL では `AND` が `OR` より先に結び付くので、条件は「（自分のノートで、タイトルに合う）**または**（誰のノートでも、本文に合う）」になったからです（図の2つ目）。
- **気づきにくい理由:** bob が「買い物」（自分のノートのタイトルにある言葉）で探すと、正しく自分のノートだけが出ます。alice が自分の言葉で探しても、正しく見えます。**他人の本文にだけある言葉**で探したときにだけ起きるので、ふつうに使っていても気づきません。
- **直し方:** `OR` を含む条件は、必ずかっこで囲みます。RustNote は、条件を `condition` という1つの文字列にまとめ、件数と一覧の両方で使っているので、1か所を直せば、両方が直ります（第10巻 UNIT 07）。

#### エラー3: 持ち主を渡し忘れた（E0061）

完了の切り替えの画面（`src/routes/notes/id/toggle.rs`）で、`.toggle_done(user.id, id)` を `.toggle_done(id)` と書いた場合です。

```text
   Compiling rustnote v0.1.0 (...\rustnote)
error[E0061]: this method takes 2 arguments but 1 argument was supplied
  --> src\routes\notes\id\toggle.rs:24:10
   |
24 |         .toggle_done(id)
   |          ^^^^^^^^^^^ -- argument #1 of type `UserId` is missing
   |
note: method defined here
  --> src\repository\notes.rs:37:14
   |
37 |     async fn toggle_done(&self, owner: UserId, id: NoteId) -> Result<Note, RepoError>;
   |              ^^^^^^^^^^^        -----
help: provide the argument
   |
24 |         .toggle_done(/* UserId */, id)
   |                      +++++++++++++

```

- **何が起きたか:** 「この関数は引数を2つとりますが、1つしか渡されていません」で、`argument #1 of type UserId is missing`（1つ目の `UserId` が足りない）と、`help:` が `/* UserId */` を入れる場所を示しています。
- **学べること:** 持ち主を引数にしておくと、「持ち主を確かめ忘れた SQL を呼ぶ」ことが、コンパイルで止まります。エラー1のように SQL の中身を間違えることは防げませんが、**呼ぶ側の確かめ忘れ**は防げます。だから、リポジトリには「持ち主を受け取らない関数」を、わざと作っていません。

### RustNote ではここで使う

- 認可の決まりは、`src/error.rs` の先頭の表の「`NotFound` … 存在しないノート、**他人のノート**（あること自体を隠す）」と、`src/repository/notes.rs` の全部の SQL の `user_id = ?` です。
- 詳細と編集は `find_own_note`（`src/routes/notes/id.rs`）を、完了と削除は、SQL の中で持ち主を確かめる `toggle_done` と `delete` を通ります（第10巻 UNIT 03 の問3）。
- RustNote には、ノートを人に見せる（共有する）仕組みも、管理者もありません。共有を足すなら、「持ち主か、共有された人か」を確かめる条件を、全部の SQL に足すことになり、例7-1の表に「共有された carol」の列を足して確かめます。
- ノートの ID は連番です（例7-2の改善点）。

### 練習問題

**問1:** 例7-1の表に、`GET /notes?q=牛乳`（一覧の検索）の行を足すと、bob と guest はどうなるでしょうか。

**問2:** 例7-2のような「ID の連番から分かること」を防ぐために、ID をランダムな値にしたとします。それでも、持ち主の条件は要るでしょうか。

**問3:** 将来、管理者の画面（全員のノートを見られる）を足すことになりました。認可の確かめを、どこに、どう足すとよいでしょうか。

### 解答と解説

**問1の解答:**

```rust
//! 第8巻 UNIT 07 練習問題1: 一覧の検索（GET /notes?q=…）に、bob とログインしていない人が来たとき。
//! 実行: cargo test --test u07_q -- --nocapture

mod common;

use common::{body_text, encode, test_app};

#[tokio::test]
async fn q1_list_for_bob_and_guest() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let bob = app
        .sign_up("bob@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "牛乳を買う", "").await;
    let url = format!("/notes?q={}", encode("牛乳"));
    for (who, cookie) in [("bob", Some(bob.as_str())), ("guest", None)] {
        let response = app.get(&url, cookie).await;
        let status = response.status();
        let html = body_text(response).await;
        println!(
            "問1 {who} GET /notes?q=牛乳 → {status}（「牛乳を買う」が出る: {}）",
            html.contains("牛乳を買う</a>")
        );
    }
}
```

```powershell
cargo test --test u07_q -- --nocapture
```

```text
問1 bob GET /notes?q=牛乳 → 200 OK（「牛乳を買う」が出る: false）
問1 guest GET /notes?q=牛乳 → 303 See Other（「牛乳を買う」が出る: false）
```

- bob は `200`（自分の一覧が開き、alice のノートは出ない）、guest は `303` でした。一覧は「他人のノート」を指す URL ではないので、404 にはなりません。代わりに、出てくる中身が「自分のノートだけ」かを確かめます（例7-1の表の外の、エラー2のテスト）。
- このように、表のテストには「状態コード」だけでなく、「中身に他人のデータが混ざらないか」も確かめる行が要ります。

**問2の解答:**

- 要ります。ランダムな ID は「当てにくくする」だけで、URL を一度でも知った人（共有された URL、ブラウザの履歴、ログに残った URL など）には、届いてしまいます。持ち主の条件は「知っていても、見せない」守りで、ランダムな ID は「知られにくくする」工夫です。2つは、役目が違います。
- ランダムな ID だけに頼る作り（「URL を知っている人なら誰でも見られる」）は、わざと公開するリンクにだけ使います。

**問3の解答:**

- 管理者かどうかは、ユーザーの役割（`users` 表の `role` など）として DB に持ち、管理者の画面の**最初**で、`require_user` のあとに「管理者か」を確かめる関数（たとえば `require_admin`）を呼びます。確かめに失敗したら、管理者の画面があることも隠すために 404 を返します。
- 管理者の画面のための SQL は、持ち主の条件の無い、別の関数として作り、名前でそれと分かるようにします（たとえば `admin_search_all`）。ふつうの画面から、うっかり呼べないように、モジュールも分けます。これが「縦」の権限の守りで、例7-1と同じように、「一般の alice が管理者の入口に近づく」表のテストで確かめます。

### まとめ

- 認可は、ログインの確かめ（303）・ID の形（404）・持ち主の条件付きの SQL（0 行なら 404）の3か所で守る。リポジトリの全部の関数が `owner: UserId` を受け取るので、渡し忘れは E0061 で止まる。
- 全部の入口を、他人とログインしていない人で確かめる表のテストを持つと、1つの SQL の持ち主の条件の外し忘れが、マス1つの違いとして見える。`OR` を含む条件のかっこ忘れは、他人の本文にだけある言葉で検索したときにだけ漏れる。
- ノートの ID は全員で1つの連番なので、自分の ID の飛び方から、他人のノートの数が分かる（改善点）。ランダムな ID は当てにくくするだけで、持ち主の条件の代わりにはならない。
