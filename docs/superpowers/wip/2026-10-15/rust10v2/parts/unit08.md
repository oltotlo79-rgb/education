## UNIT 08: ページ送り・OFFSET と cursor

### この単元のゴール

- 一覧のページ送りが、`LIMIT 20 OFFSET (ページ − 1) × 20` で1ページ分を読み、件数からページの数を `div_ceil`（切り上げ）で出し、「前へ」「次へ」のリンクにキーワードを引き継ぐことを、テストで確かめられる
- 1ページ目を見ている間にノートが増える・減ると、OFFSET の2ページ目で、**同じノートが2回出る・どこにも出ない**ノートができることを、実際に確かめて説明できる
- cursor（1ページ目の最後の行の `updated_at` と `id` を「続きの印」にする方法）なら、それが起きないことと、深いページでも速いこと（20 万件で約 35 ミリ秒と約 0.1 ミリ秒）を、数で説明できる
- OFFSET の計算・ページの数の計算・キーワードの引き継ぎを間違えたときの様子を読める

### たとえ話でつかむ

レストランの順番待ちの名簿を考えます。名簿には、新しく来た人が**いちばん上**に書き足されます。

- 店員さんが「上から 20 人目まで」を呼び終えて、次に「**21 人目から** 20 人」を呼ぼうとします。ところが、その間に1人来て、いちばん上に書き足されました。上から 21 人目は、さっき 20 人目として呼んだ人です。**同じ人を2回呼んで**しまいます。逆に、1人が帰って名前が消えると、上から数えた位置が1つずつずれて、**1人を呼び飛ばし**ます。
- 別の店員さんは、「さっき最後に呼んだのは**山田さん（14 時 05 分に来た人）**。だから、**山田さんより前に来た人**から 20 人」と呼びます。上に何人増えても減っても、山田さんの位置から続けるので、重なりも抜けもありません。
- 同じ 14 時 05 分に、2人の山田さんが来ていたら、時刻だけでは区別できません。だから、受付の番号も一緒に覚えておきます。

| 順番待ちの名簿 | RustNote の一覧 |
|---|---|
| 上から 21 人目から 20 人 | `LIMIT 20 OFFSET 20`（RustNote の今の作り） |
| 来た人・帰った人で、位置がずれる | 見ている間の作成・削除で、2回出る・抜ける（例8-2） |
| 「山田さんより前に来た人から」 | cursor：`WHERE (updated_at, id) < (?1, ?2)` |
| 時刻と、受付の番号の両方 | 続きの印は `updated_at` と `id` の2つ（問3） |

> **用語:** OFFSET（SQL で、並べた結果の先頭から何行を飛ばすかの指定。`LIMIT 20 OFFSET 40` は「41 行目から 20 行」。第7巻 UNIT 02）

<!-- -->

> **用語:** cursor（カーソル。ここでは、ページ送りの「続きの印」。前のページの最後の行の値を覚えておき、「その行より後ろ」を読む。keyset pagination とも呼ぶ）

<!-- -->

> **用語:** 行の値の比べ（`(updated_at, id) < (?1, ?2)` のように、2つの列を組にして比べる SQL の書き方。まず `updated_at` で比べ、同じなら `id` で比べる）

### 図で理解する

55 件のノートを、1ページ 20 件で送るときの OFFSET です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="20" y="26" font-size="12" font-weight="700" fill="#0f172a">新しい順に並べた 55 件（updated_at DESC, id DESC）</text>
<rect x="20" y="40" width="270" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="155" y="65" font-size="12" fill="#1e40af" text-anchor="middle">1〜20 件目</text>
<rect x="296" y="40" width="270" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="431" y="65" font-size="12" fill="#047857" text-anchor="middle">21〜40 件目</text>
<rect x="572" y="40" width="128" height="40" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="636" y="65" font-size="12" fill="#c2410c" text-anchor="middle">41〜55 件目</text>
<text x="155" y="108" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">?page=1</text>
<text x="155" y="128" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">LIMIT 20 OFFSET 0</text>
<text x="431" y="108" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">?page=2</text>
<text x="431" y="128" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">LIMIT 20 OFFSET 20</text>
<text x="636" y="108" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">?page=3</text>
<text x="636" y="128" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">OFFSET 40</text>
<rect x="20" y="150" width="680" height="66" rx="8" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="40" y="174" font-size="12" fill="#0f172a">OFFSET = (ページ − 1) × 20　（NoteQuery::offset）</text>
<text x="40" y="198" font-size="12" fill="#0f172a">ページの数 = 55 ÷ 20 を切り上げ = 3　（NotePage::total_pages の div_ceil。0 件でも 1）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: OFFSET のページ送り。ページの番号から飛ばす行の数を計算し、件数を 20 で割って切り上げた数を、ページの数にする</div>
</div>

