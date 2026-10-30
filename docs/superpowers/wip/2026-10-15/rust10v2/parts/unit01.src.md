## UNIT 01: 一覧・空のとき・持ち主の条件

### この単元のゴール

- ノートの一覧（`GET /notes`）が、ログインの確かめ → URL のクエリの読み取り → DB の検索 → HTML の組み立て、の順で作られていることを説明できる
- 一覧の SQL に、持ち主の条件（`user_id = ?1`）と、並び順（`ORDER BY updated_at DESC, id DESC`）と、ページの区切り（`LIMIT`・`OFFSET`）が入っている理由を言える
- ノートが 0 件のときの文と、検索で 0 件のときの文を出し分ける `view!` を読み、テストで確かめられる
- 利用者が入力したタイトルが、HTML のタグとしてではなく文字として表示されること（自動エスケープ）を確かめられる

### たとえ話でつかむ

マンションの、共同の宅配ボックスを考えます。ボックスには、住んでいる人全員の荷物が入っています。

- 管理人さんに「私の荷物を見せてください」と頼むと、管理人さんは、まず**部屋の鍵**を見て、本当にその部屋の人かを確かめます。
- 次に、**自分の部屋番号の札が付いた荷物だけ**を、棚から出してきます。隣の部屋の荷物は、たとえ同じ棚にあっても出しません。
- 出してきた荷物は、**新しく届いた順**に並べます。同じ時刻に届いた荷物は、受付の番号の大きいほうを先にします。こうすれば、何度頼んでも、毎回同じ順で並びます。
- 荷物が1つも無いときは、空の机を見せるのではなく「お荷物はありません」と伝えます。
- 荷物が多いときは、20 個ずつ机に並べて、「1 / 2（全 21 個）」と伝えます。

RustNote の一覧の画面も、同じ順番で作ります。

| 宅配ボックス | RustNote の一覧（GET /notes） |
|---|---|
| 部屋の鍵を見て、その部屋の人かを確かめる | `require_user(cx)`（ログインの確かめ。第8巻 UNIT 01） |
| 自分の部屋番号の札が付いた荷物だけを出す | SQL の `WHERE user_id = ?1`（持ち主の条件） |
| 新しく届いた順、同じ時刻なら受付の番号の大きい順 | `ORDER BY updated_at DESC, id DESC` |
| 20 個ずつ並べる | `LIMIT 20 OFFSET …`（`PER_PAGE = 20`） |
| 「お荷物はありません」 | `if page.notes.is_empty()` の文 |
| 「1 / 2（全 21 個）」 | ページ送りの部品（`pagination`）と、件数の SQL（`COUNT(*)`） |

> **用語:** CRUD（クラッド：Create（作る）・Read（読む）・Update（直す）・Delete（消す）の頭文字。データを扱う画面の基本の4つの操作）

<!-- -->

> **用語:** 空の状態（からのじょうたい。empty state：表示するデータが1件も無いときの画面。何も表示しないのではなく、次にすること（「最初の1件を書いてみましょう」）を伝える）

### 図で理解する

