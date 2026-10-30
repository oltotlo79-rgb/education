## UNIT 06: 守りの回帰テスト

### この単元のゴール

- 攻撃の種類ごとにテスト関数を分け、攻撃の**入口**を表に並べて、全部の入口で止まることを確かめられる（他人のノート・ほかのサイトからの POST・保存した HTML・検索・セッション）
- 守りを1つずつ外した版で、どのテストが止まるかの表を作り、**テストの抜け**を見つけられる
- cargo-mutants で、関数をわざと自動で壊し、テストが気づかなかった壊し方（missed）を読める
- まだ足していない守りのテストを、`#[ignore = "理由"]` で残しておき、`--ignored` で確かめられる

### たとえ話でつかむ

ビルの警備の、**抜き打ちの訓練**を考えます。

- 警備員（守りのテスト）が入口に立っていても、本当に止めるかは、**侵入者の役**の人を、わざと入らせてみないと分かりません。
- 入口は1つではありません。正面・裏口・荷物の搬入口・非常口。**全部の入口**で、同じ訓練をします。どこか1つでも抜けられたら、守れていません。
- 訓練の記録は、「どの入口で、どの手口を試して、止めたか」を**表**にします。「止められなかった手口」が見つかったら、それが次に直す所です。
- 警備員を1人ずつ休ませて訓練すると、「この警備員がいないと、どの手口が通るか」が分かります。だれも気づかない休みがあれば、その警備員は、**いてもいなくても同じ**だったことになります。

| ビルの警備 | RustNote の守りのテスト |
|---|---|
| 侵入者の役 | わざと攻撃の形で送るリクエスト |
| 全部の入口 | 状態を変える全部の URL（作成・編集・切り替え・削除・ログアウト・ログイン・登録） |
| 訓練の記録の表 | 攻撃の種類ごとのテスト関数と、入口の表（例6-1） |
| 警備員を1人ずつ休ませる | 守りを1つずつ外した版（例6-2）・cargo-mutants（例6-3） |
| まだ雇っていない警備員 | `#[ignore]` で置いておく、守りのヘッダーのテスト |

> **用語:** 回帰テスト（一度できた守りや、一度直した間違いが、あとの変更で元に戻っていないかを、毎回確かめるテスト。第8巻 UNIT 10 で最初の1本を作った）

<!-- -->

> **用語:** ミューテーションテスト（mutation testing：プログラムを、少しずつ、わざと壊した版（mutant）をたくさん作り、テストがそれぞれに気づくか（mutant を「倒せるか」）を調べること。気づかなかった壊し方は、テストの抜けを指す）

<!-- -->

> **用語:** `#[ignore]`（テストに付ける印。ふだんの `cargo test` では動かさず、`cargo test -- --ignored` のときだけ動かす。`#[ignore = "理由"]` と書くと、理由が表示される）

### 図で理解する

