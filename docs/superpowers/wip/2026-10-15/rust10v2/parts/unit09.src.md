## UNIT 09: お知らせ（フラッシュ）・エラーのまとめ

### この単元のゴール

- お知らせ（フラッシュ）が、POST の処理で `flash=コード`（5 分で消えるクッキー）を予約し、移った先の GET で読んで消し、`role="status"` の文として**一度だけ**出す流れを、8種類すべてで確かめられる
- クッキーには決まったコードだけを入れるので、手で書き換えられても、知らない文は画面に出ないことを説明できる
- RustNote の失敗の返事（400・403・404・405・409・500）が、「同じ画面」「エラーの画面」「Topcoat の短い返事」のどこに出るかを、表にして言える
- お知らせを消し忘れる・500 の画面に原因を出す・画面でお知らせを取り出し忘れる、の3つの間違いの様子を読める

### たとえ話でつかむ

ホテルのフロントの、伝言を考えます。

- お客さんが出かけている間に電話があると、フロントの人は**伝言のメモ**を、お客さんの部屋の番号の箱に入れます。メモには、「お母様から電話」のような、**決まった言い方**のどれかだけを書きます。
- お客さんが戻ってきたら、フロントの人はメモを**渡して、箱を空に**します。次に前を通ったときに、同じ伝言をもう一度言われることはありません。
- メモは、**1日たったら捨てます**。何日も前の伝言が、急に出てくることはありません。
- フロントで困ったことが起きても、お客さんに「金庫の暗証番号の機械が壊れて…」と、中の事情は話しません。「少々お待ちください」とだけ伝え、事情は**係の人の記録**に書きます。

| ホテルの伝言 | RustNote のお知らせ・エラー |
|---|---|
| 箱にメモを入れる | `flash::set`（クッキー `flash=note_created`） |
| 決まった言い方だけ | `Flash` の8種類のコードだけ。知らないコードは無視 |
| 渡したら箱を空にする | `flash::take` が読んで、クッキーを消す（`Max-Age=0`） |
| 1日たったら捨てる | `Max-Age=300`（5 分） |
| 中の事情は話さず、記録に書く | 500 の画面は「時間をおいてもう一度お試しください」、原因はログの `[error]` の行 |

> **用語:** フラッシュメッセージ（flash：「ノートを作成しました」のように、移った先の画面で一度だけ出すお知らせ。RustNote はクッキーで次の画面へ渡す）

<!-- -->

> **用語:** `role="status"`（読み上げソフトに「この文はお知らせです」と伝える印。画面が変わったときに、文を読み上げてもらえる。第9巻 UNIT 09）

<!-- -->

> **用語:** 公開してよい文（ここでは、利用者の画面に出してよいエラーの文。RustNote の `AppError::public_message`。500 のときは中身を隠した決まり文句にする）

### 図で理解する

お知らせが、予約されてから消えるまでの流れです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-10-0901" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="40" y="10" width="140" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="110" y="32" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">ブラウザ</text>
<rect x="540" y="10" width="140" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="610" y="32" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">RustNote</text>
<line x1="110" y1="44" x2="110" y2="290" stroke="#94a3b8" stroke-width="2" stroke-dasharray="4 4"/>
<line x1="610" y1="44" x2="610" y2="290" stroke="#94a3b8" stroke-width="2" stroke-dasharray="4 4"/>
<line x1="110" y1="72" x2="606" y2="72" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0901)"/>
<text x="360" y="64" font-size="12" fill="#0f172a" text-anchor="middle">① POST /notes（作成）</text>
<line x1="610" y1="112" x2="114" y2="112" stroke="#10b981" stroke-width="2" marker-end="url(#ar-10-0901)"/>
<text x="360" y="96" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">② 303 → /notes/1　flash::set</text>
<text x="360" y="108" font-size="10" fill="#475569" text-anchor="middle">Set-Cookie: flash=note_created; HttpOnly; SameSite=Lax; Path=/; Max-Age=300</text>
<line x1="110" y1="152" x2="606" y2="152" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0901)"/>
<text x="360" y="144" font-size="12" fill="#0f172a" text-anchor="middle">③ GET /notes/1（Cookie: flash=note_created）</text>
<line x1="610" y1="200" x2="114" y2="200" stroke="#10b981" stroke-width="2" marker-end="url(#ar-10-0901)"/>
<text x="360" y="176" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">④ 200　flash::take → 「ノートを作成しました」</text>
<text x="360" y="192" font-size="10" fill="#475569" text-anchor="middle">Set-Cookie: flash=; Path=/; Max-Age=0（読んだら消す）</text>
<line x1="110" y1="240" x2="606" y2="240" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0901)"/>
<text x="360" y="232" font-size="12" fill="#0f172a" text-anchor="middle">⑤ 再読み込み（flash のクッキーは、もう無い）</text>
<line x1="610" y1="278" x2="114" y2="278" stroke="#10b981" stroke-width="2" marker-end="url(#ar-10-0901)"/>
<text x="360" y="270" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">⑥ 200（お知らせは出ない）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: お知らせの一生。POST の処理が決まったコードをクッキーに入れて 303 で移し、移った先の GET が読んで消して表示する。再読み込みでは、もう出ない</div>
</div>