`GET /notes` のリクエストが届いてから、HTML を返すまでの流れです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-10-0101" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="20" width="160" height="56" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="90" y="44" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">GET /notes</text>
<text x="90" y="64" font-size="11" fill="#475569" text-anchor="middle">?q=…&amp;page=…・クッキー</text>
<rect x="200" y="20" width="160" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="280" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">① require_user</text>
<text x="280" y="64" font-size="11" fill="#0f172a" text-anchor="middle">誰かを確かめる</text>
<rect x="390" y="20" width="160" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="470" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">② NoteQuery::new</text>
<text x="470" y="64" font-size="11" fill="#0f172a" text-anchor="middle">?q= と ?page= を型に</text>
<rect x="390" y="110" width="320" height="100" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="550" y="134" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">③ notes.search(owner, &amp;query)（DB）</text>
<text x="410" y="160" font-size="11" fill="#0f172a">SELECT COUNT(*) … WHERE user_id = ?1 …</text>
<text x="410" y="180" font-size="11" fill="#0f172a">SELECT … WHERE user_id = ?1 …</text>
<text x="410" y="198" font-size="11" fill="#0f172a">  ORDER BY updated_at DESC, id DESC LIMIT 20 OFFSET …</text>
<rect x="10" y="110" width="340" height="150" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="180" y="134" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">④ notes_index（view! で HTML を組む）</text>
<text x="30" y="162" font-size="11" fill="#0f172a">検索のフォーム・作成のフォーム</text>
<text x="30" y="184" font-size="11" fill="#0f172a">if 0 件 → 「まだノートがありません」</text>
<text x="30" y="200" font-size="11" fill="#0f172a">      （検索中なら「条件に合うノートはありません」）</text>
<text x="30" y="222" font-size="11" fill="#0f172a">else → for note in … { 1行ずつ }</text>
<text x="30" y="244" font-size="11" fill="#0f172a">pagination → 「1 / 2 ページ（全21件）」</text>
<rect x="390" y="230" width="320" height="56" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="550" y="254" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">200 OK（HTML）</text>
<text x="550" y="274" font-size="11" fill="#475569" text-anchor="middle">レイアウト（ヘッダー）に包んで返す</text>
<line x1="170" y1="48" x2="196" y2="48" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0101)"/>
<line x1="360" y1="48" x2="386" y2="48" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0101)"/>
<line x1="470" y1="76" x2="470" y2="106" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0101)"/>
<line x1="390" y1="160" x2="354" y2="160" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0101)"/>
<path d="M350,240 H370 V258 H386" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0101)"/>
<text x="280" y="96" font-size="11" fill="#c2410c" text-anchor="middle">いなければ 303 → /login</text>
<text x="360" y="312" font-size="12" fill="#0f172a" text-anchor="middle">③の SQL は2つ。どちらにも持ち主の条件 user_id = ?1 が入っている</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: 一覧の画面ができるまで。誰かを確かめ、URL のクエリを型にし、持ち主の条件付きの SQL で数えて読み、view! で HTML にする</div>
</div>

- **① と ②** は、ページの関数 `index` の中です。`require_user(cx)` でログイン中のユーザーを確かめ、`query_params` で URL の `?q=`（キーワード）と `?page=`（ページ番号）を読み、`NoteQuery::new` で検査済みの型にします。
- **③** は、リポジトリの `search` です。件数を数える SQL と、そのページの分を読む SQL の2つを送ります。どちらにも `user_id = ?1` が入っています（問2で、片方だけ忘れたらどうなるかを確かめます）。
- **④** は、コンポーネントの `notes_index` です。読んだノートが 0 件か、そうでないかで、表示を分けます。

並び順の決まりは、次の図のとおりです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-10-0102" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="160" y="24" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">DB の notes（作った順）</text>
<rect x="30" y="40" width="260" height="40" rx="6" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="46" y="65" font-size="12" fill="#0f172a">id 1「1件目」 updated_at 10:00:00</text>
<rect x="30" y="90" width="260" height="40" rx="6" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="46" y="115" font-size="12" fill="#0f172a">id 2「2件目」 updated_at 10:00:00</text>
<rect x="30" y="140" width="260" height="40" rx="6" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="46" y="165" font-size="12" fill="#0f172a">id 3「3件目」 updated_at 10:00:00</text>
<text x="560" y="24" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">一覧（updated_at DESC, id DESC）</text>
<rect x="430" y="40" width="260" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="446" y="65" font-size="12" fill="#0f172a">① 3件目（id 3）</text>
<rect x="430" y="90" width="260" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="446" y="115" font-size="12" fill="#0f172a">② 2件目（id 2）</text>
<rect x="430" y="140" width="260" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="446" y="165" font-size="12" fill="#0f172a">③ 1件目（id 1）</text>
<line x1="300" y1="110" x2="420" y2="110" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0102)"/>
<text x="360" y="98" font-size="11" fill="#475569" text-anchor="middle">時刻が同じなら</text>
<text x="360" y="130" font-size="11" fill="#475569" text-anchor="middle">id の大きい順</text>
<text x="360" y="212" font-size="12" fill="#0f172a" text-anchor="middle">updated_at は秒までしか持たない → 同じ秒に作ると「同点」になる。id で必ず決着させる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: 一覧の並び順。最後に更新した時刻（updated_at）の新しい順に並べ、同じ時刻のときは id の大きい順にする。id は重ならないので、順番が必ず1つに決まる</div>
</div>

- `updated_at` は、`datetime('now')` で入れた「秒まで」の時刻です。テストのように速く続けて作ると、3件とも同じ時刻になります。
- そこで、2つ目の並べ方として `id DESC` を足しています。`id` は1件ごとに違う数なので、同じ時刻のノートも、必ず決まった順番に並びます。

