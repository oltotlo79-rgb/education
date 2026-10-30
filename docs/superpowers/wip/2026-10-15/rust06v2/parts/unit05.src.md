## UNIT 05: view!・コンポーネント・HTML のエスケープ

### この単元のゴール

- `view!` の中で、文字（`"…"`）・式（`( )`）・属性・`if`・`for`・`match` を使って、HTML を組み立てられる
- `#[component]` で画面を部品に分け、名前付きの引数と、中身（`child`）を渡して呼び出せる
- 利用者が書いた文字が、**置く場所ごと**にどうエスケープされるか（本文・属性の値・属性の名前）を、実際の HTML で確かめられる
- `"…"` の付け忘れ・閉じタグの書き忘れ・`view!` に入れた値の移動（E0382）のエラーを読める

### たとえ話でつかむ

お弁当屋さんの、詰め合わせの作業を考えます。

- お弁当には、**決まった仕切り**（ご飯の場所・おかずの場所）があります。仕切りの形は、いつも同じです。これが HTML のタグです。
- 仕切りの中には、**その日の材料**を入れます。材料は日によって変わります。これが `( )` に入れる式です。
- 「卵焼き」「唐揚げ」の小分けのカップは、**何度も使う部品**です。カップの作り方を一度決めておけば、どのお弁当にも同じように入れられます。これがコンポーネントです。
- お客さんから持ち込まれた材料は、そのまま入れずに、**入れる場所に合わせて包み直します**。汁物の仕切りに入れるなら汁がこぼれないように、揚げ物の仕切りに入れるなら油がしみないように。これがエスケープです。
- 「この材料で、仕切りそのものを作って」と頼まれたら断ります。仕切りが壊れると、お弁当の形が保てないからです。

| お弁当屋さん | Topcoat の view! |
|---|---|
| 決まった仕切り | HTML のタグ（`<ul>`・`<li>` など） |
| その日の材料 | `( )` に入れた式（`(title)`・`(count)`） |
| 小分けのカップ | `#[component]`（何度も使う画面の部品） |
| 入れる場所に合わせて包み直す | エスケープ（本文では `<` を `&lt;`、属性の値では `"` を `&quot;` に） |
| 仕切りそのものは作らせない | 属性の名前・タグの名前に危ない文字が来たら、panic で止める |

> **用語:** コンポーネント（component：画面の一部を作る、何度も使える部品。Topcoat では `#[component]` を付けた async 関数）

<!-- -->

> **用語:** エスケープ（escape：HTML で特別な意味を持つ文字を、ただの文字として表示される書き方に置き換えること。`<` → `&lt;` など）

<!-- -->

> **用語:** XSS（クロスサイトスクリプティング：利用者が書いた文字にプログラム（`<script>` など）を混ぜて、ほかの人の画面で動かす攻撃）

### 図で理解する

`view!` の中で使う書き方の一覧です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="20" width="330" height="46" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="36" y="40" font-size="12" fill="#0f172a" font-family="Consolas, monospace">&lt;h1&gt;"ノート"&lt;/h1&gt;</text>
<text x="36" y="58" font-size="11" fill="#475569">決まった文字は "…" で囲む</text>
<rect x="370" y="20" width="330" height="46" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="386" y="40" font-size="12" fill="#0f172a" font-family="Consolas, monospace">&lt;p&gt;(title)&lt;/p&gt;</text>
<text x="386" y="58" font-size="11" fill="#475569">変わる値は ( ) に入れる（エスケープされる）</text>
<rect x="20" y="78" width="330" height="46" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="36" y="98" font-size="12" fill="#0f172a" font-family="Consolas, monospace">data-tag=(tag)</text>
<text x="36" y="116" font-size="11" fill="#475569">None なら、属性そのものを書かない</text>
<rect x="370" y="78" width="330" height="46" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="386" y="98" font-size="12" fill="#0f172a" font-family="Consolas, monospace">disabled=(is_pending)</text>
<text x="386" y="116" font-size="11" fill="#475569">true なら disabled=""、false なら書かない</text>
<rect x="20" y="136" width="330" height="46" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="36" y="156" font-size="12" fill="#0f172a" font-family="Consolas, monospace">if done { " ✓" }</text>
<text x="36" y="174" font-size="11" fill="#475569">if・else・match で、出す部品を選ぶ</text>
<rect x="370" y="136" width="330" height="46" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="386" y="156" font-size="12" fill="#0f172a" font-family="Consolas, monospace">#[key(note.id)] for note in notes { … }</text>
<text x="386" y="174" font-size="11" fill="#475569">for で並べる。key は1件ずつを見分ける印</text>
<rect x="20" y="194" width="680" height="80" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="36" y="216" font-size="12" fill="#0f172a" font-family="Consolas, monospace">panel(title: "一覧", &lt;ul&gt; … &lt;/ul&gt;)</text>
<text x="36" y="236" font-size="11" fill="#475569">コンポーネントは、名前付きの引数で呼ぶ。引数のあとに書いた部品は、child として渡る</text>
<text x="36" y="258" font-size="12" fill="#0f172a" font-family="Consolas, monospace">#[component] async fn panel(title: &amp;str, #[default] child: Child&lt;'_&gt;)</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: view! の書き方。HTML とほぼ同じ形で書き、決まった文字は "…"、変わる値は ( ) に入れる。属性の値に Option や bool を入れると、無いときは属性ごと消える</div>
</div>

