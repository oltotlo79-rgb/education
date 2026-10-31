## UNIT 03: アセットの束を本番用に作る

### この単元のゴール

- `topcoat asset bundle --release` が、何を読み、何を作るか（中の `cargo build`・目録 `manifest.toml`・指紋付きの CSS）を、実際にできたファイルで説明できる
- 束がどう配られるか（URL・1年間のキャッシュ・gzip と br の圧縮）を、本物の応答で確かめられる
- CSS を変えたのに束を作り直し忘れると、エラーも無く古い CSS のままになることを確かめ、公開の手順に「束を作る」を必ず入れる理由を言える
- topcoat CLI と、RustNote の `topcoat` クレートの版がずれたときの知らせを読める

### たとえ話でつかむ

お店の**値札とチラシ**を考えます。

- 値札（CSS）は、**本部の印刷所**（`topcoat asset bundle`）で刷ります。印刷所は、まずお店の商品の一覧（実行ファイルの中の `asset!` の印）を見て、要る値札を全部刷ります。
- 刷った値札には、**中身から作った番号**（指紋）を入れます。値段を1円でも変えると、番号が変わります。お客さん（ブラウザ）は、「同じ番号の値札は、1年間、取り寄せ直さない」と決めています。
- 値札の束には、**目録**（`manifest.toml`）を付けます。「この商品の値札は、この番号のものです」という対応の表です。
- 遠くのお客さんには、値札を**たたんで（圧縮して）**送ります。広げると元どおりです。
- 値段を変えたのに、**値札を刷り直し忘れる**と、お店には古い値段の値札が並んだままです。だれも「刷り忘れです」とは言ってくれません。

| お店の値札 | RustNote のアセットの束 |
|---|---|
| 本部の印刷所 | `topcoat asset bundle --release` |
| お店の商品の一覧 | 実行ファイルの中の `asset!("assets/style.css")` の印 |
| 中身から作った番号 | `style-e9b983efabcfc6c3.css` の `e9b983efabcfc6c3` |
| 目録 | `assets/manifest.toml` |
| 1年間、取り寄せ直さない | `cache-control: public, max-age=31536000, immutable` |
| たたんで送る | `content-encoding: gzip`・`br` |
| 刷り直し忘れ | CSS を変えて、束を作り直さない（例3-3） |

> **用語:** アセットの束（バンドル：実行ファイルが使う CSS などのファイルを、指紋付きの名前にして、目録と一緒に1つのフォルダにまとめたもの。RustNote では、実行ファイルの隣の `assets/`）

<!-- -->

> **用語:** 圧縮（あっしゅく：送る中身を、同じ情報のまま小さくすること。gzip と br（Brotli）は、HTTP でよく使う圧縮の方式。ブラウザは `Accept-Encoding` で、読める方式を伝える）

### 図で理解する

束ができてから、ブラウザに届くまでです（例3-1・例3-2の結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-12-0301" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="200" height="110" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="120" y="44" font-size="12" font-weight="700" fill="#1d4ed8" text-anchor="middle">topcoat asset bundle --release</text>
<text x="34" y="70" font-size="11" fill="#0f172a">① 中で cargo build --release</text>
<text x="34" y="92" font-size="11" fill="#0f172a">② 実行ファイルの asset! の印を読む</text>
<text x="34" y="114" font-size="11" fill="#0f172a">③ CSS の中身から指紋を作る</text>
<rect x="260" y="20" width="210" height="110" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="365" y="44" font-size="12" font-weight="700" fill="#047857" text-anchor="middle" font-family="Consolas, monospace">target\release\assets\</text>
<text x="274" y="70" font-size="11" fill="#0f172a">manifest.toml（185 バイト）</text>
<text x="274" y="92" font-size="11" fill="#0f172a">style-e9b983efabcfc6c3.css</text>
<text x="274" y="112" font-size="11" fill="#0f172a">（4,659 バイト）</text>
<rect x="510" y="20" width="190" height="110" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="605" y="44" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">rustnote.exe が配る</text>
<text x="522" y="70" font-size="11" fill="#0f172a">/_topcoat/assets/style-…css</text>
<text x="522" y="92" font-size="11" fill="#0f172a">1年間 immutable</text>
<text x="522" y="114" font-size="11" fill="#0f172a">gzip・br で圧縮</text>
<line x1="220" y1="75" x2="256" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0301)"/>
<line x1="470" y1="75" x2="506" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0301)"/>
<text x="20" y="166" font-size="12" font-weight="700" fill="#0f172a">ブラウザが受け取るバイト数（例3-2）</text>
<rect x="20" y="180" width="466" height="22" rx="4" fill="#fed7aa"/>
<text x="496" y="196" font-size="12" fill="#0f172a">圧縮なし 4,659</text>
<rect x="20" y="210" width="144" height="22" rx="4" fill="#a7f3d0"/>
<text x="174" y="226" font-size="12" fill="#0f172a">gzip 1,437</text>
<rect x="20" y="240" width="143" height="22" rx="4" fill="#a7f3d0"/>
<text x="174" y="256" font-size="12" fill="#0f172a">br 1,433（約 31%）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図12-?: アセットの束の作られ方と配られ方。束は実行ファイルの隣に置き、配るときはブラウザが読める方式で圧縮する</div>
</div>

