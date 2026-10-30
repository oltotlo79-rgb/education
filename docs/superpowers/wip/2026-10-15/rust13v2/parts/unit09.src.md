## UNIT 09: 見た目

### この単元のゴール

- `assets/style.css` の組み立て（色の変数・全体・ヘッダー・本文の枠・ボタンとフォーム・お知らせ・一覧・詳細）を、HTML のクラスの名前と結びつけて説明できる
- CSS が実際にどう効いたか（幅・はみ出し・色・打ち消し線・改行の扱い）を、本物の Chrome で測って確かめられる
- `asset!` と `topcoat asset bundle` が、CSS を「中身から作った印（ハッシュ）付きの URL」で、1年間キャッシュさせて配信するしくみを説明できる
- CSS のまちがい（ファイルの名前・閉じ括弧・変数の名前）は、`cargo build` では見つからないことを、実験で確かめられる

### たとえ話でつかむ

お店の、内装の指示書を考えます。

- 指示書の最初に、**使う色の見本帳**（`:root` の変数）を作ります。壁・文字・ボタンの色は、見本帳の番号で指示するので、色を変えるときは見本帳だけを直せば済みます。
- 売り場の床は、**幅 760 の通路**（`.container`）を真ん中に通し、狭い店（スマホ）では、壁から 16 ずつ離します。
- 看板（ヘッダー）は、店の名前を左、案内を右に並べます。ところが、案内の札が多いと、**狭い店では看板が壁からはみ出します**（例9-2・問1）。
- 指示書を書き換えたら、**表紙の版の番号**（ハッシュ）も変わります。お客さんは「同じ版の指示書なら、1年間は取り寄せ直さない」と決めているので、版の番号が変わることで、新しい指示書に気づけます（問3）。
- 指示書の**閉じ括弧を1つ書き忘れる**と、そのあとの指示が全部「その棚の中だけの話」と読まれ、ほかの売り場に効かなくなります。しかも、だれもエラーだとは言ってくれません（エラー2）。

| 内装の指示書 | RustNote（`assets/style.css`） |
|---|---|
| 色の見本帳 | `:root { --color-primary: #c2410c; … }` |
| 見本帳の番号で指示する | `color: var(--color-primary);` |
| 真ん中の通路 | `.container { max-width: 760px; margin: 24px auto; padding: 0 16px; }` |
| 看板（左右に並べる） | `.site-header { display: flex; justify-content: space-between; }` |
| 版の番号付きの表紙 | `/_topcoat/assets/style-e9b983efabcfc6c3.css` |
| 1年間は取り寄せ直さない | `cache-control: public, max-age=31536000, immutable` |

> **用語:** CSS の変数（カスタムプロパティ：`--color-text: #0f172a;` のように `--` で始まる名前で値を決め、`var(--color-text)` で使う。RustNote は色を全部 `:root`（文書のいちばん外側）に置く）

<!-- -->

> **用語:** コントラスト比（文字の色と背景の色の明るさの比。WCAG（Web のアクセシビリティの決まり）の AA では、ふつうの大きさの文字は 4.5 : 1 以上）

### 図で理解する

