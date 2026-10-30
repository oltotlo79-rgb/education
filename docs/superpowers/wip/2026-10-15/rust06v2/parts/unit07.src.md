## UNIT 07: app_context・memoize

### この単元のゴール

- 設定や DB のように**アプリ全体で1つ**の値を、`.app_context(値)` でルーターに登録し、`app_context::<T>(cx)` でどのページからも借りられる
- `#[memoize]` で、**1つの要求の中**で同じ問い合わせを1回にまとめ、レイアウト・ページ・部品のどこから呼んでも DB を1回しか読まない形にできる
- 「ミドルウェアではなく関数」という Topcoat の考え方（`cx` を受け取る小さな関数を、必要な所で呼ぶ）を説明できる
- 登録し忘れ・同じ型を2つ登録・スレッドをまたげない値（`Rc`）のエラーを読める

### たとえ話でつかむ

大きな事務所を考えます。

- 事務所には、**全員で使う道具**があります。コピー機や、住所録です。1台ずつ全員の机に置くのではなく、決まった場所に1つ置いて、使う人が借りに行きます。置く場所には「コピー機」「住所録」と、**種類ごとに1つ**ずつ札が付いています。
- ある日の仕事（1件の依頼）の中で、同じ人の住所を何度も調べることがあります。担当者は、最初に調べた住所を**その日の依頼のメモ用紙**に書いておき、2回目からはメモを見ます。依頼が終わったら、メモ用紙は捨てます（次の依頼では、住所が変わっているかもしれないからです）。
- 「この依頼は、受付で身分を確かめてから回すこと」と、入口に決まりを貼るのではなく、**身分を確かめる手順書**を作っておき、必要な担当者が、自分の仕事の最初にその手順書を使います。

| 事務所 | Topcoat |
|---|---|
| 全員で使う道具を、決まった場所に1つ | `.app_context(値)`（ルーターに登録。型ごとに1つ） |
| 道具を借りに行く | `app_context::<T>(cx)` |
| その日の依頼のメモ用紙 | `#[memoize]`（1つの要求の中だけ、結果を覚えておく） |
| 依頼が終わったら捨てる | 次の要求では、もう一度計算する |
| 身分を確かめる手順書 | `require_user(cx)` のような、`cx` を受け取る関数 |

> **用語:** アプリのコンテキスト（app context：アプリ全体で共有する値の置き場。DB や設定を入れる。値は型で見分ける）

<!-- -->

> **用語:** メモ化（memoize：同じ引数で呼ばれた関数の結果を覚えておき、2回目からは計算せずに覚えた結果を返すこと）

<!-- -->

> **用語:** ミドルウェア（middleware：すべての要求の前後に、決まった処理（ログインの確かめなど）をはさむ仕組み。Topcoat は、これよりも関数を勧める）

### 図で理解する

アプリ全体で1つの値と、要求ごとの覚え書きの違いです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="20" width="680" height="70" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="44" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">ルーター（起動のときに1回作る）</text>
<rect x="60" y="54" width="180" height="28" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="150" y="73" font-size="12" fill="#0f172a" text-anchor="middle">AppConfig（設定）</text>
<rect x="270" y="54" width="180" height="28" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="360" y="73" font-size="12" fill="#0f172a" text-anchor="middle">Visits（数え上げ）</text>
<rect x="480" y="54" width="180" height="28" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="570" y="73" font-size="12" fill="#0f172a" text-anchor="middle">AppState（DB など）</text>
<rect x="20" y="120" width="330" height="140" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="185" y="144" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">要求 1（GET /）</text>
<text x="40" y="170" font-size="12" fill="#0f172a">app_context で借りる（全部の要求で同じもの）</text>
<rect x="40" y="184" width="290" height="60" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="185" y="206" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">memoize の覚え書き（この要求だけ）</text>
<text x="185" y="228" font-size="11" fill="#0f172a" text-anchor="middle">load_user(1) → alice（2回目からはこれを返す）</text>
<rect x="370" y="120" width="330" height="140" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="535" y="144" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">要求 2（GET /about）</text>
<text x="390" y="170" font-size="12" fill="#0f172a">同じ AppConfig・Visits を借りる</text>
<rect x="390" y="184" width="290" height="60" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="535" y="206" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">memoize の覚え書き（空から始まる）</text>
<text x="535" y="228" font-size="11" fill="#0f172a" text-anchor="middle">要求 1 の結果は使わない</text>
<line x1="150" y1="90" x2="150" y2="118" stroke="#10b981" stroke-width="2"/>
<line x1="540" y1="90" x2="540" y2="118" stroke="#10b981" stroke-width="2"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: app_context と memoize。app_context の値はアプリ全体で1つで、どの要求からも同じものを借りる。memoize の覚え書きは要求ごとに空から始まり、その要求が終わると捨てられる</div>
</div>