### まず動かす最小コード

#### 例3-1: 何も無い状態から、束だけを作る

UNIT 01 では、先に `cargo build --release --locked` をしてから束を作りました。束を作るコマンドだけを動かすと、どうなるでしょうか。何も無い `target` で試しました（`CARGO_TARGET_DIR` で、新しいフォルダを指定）。

```powershell
$env:CARGO_TARGET_DIR = "target-u03"
topcoat asset bundle --release
```

```text
bundled assets into target\release\assets
```

```text
--- target-u03/release の中（実行ファイルと assets）
drwxr-xr-x 0 assets
-rwxr-xr-x 11732480 rustnote.exe
--- assets の中
185 manifest.toml
4659 style-e9b983efabcfc6c3.css
--- manifest.toml
version = 1

[[assets]]
id = 5717027304611933330
file = "style-e9b983efabcfc6c3.css"
hash = "e9b983efabcfc6c3bb07e46232ff126ef842195c4cb5e4796bade732afe34710"
content_type = "text/css"
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: topcoat CLI は、中で cargo build をする

- 何も無い所から、束だけのコマンドで、実行ファイル（`rustnote.exe`）までできました。topcoat CLI は、中で `cargo build --release` を動かして実行ファイルを用意し、その中の `asset!` の印を読んで、束を作るからです（topcoat-cli 0.11.0 のソースで、`cargo build --message-format=json…` を呼んでいることを確かめました）。
- ただし、topcoat CLI の `cargo build` には `--locked` が付きません。だから、教材の手順と `Dockerfile` では、先に `cargo build --release --locked` で、`Cargo.lock` のとおりの版で作っておきます。そうすれば、topcoat CLI の `cargo build` は、もうできている実行ファイルをそのまま使います（UNIT 01）。

##### 解説2: 目録と、指紋付きの名前

- `manifest.toml` は、目録です。`id` は `asset!` ごとの番号、`file` は束の中のファイルの名前、`hash` は中身の SHA-256 の指紋、`content_type` は配るときの種類です。
- ファイルの名前の `e9b983efabcfc6c3` は、`hash` の先頭 16 文字です。CSS を1文字でも変えると、`hash` が変わり、名前も変わります（例3-3）。
- `id` は、`asset!` を書いたファイルの名前（`file!()`）なども使って計算します。Windows で作ると `src\views.rs`、コンテナの中（Linux）で作ると `src/views.rs` なので、同じ CSS でも `id` が違いました（例3-2のあとのコンテナの束）。実行ファイルと束は、**同じビルドで作った組**で置く必要があります。

### もう一歩: 実務に近い形

#### 例3-2: 束がどう配られるかを見る

例3-1の実行ファイルを起動し、HTML の `<link>` の URL の CSS を、`Accept-Encoding`（読める圧縮の方式）を変えて3回取りました。

```text
HTML の <link>: /_topcoat/assets/style-e9b983efabcfc6c3.css
--- Accept-Encoding なし
HTTP/1.1 200 OK
content-type: text/css
cache-control: public, max-age=31536000, immutable
vary: accept-encoding
content-length: 4659
受け取ったバイト数: 4659
--- Accept-Encoding: gzip
HTTP/1.1 200 OK
content-type: text/css
cache-control: public, max-age=31536000, immutable
vary: accept-encoding
content-encoding: gzip
受け取ったバイト数: 1437
--- Accept-Encoding: br
HTTP/1.1 200 OK
content-type: text/css
cache-control: public, max-age=31536000, immutable
vary: accept-encoding
content-encoding: br
受け取ったバイト数: 1433
```

- `cache-control: public, max-age=31536000, immutable` は、「だれが持ってもよい・1年間・中身は変わらない」です。名前に指紋が入っているので、中身が変われば URL が変わり、長くキャッシュさせても困りません（第13巻 UNIT 09）。
- `Accept-Encoding: gzip` や `br` を付けると、RustNote（Topcoat）は、圧縮して送りました。4,659 バイトが、約 1,430 バイト（約 31%）になりました。今のブラウザは、どれも `Accept-Encoding` を付けて頼むので、ふつうは圧縮されて届きます。
- `vary: accept-encoding` は、「同じ URL でも、`Accept-Encoding` によって中身が違う」という印です。途中のキャッシュ（CDN など）が、圧縮した物を、圧縮を読めない相手に渡さないようにするためです。

コンテナの中（UNIT 02 のイメージ）の束も見ました。

```powershell
docker run --rm rustnote:u02 sh -c 'ls -l /app/assets; cat /app/assets/manifest.toml'
```

```text
total 12
-rw-r--r-- 1 root root  186 Oct 30 21:40 manifest.toml
-rw-r--r-- 1 root root 4659 Oct 30 21:40 style-e9b983efabcfc6c3.css
version = 1