画面の幅ごとの、ページの枠と、ヘッダーのはみ出しです（例9-2の結果。メールは `alice.long.address@example.com`）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="20" y="22" font-size="13" font-weight="700" fill="#0f172a">幅 1280px（パソコン）</text>
<rect x="20" y="32" width="420" height="240" rx="4" fill="#f8fafc" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="20" y="32" width="420" height="30" fill="#ffffff" stroke="#e2e8f0"/>
<text x="32" y="52" font-size="11" font-weight="700" fill="#0f172a">RustNote</text>
<text x="428" y="52" font-size="10" fill="#c2410c" text-anchor="end">ノート一覧　alice.long…　ログアウト</text>
<rect x="105" y="76" width="250" height="186" rx="4" fill="#ffffff" stroke="#3b82f6" stroke-width="1.5" stroke-dasharray="5 3"/>
<text x="230" y="94" font-size="11" fill="#1d4ed8" text-anchor="middle">.container（760px・真ん中）</text>
<rect x="117" y="104" width="226" height="22" rx="4" fill="#ffffff" stroke="#e2e8f0"/>
<text x="125" y="119" font-size="10" fill="#c2410c">買い物</text>
<text x="335" y="119" font-size="9" fill="#64748b" text-anchor="end">更新: …</text>
<rect x="117" y="132" width="226" height="22" rx="4" fill="#ffffff" stroke="#e2e8f0"/>
<text x="125" y="147" font-size="10" fill="#64748b" text-decoration="line-through">宿題</text>
<rect x="150" y="139" width="26" height="12" rx="6" fill="#ecfdf5"/>
<text x="163" y="149" font-size="8" fill="#047857" text-anchor="middle">完了</text>
<text x="230" y="180" font-size="10" fill="#64748b" text-anchor="middle">1 / 1 ページ（全3件）</text>
<text x="230" y="230" font-size="10" fill="#475569" text-anchor="middle">横のはみ出し なし</text>
<text x="490" y="22" font-size="13" font-weight="700" fill="#0f172a">幅 375px（スマホ）</text>
<rect x="490" y="32" width="150" height="240" rx="4" fill="#f8fafc" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="490" y="32" width="207" height="30" fill="#fef2f2" stroke="#ef4444" stroke-width="1.5"/>
<text x="496" y="52" font-size="10" font-weight="700" fill="#0f172a">RustNote</text>
<text x="694" y="52" font-size="9" fill="#c2410c" text-anchor="end">一覧 alice… ログアウト</text>
<line x1="640" y1="70" x2="697" y2="70" stroke="#ef4444" stroke-width="1.5"/>
<text x="668" y="84" font-size="10" fill="#b91c1c" text-anchor="middle">+17px</text>
<rect x="498" y="96" width="134" height="22" rx="4" fill="#ffffff" stroke="#e2e8f0"/>
<text x="504" y="111" font-size="10" fill="#c2410c">買い物</text>
<text x="565" y="150" font-size="10" fill="#b91c1c" text-anchor="middle">ヘッダーの中身 392px</text>
<text x="565" y="166" font-size="10" fill="#b91c1c" text-anchor="middle">＞ 画面 375px</text>
<text x="565" y="190" font-size="10" fill="#475569" text-anchor="middle">→ 画面全体が</text>
<text x="565" y="206" font-size="10" fill="#475569" text-anchor="middle">横にずれる</text>
<text x="565" y="240" font-size="10" fill="#047857" text-anchor="middle">flex-wrap: wrap で</text>
<text x="565" y="256" font-size="10" fill="#047857" text-anchor="middle">折り返すと解決（問1）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: 画面の幅とページの枠。パソコンでは 760px の枠が真ん中に来る。スマホでは、ヘッダーの中身（392px）が画面からはみ出し、ページ全体に横のスクロールができる</div>
</div>