1ページ目を見ている間に、新しいノートが1件できたときの、2ページ目の読み方の違いです（はじめは 25 件）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="180" y="22" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">OFFSET 20（上から 21 行目から）</text>
<rect x="20" y="34" width="320" height="214" rx="10" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="40" y="62" font-size="12" fill="#047857">＋ 新しいノート（あとから作った）</text>
<text x="40" y="86" font-size="12" fill="#0f172a">ノート25 〜 ノート07（19 件）</text>
<line x1="40" y1="100" x2="320" y2="100" stroke="#b91c1c" stroke-width="2" stroke-dasharray="5 4"/>
<text x="180" y="116" font-size="11" fill="#b91c1c" text-anchor="middle">ここまでが 20 行（OFFSET 20）</text>
<text x="40" y="142" font-size="12" font-weight="700" fill="#b91c1c">ノート06（1ページ目でも見た）</text>
<text x="40" y="166" font-size="12" fill="#0f172a">ノート05 〜 ノート01</text>
<text x="180" y="200" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">→ ノート06 が2回出る</text>
<text x="180" y="222" font-size="11" fill="#475569" text-anchor="middle">位置で数えるので、上に1件増えると、</text>
<text x="180" y="238" font-size="11" fill="#475569" text-anchor="middle">全部が1つ下にずれる</text>
<text x="540" y="22" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">cursor（ノート06 の後ろから）</text>
<rect x="380" y="34" width="320" height="214" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="400" y="62" font-size="12" fill="#047857">＋ 新しいノート（あとから作った）</text>
<text x="400" y="86" font-size="12" fill="#0f172a">ノート25 〜 ノート07</text>
<text x="400" y="110" font-size="12" font-weight="700" fill="#047857">ノート06 ← 続きの印（updated_at と id）</text>
<line x1="400" y1="122" x2="680" y2="122" stroke="#047857" stroke-width="2" stroke-dasharray="5 4"/>
<text x="400" y="146" font-size="12" fill="#0f172a">ノート05 〜 ノート01</text>
<text x="540" y="200" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">→ 重なりも抜けも無い</text>
<text x="540" y="222" font-size="11" fill="#475569" text-anchor="middle">行の値で「どこから」を決めるので、</text>
<text x="540" y="238" font-size="11" fill="#475569" text-anchor="middle">上で何が増えても、続きの場所は同じ</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: 見ている間に1件増えたときの2ページ目。OFFSET は上から数えた位置なので1つずれて同じノートが2回出る。cursor は前のページの最後の行の値から続けるので、ずれない</div>
</div>

### まず動かす最小コード

#### 例8-1: 55 件を、ページごとに読む

ページ送りの計算は、`src/domain.rs` にあります（抜き出しています）。

```rust
/// SQL の OFFSET（何件読み飛ばすか）
pub fn offset(&self) -> u64 {
    u64::from(self.page - 1) * u64::from(PER_PAGE)
}

/// 全部で何ページあるか（0 件でも 1 ページと数える）
pub fn total_pages(&self) -> u32 {
    let pages = self.total_count.div_ceil(u64::from(PER_PAGE)).max(1);
    u32::try_from(pages).unwrap_or(u32::MAX)
}
```

「前へ」「次へ」のリンクは、`src/views.rs` の `pagination` が作ります。キーワードがあれば、`q` も付けます。

```rust
let link = |page_number: u32| {
    let mut params: Vec<(&str, String)> = Vec::new();
    if let Some(keyword) = &query.keyword {
        params.push(("q", keyword.clone()));
    }
    params.push(("page", page_number.to_string()));
    href!(crate::routes::notes::index).query(params)
};
```

> **▼ このコードがやること（先に日本語で）:** alice が、先に「メモ01〜10」の 10 件、あとから「ノート01〜45」の 45 件を作ります。1・2・3ページ目と、キーワード「ノート」で絞った2ページ目を開き、ページごとの件数と最初・最後のタイトル、ページの行と「前へ」「次へ」のリンク先を表示します。最後に、絞った2ページ目の「次へ」のリンクを、そのままたどります。

```rust
//! 第10巻 UNIT 08: 55 件のノート（メモ 10 件と、あとから作ったノート 45 件）を、1ページ 20 件で送り、
//! ページごとの中身とリンクを確かめる。キーワードで絞ったときの「次へ」も、たどってみる。
//! 実行: cargo test --test u08_pages -- --nocapture

mod common;

use common::{body_text, test_app};

/// 一覧の HTML から、ノートのタイトルを、上から順に読む
fn titles(html: &str) -> Vec<String> {
    html.split("<li class=\"note-item")
        .skip(1)
        .filter_map(|item| item.split("<a href=\"/notes/").nth(1))
        .filter_map(|t| t.split_once('>'))
        .map(|(_, rest)| rest.split('<').next().unwrap().to_string())
        .collect()
}

/// ページ送りの部分（<nav class="pagination">）から、ページの行と、前へ・次へのリンク先を読む
fn pagination(html: &str) -> String {
    let nav = html
        .split("<nav class=\"pagination\">")
        .nth(1)
        .and_then(|t| t.split("</nav>").next())
        .unwrap_or("");
    let line = nav
        .split("<span>")
        .nth(1)
        .and_then(|t| t.split("</span>").next())
        .unwrap_or("");
    let link = |label: &str| {
        nav.split("<a href=\"")
            .skip(1)
            .find(|t| t.contains(label))
            .and_then(|t| t.split('"').next())
            .map(|href| href.replace("&amp;", "&"))
            .unwrap_or_else(|| "（なし）".to_string())
    };
    format!("{line}　前へ: {}　次へ: {}", link("前へ"), link("次へ"))
}

#[tokio::test]
async fn forty_five_notes() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    for n in 1..=10 {
        app.create_note(&alice, &format!("メモ{n:02}"), "").await;
    }
    for n in 1..=45 {
        app.create_note(&alice, &format!("ノート{n:02}"), "").await;
    }
    for url in [
        "/notes",
        "/notes?page=2",
        "/notes?page=3",
        "/notes?q=ノート&page=2",
    ] {
        let html = body_text(app.get(url, Some(&alice)).await).await;
        let found = titles(&html);
        println!(
            "GET {url}: {} 件（{} 〜 {}）",
            found.len(),
            found.first().map(String::as_str).unwrap_or("-"),
            found.last().map(String::as_str).unwrap_or("-")
        );
        println!("   {}", pagination(&html));
    }

    // 「ノート」で絞った2ページ目の「次へ」を、そのままたどる
    let html = body_text(app.get("/notes?q=ノート&page=2", Some(&alice)).await).await;
    let next = pagination(&html)
        .split("次へ: ")
        .nth(1)
        .unwrap()
        .to_string();
    let html = body_text(app.get(&next, Some(&alice)).await).await;
    let found = titles(&html);
    println!(
        "「次へ」をたどる GET {next}: {} 件（{} 〜 {}）",
        found.len(),
        found.first().map(String::as_str).unwrap_or("-"),
        found.last().map(String::as_str).unwrap_or("-")
    );
}
```