利用者が書いた文字を、置く場所ごとに、どう書き換えるかです（Topcoat 0.11 の `topcoat-view` の `escape.rs` に書いてある決まりです）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="150" y="26" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">置く場所</text>
<text x="330" y="26" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">&lt;</text>
<text x="400" y="26" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">&gt;</text>
<text x="470" y="26" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">&amp;</text>
<text x="540" y="26" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">"</text>
<text x="630" y="26" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">理由</text>
<rect x="20" y="38" width="680" height="50" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="150" y="60" font-size="12" fill="#0f172a" text-anchor="middle">本文（タグの間）</text>
<text x="150" y="78" font-size="11" fill="#475569" text-anchor="middle" font-family="Consolas, monospace">&lt;p&gt;(title)&lt;/p&gt;</text>
<text x="330" y="68" font-size="12" fill="#b91c1c" text-anchor="middle" font-family="Consolas, monospace">&amp;lt;</text>
<text x="400" y="68" font-size="12" fill="#b91c1c" text-anchor="middle" font-family="Consolas, monospace">&amp;gt;</text>
<text x="470" y="68" font-size="12" fill="#b91c1c" text-anchor="middle" font-family="Consolas, monospace">&amp;amp;</text>
<text x="540" y="68" font-size="12" fill="#475569" text-anchor="middle">そのまま</text>
<text x="630" y="68" font-size="11" fill="#475569" text-anchor="middle">&lt; はタグの始まり</text>
<rect x="20" y="98" width="680" height="50" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="150" y="120" font-size="12" fill="#0f172a" text-anchor="middle">属性の値（"…" の中）</text>
<text x="150" y="138" font-size="11" fill="#475569" text-anchor="middle" font-family="Consolas, monospace">title=(title)</text>
<text x="330" y="128" font-size="12" fill="#475569" text-anchor="middle">そのまま</text>
<text x="400" y="128" font-size="12" fill="#475569" text-anchor="middle">そのまま</text>
<text x="470" y="128" font-size="12" fill="#b91c1c" text-anchor="middle" font-family="Consolas, monospace">&amp;amp;</text>
<text x="540" y="128" font-size="12" fill="#b91c1c" text-anchor="middle" font-family="Consolas, monospace">&amp;quot;</text>
<text x="630" y="128" font-size="11" fill="#475569" text-anchor="middle">" で値が終わる</text>
<rect x="20" y="158" width="680" height="56" rx="6" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="150" y="180" font-size="12" fill="#0f172a" text-anchor="middle">属性の名前・タグの名前</text>
<text x="150" y="198" font-size="11" fill="#475569" text-anchor="middle" font-family="Consolas, monospace">&lt;p (attr)="1"&gt;</text>
<text x="470" y="186" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">書き換えずに、panic で止める（空白・= ・&lt; ・&gt; ・" ・' ・/ など）</text>
<text x="470" y="204" font-size="11" fill="#475569" text-anchor="middle">名前は書き換えても元に戻らないので、危ない文字が来たら作らない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: 置く場所ごとのエスケープ。本文ではタグが始まらないように &lt; と &gt; を、属性の値では値が終わらないように " を書き換える。名前の場所には危ない文字を入れさせない</div>
</div>

### まず動かす最小コード

#### 例5-1: 部品に分けて、一覧の画面を作る

> **▼ このコードがやること（先に日本語で）:** ノート1件を表示する部品 `note_card`（完了なら `done` のクラスと「✓」を付け、タグがあれば `data-tag` 属性を付ける）と、見出しと中身を枠で囲む部品 `panel` を作ります。トップページでは、2件のノートを `for` で並べ、全部が完了かどうかを `match` で分けて表示し、最後に `checked` と `disabled` を式で書いたチェックボックスを置きます。

