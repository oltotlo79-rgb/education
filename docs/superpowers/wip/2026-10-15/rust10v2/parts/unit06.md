## UNIT 06: 削除・確かめ・消し方の選び方

### この単元のゴール

- 削除（`POST /notes/{id}/delete`）が、`DELETE … WHERE id = ?1 AND user_id = ?2` の1文で消し、消した行の数が 0 なら 404、消せたら 303 で一覧へ移ることを確かめられる
- 「削除」を、GET のリンク・ほかのサイトのフォーム・他人のボタンから引き起こせないことを、1つずつ確かめて、それぞれ何が止めているか（405・403・404）を言える
- 削除を GET にすると、**ほかのサイトのリンクを開いただけで消える**ことを、実際に確かめて説明できる
- すぐに消す（物理削除）・印を付ける（論理削除）・ごみ箱、の違いを比べ、RustNote に確かめの画面や元に戻す仕組みが無いことを、改善点として説明できる

### たとえ話でつかむ

会社の書類の捨て方を考えます。

- **シュレッダー:** 入れたら、すぐに細かくなります。元には戻せません。だから、シュレッダーは、通りがかった人がうっかり触れる廊下（**誰でも開けるリンク**）には置きません。使う人が、自分の机で、ボタンを押して使います。
- **ほかの部署からの電話:** 「〇〇の書類を捨てておいて」と、ほかの部署の人に言われても、自分の部署の書類でなければ、捨てません（**ほかのサイトからの送信**）。
- **「廃棄予定」の箱:** すぐにシュレッダーにかけず、箱に入れておき、1 週間たったら、まとめてシュレッダーにかけます。間違えて入れても、箱から取り出せます。その代わり、箱の中の書類も、探すときに「これは捨てる予定の物」と見分けなければなりません。

| 書類の捨て方 | RustNote の削除 |
|---|---|
| 廊下には置かない | 削除は POST だけ。GET のリンクでは 405 |
| ほかの部署からの頼みは断る | ほかのサイトからの POST は、Topcoat の OriginPolicy が 403 |
| 自分の部署の書類だけ | `DELETE … AND user_id = ?2`。他人のノートは 0 行 → 404 |
| シュレッダー（すぐ消える） | RustNote の今の作り（物理削除）。確かめの画面も、元に戻す仕組みも無い |
| 「廃棄予定」の箱 | 論理削除（`deleted_at` の列）やごみ箱。RustNote には無い（改善点） |

> **用語:** 物理削除（DB の行を `DELETE` で本当に消すこと。元には戻せない）

<!-- -->

> **用語:** 論理削除（行は消さずに、`deleted_at`（消した日時）のような印を付けて、消したものとして扱うこと。ソフトデリートとも言う）

<!-- -->

> **用語:** 405 Method Not Allowed（「その URL は、そのメソッド（GET や POST）では受け付けません」という返事。受け付けるメソッドを `allow` ヘッダーで知らせる）

### 図で理解する

