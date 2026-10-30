## UNIT 08: ルートの一覧と実装の順番

### この単元のゴール

- RustNote の全部の URL について、「メソッド・URL・誰が・期待する返事（状態コードと移り先）」を表にし、その表を**そのままテスト**にして、本物のルーターで1行ずつ確かめられる
- 「薄い縦切り」（1つの機能を、画面から DB まで一気に通して作る）の順番で、実装の計画を立てる理由を説明できる
- 新しい URL を足すときに、**表に行を足す → 失敗を見る → 作る → 通る**の順で進め、途中の失敗（404・500）から、何が足りないかを読める
- `#[route]` ではレイアウトのエラー画面を通らないこと、同じ URL・同じメソッドのページを2つ書いたときの panic を説明できる

### たとえ話でつかむ

ホテルのフロントの、案内表を考えます。

- フロントには、「どの部屋に・誰が来たら・どう案内するか」を書いた**案内表**があります。「会員が 7 階のラウンジに来たら通す。会員でなければ、受付に戻ってもらう」「よその部屋の鍵を見せられたら、『そのお部屋はありません』と答える」。
- 新しいサービス（朝食の持ち帰り）を始めるときは、まず**案内表に行を足します**。行を足しただけでは、まだ誰も案内できないので、確かめると「できていません」と出ます。それから、準備をします。
- ホテルの工事は、**1つのサービスを、受付から厨房まで全部**つなげてから、次のサービスに移ります。「全部の受付を先に作って、厨房はあとで」とすると、最後まで何も使えないからです。

| ホテルのフロント | RustNote |
|---|---|
| 案内表 | ルートの一覧（`tests/u08_routes.rs` の表） |
| どの部屋に・誰が来たら | メソッドと URL・未ログイン／持ち主／ほかの人 |
| どう案内するか | 期待する状態コード（200・303・404 など）と移り先 |
| 案内表に行を足してから準備する | 表に行を足す → 失敗を見る → 作る → 通る |
| 受付から厨房まで一気に作る | 薄い縦切り（画面・ルート・ドメイン・リポジトリ・DB を1機能ずつ） |

> **用語:** 薄い縦切り（vertical slice：1つの機能を、画面から DB まで、すべての層を通して小さく作ること。作り終わるたびに、動くものが1つ増える）

<!-- -->

> **用語:** 赤と緑（テストが失敗している状態を赤、通っている状態を緑と呼ぶ。先に赤を見てから緑にすると、テストが本当に確かめていると分かる）

### 図で理解する

RustNote を、薄い縦切りで作るとしたら、この順番になります。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="60" y="58" font-size="12" fill="#475569" text-anchor="middle">画面</text>
<text x="60" y="98" font-size="12" fill="#475569" text-anchor="middle">ルート</text>
<text x="60" y="138" font-size="12" fill="#475569" text-anchor="middle">ドメイン</text>
<text x="60" y="178" font-size="12" fill="#475569" text-anchor="middle">リポジトリ</text>
<text x="60" y="218" font-size="12" fill="#475569" text-anchor="middle">DB の表</text>
<rect x="110" y="36" width="80" height="196" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="150" y="252" font-size="11" font-weight="700" fill="#1e40af" text-anchor="middle">① /health</text>
<rect x="200" y="36" width="80" height="196" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="240" y="252" font-size="11" font-weight="700" fill="#047857" text-anchor="middle">② 登録・ログイン</text>
<rect x="290" y="36" width="80" height="196" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="330" y="252" font-size="11" font-weight="700" fill="#047857" text-anchor="middle">③ 一覧・作成</text>
<rect x="380" y="36" width="80" height="196" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="420" y="252" font-size="11" font-weight="700" fill="#c2410c" text-anchor="middle">④ 詳細</text>
<rect x="470" y="36" width="80" height="196" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="510" y="252" font-size="11" font-weight="700" fill="#c2410c" text-anchor="middle">⑤ 編集</text>
<rect x="560" y="36" width="70" height="196" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="595" y="252" font-size="11" font-weight="700" fill="#b91c1c" text-anchor="middle">⑥ 完了・削除</text>
<rect x="640" y="36" width="70" height="196" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="675" y="252" font-size="11" font-weight="700" fill="#475569" text-anchor="middle">⑦ 検索</text>
<line x1="100" y1="76" x2="710" y2="76" stroke="#cbd5e1" stroke-width="1"/>
<line x1="100" y1="116" x2="710" y2="116" stroke="#cbd5e1" stroke-width="1"/>
<line x1="100" y1="156" x2="710" y2="156" stroke="#cbd5e1" stroke-width="1"/>
<line x1="100" y1="196" x2="710" y2="196" stroke="#cbd5e1" stroke-width="1"/>
<text x="360" y="22" font-size="12" fill="#0f172a" text-anchor="middle">1本ずつ、上から下まで通して作り、表のテストを緑にしてから、次の1本に移る</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: 薄い縦切り。横の層（画面だけ・DB だけ）をまとめて作らずに、1つの機能を、画面から DB の表まで通して作る。どの時点で止めても、そこまでの機能は動く</div>
</div>