```powershell
cargo test --test u08_pages -- --nocapture
```

```text
GET /notes: 20 件（ノート45 〜 ノート26）
   1 / 3 ページ（全55件）　前へ: （なし）　次へ: /notes?page=2
GET /notes?page=2: 20 件（ノート25 〜 ノート06）
   2 / 3 ページ（全55件）　前へ: /notes?page=1　次へ: /notes?page=3
GET /notes?page=3: 15 件（ノート05 〜 メモ01）
   3 / 3 ページ（全55件）　前へ: /notes?page=2　次へ: （なし）
GET /notes?q=ノート&page=2: 20 件（ノート25 〜 ノート06）
   2 / 3 ページ（全45件）　前へ: /notes?q=%E3%83%8E%E3%83%BC%E3%83%88&page=1　次へ: /notes?q=%E3%83%8E%E3%83%BC%E3%83%88&page=3
「次へ」をたどる GET /notes?q=%E3%83%8E%E3%83%BC%E3%83%88&page=3: 5 件（ノート05 〜 ノート01）
```

（ページごとの結果の行だけを抜き出しています）

#### ▼ コードを1つずつ分解して解説

##### 解説1: 20 件・20 件・15 件と、新しい順に送られる

- 1ページ目はノート45〜26、2ページ目はノート25〜06、3ページ目はノート05〜メモ01 の 15 件でした。並びは `updated_at DESC, id DESC`（UNIT 01）で、同じ秒に作ったノートは id の大きい順です。
- ページの行は `1 / 3 ページ（全55件）` です。55 ÷ 20 は 2.75 なので、切り上げて 3 ページです（`div_ceil`）。
- 1ページ目には「前へ」が、3ページ目には「次へ」がありません。`has_prev`（ページが 1 より大きい）と `has_next`（ページがページの数より小さい）で決めています。

##### 解説2: 絞り込みは、ページを送っても続く

- キーワード「ノート」で絞った2ページ目は `2 / 3 ページ（全45件）` で、リンクは `/notes?q=%E3%83%8E%E3%83%BC%E3%83%88&page=3` のように、`q` を引き継いでいます（`%E3…` は「ノート」を URL の中で書いた形）。
- 「次へ」をたどった3ページ目は、ノート05〜01 の 5 件で、メモは出ません。もし `q` が抜けていたら、絞り込みが外れて、メモが混ざります（エラー3）。

### もう一歩: 実務に近い形

#### 例8-2: 見ている間に増えた・減ったときと、cursor

> **▼ このコードがやること（先に日本語で）:** 25 件のノートを作って、3つのテストをします。1つ目は、1ページ目を読んだあとで新しいノートを1件作り、2ページ目を読みます。2つ目は、1ページ目を読んだあとで、1ページ目に出ていた「ノート20」を削除し、2ページ目を読みます。どちらも、1ページ目と2ページ目に2回出たノートと、どちらにも出なかったノートを表示します。3つ目は、RustNote の一覧ではなく、SQL を直接動かし、1ページ目の最後の行の `updated_at` と `id` を続きの印にして、新しいノートを作ったあとで、その印の後ろを読みます。