削除の POST が、ノートを消すまでに通る守りです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-10-0601" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="30" width="120" height="60" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="70" y="56" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">① メソッド</text>
<text x="70" y="76" font-size="11" fill="#475569" text-anchor="middle">POST だけ</text>
<rect x="150" y="30" width="130" height="60" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="215" y="56" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">② OriginPolicy</text>
<text x="215" y="76" font-size="11" fill="#475569" text-anchor="middle">同じサイトからか</text>
<rect x="300" y="30" width="120" height="60" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="56" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">③ ログイン</text>
<text x="360" y="76" font-size="11" fill="#475569" text-anchor="middle">require_user</text>
<rect x="440" y="30" width="120" height="60" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="500" y="56" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">④ ID の形</text>
<text x="500" y="76" font-size="11" fill="#475569" text-anchor="middle">NoteId</text>
<rect x="580" y="30" width="130" height="60" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="645" y="52" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">⑤ DELETE</text>
<text x="645" y="70" font-size="10" fill="#475569" text-anchor="middle">AND user_id = ?2</text>
<text x="645" y="84" font-size="10" fill="#475569" text-anchor="middle">消した行の数を見る</text>
<line x1="130" y1="60" x2="146" y2="60" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0601)"/>
<line x1="280" y1="60" x2="296" y2="60" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0601)"/>
<line x1="420" y1="60" x2="436" y2="60" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0601)"/>
<line x1="560" y1="60" x2="576" y2="60" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0601)"/>
<line x1="70" y1="90" x2="70" y2="126" stroke="#f97316" stroke-width="2" marker-end="url(#ar-10-0601)"/>
<line x1="215" y1="90" x2="215" y2="126" stroke="#f97316" stroke-width="2" marker-end="url(#ar-10-0601)"/>
<line x1="360" y1="90" x2="360" y2="126" stroke="#f97316" stroke-width="2" marker-end="url(#ar-10-0601)"/>
<line x1="500" y1="90" x2="500" y2="126" stroke="#f97316" stroke-width="2" marker-end="url(#ar-10-0601)"/>
<line x1="620" y1="90" x2="620" y2="126" stroke="#f97316" stroke-width="2" marker-end="url(#ar-10-0601)"/>
<line x1="680" y1="90" x2="680" y2="196" stroke="#10b981" stroke-width="2" marker-end="url(#ar-10-0601)"/>
<rect x="10" y="130" width="120" height="50" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="70" y="152" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">405</text>
<text x="70" y="170" font-size="10" fill="#475569" text-anchor="middle">GET のリンク</text>
<rect x="150" y="130" width="130" height="50" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="215" y="152" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">403</text>
<text x="215" y="170" font-size="10" fill="#475569" text-anchor="middle">ほかのサイトのフォーム</text>
<rect x="300" y="130" width="120" height="50" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="152" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">303 → /login</text>
<text x="360" y="170" font-size="10" fill="#475569" text-anchor="middle">ログインしていない</text>
<rect x="440" y="130" width="120" height="50" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="500" y="152" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">404</text>
<text x="500" y="170" font-size="10" fill="#475569" text-anchor="middle">abc・0 など</text>
<rect x="580" y="130" width="80" height="50" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="620" y="152" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">404</text>
<text x="620" y="170" font-size="10" fill="#475569" text-anchor="middle">0 行</text>
<rect x="520" y="200" width="190" height="50" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="615" y="222" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">1 行 → 303 → /notes</text>
<text x="615" y="240" font-size="10" fill="#475569" text-anchor="middle">「ノートを削除しました」</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: 削除の POST が通る5つの守り。メソッド（405）・送り元のサイト（403）・ログイン（303）・ID の形（404）・持ち主の条件付きの DELETE（0 行なら 404）を、すべて通ったときだけ消える</div>
</div>

消し方の選び方です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="160" height="40" rx="6" fill="#0f172a"/>
<rect x="180" y="10" width="170" height="40" rx="6" fill="#0f172a"/>
<text x="265" y="35" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">物理削除（DELETE）</text>
<rect x="360" y="10" width="170" height="40" rx="6" fill="#0f172a"/>
<text x="445" y="35" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">論理削除（deleted_at）</text>
<rect x="540" y="10" width="170" height="40" rx="6" fill="#0f172a"/>
<text x="625" y="35" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">ごみ箱＋期限で消す</text>
<text x="90" y="80" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">元に戻せるか</text>
<text x="265" y="80" font-size="11" fill="#b91c1c" text-anchor="middle">戻せない</text>
<text x="445" y="80" font-size="11" fill="#047857" text-anchor="middle">戻せる（印を外す）</text>
<text x="625" y="80" font-size="11" fill="#047857" text-anchor="middle">期限までは戻せる</text>
<text x="90" y="124" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">読む SQL</text>
<text x="265" y="124" font-size="11" fill="#047857" text-anchor="middle">今のまま</text>
<text x="445" y="116" font-size="11" fill="#b91c1c" text-anchor="middle">全部の SELECT に</text>
<text x="445" y="132" font-size="11" fill="#b91c1c" text-anchor="middle">deleted_at IS NULL（問2）</text>
<text x="625" y="116" font-size="11" fill="#c2410c" text-anchor="middle">論理削除と同じ</text>
<text x="625" y="132" font-size="11" fill="#c2410c" text-anchor="middle">＋期限の掃除の仕事</text>
<text x="90" y="176" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">中身が残るか</text>
<text x="265" y="176" font-size="11" fill="#047857" text-anchor="middle">残らない</text>
<text x="445" y="168" font-size="11" fill="#b91c1c" text-anchor="middle">残り続ける</text>
<text x="445" y="184" font-size="11" fill="#b91c1c" text-anchor="middle">（消したつもりの個人の情報）</text>
<text x="625" y="176" font-size="11" fill="#c2410c" text-anchor="middle">期限までは残る</text>
<text x="90" y="222" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">RustNote</text>
<text x="265" y="222" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">← いまの作り</text>
<text x="445" y="222" font-size="11" fill="#475569" text-anchor="middle">使っていない</text>
<text x="625" y="222" font-size="11" fill="#475569" text-anchor="middle">使っていない</text>
<line x1="10" y1="98" x2="710" y2="98" stroke="#e2e8f0" stroke-width="1"/>
<line x1="10" y1="148" x2="710" y2="148" stroke="#e2e8f0" stroke-width="1"/>
<line x1="10" y1="198" x2="710" y2="198" stroke="#e2e8f0" stroke-width="1"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: 消し方の3つの選び方。元に戻せるほど、読む SQL の条件と、残る中身の扱いが増える。RustNote は、いちばん簡単な物理削除を選んでいる</div>
</div>