この単元の守りのテスト（`tests/u06_attacks.rs`）の、攻撃と入口の表です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="700" height="280" rx="8" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="24" y="40" font-size="12" font-weight="700" fill="#0f172a">テスト関数（攻撃）</text>
<text x="250" y="40" font-size="12" font-weight="700" fill="#0f172a">入口</text>
<text x="590" y="40" font-size="12" font-weight="700" fill="#0f172a">止まった形</text>
<line x1="20" y1="52" x2="700" y2="52" stroke="#cbd5e1" stroke-width="1.5"/>
<text x="24" y="78" font-size="12" fill="#b91c1c" font-weight="700">idor_every_entrance</text>
<text x="250" y="78" font-size="11" fill="#0f172a">GET 詳細・編集 / POST 編集・切り替え・削除（5つ）</text>
<text x="590" y="78" font-size="11" fill="#0f172a">404・変わらない</text>
<text x="24" y="118" font-size="12" fill="#c2410c" font-weight="700">csrf_every_post</text>
<text x="250" y="110" font-size="11" fill="#0f172a">POST 作成・編集・切り替え・削除・</text>
<text x="250" y="128" font-size="11" fill="#0f172a">ログアウト・ログイン・登録（7つ）</text>
<text x="590" y="118" font-size="11" fill="#0f172a">403・変わらない</text>
<text x="24" y="166" font-size="12" fill="#6d28d9" font-weight="700">stored_xss_is_text</text>
<text x="250" y="166" font-size="11" fill="#0f172a">一覧・詳細・編集の画面（3つ）</text>
<text x="590" y="166" font-size="11" fill="#0f172a">img・script が 0</text>
<text x="24" y="206" font-size="12" fill="#1d4ed8" font-weight="700">search_cannot_cross_owners</text>
<text x="250" y="206" font-size="11" fill="#0f172a">検索の言葉（% _ ' OR 1=1 など 6つ）</text>
<text x="590" y="206" font-size="11" fill="#0f172a">alice のが見えない</text>
<text x="24" y="246" font-size="12" fill="#047857" font-weight="700">session_guards</text>
<text x="250" y="246" font-size="11" fill="#0f172a">ログインし直し・ログアウト</text>
<text x="590" y="246" font-size="11" fill="#0f172a">前のクッキーは 303</text>
<text x="24" y="278" font-size="11" fill="#64748b">security_headers（#[ignore]：まだ足していない守り。--ignored のときだけ動く）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: 攻撃の種類ごとにテスト関数を分け、入口を表に並べる。止まったかどうかに加えて、「何も変わっていない」ことも確かめる</div>
</div>

守りを1つずつ外した5つの版で、どのテストが止まったかです（例6-2の結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="700" height="230" rx="8" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="24" y="38" font-size="12" font-weight="700" fill="#0f172a">外した守り</text>
<text x="400" y="38" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">u06_attacks</text>
<text x="600" y="38" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">RustNote の tests/http.rs</text>
<line x1="20" y1="50" x2="700" y2="50" stroke="#cbd5e1" stroke-width="1.5"/>
<text x="24" y="76" font-size="11" fill="#0f172a">切り替えの SQL の持ち主の条件</text>
<text x="400" y="76" font-size="12" fill="#047857" text-anchor="middle">◎ 止まった</text>
<text x="600" y="76" font-size="12" fill="#047857" text-anchor="middle">◎ 止まった</text>
<text x="24" y="110" font-size="11" fill="#0f172a">削除の SQL の持ち主の条件</text>
<text x="400" y="110" font-size="12" fill="#047857" text-anchor="middle">◎ 止まった</text>
<text x="600" y="110" font-size="12" fill="#047857" text-anchor="middle">◎ 止まった</text>
<text x="24" y="144" font-size="11" fill="#0f172a">検索の条件のかっこ（第8巻 UNIT 07）</text>
<text x="400" y="138" font-size="12" fill="#047857" text-anchor="middle">◎ 止まった</text>
<text x="400" y="154" font-size="10" fill="#b91c1c" text-anchor="middle">（直す前は、通ってしまった）</text>
<text x="600" y="144" font-size="12" fill="#b91c1c" text-anchor="middle">× 通ってしまった</text>
<text x="24" y="178" font-size="11" fill="#0f172a">Origin の検査（dangerous_disable）</text>
<text x="400" y="178" font-size="12" fill="#047857" text-anchor="middle">◎ 止まった</text>
<text x="600" y="178" font-size="12" fill="#047857" text-anchor="middle">◎ 止まった</text>
<text x="24" y="212" font-size="11" fill="#0f172a">ログアウトで DB の記録を消す</text>
<text x="400" y="212" font-size="12" fill="#047857" text-anchor="middle">◎ 止まった</text>
<text x="600" y="212" font-size="12" fill="#047857" text-anchor="middle">◎ 止まった</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: 守りを1つずつ外した表。どちらも止まらない守りがあれば、テストの抜け。検索のかっこは、u06_attacks も、テストのデータを直すまで通ってしまった（エラー3）</div>
</div>

### まず動かす最小コード

#### 例6-1: 攻撃の種類ごとに、全部の入口を確かめる