### まず動かす最小コード

#### 例1-1: 一覧の画面を、3つの場面で確かめる

> **▼ このコードがやること（先に日本語で）:** 3つのテストで、一覧の画面（`GET /notes`）の HTML を調べます。1つ目は、ノートが 0 件のときの文、3件作ったときの並び順とページ送りの文、bob から見た一覧です。2つ目は、3件目の更新の時刻だけを1日前にずらしたときの並び順です。3つ目は、タグを含むタイトルが、文字として表示されるかです。

```rust
@@FILE:rustnote/tests/u01_list.rs@@
```

コピーした `rustnote` フォルダの `tests/u01_list.rs` として保存して実行します。

```powershell
cargo test --test u01_list -- --nocapture --test-threads=1
```

**▼ 実行結果**

```text
@@OUTGREP:u01_list:^(?!   Compiling)@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 0 件のときは、空の文を出す

- ノートが 0 件のとき、一覧には「まだノートがありません。最初の1件を書いてみましょう。」が出ました（`空の文が出る: true`）。何も表示しない画面だと、利用者は「読み込みに失敗したのか」「本当に 0 件なのか」を区別できません。次にすることまで書いておくのが、空の状態の作り方です。
- bob の一覧も、空の文が出て、alice のノート（`1件目`）は出ませんでした。ページ送りも `1 / 1 ページ（全0件）` で、alice の 3 件は数えられていません。

##### 解説2: 新しい順、同じ時刻なら id の大きい順

- 3件を続けて作ると、並び順は `["3件目", "2件目", "1件目"]` でした。3件は同じ秒に作られたので、`updated_at` は同じです。`id DESC` で、あとから作った（id の大きい）ノートが上に来ました。
- 3件目の `updated_at` を1日前にずらすと、並び順は `["2件目", "1件目", "3件目"]` になりました。`updated_at` が古い 3件目は、id が大きくても下に移ります。並び順の1つ目の決まりは `updated_at`、2つ目が `id` です。
- `order` は、テストのための小さな道具です。HTML の中で、それぞれのタイトルが何文字目に出てくるかを調べ、前から順に並べ直しています。

##### 解説3: タイトルは、タグではなく文字として出る

- タイトルに `<b>太字</b> & <script>` を入れると、HTML の中には `&lt;b&gt;太字&lt;/b&gt; &amp; &lt;script&gt;` と書かれていました（`タグのまま入っている: false`）。`<` が `&lt;`、`>` が `&gt;`、`&` が `&amp;` に置き換わっています。ブラウザには、`<b>太字</b>` という**文字**が、そのまま表示されます。
- これは、RustNote の `view!` の `(note.title)` のように、`( )` で値を入れた所が、自動で**エスケープ**（HTML で特別な意味を持つ文字を、ただの文字の書き方に置き換えること）されるからです。利用者が `<script>` を書いても、プログラムとして動きません（`src/views.rs` の先頭のコメント）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 200" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-10-0103" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="40" width="150" height="80" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="85" y="64" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">フォームの入力</text>
<text x="85" y="94" font-size="12" fill="#0f172a" text-anchor="middle">&lt;b&gt;太字&lt;/b&gt;</text>
<rect x="190" y="40" width="150" height="80" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="265" y="64" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">DB の title</text>
<text x="265" y="94" font-size="12" fill="#0f172a" text-anchor="middle">&lt;b&gt;太字&lt;/b&gt;</text>
<text x="265" y="112" font-size="11" fill="#475569" text-anchor="middle">（そのまま保存）</text>
<rect x="370" y="40" width="170" height="80" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="455" y="64" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">view! の (note.title)</text>
<text x="455" y="92" font-size="11" fill="#0f172a" text-anchor="middle">&amp;lt;b&amp;gt;太字&amp;lt;/b&amp;gt;</text>
<text x="455" y="112" font-size="11" fill="#475569" text-anchor="middle">（自動でエスケープ）</text>
<rect x="570" y="40" width="140" height="80" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="640" y="64" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">ブラウザの表示</text>
<text x="640" y="94" font-size="12" fill="#0f172a" text-anchor="middle">&lt;b&gt;太字&lt;/b&gt;</text>
<text x="640" y="112" font-size="11" fill="#475569" text-anchor="middle">（文字として見える）</text>
<line x1="160" y1="80" x2="186" y2="80" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0103)"/>
<line x1="340" y1="80" x2="366" y2="80" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0103)"/>
<line x1="540" y1="80" x2="566" y2="80" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0103)"/>
<text x="360" y="160" font-size="12" fill="#0f172a" text-anchor="middle">保存するときは入力のまま、HTML にするときにエスケープする（太字にはならず、&lt;script&gt; も動かない）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: タイトルにタグを入れたときの流れ。DB には入力のまま保存し、view! が HTML にするときに置き換えるので、ブラウザでは文字として表示される</div>
</div>

> **用語:** エスケープ（escape：特別な意味を持つ文字を、ただの文字として扱われる書き方に置き換えること。HTML では `<` を `&lt;` にする）

### もう一歩: 実務に近い形

#### 例1-2: 並べ方に id を足さないと、どうなるか

`ORDER BY updated_at DESC` だけでは、同じ時刻のノートの順番は決まりません。本当にそうなるのか、DB に直接 SQL を送って確かめます。

> **▼ このコードがやること（先に日本語で）:** 一時フォルダの DB に、同じ秒のうちに3件のノートを入れます。`updated_at` の値を表示してから、`ORDER BY updated_at DESC` だけの SQL と、`, id DESC` を足した SQL で、タイトルを並べて表示します。

```rust
@@FILE:rustnote/examples/u01_order.rs@@
```

```powershell
cargo run -q --example u01_order
```

**▼ 実行結果**

```text
@@OUT:u01_order@@
```

- 3件の `updated_at` は、どれも同じ秒でした。そして、この実験では、`id DESC` を足しても足さなくても、同じ `["3件目", "2件目", "1件目"]` になりました。
- では、`id DESC` は要らないのでしょうか。いいえ、要ります。SQL の決まりでは、`ORDER BY` の値が同じ行どうしの順番は**決まっていません**。この実験で同じ順になったのは、今の SQLite が、同じ値の行を、たまたま行の番号（id）の大きい順に返したからです。索引を足したり、DB を Turso に変えたり、SQLite の版が上がったりしたときに、同じ順になる保証はありません。
- 並び順が決まっていないと、ページ送りで困ります。1ページ目と2ページ目で同点の行の順番が入れ替わると、同じノートが2つのページに出たり、どのページにも出なかったりします。だから、**最後に、重ならない列（`id`）を足して、順番を必ず1つに決める**のです。結果が同じでも、「たまたま」ではなく「決まり」で同じにしておきます。

> **注意:** 「試したら同じ順だったから大丈夫」と考えるのは危険です。テストは、今の DB の作りでの1回の結果しか確かめられません。SQL の決まりで保証されていないことは、コードで保証します。

### よくあるエラーと読み方

#### エラー1: for で回したあとに、同じ値を使った（E0382）

`notes_index` の `view!` の中では、ノートを `for note in page.notes.clone()` で1行ずつ並べています。`.clone()`（複製）は無駄に見えるので、外してみた場合です。試すときは、練習用のコピーの `src/routes/notes.rs` を書き換え、確かめたら必ず元に戻してください。

```rust
                for note in page.notes {
```

```powershell
cargo build
```

```text
@@OUT:u01_err_moved@@
```

- **何が起きたか:** 「一部が移動した値を使いました」というエラーです。89 行目の `for note in page.notes` で、`page` の中の `notes`（`Vec<Note>`）が for の中へ**移動**しました（`note:` が、for は `into_iter()` を呼んで、値の持ち主を移す、と説明しています）。そのあと、104 行目の `pagination(page: page, ...)` で `page` を丸ごと渡そうとしたので、「`notes` はもう `page` の中に無い」と止められました（第3巻）。
- **直し方:** RustNote のとおり `page.notes.clone()` で、ノートの一覧を複製してから for に渡します。`page` の中の `notes` は残るので、ページ送りの部品にも `page` を丸ごと渡せます。

#### エラー2: help の案内どおりに & を付けた（E0507・E0505）

エラー1の `help:` は、「`&page.notes` のように、借りて回してはどうか」と案内しています。案内どおりに書き換えた場合です。

```rust
                for note in &page.notes {
```

```text
@@OUTHEAD:u01_err_borrow:2@@
...
@@OUTTAIL:u01_err_borrow:3@@
```

（エラーの全文は長いので、最初の2行と最後の3行だけを載せています。間に、`note.updated_at` について同じ形の `E0507` と、`E0505` が続きます）

- **何が起きたか:** 今度は「共有の参照の後ろにある `note.title` からは、値を移動できません」というエラーが、`note.title` と `note.updated_at` で2つ、さらに `E0505`（借りている値を移動しようとした）が1つ出ました。Topcoat の `view!` は、`(note.title)` のように入れた値を**持ち主ごと受け取って** HTML にするので、借りただけの `note` からは渡せないのです。
- **学べること:** コンパイラーの `help:` は、そのエラーを消すための1つの案です。プログラム全体の作り（ここでは、`view!` が値を受け取る形）までは見ていないので、案内どおりに直すと、別のエラーになることがあります。エラーを1つずつ読んで、**なぜ移動が起きるのか**を考えるのが近道です。
- **どちらを選ぶか:** 1ページに並ぶのは多くても 20 件なので、`clone()` で複製しても、速さはほとんど変わりません。読みやすさを優先して、RustNote は `clone()` にしています。確かめたら、`src/routes/notes.rs` を元に戻します。

### RustNote ではここで使う

一覧のページの関数 `index` です（`src/routes/notes.rs` から抜き出しています）。

```rust
#[page]
pub async fn index(cx: &Cx) -> Result<impl View> {
    let user = require_user(cx).await?;
    let params = query_params::<NotesParams>(cx)?;
    let query = NoteQuery::new(params.q.as_deref(), params.page);
    let flash = flash::take(cx);

    Ok(view! {
        flash_banner(flash: flash)
        notes_index(owner: user.id, query: query, form: NoteFormState::default())
    })
}
```

- 最初の行で `require_user(cx)` を呼びます。ログインしていなければ、ここで 303 になり、下の行は動きません。
- `notes_index` に渡すのは、`owner: user.id` です。一覧の部品は、持ち主を引数で受け取るので、「誰の一覧か」を、部品の中で勝手に決めることができません。

ノートを1行ずつ並べるところです（`notes_index` の `view!` から抜き出しています）。

```rust
        if page.notes.is_empty() {
            <p class="empty">
                if keyword.is_some() {
                    "条件に合うノートはありません。"
                } else {
                    "まだノートがありません。最初の1件を書いてみましょう。"
                }
            </p>
        } else {
            <ul class="note-list">
                #[key(note.id.get())]
                for note in page.notes.clone() {
                    <li class=(class!("note-item", "done" if note.done))>
                        <a href=(href!(id::show, id::Id(note.id)))>(note.title)</a>
                        if note.done {
                            <span class="badge">"完了"</span>
                        }
                        <span class="updated">
                            "更新: "
                            (note.updated_at)
                        </span>
                    </li>
                }
            </ul>
        }
```

- `view!` の中には、Rust の `if` と `for` を、そのまま書けます。`#[key(note.id.get())]` は、1行ずつに「このノートの番号」という目印を付ける指定です。
- `class!("note-item", "done" if note.done)` は、完了したノートだけに `done` のクラスを足します（問1）。
- リンク先は `href!(id::show, id::Id(note.id))` で作ります。URL を文字列で `"/notes/" + id` と組み立てるのではなく、詳細のページの関数（`id::show`）と、その ID の型を指定するので、ページを消したり名前を変えたりすると、コンパイルエラーで気づけます（第9巻 UNIT 04）。

DB を検索するところは、図の③で見た SQL です（`src/repository/notes.rs` から抜き出しています）。

```rust
        // LIKE の中の \ を「次の文字をそのまま扱う」という意味にする（ESCAPE '\'）
        let condition = "user_id = ?1 AND (title LIKE ?2 ESCAPE '\\' OR body LIKE ?2 ESCAPE '\\')";

        // 1) 条件に合う件数（ページ数の計算に使う）
        let count_sql = format!("SELECT COUNT(*) FROM notes WHERE {condition}");
```

```rust
        // 2) このページに表示する分（新しく更新した順）
        let list_sql = format!(
            "SELECT {NOTE_COLUMNS} FROM notes WHERE {condition}
             ORDER BY updated_at DESC, id DESC
             LIMIT ?3 OFFSET ?4"
        );
```

- 持ち主の条件を含む `condition` を、件数の SQL と一覧の SQL の**両方で使い回して**います。条件を1か所に書くので、片方だけ書き忘れる心配がありません（問2）。
- キーワードが無いときは、`?2` に `%`（何にでも一致する）を入れます。キーワードの中の `%` や `_` をただの文字として扱う `escape_like` は、UNIT 07 で扱います。

### 練習問題

**問1:** 3件のノートを作り、3件とも「最後に更新したのは1時間前」にしてから、1件目だけを完了にします（`POST /notes/1/toggle`）。一覧の並び順と、完了の印（`class="note-item done"` と「完了」のバッジ）は、どうなるでしょうか。予想してから、テストで確かめてください。

**問2:** もし、件数を数える SQL にだけ持ち主の条件を書き忘れたら、alice が 3 件、bob が 0 件のとき、bob の一覧のページ送りには何と表示されるでしょうか。DB に直接 SQL を送る小さなプログラムで確かめてください。

**問3:** RustNote の一覧は、ノートが 0 件のときと、検索で 0 件のときで、出す文を変えています。それぞれ、どんな文が出るかを確かめてください。また、文を分けている理由を説明してください。

### 解答と解説

**問1の解答:**

`tests/u01_q1.rs` です。

```rust
@@FILE:rustnote/tests/u01_q1.rs@@
```

```powershell
cargo test --test u01_q1 -- --nocapture
```

```text
@@OUT:u01_q1@@
```

- 1件目がいちばん上に移りました。完了の切り替え（`toggle_done`）の SQL は、`done` を反対にするのと一緒に、`updated_at = datetime('now')` で更新の時刻を今にするからです（`src/repository/notes.rs`）。残りの2件は、どちらも1時間前の同じ時刻なので、`id DESC` で 3件目、2件目の順です。
- 完了の印の付いた行と「完了」のバッジは、それぞれ 1 つずつでした。完了にしたのは1件目だけだからです。
- 「完了にしただけで、一覧の上に移る」のが使いやすいかどうかは、アプリの決めごとです。完了したノートを下に集めたいなら、`ORDER BY done, updated_at DESC, id DESC` のように、`done`（0 が未完了、1 が完了）を最初に並べます。

**問2の解答:**

`examples/u01_q2.rs` です。

```rust
@@FILE:rustnote/examples/u01_q2.rs@@
```

```powershell
cargo run -q --example u01_q2
```

```text
@@OUT:u01_q2@@
```

- bob の一覧に並ぶノートは 0 件なのに、件数の SQL に持ち主の条件が無いと、ページ送りには `1 / 1 ページ（全3件）` と出ます。画面には何も並んでいないのに「全3件」と出るので、利用者は混乱します。
- それだけではありません。bob は、件数の数字から「ほかの人が全部で何件ノートを書いているか」を知ることができます。ノートが 21 件を超えると、「次へ」のリンクまで出て、押すと空のページが出ます。**件数も、他人のデータの一部**です。
- RustNote は、持ち主の条件を `condition` という1つの文字列にまとめ、2つの SQL で使い回しているので、この間違いが起きにくくなっています。

**問3の解答:**

`tests/u01_q3.rs` です。

```rust
@@FILE:rustnote/tests/u01_q3.rs@@
```

```powershell
cargo test --test u01_q3 -- --nocapture
```

```text
@@OUT:u01_q3@@
```

- alice は「買い物」のノートを持っているので、`/notes` には空の文が出ません。「宿題」で検索すると「条件に合うノートはありません。」、ノートを1件も持っていない bob には「まだノートがありません。最初の1件を書いてみましょう。」が出ました。
- **文を分ける理由:** 検索で 0 件のときに「まだノートがありません」と出すと、利用者は「自分のノートが消えた」と驚きます。反対に、本当に 0 件の人に「条件に合うノートはありません」と出すと、何を直せばよいのか分かりません。**0 件になった理由ごとに、次にすること**（キーワードを変える・最初の1件を書く）を伝えるのです。`view!` の中の `if keyword.is_some()` が、この出し分けをしています（第9巻 UNIT 02 の画面の状態）。

### まとめ

- 一覧（`GET /notes`）は、`require_user` で誰かを確かめ、URL のクエリを `NoteQuery` にし、持ち主の条件付きの2つの SQL（件数と、そのページの分）で読み、`view!` で HTML にする。持ち主の条件は1か所に書いて、2つの SQL で使い回す。
- 並び順は `ORDER BY updated_at DESC, id DESC`。同じ時刻の行の順番は SQL では決まっていないので、試して同じでも、最後に重ならない `id` を足して決める。
- 0 件のときは、理由ごとに次にすることを伝える文を出す。`view!` に `( )` で入れた値は自動でエスケープされ、利用者のタイトルはタグではなく文字として表示される。