### まず動かす最小コード

#### 例6-1: ノートを削除して、一覧と詳細を確かめる

削除は `src/routes/notes/id/delete.rs` です。

```rust
#[page(POST)]
pub async fn delete(cx: &Cx) -> Result<()> {
    let user = require_user(cx).await?;
    let id = *path_param::<Id>(cx)?;
    app_state(cx)
        .notes
        .delete(user.id, id)
        .await
        .map_err(AppError::from)?;
    flash::set(cx, Flash::NoteDeleted);
    Err(see_other(href!(crate::routes::notes::index).resolve(cx)).into())
}
```

リポジトリの `delete`（`src/repository/notes.rs`）は、`execute` が返す「変わった行の数」を見ます。

```rust
let deleted = conn
    .execute(
        "DELETE FROM notes WHERE id = ?1 AND user_id = ?2",
        libsql::params![id.get(), owner.get()],
    )
    .await?;
if deleted == 0 {
    return Err(RepoError::NotFound);
}
Ok(())
```

> **▼ このコードがやること（先に日本語で）:** alice がノートを2件作り、`notes` 表の id を表示します。1件目を削除の POST で消し、返事（状態コード・移動先・お知らせのクッキー）と、消したあとの id を表示します。移動先の一覧を開いて、お知らせと、2つのタイトルがあるかを表示し、最後に、消したノートの詳細を開くのと、もう一度消すのを試します。

```rust
//! 第10巻 UNIT 06: ノートを削除し、一覧・詳細・もう一度の削除が、どうなるかを確かめる。
//! 実行: cargo test --test u06_delete -- --nocapture

mod common;

use common::{TestApp, body_text, location, test_app};

/// notes 表の id を、小さい順に読む
async fn note_ids(app: &TestApp) -> Vec<i64> {
    let conn = app.db.db.connect().await.expect("DB に接続できません");
    let mut rows = conn
        .query("SELECT id FROM notes ORDER BY id", ())
        .await
        .expect("読めません");
    let mut ids = Vec::new();
    while let Some(row) = rows.next().await.expect("行を読めません") {
        ids.push(row.get(0).expect("id"));
    }
    ids
}

#[tokio::test]
async fn delete_a_note() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "牛乳を買う", "").await;
    app.create_note(&alice, "卵を買う", "").await;
    println!("1. 削除の前の id: {:?}", note_ids(&app).await);

    let response = app.post_form("/notes/1/delete", "", Some(&alice)).await;
    let status = response.status();
    if !status.is_redirection() {
        let allow = response
            .headers()
            .get("allow")
            .map(|v| format!("（allow: {}）", v.to_str().unwrap()))
            .unwrap_or_default();
        println!("2. POST /notes/1/delete → {status}{allow}");
        println!("   id: {:?}", note_ids(&app).await);
        return;
    }
    let to = location(&response).to_string();
    let flash = response
        .headers()
        .get_all("set-cookie")
        .iter()
        .map(|v| v.to_str().unwrap().split(';').next().unwrap().to_string())
        .find(|c| c.starts_with("flash="))
        .unwrap_or_default();
    println!("2. POST /notes/1/delete → {status}（Location: {to}・{flash}）");
    println!("   削除のあとの id: {:?}", note_ids(&app).await);

    let html = body_text(app.get(&to, Some(&format!("{alice}; {flash}"))).await).await;
    println!(
        "3. 一覧: お知らせ「ノートを削除しました」={}・「牛乳を買う」={}・「卵を買う」={}",
        html.contains("ノートを削除しました"),
        html.contains("牛乳を買う"),
        html.contains("卵を買う")
    );
    let status = app.get("/notes/1", Some(&alice)).await.status();
    println!("4. GET /notes/1 → {status}");
    let status = app
        .post_form("/notes/1/delete", "", Some(&alice))
        .await
        .status();
    println!("5. もう一度 POST /notes/1/delete → {status}");
}
```