CSS が、ハッシュ付きの URL で配信されるまでです（例9-2・問3の結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 200" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-0902" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="20" width="150" height="56" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="85" y="44" font-size="12" fill="#0f172a" text-anchor="middle">assets/style.css</text>
<text x="85" y="64" font-size="10" fill="#475569" text-anchor="middle">自分で書いた CSS</text>
<rect x="190" y="20" width="160" height="56" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="270" y="44" font-size="11" fill="#1d4ed8" text-anchor="middle">asset!("assets/style.css")</text>
<text x="270" y="64" font-size="10" fill="#1d4ed8" text-anchor="middle">src/views.rs の STYLESHEET</text>
<rect x="380" y="20" width="150" height="56" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="455" y="44" font-size="11" fill="#047857" text-anchor="middle">topcoat asset bundle</text>
<text x="455" y="64" font-size="10" fill="#047857" text-anchor="middle">実行ファイルの隣の assets/</text>
<rect x="560" y="20" width="150" height="56" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="635" y="40" font-size="10" fill="#c2410c" text-anchor="middle">/_topcoat/assets/</text>
<text x="635" y="56" font-size="10" fill="#c2410c" text-anchor="middle">style-e9b983efabcfc6c3.css</text>
<text x="635" y="70" font-size="9" fill="#c2410c" text-anchor="middle">max-age=31536000, immutable</text>
<line x1="160" y1="48" x2="186" y2="48" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0902)"/>
<line x1="350" y1="48" x2="376" y2="48" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0902)"/>
<line x1="530" y1="48" x2="556" y2="48" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0902)"/>
<rect x="10" y="100" width="700" height="84" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="1.5"/>
<text x="24" y="122" font-size="12" fill="#0f172a">CSS を1文字変える → 印が style-d087c1d6e359e4b6 に変わる → HTML の &lt;link&gt; も新しい URL になる</text>
<text x="24" y="146" font-size="12" fill="#0f172a">ブラウザは、同じ URL の CSS を 1年間（31,536,000 秒）取り寄せ直さないが、URL が変われば新しく取りに来る</text>
<text x="24" y="170" font-size="12" fill="#b91c1c">前の URL は 404。ファイルの名前の書き間違いは cargo build では見つからず、bundle で見つかる（エラー1）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: CSS の配信。中身から作った印を URL に入れるので、長くキャッシュさせても、変えた CSS はすぐに届く</div>
</div>

### まず動かす最小コード

#### 例9-1: assets/style.css の全文

> **▼ このファイルがやること（先に日本語で）:** RustNote の見た目を、素の CSS（Tailwind などの道具を使わない）で決めます。最初に色の変数を `:root` にまとめ、そのあと、全体・ヘッダー・本文の枠・ボタンとフォーム・お知らせ・一覧・詳細・トップとエラー画面の順に、HTML のクラスの名前（UNIT 07・08 の `view!` で付けたもの）ごとに書きます。