```rust
//! 第10巻 UNIT 08: 1ページ目を見ている間にノートが増えたり減ったりすると、
//! OFFSET のページ送りで、2ページ目に何が起きるかを確かめる。最後に、cursor（続きの印）の形と比べる。
//! 実行: cargo test --test u08_shift -- --nocapture --test-threads=1

mod common;

use common::{TestApp, body_text, test_app};

const PASSWORD: &str = "correct horse battery";

/// 一覧の HTML から、ノートのタイトルを、上から順に読む
fn titles(html: &str) -> Vec<String> {
    html.split("<li class=\"note-item")
        .skip(1)
        .filter_map(|item| item.split("<a href=\"/notes/").nth(1))
        .filter_map(|t| t.split_once('>'))
        .map(|(_, rest)| rest.split('<').next().unwrap().to_string())
        .collect()
}

async fn page(app: &TestApp, cookie: &str, n: u32) -> Vec<String> {
    let html = body_text(app.get(&format!("/notes?page={n}"), Some(cookie)).await).await;
    titles(&html)
}

/// 1ページ目と2ページ目を比べて、重なった物と、どちらにも出なかった物を数える
fn compare(all: &[String], page1: &[String], page2: &[String]) {
    let twice: Vec<_> = page2.iter().filter(|t| page1.contains(t)).collect();
    let missing: Vec<_> = all
        .iter()
        .filter(|t| !page1.contains(t) && !page2.contains(t))
        .collect();
    println!("   2ページ目: {page2:?}");
    println!("   2回出た: {twice:?}　どちらにも出なかった: {missing:?}");
}

async fn setup() -> (TestApp, String) {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    for n in 1..=25 {
        app.create_note(&alice, &format!("ノート{n:02}"), "").await;
    }
    (app, alice)
}

/// 見ている間に、1件増えた
#[tokio::test]
async fn a_note_is_added() {
    let (app, alice) = setup().await;
    let page1 = page(&app, &alice, 1).await;
    println!("1. 1ページ目: {} 〜 {}", page1[0], page1[19]);
    app.create_note(&alice, "新しいノート", "").await;
    println!("2. 「新しいノート」を作った");
    let page2 = page(&app, &alice, 2).await;
    // 比べるのは、1ページ目を見たときにあった 25 件
    let all: Vec<String> = (1..=25).rev().map(|n| format!("ノート{n:02}")).collect();
    println!("3. 2ページ目を開く");
    compare(&all, &page1, &page2);
}

/// 見ている間に、1ページ目の1件が消えた
#[tokio::test]
async fn a_note_is_deleted() {
    let (app, alice) = setup().await;
    let page1 = page(&app, &alice, 1).await;
    println!("1. 1ページ目: {} 〜 {}", page1[0], page1[19]);
    // ノート20 は id=20。1ページ目に出ている
    app.post_form("/notes/20/delete", "", Some(&alice)).await;
    println!("2. 「ノート20」を削除した");
    let page2 = page(&app, &alice, 2).await;
    let all: Vec<String> = (1..=25)
        .rev()
        .filter(|n| *n != 20)
        .map(|n| format!("ノート{n:02}"))
        .collect();
    println!("3. 2ページ目を開く");
    compare(&all, &page1, &page2);
}

/// 同じことを、cursor（1ページ目の最後の行の updated_at と id）で続きを読む SQL で行う
#[tokio::test]
async fn cursor_instead_of_offset() {
    let (app, alice) = setup().await;
    let conn = app.db.db.connect().await.unwrap();
    let first = "SELECT title, updated_at, id FROM notes WHERE user_id = 1
                 ORDER BY updated_at DESC, id DESC LIMIT 20";
    let mut rows = conn.query(first, ()).await.unwrap();
    let (mut page1, mut last) = (Vec::new(), (String::new(), 0i64));
    while let Some(row) = rows.next().await.unwrap() {
        page1.push(row.get::<String>(0).unwrap());
        last = (row.get::<String>(1).unwrap(), row.get::<i64>(2).unwrap());
    }
    println!(
        "1. 1ページ目: {} 〜 {}（続きの印: updated_at={} id={}）",
        page1[0], page1[19], last.0, last.1
    );
    app.create_note(&alice, "新しいノート", "").await;
    println!("2. 「新しいノート」を作った");
    let next = "SELECT title FROM notes WHERE user_id = 1 AND (updated_at, id) < (?1, ?2)
                ORDER BY updated_at DESC, id DESC LIMIT 20";
    let mut rows = conn.query(next, (last.0.as_str(), last.1)).await.unwrap();
    let mut page2 = Vec::new();
    while let Some(row) = rows.next().await.unwrap() {
        page2.push(row.get::<String>(0).unwrap());
    }
    let all: Vec<String> = (1..=25).rev().map(|n| format!("ノート{n:02}")).collect();
    println!("3. 続きの印のあとを読む");
    compare(&all, &page1, &page2);
}
```

```powershell
cargo test --test u08_shift -- --nocapture --test-threads=1
```

```text
1. 1ページ目: ノート25 〜 ノート06
2. 「新しいノート」を作った
3. 2ページ目を開く
   2ページ目: ["ノート06", "ノート05", "ノート04", "ノート03", "ノート02", "ノート01"]
   2回出た: ["ノート06"]　どちらにも出なかった: []
1. 1ページ目: ノート25 〜 ノート06
2. 「ノート20」を削除した
3. 2ページ目を開く
   2ページ目: ["ノート04", "ノート03", "ノート02", "ノート01"]
   2回出た: []　どちらにも出なかった: ["ノート05"]
1. 1ページ目: ノート25 〜 ノート06（続きの印: updated_at=2026-10-30 14:30:45 id=6）
2. 「新しいノート」を作った
3. 続きの印のあとを読む
   2ページ目: ["ノート05", "ノート04", "ノート03", "ノート02", "ノート01"]
   2回出た: []　どちらにも出なかった: []
```

（結果の行だけを抜き出しています）

