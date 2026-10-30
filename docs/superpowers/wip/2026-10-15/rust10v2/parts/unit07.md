## UNIT 07: 検索・LIKE のエスケープ

### この単元のゴール

- 一覧の検索（`GET /notes?q=…`）が、キーワードを整え（前後の空白を取る・空なら検索しない・100 文字まで）、`LIKE ?2 ESCAPE '\'` でタイトルと本文から探すことを、テストで確かめられる
- `%`・`_`・`\` が LIKE で特別な意味を持つことと、`escape_like` と `ESCAPE '\'` の**2つがそろって**初めて、ただの文字として探せることを説明できる
- キーワードを SQL の文字列に直接入れると、`'` で 500 になり、SQL の断片で**他人のノートが見える**ことを、実際に確かめて説明できる
- LIKE の大文字・小文字の扱い（ASCII だけ区別しない）と、`?page=abc` が 400 になる理由を言える

### たとえ話でつかむ

図書館の、本の題名の検索を考えます。

- 検索の機械には、**特別な記号**があります。「＊」は「何文字でも」、「？」は「どれか1文字」です。「ハリー＊」と入れれば、「ハリー」で始まる本が全部出ます。
- では、「100＊オフ」という題名の本を探したいときは、どうするでしょう。そのまま入れると、「100 円で…オフ会」のような本まで出てしまいます。そこで、「この＊は記号ではなく、ただの文字です」という**印**（たとえば、前に「￥」を付ける）を決めておきます。
- 印を付ける係と、印の読み方を知っている機械の、**両方**がそろわないと、うまくいきません。印を付けても、機械が印を知らなければ、「￥＊」という2文字を探してしまいます。
- それとは別に、検索の欄に「全部の棚を見せろ」のような**命令**を書かれても、機械は、それを命令として読んではいけません。題名の文字として扱います。

| 図書館の検索 | RustNote の検索 |
|---|---|
| 「＊」「？」の記号 | LIKE の `%`（何文字でも）と `_`（どれか1文字） |
| 「ただの文字です」の印を付ける係 | `escape_like`（`%` → `\%`、`_` → `\_`、`\` → `\\`） |
| 印の読み方を知っている機械 | `LIKE ?2 ESCAPE '\'`（`\` を印として読む） |
| 欄に書かれた命令は、文字として扱う | キーワードは `?2` に値として渡す（SQL の文として読まれない） |

> **用語:** LIKE（SQL で、文字の並びが「形」に合うかを調べる書き方。`%` は 0 文字以上の何か、`_` は 1 文字の何か。第7巻 UNIT 02）

<!-- -->