「ミドルウェア」と「関数」の違いです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 210" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0702" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="180" y="22" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">ミドルウェア（Topcoat は勧めない）</text>
<rect x="20" y="36" width="320" height="40" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="180" y="61" font-size="12" fill="#0f172a" text-anchor="middle">入口で、ログイン中の人を調べて「置いておく」</text>
<rect x="20" y="96" width="320" height="40" rx="6" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="180" y="121" font-size="12" fill="#0f172a" text-anchor="middle">ページは「置いてあるはず」と信じて取り出す</text>
<text x="180" y="166" font-size="11" fill="#b91c1c" text-anchor="middle">入口の設定を忘れると、ページは気づけない</text>
<text x="180" y="184" font-size="11" fill="#b91c1c" text-anchor="middle">（panic・確かめ漏れ）</text>
<line x1="180" y1="76" x2="180" y2="92" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0702)"/>
<text x="540" y="22" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">関数（Topcoat の書き方）</text>
<rect x="380" y="36" width="320" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="540" y="61" font-size="12" fill="#0f172a" text-anchor="middle">ページの最初で require_user(cx) を呼ぶ</text>
<rect x="380" y="96" width="320" height="40" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="540" y="121" font-size="12" fill="#0f172a" text-anchor="middle">部品・レイアウトも current_user(cx) を呼ぶ</text>
<text x="540" y="166" font-size="11" fill="#047857" text-anchor="middle">確かめているかが、ページを読めば分かる</text>
<text x="540" y="184" font-size="11" fill="#047857" text-anchor="middle">memoize で、何回呼んでも DB は1回</text>
<line x1="540" y1="76" x2="540" y2="92" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0702)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: ミドルウェアと関数。Topcoat では、cx を受け取る小さな関数を、必要な所で呼ぶ。同じ計算が何度も起きないように、memoize で1つの要求の中の結果を覚えておく</div>
</div>

### まず動かす最小コード

#### 例7-1: 設定と数え上げを、どのページからも借りる

> **▼ このコードがやること（先に日本語で）:** サイトの名前を持つ `AppConfig` と、表示の回数を数える `Visits` の2つを、ルーターに `.app_context(…)` で登録します。`GET /` と `GET /about` は、どちらも `config(cx)` でサイトの名前を借り、`count_visit(cx)` で回数を1つ足して表示します。

```rust
@@FILE:web_lab/src/bin/u07_context.rs@@
```

```powershell
cargo run --bin u07_context
```

2つ目の窓から、`/`・`/`・`/about`・`/` の順に開きます。

**▼ 実行結果**