- **増えたとき:** 2ページ目の先頭に、1ページ目の最後に見た `ノート06` が、もう一度出ました。新しいノートがいちばん上に入ったので、全部が1つ下にずれ、「上から 21 行目」が `ノート06` になったからです。
- **減ったとき:** 1ページ目の `ノート20` を消すと、全部が1つ上にずれ、`ノート05` が1ページ目の範囲に上がりました。でも1ページ目はもう読み終わっているので、`ノート05` は**どちらのページにも出ません**でした。
- **cursor のとき:** 続きの印（`ノート06` の `updated_at` と `id=6`）の後ろを読むと、`ノート05〜01` がきちんと続き、重なりも抜けもありませんでした。新しいノートは、印より前（新しい側）に入るので、2ページ目には影響しません。
- 新しいノートは、どの読み方でも、1ページ目に戻ったときに見えます。困るのは、**今読んでいる続き**が、ずれることです。

#### 例8-3: 深いページの速さ

もう1つの違いは、深いページ（たとえば 1 万ページ目）の速さです。20 万件のノートで、最後のページを読む時間を比べます。

> **▼ このコードがやること（先に日本語で）:** SQL で 20 万件のノートを一度に作ります（1秒ずつ違う `updated_at`）。最後のページ（下から 20 件）を、`OFFSET 199980` で読む SQL と、「ノート21」を続きの印にする cursor の SQL で読み、まず `EXPLAIN QUERY PLAN`（DB がどう読むつもりか。第7巻 UNIT 04）を表示します。次に、それぞれを3回ずつ動かして、かかった時間と、読めたタイトルを表示します。

```rust
//! 第10巻 UNIT 08: ノートが 20 万件あるときに、最後のページを OFFSET で読むのと、
//! cursor（続きの印）で読むのとで、DB の読み方と時間を比べる。
//! 実行: cargo test --test u08_deep -- --nocapture

mod common;

use std::time::Instant;

use common::test_app;

const ROWS: i64 = 200_000;

#[tokio::test]
async fn deep_page() {
    let app = test_app().await;
    app.sign_up("alice@example.com", "correct horse battery")
        .await;
    let conn = app.db.db.connect().await.unwrap();

    // 20 万件を SQL で一度に作る（1秒ずつ違う updated_at）
    let started = Instant::now();
    conn.execute(
        "INSERT INTO notes (user_id, title, updated_at)
         WITH RECURSIVE n(i) AS (SELECT 1 UNION ALL SELECT i + 1 FROM n WHERE i < ?1)
         SELECT 1, 'ノート' || i, datetime('2026-01-01', '+' || i || ' seconds') FROM n",
        [ROWS],
    )
    .await
    .unwrap();
    println!("{ROWS} 件を作った（{:.2?}）", started.elapsed());

    let offset_sql = "SELECT title FROM notes WHERE user_id = 1
                      ORDER BY updated_at DESC, id DESC LIMIT 20 OFFSET ?1";
    // 最後から2番目のページの、最後の行を続きの印にする（ノート21）
    let cursor_sql = "SELECT title FROM notes WHERE user_id = 1 AND (updated_at, id) < (?1, ?2)
                      ORDER BY updated_at DESC, id DESC LIMIT 20";
    let mut rows = conn
        .query(
            "SELECT updated_at, id FROM notes WHERE title = 'ノート21'",
            (),
        )
        .await
        .unwrap();
    let row = rows.next().await.unwrap().unwrap();
    let cursor: (String, i64) = (row.get(0).unwrap(), row.get(1).unwrap());

    for (name, sql) in [("OFFSET", offset_sql), ("cursor", cursor_sql)] {
        let mut plan = conn
            .query(&format!("EXPLAIN QUERY PLAN {sql}"), ())
            .await
            .unwrap();
        while let Some(row) = plan.next().await.unwrap() {
            println!("{name} の読み方: {}", row.get::<String>(3).unwrap());
        }
    }
    for round in 1..=3 {
        let started = Instant::now();
        let mut rows = conn.query(offset_sql, [ROWS - 20]).await.unwrap();
        let mut offset_titles = Vec::new();
        while let Some(row) = rows.next().await.unwrap() {
            offset_titles.push(row.get::<String>(0).unwrap());
        }
        let offset_time = started.elapsed();

        let started = Instant::now();
        let mut rows = conn
            .query(cursor_sql, (cursor.0.as_str(), cursor.1))
            .await
            .unwrap();
        let mut cursor_titles = Vec::new();
        while let Some(row) = rows.next().await.unwrap() {
            cursor_titles.push(row.get::<String>(0).unwrap());
        }
        let cursor_time = started.elapsed();
        println!(
            "{round} 回目: OFFSET {:>9.2?}（{} 〜 {}）／ cursor {:>9.2?}（{} 〜 {}）",
            offset_time,
            offset_titles[0],
            offset_titles[19],
            cursor_time,
            cursor_titles[0],
            cursor_titles[19]
        );
    }
}
```

```powershell
cargo test --test u08_deep -- --nocapture
```