```rust
@@FILE:web_lab/src/bin/u05_view.rs@@
```

```powershell
cargo run --bin u05_view
curl.exe -s http://127.0.0.1:3000/
```

**▼ 実行結果**

```text
@@OUT:u05_view_curl@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 文字は "…"、変わる値は ( )

- `<h1>"ノート（" (count) " 件）"</h1>` は、決まった文字を `"…"` で囲み、変わる値（`count`）を `( )` に入れています。結果は `<h1>ノート（2 件）</h1>` です。`"…"` で囲むのは、Rust のコードと HTML の文字を見分けるためです（エラー1）。
- `view!` の中の空白や改行は、HTML には出ません。字下げは、読みやすさのために自由に入れられます。

##### 解説2: 属性に Option・bool を入れると、無いときは書かない

- 1件目は `data-tag="買い物"` が付き、2件目（`tag: None`）には `data-tag` そのものがありません。属性の値の式が `None` のときは、**属性ごと書かない**決まりです。
- `checked=(count > 1)` は `true` なので `checked=""`、`disabled=(false)` は `false` なので、書かれていません。`checked`・`disabled` のような「あるかないか」だけで意味が決まる属性（ブール属性）は、`bool` の式で書きます（問2）。
- `class=(class!("note", "done" if done))` は、`done` が `true` のときだけ `done` を足して、`class="note done"` を作りました。UNIT 03 のメニューでも使いました。

##### 解説3: for・if・match は、Rust と同じ形

- `for note in notes { note_card(…) }` で、ノートの数だけ `<li>` を作りました。`#[key(note.id)]` は、1件ずつを見分ける印です。並び順が変わっても、同じノートを同じ部品として扱えるようにします（ブラウザの中で動く部分を使うときに大事になります。UNIT 09）。
- `if done { " ✓" }` は、完了のときだけ「✓」を足しました。`match status { "all" => …, _ => … }` は、2件とも完了ではないので、`_` の側（「まだ残っています」）になりました。

##### 解説4: コンポーネントは、名前付きの引数で呼ぶ

- `note_card(title: note.title, done: note.done, tag: note.tag)` のように、**引数の名前を書いて**呼びます。順番を間違えても、名前で合うので安心です。
- `panel(title: "一覧", <ul>…</ul>)` は、名前付きの引数のあとに、`<ul>…</ul>` を書きました。これは `panel` の `child` という引数に入り、`panel` の中の `(child)` の所に置かれました。`#[default]` が付いているので、中身なしで `panel(title: "空")` と呼ぶこともできます。
- コンポーネントは async 関数なので、中で DB を読むこともできます。RustNote の `notes_index` は、コンポーネントの中でノートを検索しています（第10巻 UNIT 01）。

### もう一歩: 実務に近い形

#### 例5-2: 利用者が書いた危ない文字を、置く場所ごとに確かめる

利用者がノートのタイトルに、`<script>alert("x")</script> & 'quote'` と書いた、と考えます。これを、本文と、属性の値の両方に置いてみます。さらに、利用者の文字から**属性の名前**を作ってしまう、してはいけない例も試します。

> **▼ このコードがやること（先に日本語で）:** トップページでは、危ない文字を含むタイトルを、`<p>` の本文と、`<a>` の `title` 属性の値に置きます。`/name?attr=…` のページでは、クエリの `attr` の文字を、そのまま `<p>` の属性の**名前**に使います。

```rust
@@FILE:web_lab/src/bin/u05_escape.rs@@
```

```powershell
cargo run --bin u05_escape
curl.exe -s http://127.0.0.1:3000/
```

```text
@@OUT:u05_escape_curl@@
```

- **本文**（`<p>` の中）では、`<` が `&lt;`、`>` が `&gt;`、`&` が `&amp;` になりました。ブラウザは、`<script>` を**ただの文字**として表示し、プログラムとしては動かしません。`"` と `'` は、本文では特別な意味が無いので、そのままです。
- **属性の値**（`title="…"` の中）では、`"` が `&quot;`、`&` が `&amp;` になりました。`"` をそのまま書くと、そこで値が終わり、うしろに好きな属性（`onclick=…` など）を書き足されてしまうからです。`"…"` の中では `<` はタグの始まりにならないので、書き換えていません。第10巻 UNIT 02 の問3で見たのと同じ動きです。

次に、属性の名前を試します。

```powershell
curl.exe -i "http://127.0.0.1:3000/name?attr=data-ok"
curl.exe -i "http://127.0.0.1:3000/name?attr=onclick%3Dalert(1)%20x"
```