```text
@@OUT:u07_context_curl@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 登録は、ルーターを作るときに1回

- `main` で `.app_context(AppConfig { … })` と `.app_context(Visits(…))` を書きました。値は**起動のときに1回だけ**作られ、ルーターが持ちます。
- どのページからでも `app_context::<AppConfig>(cx)` で借りられます。`/` も `/about` も、同じサイトの名前を表示しました。
- 値は**型で**見分けます。`AppConfig` を借りたいときは `app_context::<AppConfig>(cx)` です。同じ型の値を2つは登録できません（エラー2）。

##### 解説2: 要求をまたいで、同じ値を使う

- 回数は、`/`・`/`・`/about`・`/` で 1・2・3・4 と増えました。**別々の要求**（`/about` も含めて）が、同じ `Visits` を借りて足しているからです。
- `Visits` の中身は `AtomicU64`（同時に足しても数が狂わない数）です。要求は、いくつも**同時に**処理されることがあるので、共有する値は、同時に使っても安全な形にします。書き換える値なら `AtomicU64` や `Mutex`、読むだけの値（`AppConfig`）ならそのままで大丈夫です（第5巻 UNIT 05）。

##### 解説3: 借りる関数を、1つ作っておく

- `config(cx)` と `count_visit(cx)` は、`app_context` を包んだ小さな関数です。ページの中に `app_context::<AppConfig>(cx)` と何度も書かずに済み、型の名前を変えたときも、直す所が1か所になります。RustNote の `app_state(cx)` も同じ考え方です。

### もう一歩: 実務に近い形

#### 例7-2: 1つの要求の中で、同じ問い合わせを1回にする

ログイン中のユーザーは、**レイアウト**（ヘッダーの名前）でも、**ページ**でも、**部品**でも使います。そのたびに DB を読むと、1回の表示で何度も同じ問い合わせをしてしまいます。`#[memoize]` で、1つの要求の中では1回にまとめます。

> **▼ このコードがやること（先に日本語で）:** `load_user(cx, id)` は、「DB から読みました」と表示して 50 ミリ秒待ってから、id が 1 なら alice を返します。`#[memoize(as_ref)]` を付けたので、同じ要求の中の2回目からは、覚えておいた結果を返します。レイアウト・ページ・部品（`user_badge` を3回）から、合わせて5回 `load_user(cx, 1)` を、ページから1回 `load_user(cx, 2)` を呼びます。

```rust
@@FILE:web_lab/src/bin/u07_memo.rs@@
```

```powershell
cargo run --bin u07_memo
```

2つ目の窓から、2回開きます（`-w` で、かかった時間も表示します）。

```text
@@OUT:u07_memo_curl@@
```

**▼ サーバーの窓の表示**

```text
@@OUT:u07_memo_server@@
```

- 1回の表示で、`load_user(cx, 1)` は5回呼ばれましたが、「DB を読みました（id = 1）」は**1回**だけでした。id = 2 も1回です。2回の要求で、合わせて4行です。
- 2回目の要求では、もう一度 id = 1 を読みました。覚え書きは、**要求ごと**に空から始まるからです。ほかの人がプロフィールを変えたら、次の表示では新しい名前が出ます。
- `#[memoize(as_ref)]` は、戻り値の `Option<User>` を、`Option<&User>`（覚えておいた値を借りる形）に変えます。何度呼んでも、ユーザーの値を複製しません。`as_ref` を付けない `#[memoize]` は、`&Option<User>` を返します。
- かかった時間は約 0.11 秒でした。id = 1 と id = 2 の2回ぶん（50 ミリ秒 × 2）です。同時に動く部品が同じ id を読もうとしたときも、1回の読み込みを**みんなで待つ**ので、2回読むことはありません（問1で、`#[memoize]` を外したときと比べます）。

### よくあるエラーと読み方

#### エラー1: 借りる型を、登録し忘れた

```rust
@@FILE:errs/e07_missing.rs@@
```

コンパイルは通ります。開くと、500 になります。

```text
@@OUT:u07_err_missing_curl@@
```

**▼ サーバーの窓の表示**

```text
@@OUT:u07_err_missing_server@@
```

- **何が起きたか:** 「`AppConfig` 型のアプリのコンテキストを借りようとしましたが、この型は登録されていません」という panic です。`app_context::<T>` は、無い型を借りようとすると panic します。Topcoat は panic を 500 の返事に変えます。
- **直し方:** `main` で `.app_context(AppConfig { … })` を登録します。無くてもよい値なら、`try_app_context::<T>(cx)`（無ければ `None`）で借ります（問2）。
- **学べること:** 登録し忘れは、**そのページを開いたときに**初めて分かります。RustNote のテストは、本物と同じ `build_router` でルーターを作るので、登録し忘れがあれば、テストで見つかります（第11巻 UNIT 03（準備中））。