第8巻 UNIT 10 の守りのテストは、5つの守りを1つのテスト関数で確かめたので、最初の失敗で止まり、残りの守りが確かめられませんでした。ここでは、**攻撃の種類ごとに関数を分け**、それぞれの中で、入口を表に並べます。

> **▼ このコードがやること（先に日本語で）:** alice と bob を登録し、alice のノートを1つ作ります（本文にも、秘密の言葉を入れます）。そのうえで、①bob から、alice のノートの5つの入口を試して 404、②ほかのサイトから、状態を変える7つの入口に POST して 403、③HTML の入った題と本文が、3つの画面で、ただの文字になっていること、④bob が、いろいろな検索の言葉で、alice のノートを見られないこと、⑤ログインし直したときと、ログアウトしたときの、前のクッキーが使えないこと、を確かめます。①②では、止まったあとで、alice のノートが**変わっていない**ことも確かめます。⑥守りのヘッダーのテストは、`#[ignore]` で置いておきます。

```rust
@@FILE:rustnote/tests/u06_attacks.rs@@
```

```powershell
cargo test --test u06_attacks
```

**▼ 実行結果**

```text
@@OUTGREP:u06_attacks:^(running|test )@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 1つの攻撃に1つの関数、1つの入口に1つの名前

- 5つの関数は、別々に動きます。どれか1つが失敗しても、ほかの関数の結果は、ちゃんと出ます（`ok` か `FAILED` か）。
- 入口ごとに、`assert_eq!` の3つ目に `"IDOR: POST {path}"` のような名前を入れました。失敗したとき、**どの攻撃の、どの入口か**が、すぐ分かります（例6-2）。

##### 解説2: 「止まった」だけでなく「変わっていない」も確かめる

- 他人のノートの削除が 404 でも、もし先に消してから 404 を返す作りなら、ノートは消えています。だから、①②のあとで、`alice_note_is_intact` で、alice のノートの題と完了の印が、元のままかを確かめます。②では、alice がログアウトさせられていないことも確かめます。

##### 解説3: ログインと登録も、ほかのサイトからの POST は 403

- ②の7つの入口には、`/login` と `/signup` も入れ、どちらも 403 でした。Topcoat の Origin の検査は、**全部の POST** にかかるからです（第8巻 UNIT 06）。ほかのサイトから、攻撃する人のアカウントでログインさせる、ログインの CSRF（第8巻 UNIT 09）も、ここで止まります。

##### 解説4: まだ無い守りは `#[ignore = "理由"]` で置いておく

- `security_headers` は、`ignored, 改善点: 守りのヘッダーを、まだ返していない（UNIT 03 の問2）` と表示され、動きませんでした。今の RustNote では失敗するテストを、そのまま置くと、`cargo test` がいつも赤くなり、ほかの失敗に気づけなくなります。消してしまうと、守りが足りないことを忘れます。`#[ignore]` で、**理由を書いて**置いておきます（エラー1・問2）。

### もう一歩: 実務に近い形

#### 例6-2: 守りを1つずつ外して、どのテストが止まるかを表にする

守りのテストが本当に守っているかは、**守りを外してみる**と分かります。練習用のコピーで、守りを1つずつ外した5つの版を作り（第8巻・この巻の UNIT 02 で使った置き換え表）、それぞれで、`u06_attacks` と、RustNote の `tests/http.rs` を動かしました（`cap11_u06.sh` の繰り返し）。

```bash
for name in toggle delete search origin logout; do
  # 守りを1つ外す → 2つのテストを動かす → 失敗したテストを記録 → 元に戻す
done
```

```text
@@OUT:u06_matrix@@
```

- 5つの版の全部で、`u06_attacks` のどれかの関数が止まりました。止まった関数の名前が、そのまま「外された守り」を指しています（切り替え・削除の持ち主 → `idor_every_entrance`、Origin → `csrf_every_post`、ログアウト → `session_guards`、検索 → `search_cannot_cross_owners`）。
- RustNote の `tests/http.rs` は、検索のかっこを外した版だけ、**全部通って**しまいました（エラー3）。
- 失敗のメッセージも比べます。切り替えの持ち主の条件を外した版です。