```text
200000 件を作った（1.07s）
OFFSET の読み方: SEARCH notes USING INDEX idx_notes_user_updated (user_id=?)
cursor の読み方: SEARCH notes USING INDEX idx_notes_user_updated (user_id=? AND updated_at<?)
1 回目: OFFSET   33.91ms（ノート20 〜 ノート1）／ cursor  148.90µs（ノート20 〜 ノート1）
2 回目: OFFSET   35.32ms（ノート20 〜 ノート1）／ cursor  132.20µs（ノート20 〜 ノート1）
3 回目: OFFSET   33.24ms（ノート20 〜 ノート1）／ cursor  134.10µs（ノート20 〜 ノート1）
```

（結果の行だけを抜き出しています。時間は、この PC で、debug のテストとして測ったものです）

- どちらも `idx_notes_user_updated` の索引を使い、同じ `ノート20〜ノート1` を読みました。違うのは時間で、OFFSET は約 **35 ミリ秒**、cursor は約 **0.1 ミリ秒**でした。
- OFFSET は、索引をたどって **199,980 行を数えながら読み飛ばして**から、20 行を返します。cursor は、索引の中で「続きの印より後ろ」の位置へ直接飛び、そこから 20 行だけを読みます（読み方の `updated_at<?` が、その印です）。
- RustNote は、1人のノートが数万件になることは少ないと考えて、ページの番号で送れる OFFSET を選んでいます。「5ページ目へ飛ぶ」ことは OFFSET ならできますが、cursor ではできません。どちらにも、得意なことがあります。

### よくあるエラーと読み方

#### エラー1: OFFSET の計算で、1 を引き忘れた

`offset` を `u64::from(self.page) * u64::from(PER_PAGE)` にした場合です（練習用のコピーの `src/domain.rs` を書き換え、確かめたら元に戻します）。例8-1のテストを動かします。

```text
GET /notes: 20 件（ノート25 〜 ノート06）
   1 / 3 ページ（全55件）　前へ: （なし）　次へ: /notes?page=2
GET /notes?page=2: 15 件（ノート05 〜 メモ01）
   2 / 3 ページ（全55件）　前へ: /notes?page=1　次へ: /notes?page=3
GET /notes?page=3: 0 件（- 〜 -）
   3 / 3 ページ（全55件）　前へ: /notes?page=2　次へ: （なし）
GET /notes?q=ノート&page=2: 5 件（ノート05 〜 ノート01）
   2 / 3 ページ（全45件）　前へ: /notes?q=%E3%83%8E%E3%83%BC%E3%83%88&page=1　次へ: /notes?q=%E3%83%8E%E3%83%BC%E3%83%88&page=3
「次へ」をたどる GET /notes?q=%E3%83%8E%E3%83%BC%E3%83%88&page=3: 0 件（- 〜 -）
```

- **何が起きたか:** 1ページ目に、ノート25〜06 が出ました。**最新の 20 件（ノート45〜26）が、どのページにも出ません**。3ページ目は 0 件です。画面にも状態コードにもエラーは無く、「1 / 3 ページ（全55件）」の行もそれらしく見えます。
- **見つけ方:** RustNote の `cargo test --lib`（`src/` の中のユニットテスト）が、この間違いを見つけます。

```text
test domain::tests::note_title_counts_characters_not_bytes ... ok
test domain::tests::email_rejects_bad_forms ... ok
test domain::tests::note_id_parses_only_positive_numbers ... ok
test domain::tests::email_is_normalized ... ok
test domain::tests::new_note_collects_all_errors ... ok
test domain::tests::note_page_counts_pages ... ok
test domain::tests::note_body_normalizes_newlines_and_limits_length ... ok
test domain::tests::note_title_is_trimmed ... ok
test domain::tests::note_title_rejects_empty_and_spaces ... ok
test domain::tests::password_length_is_checked_and_hidden_in_debug ... ok
test domain::tests::note_query_defaults_and_offset ... FAILED
thread 'domain::tests::note_query_defaults_and_offset' (216476) panicked at src\domain.rs:494:9:
assertion `left == right` failed
  left: 20
test result: FAILED. 18 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

- `domain.rs` の 494 行目 `assert_eq!(query.offset(), 0)`（1ページ目は 0 行飛ばす）で、`left: 20`（実際）と `right: 0`（期待）が食い違いました。計算の決まりを、小さなユニットテストで押さえておくと、画面のテストより先に気づけます（第11巻 UNIT 01）。

#### エラー2: ページの数を、切り捨てで数えた

`total_pages` を `(self.total_count / u64::from(PER_PAGE)).max(1)`（割り算の切り捨て）にした場合です。

```text
GET /notes: 20 件（ノート45 〜 ノート26）
   1 / 2 ページ（全55件）　前へ: （なし）　次へ: /notes?page=2
GET /notes?page=2: 20 件（ノート25 〜 ノート06）
   2 / 2 ページ（全55件）　前へ: /notes?page=1　次へ: （なし）
GET /notes?page=3: 15 件（ノート05 〜 メモ01）
   3 / 2 ページ（全55件）　前へ: /notes?page=2　次へ: （なし）
GET /notes?q=ノート&page=2: 20 件（ノート25 〜 ノート06）
   2 / 2 ページ（全45件）　前へ: /notes?q=%E3%83%8E%E3%83%BC%E3%83%88&page=1　次へ: （なし）