[[assets]]
id = 12436892214887874785
file = "style-e9b983efabcfc6c3.css"
hash = "e9b983efabcfc6c3bb07e46232ff126ef842195c4cb5e4796bade732afe34710"
content_type = "text/css"
```

- ファイルの名前（`style-e9b983efabcfc6c3.css`）と `hash` は、Windows で作った束と同じでした。CSS の中身が同じだからです。`id` だけが違うのは、解説2のとおりです。

#### 例3-3: CSS を変えたのに、束を作り直し忘れた

練習用のコピーの `assets/style.css` の最後に、トップページの見出しに線を引く規則を1つ足しました。そして、束を作り直さずに起動し、そのあと作り直して起動しました。

```text
--- CSS を変えて、束を作り直さずに起動
HTML の <link>: /_topcoat/assets/style-e9b983efabcfc6c3.css
CSS の中に .hero の行があるか: 0 行
--- topcoat asset bundle --release で作り直して起動
HTML の <link>: /_topcoat/assets/style-de9c5d85919fa819.css
CSS の中に .hero の行があるか: 1 行
前の URL: 404
--- assets の中
manifest.toml
style-de9c5d85919fa819.css
```

- 作り直す前は、`<link>` は前と同じ URL で、配られた CSS に、足した規則（`border-top: 4px`）はありませんでした。**エラーも警告も出ません**。束を作るのは topcoat CLI で、実行ファイルは、置いてある束をそのまま配るだけだからです。
- 作り直すと、指紋が `de9c5d85919fa819` に変わり、足した規則が入りました。前の URL は 404 です。束の中には、新しい CSS だけがあります。
- だから、公開の手順（`Dockerfile`）では、`cargo build` のあとに、**必ず**束を作ります。CSS だけを直したときも、ビルドと束を、いつも一緒に作り直します。

### よくあるエラーと読み方

#### エラー1（警告）: topcoat CLI の版が、RustNote の topcoat とちがう

UNIT 01 で前に入れてあった topcoat CLI 0.10.0 で、topcoat 0.11.0 の RustNote の束を作りました。

```text
topcoat 0.10.0
```

```text
warning: this project depends on topcoat 0.11.0, but the topcoat CLI is 0.10.0
  install a matching CLI with `cargo install topcoat-cli@0.11 --locked`