#### エラー2: 同じ型の値を、2つ登録しようとした

本番の DB と、控えの DB の2つを、同じ `Database` 型で登録しようとした場合です（`errs/e07_twice.rs`）。

```rust
@@FILE:errs/e07_twice.rs@@
```

```text
@@OUT:u07_err_twice@@
```

- **何が起きたか:** 「`Database` 型のコンテキストが二重です」という panic で、起動した途端に止まりました。値は型で見分けるので、同じ型が2つあると、`app_context::<Database>(cx)` でどちらを返せばよいか決められないからです。
- **直し方:** 型を分けます。`struct PrimaryDb(Database);` と `struct ReplicaDb(Database);` のように、**包む型**（newtype。第4巻 UNIT 01）を作って、それぞれを登録します。

#### エラー3: スレッドをまたげない Rc を入れた（E0277）

```rust
@@FILE:errs/e07_rc.rs@@
```

```text
@@OUTHEAD:u07_err_rc:25@@
```

（このあとに、`Sync` についての同じ形のエラーが続きます）

- **何が起きたか:** 「`Rc<String>` は、スレッドの間で安全に送れません」というエラーです。最後の `note:` が、`app_context` の決まり `T: Any + Send + Sync` を指しています。要求はいくつものスレッドで同時に処理されるので、登録する値は、スレッドをまたいで送れて（`Send`）、同時に借りられる（`Sync`）必要があります。`Rc` は、どちらでもありません（第3巻 UNIT 10）。
- **直し方:** `Rc` を `Arc` に変えます。この例の `site_name` なら、包む必要は無く、ただの `String` で十分です。RustNote の `AppState` は、リポジトリを `Arc<dyn NoteRepository + Send + Sync>` で持っています。

### RustNote ではここで使う

アプリ全体で共有する値は、`src/state.rs` の `AppState` にまとめています（抜き出しています）。

```rust
/// アプリ全体で共有する状態。clone は Arc の参照を増やすだけなので安い
#[derive(Clone)]
pub struct AppState {
    /// DB そのもの（GET /health の確認に使う）
    pub db: Db,
    pub users: Arc<dyn UserRepository + Send + Sync>,
    pub notes: Arc<dyn NoteRepository + Send + Sync>,
    pub sessions: Arc<dyn SessionRepository + Send + Sync>,
}

/// リクエストの処理の中から AppState を借りる
pub fn app_state(cx: &Cx) -> &AppState {
    app_context::<AppState>(cx)
}
```

- `routes.rs` の `build_router` で `.app_context(state)` と登録し（UNIT 03）、ページからは `app_state(cx).notes.search(…)` のように使います。例7-1の `config(cx)` と同じく、借りる関数を1つ作っています。
- DB とリポジトリを1つの型にまとめたので、登録する型は `AppState` の1つだけです。テストでは、一時フォルダの DB で作った `AppState` を登録します（第9巻 UNIT 01・第11巻 UNIT 02）。

ログイン中のユーザーを読む関数は、`src/auth.rs` で `#[memoize(as_ref)]` を付けています（抜き出しています）。