> **用語:** ESCAPE（LIKE の形の中で、「次の1文字は記号ではなく、ただの文字」という印に使う文字を決める指定。RustNote は `\`）

<!-- -->

> **用語:** SQL インジェクション（入力に書かれた SQL の断片が、SQL の文の一部として動いてしまう弱さ。値をパラメーター（`?1` など）で渡せば起きない。第7巻 UNIT 07）

### 図で理解する

検索のキーワードが、SQL に届くまでの流れです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-10-0701" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="30" width="140" height="80" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="80" y="56" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">?q=　100%　</text>
<text x="80" y="78" font-size="11" fill="#475569" text-anchor="middle">URL のクエリ</text>
<text x="80" y="96" font-size="11" fill="#475569" text-anchor="middle">（前後に空白）</text>
<rect x="180" y="30" width="160" height="80" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="260" y="54" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">NoteQuery::new</text>
<text x="260" y="74" font-size="11" fill="#0f172a" text-anchor="middle">前後の空白を取る → 100%</text>
<text x="260" y="92" font-size="11" fill="#475569" text-anchor="middle">空なら None・100 文字まで</text>
<rect x="370" y="30" width="160" height="80" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="450" y="54" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">escape_like</text>
<text x="450" y="76" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">100% → 100\%</text>
<text x="450" y="96" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">形: %100\%%</text>
<rect x="560" y="30" width="150" height="80" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="635" y="54" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">SQL</text>
<text x="635" y="76" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">LIKE ?2 ESCAPE '\'</text>
<text x="635" y="96" font-size="11" fill="#475569" text-anchor="middle">形は ?2 に値で渡す</text>
<line x1="150" y1="70" x2="176" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0701)"/>
<line x1="340" y1="70" x2="366" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0701)"/>
<line x1="530" y1="70" x2="556" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0701)"/>
<rect x="10" y="140" width="700" height="96" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="30" y="166" font-size="12" font-weight="700" fill="#c2410c">形の読み方（ESCAPE <tspan font-family="Consolas, monospace">'\'</tspan> のとき）</text>
<text x="30" y="190" font-size="12" fill="#0f172a" font-family="Consolas, monospace">%   100   \%   %</text>
<text x="230" y="190" font-size="11" fill="#0f172a">何でも ／「100」／ ただの「%」／ 何でも</text>
<text x="30" y="216" font-size="11" fill="#475569">→「100%」を含むタイトルか本文だけに合う。「100 円の牛乳」は合わない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: 検索のキーワードが SQL に届くまで。整えたキーワードの記号に印を付け、前後に % を足した「形」を、パラメーターで LIKE に渡す。ESCAPE の指定で、印の付いた % はただの文字になる</div>
</div>

検索には、3種類の「エスケープ」が関わっています。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 220" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="160" height="36" rx="6" fill="#0f172a"/>
<text x="90" y="33" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">どこで</text>
<rect x="180" y="10" width="170" height="36" rx="6" fill="#0f172a"/>
<text x="265" y="33" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">特別な文字</text>
<rect x="360" y="10" width="350" height="36" rx="6" fill="#0f172a"/>
<text x="535" y="33" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">RustNote の守り</text>
<text x="90" y="78" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">LIKE の形</text>
<text x="265" y="78" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">%  _  \</text>
<text x="535" y="72" font-size="11" fill="#0f172a" text-anchor="middle">escape_like で印を付け、ESCAPE <tspan font-family="Consolas, monospace">'\'</tspan> で読ませる</text>
<text x="535" y="88" font-size="11" fill="#475569" text-anchor="middle">（片方だけでは働かない。エラー1・エラー2）</text>
<text x="90" y="134" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">SQL の文</text>
<text x="265" y="134" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">'  )  OR</text>
<text x="535" y="128" font-size="11" fill="#0f172a" text-anchor="middle">形は ?2 のパラメーターで渡す（文に混ぜない）</text>
<text x="535" y="144" font-size="11" fill="#475569" text-anchor="middle">（混ぜると、他人のノートが見える。エラー3）</text>
<text x="90" y="190" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">HTML</text>
<text x="265" y="190" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">&lt;  &gt;  &amp;  "</text>
<text x="535" y="184" font-size="11" fill="#0f172a" text-anchor="middle">view! が、入力欄の value もタイトルも置き換える</text>
<text x="535" y="200" font-size="11" fill="#475569" text-anchor="middle">（第6巻 UNIT 05・UNIT 03 の例3-2）</text>
<line x1="10" y1="106" x2="710" y2="106" stroke="#e2e8f0" stroke-width="1"/>
<line x1="10" y1="162" x2="710" y2="162" stroke="#e2e8f0" stroke-width="1"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: 検索に関わる3種類のエスケープ。LIKE の記号・SQL の文・HTML で、特別な文字も守り方も違う。どれか1つを守っても、ほかの2つは守れない</div>
</div>

### まず動かす最小コード

#### 例7-1: `%` や `_` や `\` を含むタイトルを探す

キーワードを整えるのは `src/domain.rs` の `NoteQuery::new`、探すのは `src/repository/notes.rs` の `search` です（抜き出しています）。

```rust
// NoteQuery::new: 空白だけのキーワードは無いのと同じに扱う
let keyword = keyword
    .map(str::trim)
    .filter(|k| !k.is_empty())
    .map(|k| k.chars().take(MAX_KEYWORD_CHARS).collect());