- ①の `/health` は、画面も DB の表もいらない、いちばん薄い1本です。最初にこれを通すと、「ルーター・起動・DB への接続・テストの道具」が、ひとまず全部つながります。
- ②から先は、前の1本が使えることを前提に、少しずつ足していきます。②の登録とログインが無いと、③の「自分のノート」が作れないからです。

新しい URL を、表から足すときの流れです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 170" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-0802" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="30" width="200" height="100" rx="10" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="120" y="56" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">① 表に3行を足す</text>
<text x="120" y="80" font-size="11" fill="#0f172a" text-anchor="middle">まだ作っていない</text>
<text x="120" y="100" font-size="11" fill="#b91c1c" text-anchor="middle">→ 赤（2行 NG: 404）</text>
<rect x="260" y="30" width="200" height="100" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="56" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">② export.rs を書く</text>
<text x="360" y="80" font-size="11" fill="#0f172a" text-anchor="middle">#[route(GET)] と pub mod</text>
<text x="360" y="100" font-size="11" fill="#c2410c" text-anchor="middle">→ 赤（1行 NG: 500）</text>
<rect x="500" y="30" width="200" height="100" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="600" y="56" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">③ 404 の返し方を直す</text>
<text x="600" y="80" font-size="11" fill="#0f172a" text-anchor="middle">not_found() を返す</text>
<text x="600" y="100" font-size="11" fill="#047857" text-anchor="middle">→ 緑（全部 ok）</text>
<line x1="220" y1="80" x2="256" y2="80" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0802)"/>
<line x1="460" y1="80" x2="496" y2="80" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0802)"/>
<text x="360" y="158" font-size="12" fill="#0f172a" text-anchor="middle">赤のたびに、どの行が、何番で違うかが表に出るので、次にすることが分かる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: 表から足す流れ。先に約束（表の行）を書き、失敗を見てから作る。途中の失敗の番号（404・500）が、足りないものを教えてくれる</div>
</div>

### まず動かす最小コード

#### 例8-1: ルートの一覧を、本物のルーターで確かめる

> **▼ このコードがやること（先に日本語で）:** alice と bob を登録し、alice のノートを1件（`/notes/1`）作ります。表の1行は「メソッド・URL・送る本文・誰として送るか・期待する状態コード・303 の移り先」です。20 行の表を上から順に、本物のルーター（`app.router.handle`）に送り、結果が表と合えば `ok`、違えば `NG` と表示し、最後に `NG` が0行であることを確かめます。

```rust
@@FILE:book/rustnote/tests/u08_routes.rs@@
```

```powershell
cargo test --test u08_routes -- --nocapture
```

**▼ 実行結果**（アクセスログの行は省いています）