```css
@@FILE:rustnote/assets/style.css@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 色は変数にまとめる

- `:root` に、文字・薄い文字・線・背景・カード・主な色（濃い橙）・成功・注意・危険の色を、`--color-…` の名前で置きます。ほかの規則は、`var(--color-primary)` のように、名前で使います。色を変えるときは、`:root` の1か所を直せば、リンク・ボタン・お知らせが、まとめて変わります。
- 名前を書き間違えても、CSS はエラーを出さず、その指定が無いのと同じになります（エラー3）。

##### 解説2: 全体と、本文の枠

- `* { box-sizing: border-box; }` は、幅に、内側の余白（`padding`）と線（`border`）を含めて数える指定です。`input` の `width: 100%` に `padding` を足しても、枠からはみ出しません。
- `body` の `font-family` は、OS ごとの日本語の文字（`"Hiragino Sans"`・`"Noto Sans JP"` など）を、使える順に並べています。`line-height: 1.7` は、日本語が読みやすいよう、行の間を広めにしています。
- `.container` は、`max-width: 760px` と `margin: 24px auto`（左右を自動）で、パソコンでは真ん中に 760px の枠を作ります。スマホでは、画面の幅いっぱいになり、`padding: 0 16px` で、端から 16px 離します（例9-2）。

##### 解説3: ヘッダー・ボタン・フォーム

- `.site-header` は `display: flex` と `justify-content: space-between` で、アプリの名前を左、`nav`（リンク・メール・ログアウト）を右に並べます。`nav` も `display: flex` で、`gap: 16px` ずつ空けて横に並べます。折り返しの指定（`flex-wrap`）が無いので、狭い画面では、はみ出します（問1）。
- `button` と `.button` は、同じ見た目（橙の背景・白い文字・角の丸み）です。`.secondary`（白い背景・橙の文字）と `.danger`（赤い線と文字）は、目立たせ方を変えます。`.link-button` は、ログアウトのボタン（`POST` のフォーム）を、リンクのように見せます（UNIT 07）。
- `label` は `display: block` で、入力欄の上に1行で置きます。`input` と `textarea` は `font: inherit` で、ブラウザの入力欄の文字の大きさではなく、ページと同じ文字にします。
- `.field-error`（欄の下の赤い文）・`.form-notice`（フォームの上の注意。409 の知らせなど）・`.flash.success` と `.flash.warning`（お知らせ）は、UNIT 06〜08 で出てきた部品の見た目です。

##### 解説4: 一覧と詳細

- `.note-list li` は、1件ずつ、白いカード（線・角の丸み・余白）にします。`display: flex` と `align-items: baseline` で、タイトル・「完了」の印・更新日時を、文字の下の線をそろえて横に並べ、`.updated` の `margin-left: auto` で、更新日時だけを右端に寄せます。
- `.note-list li.done a` は、完了したノートのタイトルを、薄い色と打ち消し線にします（UNIT 08 の `class!("note-item", "done" if note.done)`）。
- `.badge` は、`border-radius: 999px` で、両端が丸い「完了」の印にします。
- `.note-body` の `white-space: pre-wrap` は、本文の改行と空白を、書いたとおりに表示し、枠の端では折り返す指定です。HTML の中では改行が `\n` のままなので（UNIT 08）、この指定が無いと、改行が1つの空白になります（エラー2）。`overflow-wrap: anywhere` は、長い URL のような、区切りの無い文字の並びも、どこででも折り返します（例9-2）。

### もう一歩: 実務に近い形

#### 例9-2: 本物の Chrome で、CSS が効いた結果を測る

release の実行ファイルを一時フォルダで起動し、まず、CSS の配信を `curl` で確かめました。

```text
@@OUT:u09_serve@@
```

- HTML の `<link>` は、印（`e9b983efabcfc6c3`）付きの URL を指していました。応答は `text/css` で、`cache-control: public, max-age=31536000, immutable`（だれが持ってもよい・1年間・中身は変わらない）でした。
- 中身は、書いたとおりの CSS（コメントも残る）で、縮めたり（minify）はしていません。印を変えた URL は 404 でした。

次に、Playwright（第11巻 UNIT 05）で、インストール済みの Chrome を動かし、登録してノートを3つ作り（1つは長い URL の本文・1つは完了）、画面の幅を 1280px・375px・320px に変えながら、要素の幅・横のはみ出し・CSS が効いた結果（`getComputedStyle`）を測りました。最後に、axe でアクセシビリティを検査しました。

```javascript
@@FILE:u09lab/measure.mjs@@
```

```powershell
node measure.mjs all
```

```text
@@OUT:u09_measure@@
```

- 1280px では、`.container` は 760px で、はみ出しはありませんでした。長い URL の本文も、`overflow-wrap: anywhere` で折り返され、`.note-body` は枠の中（686px）に収まりました。
- 375px と 320px では、**ヘッダーの中身が 392px** になり、画面の幅を超えて、ページ全体に横のはみ出しができました。`.container` と本文は、画面の幅に合わせて縮んでいます。原因はヘッダーだけです（問1）。
- 完了のリンクは、薄い色（`rgb(100, 116, 139)` ＝ `#64748b`）と打ち消し線、ふつうのリンクは、橙（`rgb(194, 65, 12)` ＝ `#c2410c`）と下線でした。本文は `pre-wrap` で、2行（牛乳・卵）の高さ 54px でした。
- axe は、一覧で `landmark-unique`（名前の無い `nav` が2つあり、区別できない）を1か所見つけました（UNIT 07 の改善点・第11巻 UNIT 05）。色のコントラストの問題は、ありませんでした。

色の組ごとのコントラスト比を、`:root` の色から計算しました（`contrast13.py`、WCAG 2 の式）。

```text
@@OUT:u09_contrast@@
```

- 10 の組は、どれも AA（4.5 : 1 以上）でした。いちばん低いのは、薄い文字を背景の上に置いたとき（4.55 : 1）で、一覧の「ノートがありません」やページ送りの文字が、これに当たります。

### よくあるエラーと読み方