```powershell
cargo test --test u06_delete -- --nocapture
```

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.17s
     Running tests\u06_delete.rs (target\debug\deps\u06_delete-3b5156e880dce559.exe)

running 1 test
POST /signup -> 303 See Other (444.1683ms)
POST /notes -> 303 See Other (10.2683ms)
POST /notes -> 303 See Other (8.7761ms)
1. 削除の前の id: [1, 2]
POST /notes/1/delete -> 303 See Other (7.6138ms)
2. POST /notes/1/delete → 303 See Other（Location: /notes・flash=note_deleted）
   削除のあとの id: [2]
GET /notes -> 200 OK (5.7574ms)
3. 一覧: お知らせ「ノートを削除しました」=true・「牛乳を買う」=false・「卵を買う」=true
GET /notes/1 -> 404 Not Found (4.9156ms)
4. GET /notes/1 → 404 Not Found
POST /notes/1/delete -> 404 Not Found (4.6181ms)
5. もう一度 POST /notes/1/delete → 404 Not Found
test delete_a_note ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.51s
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 消したら、一覧へ移る

- 2の返事は `303 See Other` で、移動先は詳細ではなく **一覧（`/notes`）**でした。消したノートの詳細へ移しても、404 になるだけだからです。お知らせのクッキーは `flash=note_deleted` で、3の一覧に「ノートを削除しました」が出て、`牛乳を買う` が消え、`卵を買う` は残りました。
- 表の id は `[1, 2]` から `[2]` になりました。行そのものが無くなる、物理削除です。

##### 解説2: 消したノートは、もう 404

- 4の詳細は `404 Not Found`、5のもう一度の削除も `404` でした。2回目の `DELETE` は、条件に合う行が無いので、消した行の数が `0` になり、リポジトリが `RepoError::NotFound` を返したからです。
- 「ブラウザの『戻る』で詳細に戻り、もう一度『削除』を押した」ときも、この 404 になります。1回目で、もう消えているので、困ることはありません。

### もう一歩: 実務に近い形

#### 例6-2: 削除を、ボタン以外のやり方で引き起こせないか

「削除」は取り返しのつかない操作なので、利用者が自分のボタンを押したとき**以外**には、動いてはいけません。6つのやり方で試します。ほかのサイトから来たリクエストは、ブラウザが付ける `Origin` や `Sec-Fetch-Site` のヘッダーを、テストで付けて再現します（第8巻 UNIT 01）。

> **▼ このコードがやること（先に日本語で）:** alice と bob を登録します。6つのやり方（RustNote の中のリンク・ほかのサイトのリンク・`Origin` がほかのサイトのフォーム・`Sec-Fetch-Site: cross-site` のフォーム・bob のボタン・alice のボタン）ごとに、alice のノートを1件作り、そのノートの削除の URL へリクエストを送ります。返事の状態コード（405 なら `allow` ヘッダーも）と、ノートが残っているかを表示します。