失敗の返事が、どこに出るかです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 240" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="226" height="220" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="123" y="36" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">同じ画面に出す</text>
<text x="123" y="56" font-size="11" fill="#475569" text-anchor="middle">直せば通る失敗。入力を残す</text>
<text x="30" y="90" font-size="12" fill="#0f172a">400 入力の間違い</text>
<text x="40" y="108" font-size="11" fill="#475569">→ 入力欄の下（field-error）</text>
<text x="30" y="142" font-size="12" fill="#0f172a">409 古い version</text>
<text x="40" y="160" font-size="11" fill="#475569">→ フォームの上（form-notice）</text>
<rect x="247" y="10" width="226" height="220" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="36" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">エラーの画面に出す</text>
<text x="360" y="56" font-size="11" fill="#475569" text-anchor="middle">error_page（レイアウトの中）</text>
<text x="267" y="90" font-size="12" fill="#0f172a">400 フォームの形の間違い</text>
<text x="267" y="124" font-size="12" fill="#0f172a">404 無い・他人のノート</text>
<text x="267" y="158" font-size="12" fill="#0f172a">500 想定外の失敗</text>
<text x="277" y="176" font-size="11" fill="#475569">→ 決まり文句だけ。原因はログ</text>
<rect x="484" y="10" width="226" height="220" rx="10" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="597" y="36" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">Topcoat の短い返事</text>
<text x="597" y="56" font-size="11" fill="#475569" text-anchor="middle">ページの処理まで届かない</text>
<text x="504" y="90" font-size="12" fill="#0f172a">403 ほかのサイトから</text>
<text x="514" y="108" font-size="11" fill="#475569">→ 本文 forbidden</text>
<text x="504" y="142" font-size="12" fill="#0f172a">405 メソッドの違い</text>
<text x="514" y="160" font-size="11" fill="#475569">→ 本文なし・allow ヘッダー</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: 失敗の返事の出し方は3つ。利用者が直せる失敗は同じ画面に入力を残して出し、ページの処理の失敗はエラーの画面に、ルーターの入口で止めた物は Topcoat が短く返す</div>
</div>

### まず動かす最小コード

#### 例9-1: 8種類のお知らせを、順にたどる

お知らせは `src/flash.rs` です。予約と取り出しの部分を抜き出します。

```rust
/// 次の画面で出すメッセージを予約する
pub fn set(cx: &Cx, flash: Flash) {
    cookies(cx).add(cookie! {
        "flash" = flash.code();
        Path = "/";
        HttpOnly;
        SameSite = Lax;
        MaxAge = Duration::minutes(5)
    });
}

/// 予約されたメッセージを取り出し、クッキーを消す（一度だけ表示するため）
pub fn take(cx: &Cx) -> Option<Flash> {
    let jar = cookies(cx);
    let code = jar.get(COOKIE_NAME)?.value().to_string();
    // 消すときは、書いたときと同じ Path を指定する
    jar.remove(Cookie::build((COOKIE_NAME, "")).path("/").build());
    Flash::from_code(&code)
}
```