```text
@@OUTGREP:u08_routes:^(ok|NG|test result)@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 表の1行が、1つの約束

- 20 行が全部 `ok` でした。たとえば `GET /notes`（未ログイン）→ `303 /login`、`GET /notes/1`（bob）→ `404`、`DELETE /notes/1` → `405` です。どの行も、第8巻・第10巻で1つずつ確かめてきた約束を、1行にまとめたものです。
- 「未ログイン・持ち主・ほかの人」の3通りを、同じ URL について並べるのが大事です。正しい人が使えるかだけでなく、**使えてはいけない人が、ちゃんと断られるか**も、表に入ります。

##### 解説2: 表の形のテスト

- 第11巻 UNIT 01 の表の形のテストと同じく、1行ずつ同じ確かめ方を当て、合わない行を数えて、最後にまとめて報告します。1行目で止まらずに、**全部の行の結果**が見えるので、壊れた所が一度に分かります。
- `row(…)` は、本文も移り先もない、よくある行を短く書くための関数です。`Row { to: Some("/login"), ..row(…) }` の `..` は、「ほかの項目は `row(…)` と同じ」という書き方です（第2巻 UNIT 07）。

##### 解説3: RustNote の README の一覧と、同じもの

- RustNote の `README.md` には、URL の一覧（成功したときの返事）と、エラーのときの状態コードの表があり、「ログインが必要な URL に未ログインで来ると、303 で `/login` へ移動します」と書いてあります。これは、人が読む約束です。このテストは、**同じ約束を、機械が確かめる形**にしたものです。URL を足したり変えたりしたときに、README の表とテストの表の両方を直せば、約束と実物がずれません。

### もう一歩: 実務に近い形

#### 例8-2: 表から、新しい URL を足す（ノートを文字だけで書き出す）

ノートを、文字だけ（`text/plain`）で書き出す `GET /notes/{id}/export` を足します。作業用にもう1つコピーした RustNote（`u08work`）で進めます。

**① 先に、表に3行を足す。** 持ち主は 200、ほかの人は 404、未ログインはログイン画面へ、という約束です。

```rust
        // 足した3行: ノートを文字だけで書き出す
        row(Method::GET, "/notes/1/export", Who::Alice, 200),
        row(Method::GET, "/notes/1/export", Who::Bob, 404),
        Row {
            to: Some("/login"),
            ..row(Method::GET, "/notes/1/export", Who::Anon, 303)
        },
```

```text
@@OUT:u08_red1@@
```

- まだ作っていないので、赤です。alice と未ログインが `404`（その URL が無い）で `NG` になりました。bob の行が `ok` なのは、**たまたま**です（無い URL も 404 なので）。赤を見ておくと、「作ったから通った」のか「たまたま通った」のかを、区別できます。

**② 作る。** `src/routes/notes/id/export.rs` を書き、`src/routes/notes/id.rs` に `pub mod export;` を足します（ファイルを作っても、`pub mod` が無ければコンパイルされず、①と同じ 404 のままです。第6巻 UNIT 03）。

```rust
@@FILE:u08_export_v1.rs@@
```

```text
@@OUT:u08_red2@@
```

- alice は 200、未ログインは 303 になりました。ところが、**bob が 500** です。`find_own_note` は、他人のノートで `AppError::NotFound` を返します。ページ（`#[page]`）なら、レイアウトの `error_page` がそれを 404 の画面にします（第6巻 UNIT 10）。ところが `#[route]` は、**レイアウトを通らない**ので、Topcoat は `AppError` を知らないエラーとして、500 にしたのです。
- 表に「bob は 404」と書いていなければ、この 500 は見過ごされていました。

**③ 直す。** `#[route]` の中では、404 を Topcoat の `not_found()` で返します。

```rust
@@FILE:u08_export.rs@@
```

```text
@@OUT:u08_green@@
```

- 3行とも `ok` になり、緑です。ほかのテスト（全部のテストのファイル）も、通ることを確かめました。
- この1本の縦切りは、「表の3行 → ルート → 既にあるドメインとリポジトリ（`find_own_note`）」で、新しい DB の表は要りませんでした。**縦切りの太さは、機能によって違います**。

### よくあるエラーと読み方

#### エラー1: 同じ URL・同じメソッドのページを、2つ書いた

書き出しを、`export.rs` に分けずに、`id.rs` に `#[page]` でもう1つ書いてしまった場合です。

```rust
/// GET /notes/{id}: 書き出しを、id.rs に書いてしまった（show と同じ URL・同じ GET）
#[page]
pub async fn export_here(cx: &Cx) -> Result<impl View> {
```

```text
@@OUT:u08_err_dup@@
```