#### エラー1: `asset!` のファイルの名前を書き間違えた

教材のコードのコピーで、`asset!("assets/style.css")` を `asset!("assets/styles.css")`（s が1つ多い）にして、`cargo build` と `topcoat asset bundle --release` をしました。

```text
@@OUT:u09_err1_build@@
```

```text
@@OUT:u09_err1_bundle@@
```

- **何が起きたか:** `cargo build` は通りました（`exit=0`）。`topcoat asset bundle` で初めて「`assets\styles.css` が見つかりません」（os error 2）と止まりました。
- **読み方:** `asset!` は、コンパイルのときにはファイルがあるかを確かめず、束を作るときに読みます。だから、CI（UNIT 10）や公開の手順（第12巻）で、`topcoat asset bundle` まで動かして、初めて見つかります。
- **直し方:** 名前を、`assets/` の中の本当のファイルの名前に合わせます。

#### エラー2: 閉じ括弧を1つ書き忘れた（エラーが出ない）

コピーの `style.css` で、`.empty { … }` の閉じ括弧 `}` を消して、`cargo build` と束を作り直し、Chrome で測りました。

```text
@@OUT:u09_err2_build@@
```

```text
@@OUT:u09_err2_measure@@
```

- **何が起きたか:** ビルドも、束も、ブラウザも、エラーを出しませんでした。ところが、`.empty` より**あと**に書いた `.pagination` は `display: block`（横に並ばない）、`.note-body` は `white-space: normal`（改行が消える）になりました。`.empty` より**前**の `.search-form` は、`display: flex` のままでした。
- **読み方:** 今のブラウザの CSS は、規則の中に規則を書ける（入れ子）ので、閉じ忘れた `.empty {` のあとの規則は、全部「`.empty` の中の `.pagination`」のように読まれ、どの要素にも当たらなくなります。
- **見つけ方:** CSS は、まちがえても止まらないので、例9-2のように、ブラウザで「効いた結果」を測るか、CSS を検査する道具（stylelint など）を CI で動かします。

#### エラー3: CSS の変数の名前を書き間違えた（エラーが出ない）

コピーの `style.css` で、`a { color: var(--color-primary); }` を `var(--color-primay)`（r が1つ無い）にして、Chrome で測りました。

```text
@@OUT:u09_err3_measure@@
```

- **何が起きたか:** リンクの色が、橙ではなく、本文と同じ `rgb(15, 23, 42)`（`#0f172a`）になりました。無い変数を使った `color` は、「指定が無い」のと同じになり、親（`body`）の色を引き継いだからです。
- **直し方:** 変数の名前を `:root` の名前に合わせます。リンクが本文と同じ色だと、どこがリンクか分かりにくくなるので、見た目のテスト（例9-2の測り方）で、主な部品の色を確かめておくと安心です。

### RustNote ではここで使う

- **CSS の読み込み:** `src/views.rs` の `STYLESHEET`（`asset!`）を、画面の枠（`root_layout`）の `<link rel="stylesheet" href=(STYLESHEET)>` で使います（UNIT 07）。
- **クラスの名前:** `site-header`・`flash`・`field-error`・`note-form`・`note-list`・`badge`・`pagination`・`note-body` など、`view!` で付けたクラスの名前と、CSS の名前を合わせています。名前を片方だけ変えると、エラー2・3と同じく、何も言われずに見た目だけが崩れます。
- **テスト:** `tests/common/mod.rs` は、本物の束の代わりに、`STYLESHEET` の ID と仮のファイル名だけの目録（Manifest）を渡します（UNIT 10）。CSS の中身は、HTTP のテストでは確かめていません。
- **改善点（この単元で見つかったこと）:**
  - 幅 375px・320px で、ヘッダーがはみ出し、横のスクロールができます（問1）。
  - 名前の無い `nav` が2つあります（axe の `landmark-unique`）。
  - 薄い文字を背景の上に置いたときのコントラスト比が 4.55 : 1 で、AA の線にほぼ並んでいます（問2）。