画面に出すのは、`src/views.rs` の `flash_banner` です。

```rust
#[component]
pub async fn flash_banner(flash: Option<Flash>) -> Result<impl View> {
    Ok(view! {
        if let Some(flash) = flash {
            <p class=(class!("flash", flash.kind())) role="status">(flash.message())</p>
        }
    })
}
```

> **▼ このコードがやること（先に日本語で）:** ブラウザの代わりに、セッションと `flash` のクッキーを覚える小さな入れ物（`Jar`）を作ります。新規登録・作成・保存・完了の切り替え・削除・ログアウト・ログインせずに一覧・ログインの8つの操作を順に行い、それぞれで、返事と予約されたクッキー、移動先を開いたときのお知らせと消すクッキー、もう一度開いたときのお知らせを表示します。

```rust
@@FILE:rustnote/tests/u09_flash.rs@@
```

```powershell
cargo test --test u09_flash -- --nocapture
```

```text
@@OUTGREP:u09_flash:^([0-9]\. |   (予約|/|消す|再読み込み))@@
```

（操作ごとの結果の行だけを抜き出しています。Expires の日時は、実行した日によって変わります）

#### ▼ コードを1つずつ分解して解説

##### 解説1: 8つとも、一度だけ出て、消えた

- 8つの操作は、どれも `303 See Other` で移り、`flash=コード; HttpOnly; SameSite=Lax; Path=/; Max-Age=300` が予約されました。移動先の画面にお知らせが出て、同じ返事で `flash=; Path=/; Max-Age=0`（すぐ消える、1年前の Expires）が来て、再読み込みでは `（なし）` でした。
- 7の「ログインせずに一覧」は、POST ではなく GET でも、`require_user` が `LoginRequired` を予約しています（UNIT 03 の図の①）。この1つだけ `class="flash warning"`（注意の色）で、ほかの7つは `flash success`（成功の色）です。`Flash::kind` で分けています（第9巻 UNIT 09 の色の決まり）。

##### 解説2: クッキーに入れるのは、コードだけ

- クッキーの値は `note_created` のような**決まったコード**で、画面の文（「ノートを作成しました」）は、`Flash::message` がコードから作ります。クッキーは利用者が書き換えられるので、文そのものを入れると、好きな文を RustNote のお知らせとして出せてしまいます（問1）。
- `HttpOnly`（JavaScript から読めない）と `SameSite=Lax` は、セッションのクッキーと同じ守りです（第8巻 UNIT 01）。`Max-Age=300` は、303 のあとに移動先を開けなかったときにも、古いお知らせがいつまでも残らないための期限です。
- `take` で消すときに `.path("/")` を付けているのは、ブラウザが、名前と Path の組でクッキーを区別するからです。Path が違うと、別のクッキーとして扱われ、元の `flash` が消えません。

### もう一歩: 実務に近い形

#### 例9-2: 失敗の返事を、状態コードごとに並べる

> **▼ このコードがやること（先に日本語で）:** alice がノートを1件作り、7つの失敗（空のタイトル・フォームの形の間違い・無いノート・古い version・GET での削除・ほかのサイトからの POST・DB に届かないときのログイン）を1つずつ起こします。返事の状態コードと、文が「同じ画面の入力欄の下」「同じ画面のフォームの上」「エラーの画面」「Topcoat の短い返事」のどこに出たか、その文を表示します。

```rust
@@FILE:rustnote/tests/u09_errors.rs@@
```

```powershell
cargo test --test u09_errors -- --nocapture
```

```text
@@OUTGREP:u09_errors:^(入力|フォーム|無い|古い|GET で|ほかの|DB に|\[error\])@@
```