```text
@@OUTGREP:u06_kill_toggle_attacks:^(thread|assertion|  left| right)@@
@@OUTGREP:u06_kill_toggle_http:^(thread|assertion|  left| right)@@
```

- `u06_attacks` は「`IDOR: POST /notes/1/toggle`」と、攻撃と入口を言います。`tests/http.rs` は、行番号（218 行目）しか言わないので、ファイルを開いて、その行を読む必要があります。

#### 例6-3: cargo-mutants で、自動でわざと壊す

例6-2では、壊し方を人が5つ考えました。cargo-mutants は、関数の中身を「いつも `Ok(true)` を返す」「何もしないで `Ok(())` を返す」のように、**機械的に**書き換えた版を作り、1つずつテストを動かします。テストが失敗すれば「倒した」（caught）、全部通れば「見逃した」（missed）です。

cargo-mutants 27.1.0 は、Windows（ARM64）では組み立てられなかったので（エラー2）、WSL の Ubuntu（aarch64）に入れて動かしました。壊した版ごとにテストを組み立て直すので、比べたいテスト（`tests/http.rs` と `tests/u06_attacks.rs`、あとで `tests/repository.rs`）だけを入れたコピーで動かしました。

```bash
# WSL（Ubuntu）の中で
cargo install cargo-mutants --version 27.1.0 --locked
cargo mutants --list --file src/auth.rs
```

```text
@@OUT:u06_mutants_list@@
```

- `src/auth.rs` の関数を、1つずつ「いつも決まった値を返すだけ」に書き換えた、13 個の版が作られます。たとえば `verify_password` を `Ok(true)`（どんなパスワードでも合っていることにする）にした版や、`log_out` を `Ok(())`（何もしないでログアウトしたことにする）にした版です。

RustNote の `tests/http.rs` だけで、13 個を確かめます（`--` のうしろは、`cargo test` に渡す引数です）。

```bash
cargo mutants --file src/auth.rs --timeout 300 -- --test http
```

```text
@@OUT:u06_mutants_http@@
```

```text
@@OUT:u06_mutants_http_unviable@@
```

- 9 個は倒し（caught）、4 個は **unviable**（組み立てられない版）でした。4 個とも「`Default::default()` の `User` を返す」版で、RustNote の `User` に `Default`（決まった初めの値）が無いので、コンパイルできません。組み立てられない版は、テストを動かすまでもなく、ありえない壊し方です。
- **見逃し（missed）は 0 個**でした。`tests/http.rs` は、`auth.rs` の関数を丸ごと何もしない版にすると、どれでも気づきます。`u06_attacks` を足しても同じでした。

```text
@@OUT:u06_mutants_both@@
```

検索や削除の SQL がある `src/repository/notes.rs` でも、HTTP のテスト（`tests/http.rs` と `u06_attacks`）で試しました。

```bash
cargo mutants --file src/repository/notes.rs --timeout 300 -- --test http --test u06_attacks
```

```text
@@OUT:u06_mut_notes_http@@
```

```text
@@OUT:u06_mut_notes_http_caught@@
```

- 12 個のうち、組み立てられる 6 個を全部倒しました（ほかの 6 個は、`Note` や `NotePage` に `Default` が無いので unviable）。削除で「消えた行の数が 0 か」を確かめる `==` を `!=` にした版や、`escape_like` を空の文字列を返す版も、倒せました。
- リポジトリの結合テストとユニットテスト（`--lib --test repository`）も足して動かすと、次のとおりでした。

```text
@@OUT:u06_mut_notes_all@@
```

- **大事な点:** cargo-mutants は、Rust のコード（関数の中身・`==` などの記号）を壊しますが、**SQL の文字列の中は壊しません**。例6-2で、`u06_attacks` も `tests/http.rs` も見逃した「検索の条件のかっこを外す」は、SQL の文字列の中の間違いなので、cargo-mutants の一覧に、もともと入っていません。「見逃し 0」は、「cargo-mutants が作った壊し方は、全部倒した」という意味で、「守りに抜けが無い」という意味ではありません。SQL の守りは、例6-2のように、人が壊し方を考えて確かめます。