```rust
//! 第10巻 UNIT 06: 「削除」を、ボタン以外のやり方で引き起こせないかを確かめる。
//! やり方ごとに alice のノートを1件ずつ作り、そのノートを消そうとする。
//! 実行: cargo test --test u06_guard -- --nocapture

mod common;

use common::test_app;
use topcoat::router::{Body, Method, header, request::Request};

/// 1つのやり方: (説明, メソッド, クッキー, 付けるヘッダー)
type Case<'a> = (&'a str, Method, &'a str, &'a [(&'a str, &'a str)]);

#[tokio::test]
async fn ways_to_trigger_delete() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let bob = app
        .sign_up("bob@example.com", "correct horse battery")
        .await;

    let cross_site_link = [
        ("sec-fetch-site", "cross-site"),
        ("sec-fetch-mode", "navigate"),
    ];
    let cases: [Case; 6] = [
        ("RustNote の中のリンクを開く", Method::GET, &alice, &[]),
        (
            "ほかのサイトのリンクを開く",
            Method::GET,
            &alice,
            &cross_site_link,
        ),
        (
            "ほかのサイトのフォーム（Origin）",
            Method::POST,
            &alice,
            &[("origin", "https://evil.example")],
        ),
        (
            "ほかのサイトのフォーム（Sec-Fetch-Site）",
            Method::POST,
            &alice,
            &[("sec-fetch-site", "cross-site")],
        ),
        ("bob のボタン", Method::POST, &bob, &[]),
        ("alice のボタン", Method::POST, &alice, &[]),
    ];
    for (n, (what, method, cookie, extra)) in cases.into_iter().enumerate() {
        let url = app.create_note(&alice, &format!("{}件目", n + 1), "").await;
        let mut builder = Request::builder()
            .method(method.clone())
            .uri(format!("{url}/delete"))
            .header(header::COOKIE, cookie);
        if method == Method::POST {
            builder = builder.header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
        }
        for (name, value) in extra {
            builder = builder.header(*name, *value);
        }
        let response = app
            .router
            .handle(builder.body(Body::empty()).unwrap())
            .await;
        let allow = response
            .headers()
            .get("allow")
            .map(|v| format!("（allow: {}）", v.to_str().unwrap()))
            .unwrap_or_default();
        let exists = app.get(&url, Some(&alice)).await.status().is_success();
        println!(
            "{}. {what}: {method} {url}/delete → {}{allow}／ノートは{}",
            n + 1,
            response.status(),
            if exists {
                "残っている"
            } else {
                "消えた"
            }
        );
    }
}
```

```powershell
cargo test --test u06_guard -- --nocapture
```

```text
1. RustNote の中のリンクを開く: GET /notes/1/delete → 405 Method Not Allowed（allow: POST）／ノートは残っている
2. ほかのサイトのリンクを開く: GET /notes/2/delete → 405 Method Not Allowed（allow: POST）／ノートは残っている
3. ほかのサイトのフォーム（Origin）: POST /notes/3/delete → 403 Forbidden／ノートは残っている
4. ほかのサイトのフォーム（Sec-Fetch-Site）: POST /notes/4/delete → 403 Forbidden／ノートは残っている
5. bob のボタン: POST /notes/5/delete → 404 Not Found／ノートは残っている
6. alice のボタン: POST /notes/6/delete → 303 See Other／ノートは消えた
```

（結果の行だけを抜き出しています）

- 消えたのは、6の alice のボタンだけでした。
- 1と2の **GET** は `405 Method Not Allowed`（`allow: POST`）です。`#[page(POST)]` のページは、POST しか受け付けません。リンクを開いただけ・ブラウザが先読みしただけ・検索サイトの巡回が来ただけ、では消えません。
- 3と4の、ほかのサイトからの **POST** は `403 Forbidden` です。Topcoat の OriginPolicy が、ルーターの入口で止めました。alice のクッキーが付いていても、止まります（第8巻 UNIT 01・06（準備中））。
- 5の **bob** は `404` です。SQL の `AND user_id = ?2` で、消せる行が 0 行だったからです（UNIT 03 の問3）。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: 削除を GET のリンクにした