- **同じ画面:** 空のタイトル（400）は入力欄の下に「タイトルを入力してください」、古い version（409）はフォームの上に「ほかの画面で先に更新されました…」が出ました。どちらも、利用者が直せば通る失敗なので、入力を残したまま、同じ画面で知らせます（UNIT 02・UNIT 04）。
- **エラーの画面:** フォームの形の間違い（400）・無いノート（404）・DB に届かない（500）は、レイアウトの中のエラーの画面（`error_page`）に出ました。500 の文は「時間をおいてもう一度お試しください」だけで、原因（`tcp connect error` など）は、サーバーのログの `[error]` の行にだけ書かれています。
- **Topcoat の短い返事:** GET での削除（405）は本文なし、ほかのサイトからの POST（403）は `forbidden` の1語でした。ルーターの入口で止めた物は、RustNote のページの処理（とレイアウト）まで届かないので、エラーの画面にはなりません（UNIT 06）。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: 取り出したお知らせを消さなかった

`take` から `jar.remove(…)` を外し、読むだけにした場合です（練習用のコピーの `src/flash.rs` を書き換え、確かめたら元に戻します）。例9-1のテストを動かします（最初の4つの操作の、お知らせの行だけを抜き出しています）。

```text
@@OUTGREP:u09_err_remove:^([1-4]\. |   (/notes|再読み込み))@@
```

- **何が起きたか:** 再読み込みでも、お知らせが出続けました。消す `Set-Cookie` が来ないので、ブラウザは `flash` を持ったままで、5 分たつか、次のお知らせで上書きされるまで、どの画面を開いても同じ文が出ます。「作成しました」を見たあとで一覧に戻ると、また「作成しました」と言われます。
- **学べること:** 「一度だけ」は、**読んだ人が消す**ことで実現しています。表示のテストに、「もう一度開くと出ない」を1行足しておくと、この間違いに気づけます（例9-1の「再読み込み」の行）。

#### エラー2（エラーが出ない間違い）: 500 の画面に、原因をそのまま出した

`error_page` で、`AppError` のときの文を `public_message()` ではなく、原因の鎖（`error.chain()`）にした場合です。例9-2のテストを動かします。

```text
@@OUTGREP:u09_err_leak:^(無い|DB に)@@
```

- **何が起きたか:** 500 の画面に、`DB の操作に失敗しました <- Hrana: … tcp connect error … (os error 10061)` が、そのまま出ました（`<-` は、HTML の中で `&lt;-` に置き換えられています）。404 は「ページが見つかりません」のままです。404 の原因の鎖は、その1つだけだからです。
- **困ること:** DB に何を使っているか（Hrana は、libSQL・Turso の通信の決まり）、どこにつなごうとしたか、が、使う人の誰にでも見えます。攻撃する人にとっては、手がかりになります。使う人にとっては、読んでも何もできない文です。
- **学べること:** 画面の文（`public_message`）と、ログの文（`log_if_internal` の `[error]` の行）を、**分けて**作ります（第4巻 UNIT 05）。

#### エラー3（エラーが出ない間違い）: 詳細の画面で、お知らせを取り出し忘れた

詳細の画面（`show`）で、`let flash = flash::take(cx);` を `let flash: Option<Flash> = None;` にした場合です。完了を切り替えて詳細へ移り、そのあと一覧を開くテスト（`tests/u09_late.rs`）を動かします。

```rust
@@FILE:rustnote/tests/u09_late.rs@@
```

まず、RustNote のコードです。

```text
@@OUTGREP:u09_late:^[0-9]\. @@
```

次に、取り出し忘れた版です。

```text
@@OUTGREP:u09_err_late:^([0-9]\. |warning: unused)@@
```

- **何が起きたか:** 切り替えた直後の詳細の画面にはお知らせが出ず、**次に開いた一覧の画面に**「完了の状態を切り替えました」が出ました。お知らせが、1画面遅れて出たのです。一覧で切り替えたわけではないので、利用者は戸惑います。
- **気づき方:** コンパイラーの `warning: unused import: crate::flash`（使われていない `use`）が、唯一の手がかりでした。お知らせを予約する操作の、**移動先の画面で**取り出すテスト（例9-1）があれば、2の行で気づけます。

### RustNote ではここで使う