```text
@@OUT:u05_attr_curl@@
```

**▼ サーバーの窓の表示**

```text
@@OUT:u05_attr_server@@
```

- `data-ok` は、そのまま属性の名前になりました。`onclick=alert(1) x`（`%3D` は `=`、`%20` は空白）は、**500 Internal Server Error** で、サーバーの窓に `invalid attribute key "onclick=alert(1) x": forbidden character '='`（属性の名前が不正です。使えない文字 `=` があります）が出ました。
- 属性の名前やタグの名前は、書き換えても意味が保てないので、Topcoat は**危ない文字を見つけたら、HTML を作らずに止めます**。止めずに書いていたら、`<p onclick=alert(1) x="1">` という、押すとプログラムが動く HTML ができていました。
- そもそも、**利用者の文字から属性やタグの名前を作らない**のが正しい書き方です。名前は、プログラムの中で決めた固定の文字にします。

> **注意:** Topcoat にも、エスケープをしない書き方（`PartsWriter` の `push_str_unescaped` など）があります。利用者の文字には、決して使いません。自分で書いた、固定の HTML だけに使う道具です。

### よくあるエラーと読み方

#### エラー1: 文字を "…" で囲み忘れた

```rust
@@FILE:errs/e05_noquote.rs@@
```

```text
@@OUT:u05_err_noquote@@
```

- **何が起きたか:** 「view の部品が来るはずです」というエラーで、`こんにちは` の所を指しています。`view!` の中では、`"…"` で囲んでいない言葉は、タグの名前や Rust のコードとして読もうとします。日本語の文字は、どちらにもならないので、エラーになりました。
- **直し方:** `<h1>"こんにちは"</h1>` と、`"…"` で囲みます。

#### エラー2: 閉じタグを書き忘れた

```rust
@@FILE:errs/e05_noclose.rs@@
```

```text
@@OUT:u05_err_noclose@@
```

- **何が起きたか:** 「閉じタグ `section` が、開きタグ `p` と合いません」というエラーです。12 行目の `<p>` を閉じていないので、`view!` は、次の `</section>` を「`<p>` を閉じるはずの所」と読みました。エラーが指すのは、書き忘れた所（12 行目）ではなく、**合わないと分かった所**（14 行目）です。
- **直し方:** 12 行目の終わりに `</p>` を足します。ブラウザの HTML は閉じタグを省いても表示されることがありますが、`view!` では、`<br>`・`<input>` のような空の要素のほかは、必ず閉じます。エラーの指す行より**上**を探すのが近道です。

#### エラー3: view! に入れた値を、あとでもう一度使った（E0382）

```rust
@@FILE:errs/e05_moved.rs@@
```

```text
@@OUT:u05_err_moved@@
```

- **何が起きたか:** 「移動した値 `title` を使いました」というエラーです。`view! { <h1>(title)</h1> }` は、`async move` のブロックのように、中で使う値を**持ち主ごと受け取ります**（`variable moved due to use in coroutine`）。`title` は `header` の中に移ったので、あとの `title.len()` では使えません（第3巻 UNIT 02）。
- **直し方:** あとでも使う値は、先に必要なものを取り出しておきます。たとえば `let len = title.len();` を `header` を作る**前**に書いて、`(len)` を使います。どちらにも `String` が要るときは、`title.clone()` を `view!` に入れます。

### RustNote ではここで使う

RustNote の画面の部品は、`src/views.rs` にまとめてあります。入力欄の下にエラーを出す部品です（抜き出しています）。

```rust
/// 入力欄の下に出すエラー文（無ければ何も出さない）
#[component]
pub async fn field_error(message: Option<String>) -> Result<impl View> {
    Ok(view! {
        if let Some(message) = message {
            <p class="field-error">(message)</p>
        }
    })
}
```

- `message` が `None` なら、何も出しません。作成と編集の2つのフォームの、タイトルと本文の4か所で使っています。同じ形のエラー表示を1か所で決められるのが、コンポーネントの良さです（第10巻 UNIT 02）。

一覧の1行は、`src/routes/notes.rs` の `notes_index` の中で、`for` で並べています（抜き出しています）。

```rust
            <ul class="note-list">
                #[key(note.id.get())]
                for note in page.notes.clone() {
                    <li class=(class!("note-item", "done" if note.done))>
                        <a href=(href!(id::show, id::Id(note.id)))>(note.title)</a>
```