### よくあるエラーと読み方

#### エラー1: まだ無い守りのテストを、`--ignored` で動かした

```powershell
cargo test --test u06_attacks -- --ignored
```

```text
@@OUTGREP:u06_ignored:^(running|test |thread|ヘッダー|test result)@@
```

- **何が起きたか:** `--ignored` は、`#[ignore]` の付いたテスト**だけ**を動かします。`x-content-type-options` が無いので、1つ目の確かめで止まりました。今の RustNote では、これが正しい結果です（改善点がまだ直っていない、という記録）。
- **読み方:** 守りのヘッダーを足したら、`#[ignore]` を外して、ふだんのテストに戻します。全部を一度に動かすときは `--include-ignored` を使います（問2）。

#### エラー2: Windows（ARM64）で、cargo-mutants の組み立てに失敗した

例6-3の cargo-mutants 27.1.0 を、Windows（ARM64）に `cargo install cargo-mutants --version 27.1.0 --locked` で入れようとした結果です。

```text
@@OUT:u06_mutants_windows@@
```

- **何が起きたか:** cargo-mutants が使っている古いクレート（`kernel32-sys 0.2.2` が使う `winapi 0.2.8`）が、32 ビット（x86）と 64 ビット（x86_64）の Windows のための型しか持っておらず、ARM64 の Windows では `ULONG_PTR` などの型が見つかりませんでした（285 個の E0425）。`the item is gated behind the x86 feature` が、その手がかりです。
- **直し方:** 道具の側の問題なので、自分のコードでは直せません。この単元では、WSL の Ubuntu（aarch64 の Linux）に入れて動かしました。Linux では `winapi` を組み立てないので、入りました。道具の不具合は、エラーの先頭のクレートの名前（ここでは `winapi`）から、その道具の課題（issue）を探します。

#### エラー3（通ってしまう）: 検索のかっこを外したのに、守りのテストが通った

例6-2の表を、最初に作ったときの結果です。そのときの `u06_attacks` は、alice のノートの本文が空でした。

```text
@@OUT:u06_matrix_before@@
```

- **何が起きたか:** 検索の条件 `user_id = ?1 AND (title LIKE ?2 … OR body LIKE ?2 …)` のかっこを外すと、`user_id = ?1 AND title LIKE ?2 … OR body LIKE ?2 …` になり、`OR` のうしろの「本文が合う」だけで、**だれのノートでも**見つかります（第8巻 UNIT 07）。ところが、テストの alice のノートは本文が**空**だったので、本文から漏れる道すじを、一度も試していませんでした。`u06_attacks` も `tests/http.rs` も、通ってしまいました。
- **直し方:** テストのデータに、攻撃が通る道すじを用意します。alice のノートの本文に秘密の言葉（「金庫の番号は 1234」）を入れ、bob がその言葉で検索する行を足しました。

```diff
@@FILE:out/u06_diff_attacks.txt@@
```

```text
@@OUTGREP:u06_kill_search_attacks:^(thread|検索)@@
```

- 直したあとは、「金庫」で検索したときに、alice のノートが見えて、止まりました。
- **学べること:** 守りのテストは、「攻撃の形のリクエスト」だけでなく、「攻撃が成功したら困るデータ」も用意して、初めて守りになります。守りを外して試す（例6-2）と、こうした抜けが見つかります。

### RustNote ではここで使う

- RustNote の `tests/http.rs` は、他人のノート（`other_users_notes_are_404`）・ほかのサイトからの POST（`cross_site_post_is_403`、`/notes` の1つだけ）・ログアウト（`login_and_logout`）を確かめています。例6-2のとおり、5つの壊し方のうち4つには気づきます。
- **改善点:** 検索のテスト（`list_searches_and_paginates`）は、bob から alice のノートが見えないことを確かめていないので、検索のかっこを外しても気づきません（エラー3）。CSRF は `/notes` しか確かめていないので、ほかの6つの入口も表にするとよいです。`assert` に、攻撃と入口の名前を書くと、失敗を読みやすくなります。守りのヘッダーは、まだありません（`#[ignore]` のテスト）。
- この単元の `tests/u06_attacks.rs` は、練習用のコピーにだけ足しました。CI で毎回動かす方法は、UNIT 10 で扱います。