```

- **何が起きたか:** 55 ÷ 20 の切り捨ては 2 なので、`1 / 2 ページ` になり、2ページ目に「次へ」がありません。**最後の 15 件（ノート05〜メモ01）に、リンクでたどり着けません**。URL に `?page=3` と直接書けば出ますが、ページの行は `3 / 2 ページ` という、おかしな表示になります。
- これも、`cargo test --lib` の `note_page_counts_pages` が、513 行目 `assert_eq!(page(21, 1).total_pages(), 2)`（21 件なら 2 ページ）で、`left: 1` と知らせました。「ちょうど割り切れる数（20）」と「1つ多い数（21）」の両方を試すのが、こういう計算の確かめ方です。

#### エラー3（エラーが出ない間違い）: ページ送りのリンクで、q を引き継がなかった

`pagination` から、`if let Some(keyword) = &query.keyword { params.push(("q", keyword.clone())); }` を消した場合です。

```text
GET /notes: 20 件（ノート45 〜 ノート26）
   1 / 3 ページ（全55件）　前へ: （なし）　次へ: /notes?page=2
GET /notes?page=2: 20 件（ノート25 〜 ノート06）
   2 / 3 ページ（全55件）　前へ: /notes?page=1　次へ: /notes?page=3
GET /notes?page=3: 15 件（ノート05 〜 メモ01）
   3 / 3 ページ（全55件）　前へ: /notes?page=2　次へ: （なし）
GET /notes?q=ノート&page=2: 20 件（ノート25 〜 ノート06）
   2 / 3 ページ（全45件）　前へ: /notes?page=1　次へ: /notes?page=3
「次へ」をたどる GET /notes?page=3: 15 件（ノート05 〜 メモ01）
```

- **何が起きたか:** 「ノート」で絞った2ページ目のリンクが `/notes?page=3` になり、たどると、メモが混ざった 15 件が出ました。利用者は、いつの間にか絞り込みが外れたことに、気づきにくいです。
- このときは、`cargo test --lib` は通ります。リンクの組み立ては画面の部品の中にあるので、例8-1のような、**リンクをたどるテスト**でしか見つかりません。

### RustNote ではここで使う

- 1ページの件数は `src/domain.rs` の `PER_PAGE`（20）、ページの番号は `?page=`（`NotesParams` の `page: Option<u32>`）です。
- 一覧の SQL は `src/repository/notes.rs` の `search` で、`ORDER BY updated_at DESC, id DESC LIMIT ?3 OFFSET ?4` です。件数の SQL と同じ `condition`（持ち主とキーワード）を使っています（UNIT 07）。
- 並びに合わせた索引 `idx_notes_user_updated ON notes (user_id, updated_at DESC, id DESC)` が `migrations/0002_notes.sql` にあり、OFFSET でも cursor でも使われます（例8-3）。
- cursor のページ送りは、RustNote にはありません。ノートの数がとても多くなったり、「もっと見る」で続きを読み足す画面にしたりするときの、作り直しの候補です。

### 練習問題

**問1:** 25 件（2ページ分）のときに、`/notes?page=3` を開くと、何が表示されるでしょうか。

**問2:** `/notes?page=4294967295` と `/notes?page=4294967296` は、それぞれどうなるでしょうか。

**問3:** cursor の続きの印を、`updated_at` だけにすると、どうなるでしょうか。25 件を同じ秒に作ったときで確かめてください。

### 解答と解説

```rust
//! 第10巻 UNIT 08 練習問題。
//! 実行: cargo test --test u08_q -- --nocapture --test-threads=1

mod common;

use common::{body_text, test_app};

const PASSWORD: &str = "correct horse battery";

/// 一覧の HTML から、ノートのタイトルを、上から順に読む
fn titles(html: &str) -> Vec<String> {
    html.split("<li class=\"note-item")
        .skip(1)
        .filter_map(|item| item.split("<a href=\"/notes/").nth(1))
        .filter_map(|t| t.split_once('>'))
        .map(|(_, rest)| rest.split('<').next().unwrap().to_string())
        .collect()
}

/// <p class="empty"> の文と、ページの行
fn empty_and_line(html: &str) -> (String, String) {
    let empty = html
        .split("<p class=\"empty\">")
        .nth(1)
        .and_then(|t| t.split("</p>").next())
        .unwrap_or("（なし）")
        .to_string();
    let line = html
        .split("<nav class=\"pagination\">")
        .nth(1)
        .and_then(|t| t.split("<span>").nth(1))
        .and_then(|t| t.split("</span>").next())
        .unwrap_or("（なし）")
        .to_string();
    (empty, line)
}

/// 問1・問2: ページの番号が、ページの数より大きいとき・とても大きいとき
#[tokio::test]
async fn q1_q2_page_out_of_range() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    for n in 1..=25 {
        app.create_note(&alice, &format!("ノート{n:02}"), "").await;
    }
    for url in [
        "/notes?page=3",
        "/notes?page=4294967295",
        "/notes?page=4294967296",
    ] {
        let response = app.get(url, Some(&alice)).await;
        let status = response.status();
        let html = body_text(response).await;
        let (empty, line) = empty_and_line(&html);
        println!(
            "問 GET {url} → {status}・{} 件・{line}・空のときの文: {empty}",
            titles(&html).len()
        );
    }
}