```

```rust
// search: キーワードが無いときは「%」（何にでも一致する）で検索する
let pattern = match &query.keyword {
    Some(keyword) => format!("%{}%", escape_like(keyword)),
    None => "%".to_string(),
};
// LIKE の中の \ を「次の文字をそのまま扱う」という意味にする（ESCAPE '\'）
let condition = "user_id = ?1 AND (title LIKE ?2 ESCAPE '\\' OR body LIKE ?2 ESCAPE '\\')";
```

```rust
/// LIKE で特別な意味を持つ文字（% と _ と \）を、ただの文字として扱わせる
fn escape_like(keyword: &str) -> String {
    let mut escaped = String::with_capacity(keyword.len());
    for c in keyword.chars() {
        if matches!(c, '\\' | '%' | '_') {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    escaped
}
```

> **▼ このコードがやること（先に日本語で）:** alice が、`%`・`_`・`\` を含むタイトルや、本文に「牛乳」があるノートなど、6件を作ります。8つのキーワード（`100%`・`file_name`・`\`・`牛乳`・`%`・`_`・前後に空白のある `牛乳`・空白だけ）で一覧を検索し、見つかったタイトル・検索の入力欄に残った値・ページの行（全何件か）を表示します。

```rust
//! 第10巻 UNIT 07: 一覧の検索（GET /notes?q=…）で、% や _ や \ を、ただの文字として探せるかを確かめる。
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

/// 検索の入力欄に入っている値（value 属性）。無ければ "（なし）"
fn search_value(html: &str) -> String {
    html.split("<input")
        .find(|t| t.contains("name=\"q\""))
        .and_then(|t| t.split("value=\"").nth(1))
        .map(|t| t.split('"').next().unwrap().to_string())
        .unwrap_or_else(|| "（なし）".to_string())
}

/// 「1 / 1 ページ（全3件）」のような、ページの行
fn page_line(html: &str) -> String {
    html.split("<nav class=\"pagination\">")
        .nth(1)
        .and_then(|t| t.split("<span>").nth(1))
        .and_then(|t| t.split("</span>").next())
        .unwrap_or("（なし）")
        .to_string()
}

#[tokio::test]
async fn search_special_characters() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    for (title, body) in [
        ("セール 100% オフ", ""),
        ("100 円の牛乳", ""),
        ("file_name の決め方", ""),
        ("file-name を決める", ""),
        (r"C:\tmp の掃除", ""),
        ("買い物メモ", "牛乳と卵"),
    ] {
        app.create_note(&alice, title, body).await;
    }
    for q in [
        "100%",
        "file_name",
        r"\",
        "牛乳",
        "%",
        "_",
        "  牛乳  ",
        "   ",
    ] {
        let url = format!("/notes?q={}", encode(q));
        let html = body_text(app.get(&url, Some(&alice)).await).await;
        println!("q={q:?}");
        println!("   見つかった: {:?}", titles(&html));
        println!(
            "   入力欄の値: {:?}・{}",
            search_value(&html),
            page_line(&html)
        );
    }
}
```

```powershell
cargo test --test u07_search -- --nocapture
```

```text
q="100%"
   見つかった: ["セール 100% オフ"]
   入力欄の値: "100%"・1 / 1 ページ（全1件）
q="file_name"
   見つかった: ["file_name の決め方"]
   入力欄の値: "file_name"・1 / 1 ページ（全1件）
q="\\"
   見つかった: ["C:\\tmp の掃除"]
   入力欄の値: "\\"・1 / 1 ページ（全1件）
q="牛乳"
   見つかった: ["買い物メモ", "100 円の牛乳"]
   入力欄の値: "牛乳"・1 / 1 ページ（全2件）
q="%"
   見つかった: ["セール 100% オフ"]
   入力欄の値: "%"・1 / 1 ページ（全1件）
q="_"
   見つかった: ["file_name の決め方"]
   入力欄の値: "_"・1 / 1 ページ（全1件）
q="  牛乳  "
   見つかった: ["買い物メモ", "100 円の牛乳"]
   入力欄の値: "牛乳"・1 / 1 ページ（全2件）
q="   "
   見つかった: ["買い物メモ", "C:\\tmp の掃除", "file-name を決める", "file_name の決め方", "100 円の牛乳", "セール 100% オフ"]
   入力欄の値: "（なし）"・1 / 1 ページ（全6件）
```

（キーワードごとの結果の行だけを抜き出しています。`{:?}` で表示しているので、`\` は `\\` と2つに見えます）

#### ▼ コードを1つずつ分解して解説

##### 解説1: 記号は、ただの文字として探された

- `100%` は `セール 100% オフ` だけで、`100 円の牛乳` は出ませんでした。`file_name` も `file_name の決め方` だけで、`file-name を決める` は出ません。`\` は `C:\tmp の掃除` だけです。
- `%` だけ、`_` だけで探しても、その記号を含む1件だけでした。もし記号として働いていたら、`%` は「何でも」なので、全部のノートが出るはずです（エラー1）。
- `牛乳` は、タイトルの `100 円の牛乳` と、本文に「牛乳と卵」がある `買い物メモ` の2件です。SQL が `title LIKE ?2 … OR body LIKE ?2 …` と、両方を探しているからです。並びは、一覧と同じ新しい順です。

##### 解説2: キーワードは整えてから使い、入力欄に戻す

- `  牛乳  ` は、前後の空白を取った `牛乳` で探され、入力欄の値も `牛乳` になりました。空白だけのキーワードは「無い」と同じで、6件全部が出て、入力欄は空（`value` 属性が無い）です。
- 入力欄に値を戻しているのは、`value=(keyword.as_deref())` です。何で探した結果なのかが画面で分かり、少し直して探し直すこともできます。`None` のときは、`value` 属性そのものが出ません。
- 入力欄の値の `100%` や `\` も、`view!` が属性の値として安全に入れています（図の HTML の段）。

### もう一歩: 実務に近い形

#### 例7-2: LIKE の形を、SQL で直接確かめる

`escape_like` と `ESCAPE '\'` が、それぞれ何をしているのかを、SQL を直接動かして確かめます。

> **▼ このコードがやること（先に日本語で）:** 5件のノートを作り、`notes` 表に、7つの「形」と「ESCAPE があるか・無いか」の組み合わせで `SELECT title … WHERE title LIKE ?1` を動かして、合ったタイトルを表示します。

```rust
//! 第10巻 UNIT 07: SQLite の LIKE が、% と _ と ESCAPE をどう扱うかを、SQL を直接動かして確かめる。
//! 実行: cargo test --test u07_like -- --nocapture

mod common;

use common::test_app;

#[tokio::test]
async fn like_patterns() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    for title in [
        "セール 100% オフ",
        "100 円の牛乳",
        "file_name の決め方",
        "file-name を決める",
        r"C:\tmp の掃除",
    ] {
        app.create_note(&alice, title, "").await;
    }
    let conn = app.db.db.connect().await.unwrap();
    let plain = "SELECT title FROM notes WHERE title LIKE ?1 ORDER BY id";
    let escaped = r"SELECT title FROM notes WHERE title LIKE ?1 ESCAPE '\' ORDER BY id";
    let cases = [
        ("%100%%", plain),
        (r"%100\%%", plain),
        (r"%100\%%", escaped),
        ("%file_name%", plain),
        (r"%file\_name%", escaped),
        (r"%\%", plain),
        (r"%\\%", escaped),
    ];
    for (pattern, sql) in cases {
        let mut rows = conn.query(sql, [pattern]).await.unwrap();
        let mut found = Vec::new();
        while let Some(row) = rows.next().await.unwrap() {
            found.push(row.get::<String>(0).unwrap());
        }
        let escape = if sql.contains("ESCAPE") {
            "ESCAPE あり"
        } else {
            "ESCAPE なし"
        };
        println!("{pattern:<14} {escape} → {found:?}");
    }
}
```

```powershell
cargo test --test u07_like -- --nocapture
```

```text
%100%%         ESCAPE なし → ["セール 100% オフ", "100 円の牛乳"]
%100\%%        ESCAPE なし → []
%100\%%        ESCAPE あり → ["セール 100% オフ"]
%file_name%    ESCAPE なし → ["file_name の決め方", "file-name を決める"]
%file\_name%   ESCAPE あり → ["file_name の決め方"]
%\%            ESCAPE なし → ["C:\\tmp の掃除"]
%\\%           ESCAPE あり → ["C:\\tmp の掃除"]
```

- `%100%%`（印なし）は、`100 円の牛乳` にも合いました。`%100%%` は「何でも・100・何でも・何でも」なので、「100」を含めば何でも合います。
- `%100\%%` は、**ESCAPE が無いと 0 件**、ESCAPE があると `セール 100% オフ` だけでした。ESCAPE が無いと、`\` は印ではなく、ただの `\` という文字なので、「`100\`」を含むタイトルを探してしまいます。
- `%file_name%`（印なし）は、`file-name を決める` にも合いました。`_` は「どれか1文字」なので、`-` も合います。`%file\_name%` に ESCAPE を付けると、`_` だけになります。
- `\` そのものを探すときは、ESCAPE が無ければ `%\%`、ESCAPE があれば `%\\%`（印の `\` と、ただの `\`）です。`escape_like` が `\` も2つにしているのは、このためです。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: escape_like を通さなかった

`search` で、`format!("%{}%", escape_like(keyword))` を `format!("%{keyword}%")` にした場合です（練習用のコピーで書き換え、確かめたら元に戻します）。例7-1のテストを動かします。

```text
warning: function `escape_like` is never used
q="100%"
   見つかった: ["100 円の牛乳", "セール 100% オフ"]
   入力欄の値: "100%"・1 / 1 ページ（全2件）
q="file_name"
   見つかった: ["file-name を決める", "file_name の決め方"]
   入力欄の値: "file_name"・1 / 1 ページ（全2件）
q="\\"
   見つかった: []
   入力欄の値: "\\"・1 / 1 ページ（全0件）
q="牛乳"
   見つかった: ["買い物メモ", "100 円の牛乳"]
   入力欄の値: "牛乳"・1 / 1 ページ（全2件）
q="%"
   見つかった: ["買い物メモ", "C:\\tmp の掃除", "file-name を決める", "file_name の決め方", "100 円の牛乳", "セール 100% オフ"]
   入力欄の値: "%"・1 / 1 ページ（全6件）
q="_"
   見つかった: ["買い物メモ", "C:\\tmp の掃除", "file-name を決める", "file_name の決め方", "100 円の牛乳", "セール 100% オフ"]
   入力欄の値: "_"・1 / 1 ページ（全6件）
q="  牛乳  "
   見つかった: ["買い物メモ", "100 円の牛乳"]
   入力欄の値: "牛乳"・1 / 1 ページ（全2件）
q="   "
   見つかった: ["買い物メモ", "C:\\tmp の掃除", "file-name を決める", "file_name の決め方", "100 円の牛乳", "セール 100% オフ"]
   入力欄の値: "（なし）"・1 / 1 ページ（全6件）
```

- **何が起きたか:** `100%` で `100 円の牛乳` まで、`file_name` で `file-name を決める` まで出ました。`%` と `_` だけで探すと、**6件全部**が出ます。`\` は 0 件です（ESCAPE の印として読まれ、次の `%` を「ただの %」にしたので、「% で終わるタイトル」を探す意味になりました）。
- コンパイラーの `warning: function escape_like is never used`（使われていない関数）が、唯一の手がかりです。警告を読み飛ばさないことが、ここでも役に立ちます。
- **困ること:** 自分のノートの中だけを探しているので、他人のノートは出ません。それでも、「100%」で探した人には、関係の無い結果が混ざり、`%` や `_` だけで探すと、何を探しても全部が出る検索になります。

#### エラー2（エラーが出ない間違い）: `ESCAPE '\'` を外した

`escape_like` は残したまま、SQL の `ESCAPE '\\'` を2つとも外した場合です。

```text
q="100%"
   見つかった: []
   入力欄の値: "100%"・1 / 1 ページ（全0件）
q="file_name"
   見つかった: []
   入力欄の値: "file_name"・1 / 1 ページ（全0件）
q="\\"
   見つかった: []
   入力欄の値: "\\"・1 / 1 ページ（全0件）
q="牛乳"
   見つかった: ["買い物メモ", "100 円の牛乳"]
   入力欄の値: "牛乳"・1 / 1 ページ（全2件）
q="%"
   見つかった: ["C:\\tmp の掃除"]
   入力欄の値: "%"・1 / 1 ページ（全1件）
q="_"
   見つかった: ["C:\\tmp の掃除"]
   入力欄の値: "_"・1 / 1 ページ（全1件）
q="  牛乳  "
   見つかった: ["買い物メモ", "100 円の牛乳"]
   入力欄の値: "牛乳"・1 / 1 ページ（全2件）
q="   "
   見つかった: ["買い物メモ", "C:\\tmp の掃除", "file-name を決める", "file_name の決め方", "100 円の牛乳", "セール 100% オフ"]
   入力欄の値: "（なし）"・1 / 1 ページ（全6件）
```

- **何が起きたか:** 今度は逆に、`100%`・`file_name`・`\` が **0 件**になりました。`escape_like` が付けた `\` を、SQL が印として読まないので、「`100\%`」という文字の並びを探しているからです。そして、`%` や `_` で探すと、`C:\tmp の掃除` が出てきます。`\%` の `\` が、ただの `\` として、`C:\tmp` の `\` に合ったのです。
- **学べること:** 印を付ける係（`escape_like`）と、印の読み方（`ESCAPE '\'`）は、**2つで1組**です。片方だけでは、エラー1とエラー2のように、どちらの方向にも間違えます。第11巻 UNIT 02 のリポジトリのテストは、このエラー2を、テストが失敗することで見つけています。

#### エラー3（エラーが出ない間違い）: キーワードを SQL の文字列に直接入れた

`?2` で渡す代わりに、形を SQL の文字列に `'{pattern}'` と直接入れた場合です。

```rust
let condition = format!(
    "user_id = ?1 AND (title LIKE '{pattern}' ESCAPE '\\' OR body LIKE '{pattern}' ESCAPE '\\')"
);
```

alice が「給料の明細」と「病院の予約」の2件、bob が「買い物」の1件を持っているときに、bob が3つのキーワードで探すテストです。

```rust
//! 第10巻 UNIT 07: bob が、検索のキーワードに ' や SQL の断片を入れたら、どうなるかを確かめる。
//! 実行: cargo test --test u07_inject -- --nocapture

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
async fn keyword_with_sql() {
    let app = test_app().await;
    let alice = app
        .sign_up("alice@example.com", "correct horse battery")
        .await;
    let bob = app
        .sign_up("bob@example.com", "correct horse battery")
        .await;
    app.create_note(&alice, "alice の給料の明細", "").await;
    app.create_note(&alice, "alice の病院の予約", "").await;
    app.create_note(&bob, "bob の買い物", "").await;

    for q in ["買い物", "'", "' OR 1=1) OR (title LIKE '"] {
        let response = app
            .get(&format!("/notes?q={}", encode(q)), Some(&bob))
            .await;
        let status = response.status();
        let html = body_text(response).await;
        println!("bob が q={q:?} で検索 → {status}");
        println!("   見つかった: {:?}", titles(&html));
    }
}
```

まず、RustNote のコード（`?2` で渡す版）です。

```text
bob が q="買い物" で検索 → 200 OK
   見つかった: ["bob の買い物"]
bob が q="'" で検索 → 200 OK
   見つかった: []
bob が q="' OR 1=1) OR (title LIKE '" で検索 → 200 OK
   見つかった: []
```

次に、SQL の文字列に直接入れた版です。

```text
bob が q="買い物" で検索 → 200 OK
   見つかった: ["bob の買い物"]
[error] サーバーの中でエラーが起きました <- DB の操作に失敗しました <- SQLite failure: `unrecognized token: "\"`
bob が q="'" で検索 → 500 Internal Server Error
   見つかった: []
bob が q="' OR 1=1) OR (title LIKE '" で検索 → 200 OK
   見つかった: ["bob の買い物", "alice の病院の予約", "alice の給料の明細"]
```

- **何が起きたか:** `'` だけで **500** になりました。`'` が SQL の文字列を途中で閉じてしまい、残りが SQL の文として読めなくなったのです。そして、`' OR 1=1) OR (title LIKE '` では、**bob に alice の2件が見えました**。`OR 1=1`（いつも正しい）が SQL の文の一部になり、`user_id = ?1` の条件を外側から無効にしたからです。
- **RustNote の版では:** 3つとも `200` で、`'` も SQL の断片も、ただの文字として探され、0 件でした。`?2` に渡した値は、どんな文字が入っていても、SQL の文としては読まれません（第7巻 UNIT 07）。
- **学べること:** `escape_like` は LIKE の記号の守りで、`'` は守りません。SQL の文の守りは、**値をパラメーターで渡すこと**です。2つは別の守りで、片方ではもう片方を守れません（図の2つ目）。

### RustNote ではここで使う

- 検索の入力欄は、一覧の画面（`src/routes/notes.rs` の `notes_index`）の上にある `<form method="get">` です。GET なので、検索の結果の URL（`/notes?q=牛乳`）を、そのままブックマークしたり、人に送ったりできます。DB を変えない操作なので、GET でかまいません（UNIT 06 との違い）。
- 結果が 0 件のときは、キーワードがあれば「条件に合うノートはありません。」、無ければ「まだノートがありません。最初の1件を書いてみましょう。」と、文を分けています（UNIT 01）。
- ページ送りのリンクは、キーワードを `?q=…&page=2` のように引き継ぎます（UNIT 08）。
- 検索の入力欄にラベルが無いことは、第9巻 UNIT 09 で見つけた改善点です。

### 練習問題

**問1:** タイトルが `Rust 入門`・`ｒｕｓｔ の本`（全角）・`Äpfel kaufen` の3件のときに、`rust`・`RUST`・`ｒｕｓｔ`・`äpfel`・`Äpfel` で探すと、それぞれ何が見つかるでしょうか。

**問2:** 101 文字のキーワードで探すと、検索の入力欄には何文字が残るでしょうか。

**問3:** `/notes?q=牛乳&page=abc`・`page=0`・`page=-1` を開くと、それぞれ何が返るでしょうか。

### 解答と解説

```rust
//! 第10巻 UNIT 07 練習問題。
//! 実行: cargo test --test u07_q -- --nocapture --test-threads=1

mod common;

use common::{body_text, encode, test_app};

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

/// 検索の入力欄に入っている値（value 属性）
fn search_value(html: &str) -> String {
    html.split("<input")
        .find(|t| t.contains("name=\"q\""))
        .and_then(|t| t.split("value=\"").nth(1))
        .map(|t| t.split('"').next().unwrap().to_string())
        .unwrap_or_default()
}

/// 問1: 大文字と小文字、全角と半角
#[tokio::test]
async fn q1_case_and_width() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    for title in ["Rust 入門", "ｒｕｓｔ の本", "Äpfel kaufen"] {
        app.create_note(&alice, title, "").await;
    }
    for q in ["rust", "RUST", "ｒｕｓｔ", "äpfel", "Äpfel"] {
        let html = body_text(
            app.get(&format!("/notes?q={}", encode(q)), Some(&alice))
                .await,
        )
        .await;
        println!("問1 q={q:<6} → {:?}", titles(&html));
    }
}

/// 問2: とても長いキーワード
#[tokio::test]
async fn q2_long_keyword() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let keyword = "あ".repeat(101);
    let html = body_text(
        app.get(&format!("/notes?q={}", encode(&keyword)), Some(&alice))
            .await,
    )
    .await;
    println!(
        "問2 送ったキーワード: {} 文字 → 入力欄の値: {} 文字",
        keyword.chars().count(),
        search_value(&html).chars().count()
    );
}

/// 問3: ページの番号がおかしいとき
#[tokio::test]
async fn q3_bad_page_number() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    app.create_note(&alice, "牛乳を買う", "").await;
    for url in [
        "/notes?q=%E7%89%9B%E4%B9%B3&page=abc",
        "/notes?q=%E7%89%9B%E4%B9%B3&page=0",
        "/notes?q=%E7%89%9B%E4%B9%B3&page=-1",
    ] {
        let response = app.get(url, Some(&alice)).await;
        let status = response.status();
        let html = body_text(response).await;
        println!(
            "問3 GET {url} → {status}（見つかった: {:?}）",
            titles(&html)
        );
    }
}
```

```powershell
cargo test --test u07_q -- --nocapture --test-threads=1
```

```text
問1 q=rust   → ["Rust 入門"]
問1 q=RUST   → ["Rust 入門"]
問1 q=ｒｕｓｔ   → ["ｒｕｓｔ の本"]
問1 q=äpfel  → []
問1 q=Äpfel  → ["Äpfel kaufen"]
問2 送ったキーワード: 101 文字 → 入力欄の値: 100 文字
問3 GET /notes?q=%E7%89%9B%E4%B9%B3&page=abc → 400 Bad Request（見つかった: []）
問3 GET /notes?q=%E7%89%9B%E4%B9%B3&page=0 → 200 OK（見つかった: ["牛乳を買う"]）
問3 GET /notes?q=%E7%89%9B%E4%B9%B3&page=-1 → 400 Bad Request（見つかった: []）
```

（問題の答えの行だけを抜き出しています）

**問1の解答:**

- `rust` と `RUST` は、どちらも `Rust 入門` に合いました。SQLite の LIKE は、**ASCII の英字（A〜Z）だけ**、大文字と小文字を区別しません。
- `ｒｕｓｔ`（全角）は `ｒｕｓｔ の本` だけで、半角の `Rust 入門` には合いません。全角と半角は、別の文字です。
- `äpfel` は 0 件で、`Äpfel` だけが `Äpfel kaufen` に合いました。`ä` と `Ä` は ASCII の外なので、LIKE は区別します。日本語のひらがなとカタカナも、もちろん別の文字です。
- 区別せずに探したいときは、保存するときと探すときに、同じ形にそろえる（全角を半角に、などの「正規化」）方法があります。RustNote は、そろえていません。

**問2の解答:**

- 101 文字を送ると、入力欄には **100 文字**が残りました。`NoteQuery::new` が `k.chars().take(MAX_KEYWORD_CHARS)`（100 文字）で切っているからです。`chars()` で数えるので、日本語でも 1 文字ずつ数えます。
- エラーにせず切るのは、検索は「間違えても困らない」操作だからです。ノートのタイトル（80 文字まで）は、保存する値なので、切らずに 400 で知らせます（UNIT 02）。

**問3の解答:**

- `page=abc` と `page=-1` は **`400 Bad Request`** です。`NotesParams` の `page: Option<u32>` に、`abc` も `-1`（u32 はマイナスを持てない）も入らないので、`#[query_params(error = bad_request)]` が 400 にしました。
- `page=0` は `200 OK` で、1ページ目が出ました。`NoteQuery::new` が `page.unwrap_or(1).max(1)` で、0 を 1 にしているからです。型（`u32`）で守れない「0」は、コードで直しています。ページ送りの決まりは、次の UNIT 08 で扱います。

### まとめ

- 検索は、キーワードを整え（前後の空白を取る・空なら None・100 文字まで）、`escape_like` で `%`・`_`・`\` に印を付け、`LIKE ?2 ESCAPE '\'` でタイトルと本文から探す。入力欄には、整えたキーワードを戻す。
- `escape_like` と `ESCAPE '\'` は2つで1組。印を付けなければ `%` で全部が出て、ESCAPE が無ければ記号を含む検索が 0 件になる。どちらも、エラーは出ない。
- キーワードを SQL の文字列に直接入れると、`'` で 500、SQL の断片で他人のノートが見える。SQL の文の守りは、値をパラメーターで渡すこと。LIKE は ASCII の英字だけ大文字小文字を区別しない。