bundled assets into target\release\assets
exit=0
--- 古い CLI（0.10.0）で作った束で起動
HTML の <link>: /_topcoat/assets/style-e9b983efabcfc6c3.css
その CSS: 200
```

- **何が起きたか:** 「このプロジェクトは topcoat 0.11.0 を使っていますが、topcoat CLI は 0.10.0 です」という警告と、「合う CLI を `cargo install topcoat-cli@0.11 --locked` で入れてください」という案内が出ました。topcoat CLI は、`Cargo.lock` を読んで、版を比べています。
- **読み方:** この組み合わせでは、束は作れ、CSS も配れました（200）。ただし、版がずれていても動くという約束はありません。束の形が変わる版では、作れても配れないことがあり得ます。警告が出たら、案内のとおり、CLI を同じ版にそろえます。
- **直し方:** `cargo install topcoat-cli --version 0.11.0 --locked`。RustNote の `Dockerfile` も、CLI の版を `Cargo.toml` の topcoat と同じ 0.11.0 に固定しています（第13巻 UNIT 01）。

#### エラー2: 束の場所がちがう（no asset bundle at …）

UNIT 01 のエラー1と問1で見たとおり、実行ファイルは「自分の隣の `assets\`」しか探しません。`--release` を付け忘れた束（`target\debug\assets`）や、実行ファイルだけをコピーしたフォルダでは、起動の時点で止まります。止まるのは DB に触る前なので、DB のファイルは作られません。

### RustNote ではここで使う

- **Dockerfile:** builder の段の最後の `RUN topcoat asset bundle --release` が、`/app/target/release/assets/` に束を作り、実行用の段の `COPY --from=builder /app/target/release/assets /app/assets` で、実行ファイルの隣に置きます（UNIT 02）。
- **RustNote のコード:** `src/views.rs` の `STYLESHEET`（`asset!("assets/style.css")`）と、`src/main.rs` の `AssetBundle::load()` です（第13巻 UNIT 03・09）。
- **テスト:** テストは、本物の束の代わりに、目録だけを作って渡します（第13巻 UNIT 10）。束の作り直し忘れは、テストでは見つかりません。

### 練習問題

**問1:** 例3-1の目録の `hash` と、CSS のファイルの名前を比べてください。ファイルの名前は、どう決まっているでしょうか。

**問2:** 例3-3のように、CSS だけを直して公開するとき、`Dockerfile` を使う場合と、自分の PC で作った実行ファイルを置く場合で、それぞれ何をすればよいでしょうか。

**問3:** 圧縮で、CSS は約 31% の大きさになりました。RustNote の CSS は 4,659 バイトと小さいのに、なぜ圧縮が役に立つのでしょうか。

### 解答と解説

**問1の解答:**

- `hash` は `e9b983efabcfc6c3bb07e46232ff126ef842195c4cb5e4796bade732afe34710`、ファイルの名前は `style-e9b983efabcfc6c3.css` でした。元のファイルの名前（`style`）に、`hash` の先頭 16 文字を付けた名前です。
- 例3-3で CSS を変えたあとは、`style-de9c5d85919fa819.css` になりました。中身が同じなら、いつ・どこで作っても同じ名前（コンテナの中でも同じ）、中身が変われば別の名前です。

**問2の解答:**

- **Dockerfile を使う場合:** そのまま、もう一度イメージを作ります。`COPY . .` で新しい CSS が入るので、`cargo build` と `topcoat asset bundle --release` が両方やり直しになり、新しい束が入ります（UNIT 02 の例2-3）。
- **自分の PC で作る場合:** `topcoat asset bundle --release` を、もう一度動かします。そして、実行ファイルと `assets\` を、**両方**置き直します。CSS だけを直したつもりでも、束の目録（`manifest.toml`）と実行ファイルは組なので、片方だけを替えると、`id` や名前が合わなくなるおそれがあります。

**問3の解答:**

- 1つのファイルは小さくても、CSS は、初めて開くすべてのページで読み込まれます。スマホの回線では、1回の往復と、受け取るバイト数の両方が待ち時間になります。約 3,200 バイト減れば、それだけ早く画面が整います。
- しかも、圧縮は、Topcoat が自動でしてくれるので、こちらの手間はありません。指紋付きの名前と1年間のキャッシュで、2回目からは取りにさえ来ないので、圧縮と合わせて、CSS の待ち時間は、ほとんど無くなります。

### まとめ

- `topcoat asset bundle --release` は、中で `cargo build --release` をして、実行ファイルの `asset!` の印を読み、中身の SHA-256 の先頭 16 文字を付けた名前の CSS と、目録 `manifest.toml` を、実行ファイルの隣の `assets\` に作る。`--locked` は付かないので、先に `cargo build --release --locked` をしておく。
- 束は `/_topcoat/assets/style-<指紋>.css` で、1年間 `immutable` のキャッシュと、gzip・br の圧縮（4,659 → 約 1,430 バイト）で配られる。
- CSS を変えて束を作り直し忘れても、エラーは出ず、古い CSS のまま。ビルドと束は、いつも一緒に作り、実行ファイルと `assets\` を組で置く。CLI の版がずれると警告が出るので、CLI を同じ版にそろえる。