- **何が起きたか:** 「`GET /notes/{id}` に、ルートが二重に登録されました」という panic で、表のテストの最初（ルーターを組み立てる所）で止まりました。同じモジュールのページは、同じ URL になるからです（第6巻 UNIT 03 のエラー1）。
- **学べること:** この panic は、アプリを起動したときにも起きます。表のテストがあれば、起動する前に、`cargo test` で気づけます。

#### エラー2（エラーが出ない間違い）: #[route] の中で、ページと同じエラーの返し方をした

例8-2の②の形です。コンパイルは通り、持ち主にはちゃんと書き出されます。困るのは、**ほかの人が開いたとき**だけです。

- **何が起きたか:** 他人のノートで 500 になり、サーバーの窓にも、何も表示されませんでした（`error_page` の `log_if_internal` も通らないからです）。利用者には「サーバーが壊れている」ように見え、運用する人は、記録が無いので原因を探せません。
- **直し方:** `#[route]` では、404・400 などを、Topcoat の `not_found()`・`bad_request(…)` で返します（例8-2の③）。人が見る画面なら `#[page]` にして、レイアウトの `error_page` に任せます。RustNote のログアウトが `#[page(POST)]` なのは、このためです（第6巻 UNIT 06）。

### RustNote ではここで使う

- RustNote の `src/routes.rs` の先頭のコメントに、ファイルと URL の対応の表があります（第6巻 UNIT 03）。`README.md` には、URL の一覧と、エラーのときの状態コードの表があります。例8-1のテストは、その表を、機械が確かめる形にしたものです。
- RustNote の `tests/http.rs` の 14 個のテストは、この表の行のいくつかを、1つずつ名前を付けて確かめています（第11巻 UNIT 03（準備中））。表の形と、名前付きのテストは、どちらを使ってもかまいません。全部の URL を一度に見渡したいときは表、1つの約束を詳しく確かめたいときは名前付きのテストが向いています。
- この巻が勧める順番は、図の①〜⑦です。②の登録・ログインは第8巻で、③〜⑦は第10巻で、おおむねこの順番に、1本ずつ作っていきます。

### 練習問題

**問1:** 表に「`GET /notes?page=abc`（alice）」の行を足すとしたら、期待する状態コードは何番にするでしょうか（第6巻 UNIT 04）。

**問2:** 「`HEAD /health`（未ログイン）」の行は、何番にするでしょうか。

**問3:** 「ほかのサイトのページから、alice のブラウザで `POST /notes` を送らせる」行は、何番にするでしょうか。送る要求には、`Origin: https://evil.example` と `Sec-Fetch-Site: cross-site` が付きます。

### 解答と解説

3つとも、`tests/u08_q.rs` で、実際に送って確かめました。

```rust
@@FILE:book/rustnote/tests/u08_q.rs@@
```

```text
@@OUT:u08_q@@
```

**問1の解答:** 400 です。`?page=` を読む `NotesParams` が `#[query_params(error = bad_request)]` なので、数でないページの番号は 400 になります。ページ自体はあるので、404 ではありません。

**問2の解答:** 200 です。`GET` のルートには、Topcoat が `HEAD` にも自動で答えます（第6巻 UNIT 03 の問3）。Fly.io のヘルスチェックの設定によっては、HEAD で確かめることもあるので、表に入れておくと安心です。

**問3の解答:** 403 です。Topcoat の OriginPolicy が、ほかのサイトからの POST を、ルーターの最初で止めます（第6巻 UNIT 10・第8巻 UNIT 01）。この行を表に入れておくと、もし誰かが OriginPolicy を外す設定を足しても、テストが気づかせてくれます（第11巻 UNIT 06（準備中））。

### まとめ

- ルートの一覧は、「メソッド・URL・誰が・期待する状態コードと移り先」の表にし、本物のルーターで1行ずつ確かめるテストにする。未ログイン・持ち主・ほかの人の3通りを並べ、断られるべき人が断られることも入れる。
- 実装は薄い縦切りで、1つの機能を画面から DB まで通してから、次に移る。新しい URL は、表に行を足す → 赤を見る → 作る → 緑、の順に進める。
- `#[route]` はレイアウトのエラー画面を通らないので、404 などは Topcoat の `not_found()` などで返す。同じ URL・同じメソッドのページを2つ書くと、ルーターを組み立てる所で panic になる。