### 練習問題

**問1:** 切り替え・削除・ログアウトの入口は、POST だけです。ほかのサイトのリンクから、GET で開かせたら、どうなるでしょうか。alice のノートと、ログインの状態は、どうなるでしょうか。

**問2:** `#[ignore]` のテストも含めて、全部を一度に動かすには、どうすればよいでしょうか。

**問3:** 例6-2で人が作った5つの壊し方のうち、cargo-mutants が作る壊し方の一覧（例6-3）に入っているのは、どれでしょうか。

### 解答と解説

**問1の解答:**

```rust
@@FILE:rustnote/tests/u06_q1.rs@@
```

```text
@@OUTGREP:u06_q1:^(ほかの|alice)@@
```

- 3つとも `405 Method Not Allowed`（その方法は受け付けない）で、`Allow: POST`（POST なら受け付ける）と返りました。ノートも、ログインの状態も、変わりませんでした。
- Origin の検査は、GET を止めません（ほかのサイトのリンクから開くのは、ふつうのことだから）。だから、**状態を変える入口を POST だけにする**こと自体が、守りの1つです（第10巻 UNIT 06 で、削除を GET にした版が、ほかのサイトのリンクで消えたことを確かめました）。この表も、守りのテストに足せます。

**問2の解答:**

```powershell
cargo test --test u06_attacks -- --include-ignored
```

```text
@@OUTGREP:u06_include:^(test |ヘッダー|test result)@@
```

- `--include-ignored` は、ふだんのテストと、`#[ignore]` のテストを、**両方**動かします（`--ignored` は、`#[ignore]` のテストだけ）。5つが通り、守りのヘッダーの1つが失敗しました。
- CI では、ふだんは `cargo test`（赤にならない）、週に1回などは `--include-ignored` で、「まだ直していない守り」を数える、という使い分けができます。

**問3の解答:**

- 今回、cargo-mutants の見逃しは、`src/auth.rs` でも `src/repository/notes.rs` でも 0 個でした。なので、ここでは逆に、「例6-2で人が作った5つの壊し方のうち、cargo-mutants の一覧に入っているのはどれか」を考えます。
- `log_out` を `Ok(())` にする版は、「ログアウトで何もしない」ので、例6-2の「ログアウトで DB の記録を消さない」に近い壊し方です（cargo-mutants の版は、クッキーも消しません）。例6-2の版も、cargo-mutants の版も、`tests/http.rs` だけで倒せました。
- 残りの4つ（切り替え・削除の持ち主の条件、検索のかっこ、Origin の検査）は、SQL の文字列の中か、ルーターの組み立て（`src/routes.rs`）の1行で、cargo-mutants は作りません。だから、cargo-mutants で「見逃し 0」でも、例6-2のような、人が考えた壊し方の表は残しておきます。2つは、置き換えではなく、組み合わせて使います。

### まとめ

- 守りの回帰テストは、攻撃の種類ごとに関数を分け、入口を表に並べて、全部の入口で確かめる。止まったことと、何も変わっていないことの両方を確かめ、`assert` に攻撃と入口の名前を書く。Topcoat の Origin の検査は、ログインと登録を含む全部の POST で 403 になった。
- 守りを1つずつ外すと、テストの抜けが見つかる。検索のかっこを外した版は、テストのデータ（本文が空）が攻撃の道すじを通っていなかったので、u06_attacks も tests/http.rs も通ってしまった。本文に秘密の言葉を入れると、気づくようになった。
- cargo-mutants は、関数を機械的に壊した版で、テストの抜けを探す。Windows（ARM64）では組み立てられず、WSL で動かした。まだ無い守りのテストは `#[ignore = "理由"]` で置き、`--ignored`・`--include-ignored` で動かす。