- お知らせを出す画面（GET）は、どれも処理の最初の方で `flash::take(cx)` を呼び、`view!` の先頭に `flash_banner(flash: flash)` を置いています（一覧・詳細・ログイン・トップ）。
- お知らせの見た目は、`assets/style.css` の `.flash.success`（緑）と `.flash.warning`（黄）です（第9巻 UNIT 09）。`role="status"` が付いているので、読み上げソフトにも伝わります。
- `src/flash.rs` の先頭のコメントには、「クッキーの書き込みは、ページの処理が値を返す前に行う」という Topcoat の決まりも書いてあります。`set` は、`Err(see_other(…))` を返す前に呼びます。
- エラーの分類と状態コードは、`src/error.rs` の先頭の表と `AppError::status_code` に、画面に出す文は `AppError::public_message` に、まとめてあります。

### 練習問題

**問1:** `flash` のクッキーを、手で `<script>alert(1)</script>`・`note_deleted`・`NOTE_DELETED` にして一覧を開くと、それぞれ何が出るでしょうか。

**問2:** タブ A でノートを作成した直後（303 の移動先を開く前）に、タブ B で一覧を開きました。お知らせは、どちらのタブに出るでしょうか。

**問3:** 例9-2の7つの失敗のうち、「入力を残したまま、同じ画面に出す」のは、どれとどれでしょうか。ほかの失敗を同じ画面に出さないのは、なぜでしょうか。

### 解答と解説

```rust
@@FILE:rustnote/tests/u09_q.rs@@
```

```powershell
cargo test --test u09_q -- --nocapture --test-threads=1
```

```text
@@OUTGREP:u09_q:^問@@
```

（問題の答えの行だけを抜き出しています）

**問1の解答:**

- `<script>…</script>` と `NOTE_DELETED`（大文字）は、お知らせが出ませんでした。`Flash::from_code` は、8つのコードと**ちょうど同じ**文字だけを受け付け、ほかは `None` にします。好きな文や、プログラムを画面に出すことはできません。
- `note_deleted` は「ノートを削除しました」が出ました。何も消していないのに出るので、少し変ですが、出せるのは RustNote が決めた8つの文だけで、見るのはクッキーを書き換えた本人だけです。困ることはありません。

**問2の解答:**

- タブ B の一覧に「ノートを作成しました」が出て、タブ A の詳細には出ませんでした。クッキーは、**同じブラウザの全部のタブで共有**されるので、先に画面を開いたタブ B が、お知らせを取り出して消したからです。
- お知らせの文は「何かを作成した」ことしか言わないので、取り違えても困ることは小さいです。どのタブの操作かを正しく伝えたいときは、移動先の URL に印を付ける（`/notes/1?created=1` など）方法もあります。

**問3の解答:**

- 「入力を残したまま、同じ画面に出す」のは、空のタイトル（400）と、古い version（409）の2つです。どちらも、利用者が入力を直したり、確かめたりすれば、そのまま保存できる失敗です。
- フォームの形の間違い（400）は、RustNote の画面から送れば起きない失敗で、残す入力がありません。無いノート（404）は、出すフォームがありません。500 は、利用者が直せる失敗ではありません。だから、この3つは、エラーの画面で短く知らせます。403 と 405 は、ルーターの入口で止まるので、ページの処理まで届きません。

### まとめ

- お知らせは、POST の処理が `flash::set` で決まったコード（8種類）を 5 分のクッキーに入れ、移動先の GET が `flash::take` で読んで消し、`role="status"` の文として一度だけ出す。知らないコードは無視するので、クッキーを書き換えられても、決まった文しか出ない。
- 失敗の返事は、直せる失敗（400 の入力・409）は同じ画面に入力を残して、ページの処理の失敗（400 の形・404・500）はエラーの画面に、ルーターの入口の守り（403・405）は Topcoat の短い返事で出る。500 の画面には決まり文句だけを出し、原因はログに書く。
- お知らせを消し忘れると出続け、取り出し忘れると1画面遅れて出る。どちらもエラーは出ないので、「移動先で出る」「もう一度開くと出ない」をテストで確かめる。