「フォームを書くより、リンクのほうが簡単だ」と考えて、削除のページを `#[page]`（GET）にし、詳細の画面の削除のボタンを `<a class="button danger" href=…>"削除"</a>` のリンクにした場合です（練習用のコピーの `src/routes/notes/id/delete.rs` と `src/routes/notes/id.rs` を書き換え、確かめたら元に戻します）。例6-2のテストを動かします。

```text
1. RustNote の中のリンクを開く: GET /notes/1/delete → 303 See Other／ノートは消えた
2. ほかのサイトのリンクを開く: GET /notes/2/delete → 303 See Other／ノートは消えた
3. ほかのサイトのフォーム（Origin）: POST /notes/3/delete → 403 Forbidden／ノートは残っている
4. ほかのサイトのフォーム（Sec-Fetch-Site）: POST /notes/4/delete → 403 Forbidden／ノートは残っている
5. bob のボタン: POST /notes/5/delete → 405 Method Not Allowed（allow: GET, HEAD）／ノートは残っている
6. alice のボタン: POST /notes/6/delete → 405 Method Not Allowed（allow: GET, HEAD）／ノートは残っている
```

- **何が起きたか:** 画面の見た目は変わらず、リンクを押せば消せます。ところが、1と2の **GET で、ノートが消えました**。2は、ほかのサイトに置かれたリンクを、alice が開いた場合です。ブラウザは、ほかのサイトのリンクから開いたときも、画面ごと移る GET なら、`SameSite=Lax` のクッキーを送ります。OriginPolicy は、GET を止めません。GET は「読むだけ」の約束だからです。
- 逆に、5と6のボタン（POST）は `405`（`allow: GET, HEAD`）で、消せなくなりました。
- **困ること:** 「`/notes/1/delete` を開いてね」というリンクを送られて、開いただけで消えます。ブラウザの先読みや、リンクを確かめる道具が開いても消えます。DB を変える操作は、必ず POST（などの GET 以外）にします。

#### エラー2（エラーが出ない間違い）: 消した行の数を見なかった

リポジトリの `delete` から `if deleted == 0 { return Err(RepoError::NotFound); }` を消した場合です。例6-2のテストを動かします。

```text
1. RustNote の中のリンクを開く: GET /notes/1/delete → 405 Method Not Allowed（allow: POST）／ノートは残っている
2. ほかのサイトのリンクを開く: GET /notes/2/delete → 405 Method Not Allowed（allow: POST）／ノートは残っている
3. ほかのサイトのフォーム（Origin）: POST /notes/3/delete → 403 Forbidden／ノートは残っている
4. ほかのサイトのフォーム（Sec-Fetch-Site）: POST /notes/4/delete → 403 Forbidden／ノートは残っている
5. bob のボタン: POST /notes/5/delete → 303 See Other／ノートは残っている
6. alice のボタン: POST /notes/6/delete → 303 See Other／ノートは消えた
```

- **何が起きたか:** 5の bob のボタンが `303 See Other` になりました。ノートは**残っています**（SQL の持ち主の条件は残っているので、0 行の削除です）。それでも、`delete` が `Ok(())` を返すので、画面は一覧へ移り、コードのとおり「ノートを削除しました」のお知らせまで予約されます。
- **困ること:** 消していないのに「削除しました」と言うのは、嘘のお知らせです。自分のノートを、別のタブで先に消していた場合も、2回目に「削除しました」と出て、何が起きたのか分からなくなります。UNIT 05 の切り替えで `RETURNING` を見ていたのと同じく、**「何行変わったか」を確かめて**、0 行なら 404 にします。

#### エラー3: #[page(DELETE)] にした（405）

「削除なら、HTTP の DELETE メソッドが正しい」と考えて、`#[page(POST)]` を `#[page(DELETE)]` にした場合です。コンパイルは通ります。例6-1のテストを動かします。

```text
1. 削除の前の id: [1, 2]
2. POST /notes/1/delete → 405 Method Not Allowed（allow: DELETE）
   id: [1, 2]
```