### 練習問題

**問1:** 例9-2で、幅 375px・320px のときに、ヘッダーがはみ出しました。CSS をどう直せばよいでしょうか。

**問2:** 例9-2のコントラスト比で、AA の 4.5 : 1 にいちばん近いのは、どの組でしたか。色を変えるときに、気をつけることは何でしょうか。

**問3:** `style.css` を1文字だけ変えて、束を作り直すと、HTML の `<link>` の URL と、前の URL は、どうなるでしょうか。`cache-control` の値と合わせて考えてください。

### 解答と解説

**問1の解答:**

教材のコードのコピーで、`.site-header` と `.site-header nav` に `flex-wrap: wrap;`（入りきらなければ折り返す）を足し、同じように測りました。

```diff
 .site-header {
   display: flex;
   align-items: center;
   justify-content: space-between;
+  flex-wrap: wrap;
   gap: 16px;
   padding: 12px 24px;
```

```diff
 .site-header nav {
   display: flex;
+  flex-wrap: wrap;
   align-items: center;
   gap: 16px;
 }
```

```text
@@OUT:u09_q1_measure@@
```

- 375px と 320px でも、ヘッダーの中身が画面の幅に収まり（375px / 375px・320px / 320px）、横のはみ出しが無くなりました。入りきらない分は、次の行に折り返されます。1280px は、変わりません。
- 長いメールアドレスが原因なので、メールを `text-overflow: ellipsis`（入りきらない分を「…」にする）で短く見せる、狭い画面ではメールを出さない、などの方法もあります。

**問2の解答:**

- いちばん近いのは、薄い文字（`#64748b`）を背景（`#f8fafc`）の上に置いた組で、4.55 : 1 でした。カードの中（白の上）なら 4.76 : 1 です。
- `--color-muted` を、少しでも明るく（薄く）すると、AA を下回るおそれがあります。色の変数を変えたら、`contrast13.py` のような計算と、axe の検査（例9-2）を、もう一度動かします。変数にまとめてあるので、計算する組も、`:root` から決まります。

**問3の解答:**

教材のコードのコピーで、1行目のコメントに「。問3の実験」を足して、束を作り直しました。

```text
@@OUT:u09_q3@@
```

- 印が `e9b983efabcfc6c3` から `d087c1d6e359e4b6` に変わり、HTML の `<link>` も新しい URL になりました。前の URL は 404 です。
- `cache-control` は `max-age=31536000, immutable`（1年間、変わらない）なので、ブラウザは、同じ URL の CSS を、1年間は取り寄せ直しません。CSS を変えると URL が変わるので、新しい HTML を読んだブラウザは、新しい CSS を取りに来ます。中身が変わらない限り URL も変わらないので、長くキャッシュさせても困りません。
- ただし、前の URL は 404 になるので、新しい版を公開した直後に、古い HTML を開いたままの画面で、CSS を読み直すと、見た目が崩れることがあります。

### まとめ

- `assets/style.css` は素の CSS。色は `:root` の変数にまとめ、`.container`（760px・真ん中・端から 16px）、flex で横に並べるヘッダー・一覧、カードの見た目、`pre-wrap` と `overflow-wrap: anywhere` の本文、と、`view!` のクラスの名前ごとに書く。
- 本物の Chrome で測ると、1280px は問題なし。375px・320px では、ヘッダーの中身が 392px になって横にはみ出し、`flex-wrap: wrap` で直った。コントラスト比は 10 の組すべて AA（いちばん低いのは 4.55 : 1）。
- `asset!` と `topcoat asset bundle` は、中身から作った印付きの URL で、1年間 `immutable` のキャッシュをさせる。CSS を変えれば URL が変わり、前の URL は 404。
- CSS のまちがいは止まらない。ファイルの名前の書き間違いは `cargo build` では通り、束を作るときに見つかる。閉じ括弧の書き忘れは、そのあとの規則を全部効かなくし、変数の名前の書き間違いは、親の色を引き継ぐ。