- 例5-1と同じく、`#[key]` で1件ずつを見分け、`class!` で完了のクラスを足しています。`key` には、`NoteId` の中の数（`note.id.get()`）を使っています。
- `(note.title)` は、利用者が書いたタイトルです。`view!` が本文の場所に合わせてエスケープするので、タイトルに `<script>` と書かれても、ただの文字として表示されます（例5-2）。

エラーの画面を作る `error_page` は、コンポーネントではない、ふつうの関数です。そのため、`view!` の最初に `cx =>` と書いて、文脈（`Cx`）を渡しています（抜き出しています）。

```rust
    // 普通の関数の中で view! を使うときは、先頭に `cx =>` を書いて文脈を渡す
    Ok(view! {
        cx =>
        (status)
```

- コンポーネント（`#[component]`）や `#[page]` の中では、`cx` は自動で渡ります。ふつうの関数の中では、`cx =>` を書かないと、`view!` が文脈を見つけられずに、``cannot find value `__cx` in this scope``（`__cx` という値が見つかりません）というエラーになります。

### 練習問題

**問1:** 例5-1の `note_card` に、タグがあるときだけ `<span class="tag">タグ</span>` を足してください。`data-tag` 属性とは別に、画面に見える形で出します。

**問2:** 次の3つのボタンは、それぞれどんな HTML になるでしょうか。`maybe_title` は `None`、`is_disabled` と `expanded` は `false` とします。

```rust
<button title=(maybe_title) disabled=(is_disabled)>"A"</button>
<button disabled="false">"B"</button>
<button aria-expanded=(if expanded { "true" } else { "false" })>"C"</button>
```

**問3:** UNIT 04 のエラー3では、自分で作った `NoteId` を `view!` に `(id)` と入れて、`E0277` になりました。`NoteId` に `NodeViewParts` を実装して、`(id)` と書けるようにしてください。

### 解答と解説

**問1の解答:**

`if let Some(tag) = tag { … }` で、タグがあるときだけ中身を取り出して表示します（`src/bin/u05_q1.rs`。変えた所だけを抜き出しています）。

```rust
            // タグがあるときだけ、中身を取り出して表示する
            if let Some(tag) = tag {
                <span class="tag">(tag)</span>
            }
```

```text
@@OUT:u05_q1@@
```

- 1件目にだけ `<span class="tag">買い物</span>` が付きました。`view!` の `if` には、Rust と同じく `if let` も書けます。

**問2の解答:**

```text
@@OUT:u05_q2@@
```

- A は、`title` も `disabled` も書かれていません。`None` と `false` の式は、属性ごと消えます。
- B は `disabled="false"` と書かれ、**ボタンは押せない**状態になります。`disabled` はブール属性なので、値が `"false"` でも、**あるだけで** `true` の意味になるからです。文字で書いた属性は、そのまま出るので、「押せるかどうか」を変えたいときは、必ず `disabled=(bool の式)` で書きます。
- C は `aria-expanded="false"` です。`aria-expanded` は、`"true"` と `"false"` という**文字**で意味が決まる属性なので、`bool` ではなく文字を渡します（`bool` の `false` を渡すと、属性そのものが消えて、「開いているかどうか分からない」という別の意味になってしまいます）。

**問3の解答:**

`NodeViewParts` の `into_view_parts` で、画面に出す文字を `push_string` で渡します（`src/bin/u05_q3.rs`）。

```rust
@@FILE:web_lab/src/bin/u05_q3.rs@@
```

```text
@@OUT:u05_q3@@
```

- `(id)` と書けるようになり、`No.7` と表示されました。表示の形（`No.` を付ける）を、型の側で1か所に決められます。
- `push_string` で入れた文字は、置いた場所に合わせて、Topcoat がエスケープしてくれます。名前に `_unescaped` が付いた方法（エスケープしない）は使いません。
- RustNote は、`NoteId` を画面に出すことがほとんど無く（URL の中だけ）、`NodeViewParts` は実装していません。画面に何度も出す型があるときに、この方法を使います。

### まとめ

- `view!` は HTML とほぼ同じ形で書き、決まった文字は `"…"`、変わる値は `( )`。属性に `Option`・`bool` の式を入れると、`None`・`false` のときは属性ごと消える。`if`・`for`（`#[key]` 付き）・`match`・`let` も使える。
- `#[component]` で部品を作り、名前付きの引数で呼ぶ。引数のあとに書いた部品は `child` に入る。ふつうの関数の中の `view!` には `cx =>` が要る。
- `( )` に入れた値は、置く場所に合わせてエスケープされる（本文は `<`・`>`・`&`、属性の値は `"`・`&`）。属性やタグの名前に危ない文字が来ると、HTML を作らずに panic で止まる。利用者の文字で名前を作らない。