/// 問3: 続きの印を updated_at だけにすると、同じ秒のノートはどうなるか
#[tokio::test]
async fn q3_cursor_without_id() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    for n in 1..=25 {
        app.create_note(&alice, &format!("ノート{n:02}"), "").await;
    }
    let conn = app.db.db.connect().await.unwrap();
    let mut rows = conn
        .query(
            "SELECT COUNT(DISTINCT updated_at) FROM notes WHERE user_id = 1",
            (),
        )
        .await
        .unwrap();
    let distinct: i64 = rows.next().await.unwrap().unwrap().get(0).unwrap();
    println!("問3 25 件の updated_at の種類: {distinct}");

    // 1ページ目の最後の行（ノート06）を続きの印にする
    let mut rows = conn
        .query(
            "SELECT updated_at, id FROM notes WHERE title = 'ノート06'",
            (),
        )
        .await
        .unwrap();
    let row = rows.next().await.unwrap().unwrap();
    let (updated_at, id): (String, i64) = (row.get(0).unwrap(), row.get(1).unwrap());
    for (name, sql) in [
        (
            "updated_at だけ",
            "SELECT title FROM notes WHERE user_id = 1 AND updated_at < ?1
             ORDER BY updated_at DESC, id DESC LIMIT 20",
        ),
        (
            "updated_at と id",
            "SELECT title FROM notes WHERE user_id = 1 AND (updated_at, id) < (?1, ?2)
             ORDER BY updated_at DESC, id DESC LIMIT 20",
        ),
    ] {
        let mut rows = if sql.contains("?2") {
            conn.query(sql, (updated_at.as_str(), id)).await.unwrap()
        } else {
            conn.query(sql, [updated_at.as_str()]).await.unwrap()
        };
        let mut found = Vec::new();
        while let Some(row) = rows.next().await.unwrap() {
            found.push(row.get::<String>(0).unwrap());
        }
        println!("問3 続きの印が {name} → 2ページ目: {found:?}");
    }
}
```

```powershell
cargo test --test u08_q -- --nocapture --test-threads=1
```

```text
問 GET /notes?page=3 → 200 OK・0 件・3 / 2 ページ（全25件）・空のときの文: まだノートがありません。最初の1件を書いてみましょう。
問 GET /notes?page=4294967295 → 200 OK・0 件・4294967295 / 2 ページ（全25件）・空のときの文: まだノートがありません。最初の1件を書いてみましょう。
問 GET /notes?page=4294967296 → 400 Bad Request・0 件・（なし）・空のときの文: （なし）
問3 25 件の updated_at の種類: 1
問3 続きの印が updated_at だけ → 2ページ目: []
問3 続きの印が updated_at と id → 2ページ目: ["ノート05", "ノート04", "ノート03", "ノート02", "ノート01"]
```

（問題の答えの行だけを抜き出しています）

**問1の解答:**

- `200 OK` で 0 件、ページの行は `3 / 2 ページ（全25件）`、そして空のときの文は「**まだノートがありません。最初の1件を書いてみましょう。**」でした。
- ノートは 25 件あるのに、「まだノートがありません」と言っています。空のときの文は「キーワードがあるか」だけで分けていて（UNIT 07）、「ページが範囲の外か」を見ていないからです。範囲の外なら「このページにはノートがありません。1ページ目へ」と案内するか、最後のページへ 303 で移す、などの直し方があります。これは RustNote に残る改善点です（RustNote のコードは変えていません）。

**問2:**

- `4294967295`（`u32` のいちばん大きい数）は `200 OK` で 0 件でした。`offset` は `u64` で計算するので、`(4294967295 − 1) × 20` もあふれません（第12巻 UNIT 01 で、`u32` で計算したときの折り返しを確かめています）。
- `4294967296` は `u32` に入らないので、`400 Bad Request` です（UNIT 07 の問3と同じ）。

**問3:**

- 25 件の `updated_at` は、1種類（全部が同じ秒）でした。続きの印が `updated_at` だけだと、`updated_at < ?1` に合う行が無いので、**2ページ目は空**になり、ノート05〜01 が抜けました。
- `(updated_at, id) < (?1, ?2)` なら、同じ秒の中では `id` で比べるので、ノート05〜01 が続きました。並びの `ORDER BY` に `id DESC` を足したのと同じ理由です（UNIT 01）。続きの印は、**並びの列を全部**含めて、1つの行を必ず決められる組にします。

### まとめ

- RustNote のページ送りは、`LIMIT 20 OFFSET (ページ − 1) × 20` と、件数を 20 で割って切り上げたページの数（0 件でも 1）。リンクはキーワードを引き継ぐ。1 の引き忘れや切り捨ては、`cargo test --lib` のユニットテストが見つけ、`q` の抜けは、リンクをたどるテストが見つける。
- OFFSET は位置で数えるので、見ている間に増えると同じノートが2回出て、減ると抜ける。cursor（前のページの最後の行の `updated_at` と `id` の後ろを読む）は、ずれない。続きの印は、並びの列を全部含める。
- 20 万件の最後のページで、OFFSET は約 35 ミリ秒、cursor は約 0.1 ミリ秒。OFFSET は好きなページへ飛べ、cursor は深くても速い。範囲の外のページで「まだノートがありません」と出るのは、RustNote に残る改善点。