- **何が起きたか:** 削除のボタンの POST が `405 Method Not Allowed`（`allow: DELETE`）になり、消せなくなりました。
- **理由:** HTML の `<form>` の `method` に書けるのは、`get` と `post` だけです（`dialog` という特別なものを除く）。`method="delete"` と書いても、ブラウザは GET として送ります。DELETE を送れるのは、JavaScript の `fetch` などです。JavaScript を使わずに、フォームだけで動く RustNote では、削除は POST にして、URL の最後を `/delete` にして区別します。

### RustNote ではここで使う

- 削除は `src/routes/notes/id/delete.rs` と、リポジトリの `delete` です。詳細の画面の「削除」は、`<form method="post">` の中の `<button class="danger">` です（UNIT 03 の例3-2）。赤い「危険」のボタンの見た目は、第9巻 UNIT 09 の色の決まりです。
- **確かめの画面はありません。** 詳細の画面で「削除」を押すと、すぐに消えます。押し間違いを防ぐには、`GET /notes/{id}/delete` で「本当に削除しますか」の画面を出し、そこに POST のボタンを置く、という作り方があります。元に戻せるようにするなら、論理削除かごみ箱です。どちらも、RustNote に残る改善点です（RustNote のコードは変えていません）。
- ユーザーを消したときは、`REFERENCES users(id) ON DELETE CASCADE` で、そのユーザーのノートとセッションも一緒に消えます（問1）。

### 練習問題

**問1:** alice がノートを3件、bob が1件持っているときに、DB の `users` 表から alice の行を消すと、`notes` 表と `sessions` 表は、どうなるでしょうか。alice のクッキーは、どうなるでしょうか。

**問2:** RustNote を論理削除（`deleted_at` の列）に変えるとします。`src/repository/notes.rs` の SQL のうち、直さなければならないのはどれでしょうか。

**問3:** 確かめの画面（「本当に削除しますか」）と、元に戻す仕組み（削除のあとに「元に戻す」のボタン）は、どちらも押し間違いへの守りです。それぞれ、どんな場面に向いているでしょうか。

### 解答と解説

**問1の解答:**

```rust
//! 第10巻 UNIT 06 練習問題1: ユーザーを消すと、そのユーザーのノートとセッションは、どうなるか。
//! 実行: cargo test --test u06_q -- --nocapture

mod common;

use common::{TestApp, test_app};

/// 表ごとの行の数を、"notes=3 sessions=2" の形にする
async fn counts(app: &TestApp) -> String {
    let conn = app.db.db.connect().await.unwrap();
    let mut parts = Vec::new();
    for sql in [
        "SELECT COUNT(*) FROM users",
        "SELECT COUNT(*) FROM notes",
        "SELECT COUNT(*) FROM notes WHERE user_id = 2",
        "SELECT COUNT(*) FROM sessions",
    ] {
        let mut rows = conn.query(sql, ()).await.unwrap();
        let n: i64 = rows.next().await.unwrap().unwrap().get(0).unwrap();
        parts.push(n.to_string());
    }
    format!(
        "users={} notes={}（うち bob の分 {}） sessions={}",
        parts[0], parts[1], parts[2], parts[3]
    )
}

#[tokio::test]
async fn q1_delete_a_user() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let bob = app
        .sign_up("bob@example.com", "correct horse battery")
        .await;
    for title in ["1件目", "2件目", "3件目"] {
        app.create_note(&alice, title, "").await;
    }
    app.create_note(&bob, "bob の1件目", "").await;
    println!("問1 消す前:     {}", counts(&app).await);
    let conn = app.db.db.connect().await.unwrap();
    let deleted = conn
        .execute("DELETE FROM users WHERE email = 'alice@example.com'", ())
        .await
        .unwrap();
    println!("問1 DELETE FROM users … → {deleted} 行");
    println!("問1 alice を消したあと: {}", counts(&app).await);
    let status = app.get("/notes", Some(&alice)).await.status();
    println!("問1 alice のクッキーで GET /notes → {status}");
}
```

```powershell
cargo test --test u06_q -- --nocapture
```

```text
問1 消す前:     users=2 notes=4（うち bob の分 1） sessions=2
問1 DELETE FROM users … → 1 行
問1 alice を消したあと: users=1 notes=1（うち bob の分 1） sessions=1
問1 alice のクッキーで GET /notes → 303 See Other
```