```rust
/// クッキーのトークンから、ログイン中のユーザーを DB で探す。
/// `#[memoize(as_ref)]` により、1つのリクエストの中では1回しか DB に問い合わせない
/// （レイアウトとページの両方から呼ばれても、2回目は覚えておいた結果を返す）。
#[memoize(as_ref)]
async fn load_current_user(cx: &Cx) -> Result<Option<User>, AppError> {
```

- レイアウト（`root_layout` のヘッダー）と、ページ（`require_user`）の両方が、ログイン中のユーザーを使います。例7-2と同じく、どこから何回呼んでも、1つの要求の中では DB を1回しか読みません。
- 「ログインが必要なページ」は、ページの最初で `require_user(cx)` を呼びます。入口のミドルウェアではなく関数なので、ページを読めば「このページはログインを確かめている」と分かります（第8巻 UNIT 01）。

### 練習問題

**問1:** 例7-2の `load_user` から `#[memoize(as_ref)]` を外すと、1回の表示で「DB を読みました」は何回出るでしょうか。かかった時間はどう変わるでしょうか。

**問2:** 「お知らせの文」（`Banner`）を、登録してもしなくてもよい値にしたいとき、どう借りればよいでしょうか。登録したルーターとしていないルーターを、1つのプログラムで作って比べてください。

**問3:** UNIT 06 の例6-1は、メモを `static MEMOS: Mutex<Vec<String>>` に覚えていました。これを `app_context` で共有する形に直してください。直すと、何が良くなるでしょうか。

### 解答と解説

**問1の解答:**

`#[memoize(as_ref)]` と `memoize` の `use` を外し、使わなくなった `cx` を `_cx` にしました（`src/bin/u07_q1.rs`）。

```text
@@OUT:u07_q1_curl@@
```

**▼ サーバーの窓の表示**

```text
@@OUT:u07_q1_server@@
```

- 1回の表示で6回（id = 1 が5回、id = 2 が1回）読みました。呼んだ回数だけ、そのまま DB を読んでいます。
- 時間は約 0.25 秒で、`#[memoize]` があったとき（約 0.11 秒）の倍ほどです。部品はいくつか同時に動くので、6 × 50 ミリ秒（0.3 秒）よりは短くなっていますが、本物の DB なら、問い合わせの数がそのまま DB の負担になります。

**問2の解答:**

`try_app_context::<Banner>(cx)` で借ります。登録されていなければ `None` で、panic しません（`src/bin/u07_q2.rs`）。

```rust
@@FILE:web_lab/src/bin/u07_q2.rs@@
```

```powershell
cargo run --bin u07_q2
```

```text
@@OUT:u07_q2@@
```

- 同じページの関数が、ルーターによって違う結果を返しました。`Banner` を登録したルーターだけ、お知らせが出ています。
- `get_home` は、サーバーを起動せずに、`router.handle(…)` で要求を1つ処理させています。RustNote のテスト（`tests/common/mod.rs` の `get` など）と同じやり方です。

**問3の解答:**

`Memos(Mutex<Vec<String>>)` という型を作り、`.app_context(Memos(…))` で登録して、`memos(cx)` で借ります（`src/bin/u07_q3.rs`。抜き出しています）。

```rust
/// 保存したメモ。ルーターに1つ登録して、どのページからも借りる
struct Memos(Mutex<Vec<String>>);

fn memos(cx: &Cx) -> &Memos {
    app_context::<Memos>(cx)
}
```

```text
@@OUT:u07_q3@@
```

- 動きは例6-1と同じです。良くなるのは、**メモの置き場が、ルーターごとに別になる**ことです。`static` は、プログラムの中に1つしかないので、問2のようにルーターを2つ作っても、同じメモを共有してしまいます。テストを同時にいくつも動かすと、ほかのテストのメモが混ざります（第11巻 UNIT 02 のエラー2と同じ問題です）。
- `app_context` なら、テストごとに新しい `Memos`（RustNote なら、新しい DB の `AppState`）を登録したルーターを作れます。

### まとめ

- アプリ全体で1つの値（設定・DB）は `.app_context(値)` で登録し、`app_context::<T>(cx)` で借りる。値は型で見分け、同じ型は1つだけ。`Send + Sync` でなければならず、書き換えるなら `AtomicU64` や `Mutex` を使う。
- `#[memoize]` は、1つの要求の中だけ結果を覚える。レイアウト・ページ・部品のどこから何回呼んでも、同じ引数なら1回しか計算しない。`as_ref` を付けると `Option<&T>` で借りられる。
- Topcoat は、入口のミドルウェアではなく、`cx` を受け取る小さな関数（`app_state(cx)`・`require_user(cx)`）を、必要な所で呼ぶ形を勧める。無くてもよい値は `try_app_context` で借りる。