- `users` の行を1つ消しただけで、`notes` は 4 件から 1 件に、`sessions` は 2 件から 1 件になりました。残ったのは、bob のノート1件と、bob のセッション1件です。`notes` と `sessions` の `user_id` に付けた `ON DELETE CASCADE`（「親の行が消えたら、子の行も消す」）が働いたからです（第7巻 UNIT 01・03）。
- alice のクッキーで一覧を開くと `303 See Other`（ログイン画面へ）でした。セッションの行が消えたので、クッキーのトークンに合う記録が、もうありません。
- RustNote には、ユーザーが自分で退会する画面はありません。作るなら、この `DELETE FROM users` 1文で、そのユーザーのデータが全部消えます。

**問2の解答:**

```text
### Select-String -Path src\repository\notes.rs -Pattern "FROM notes|UPDATE notes|INSERT INTO notes"
src\repository\notes.rs:62:            "INSERT INTO notes (user_id, title, body) VALUES (?1, ?2, ?3) RETURNING {NOTE_COLUMNS}"
src\repository\notes.rs:79:        let sql = format!("SELECT {NOTE_COLUMNS} FROM notes WHERE id = ?1 AND user_id = ?2");
src\repository\notes.rs:100:        let count_sql = format!("SELECT COUNT(*) FROM notes WHERE {condition}");
src\repository\notes.rs:111:            "SELECT {NOTE_COLUMNS} FROM notes WHERE {condition}
src\repository\notes.rs:144:            "UPDATE notes
src\repository\notes.rs:175:            "UPDATE notes
src\repository\notes.rs:193:                "DELETE FROM notes WHERE id = ?1 AND user_id = ?2",
```

- `notes` を読む SQL は、`find`（79 行目）・件数（100 行目）・一覧（111 行目）の3つです。この3つ全部の `WHERE` に `AND deleted_at IS NULL` を足さなければなりません。1つでも忘れると、消したはずのノートが、詳細に出たり、件数にだけ数えられたりします（UNIT 01 の、件数の SQL にだけ持ち主の条件を忘れた事故と同じ形です）。
- 書く SQL も変わります。`delete`（193 行目）は `UPDATE notes SET deleted_at = datetime('now') WHERE …` に、編集と切り替えの `UPDATE`（144・175 行目）にも `AND deleted_at IS NULL` が要ります。消したノートを直せてしまわないようにです。
- 一覧と件数は、`condition` という1つの文字列を使い回しているので、そこに1回足せば両方に効きます。条件を1か所にまとめておくと、こういう変更が楽になります。

**問3の解答:**

- **確かめの画面**は、「消す前に、もう一度考えてもらう」守りです。押す回数が1回増えるので、めったにしない、取り返しのつかない操作（退会や、全部のノートを消すなど）に向いています。毎回出すと、読まずに「はい」を押す癖がつきます。
- **元に戻す仕組み**は、「消したあとで、気づいたら戻せる」守りです。消す操作はいつも1回で済み、間違えたときだけ戻します。よく使う操作（ノート1件の削除など）に向いていますが、論理削除かごみ箱が必要で、問2の SQL の変更が付いてきます。
- RustNote は、どちらも持っていません。ノート1件の削除には、元に戻す仕組みのほうが使いやすいでしょう。

### まとめ

- 削除は `#[page(POST)]` と `DELETE … WHERE id = ?1 AND user_id = ?2` の1文で、消した行の数が 0 なら 404、消せたら 303 で一覧へ移り、お知らせを出す。
- GET のリンクは 405、ほかのサイトからの POST は OriginPolicy の 403、他人のボタンは 404 で、消えるのは本人のボタンだけ。削除を GET にすると、ほかのサイトのリンクを開いただけで消える。行の数を見ないと、消していないのに「削除しました」と言う。HTML のフォームは DELETE を送れない。
- 消し方には、物理削除・論理削除・ごみ箱がある。RustNote は物理削除で、確かめの画面も元に戻す仕組みも無い（改善点）。ユーザーを消すと、`ON DELETE CASCADE` でノートとセッションも消える。
