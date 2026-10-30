## UNIT 04: 編集・入力済みのフォーム・競合

### この単元のゴール

- 編集のフォーム（`GET /notes/{id}/edit`）が、今の値を入れた入力欄と、`version` の hidden を出すことと、保存（`POST`）で `version` が 1 増えることを確かめられる
- 2つのタブで同じノートを直したとき、あとから保存したほうが **409** になり、入力を残したまま、最新の `version` でフォームを出し直す流れを説明できる
- `UPDATE … AND version = ?5` の条件を外すと、**先に保存した内容が黙って消える**（失われた更新）ことを、実際に確かめて説明できる
- hidden の `version` を忘れたときの 400 と、`Option<i64>` の場所に `i64` を入れたときの `E0308` を読める

### たとえ話でつかむ

みんなで使う、ホワイトボードの予定表を考えます。

- 書き直したい人は、まず予定表の**写真**を撮り、自分の席で直す内容を考えます。写真の隅には、ボードの**版の番号**（たとえば「1版」）が写っています。
- 席から戻って書き直すとき、ボードの番号が写真と同じ「1版」なら、そのまま書き直し、番号を「2版」にします。
- もし番号が「2版」になっていたら、ほかの人が先に書き直したということです。黙って自分の内容で上書きすると、その人の書いた所が消えてしまいます。そこで、いったん手を止めて、「**先に直した人がいます。今のボードはこうです。それでも書き直しますか**」と確かめます。
- ボードを使っている間じゅう、ほかの人に触らせない（鍵をかける）方法もあります。でも、写真を撮ったまま帰ってしまう人がいたら、誰も書けなくなります。

| ホワイトボード | RustNote の編集 |
|---|---|
| 写真を撮る（隅に版の番号） | 編集のフォームを開く（hidden の `version`） |
| 番号が同じなら書き直し、番号を進める | `UPDATE … WHERE … AND version = ?5` と `version = version + 1` |
| 番号が進んでいたら、手を止めて確かめる | 0 行の更新 → `VersionMismatch` → **409** で、最新の版の番号と題を見せて出し直す |
| ほかの人に触らせない鍵 | 悲観的ロック。Web の画面では使わない |

> **用語:** 楽観的ロック（「同時に直す人は、めったにいない」と考えて、鍵はかけずに、保存のときに版の番号（`version`）が変わっていないかだけを確かめる方法。第7巻 UNIT 05）

<!-- -->

> **用語:** 失われた更新（lost update：2人が同じデータを直したとき、あとから保存した人の内容が、先に保存した人の内容を、黙って上書きしてしまうこと）

<!-- -->

> **用語:** 409 Conflict（「今のデータの状態と、ぶつかっています」という返事。RustNote では、ほかの画面で先に保存されたときに使う）

### 図で理解する

2つのタブで、同じノートを直したときの流れです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 380" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-10-0401" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="10" width="140" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="90" y="32" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">タブ A</text>
<rect x="290" y="10" width="140" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="32" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">DB（notes）</text>
<rect x="560" y="10" width="140" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="630" y="32" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">タブ B</text>
<line x1="90" y1="44" x2="90" y2="370" stroke="#94a3b8" stroke-width="2" stroke-dasharray="4 4"/>
<line x1="360" y1="44" x2="360" y2="370" stroke="#94a3b8" stroke-width="2" stroke-dasharray="4 4"/>
<line x1="630" y1="44" x2="630" y2="370" stroke="#94a3b8" stroke-width="2" stroke-dasharray="4 4"/>
<text x="360" y="68" font-size="12" fill="#0f172a" text-anchor="middle">① A も B も編集のフォームを開く（どちらも version=1）</text>
<line x1="90" y1="104" x2="356" y2="104" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0401)"/>
<text x="225" y="96" font-size="11" fill="#0f172a" text-anchor="middle">② 保存（version=1）</text>
<text x="372" y="108" font-size="11" fill="#047857" text-anchor="start">1行を更新 → version=2</text>
<line x1="356" y1="128" x2="94" y2="128" stroke="#10b981" stroke-width="2" marker-end="url(#ar-10-0401)"/>
<text x="225" y="146" font-size="11" font-weight="700" fill="#047857" text-anchor="middle">303 → 詳細「保存しました」</text>
<line x1="630" y1="184" x2="364" y2="184" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0401)"/>
<text x="495" y="176" font-size="11" fill="#0f172a" text-anchor="middle">③ 保存（version=1 のまま）</text>
<text x="348" y="188" font-size="11" fill="#c2410c" text-anchor="end">version=2 なので、更新は 0 行</text>
<line x1="364" y1="216" x2="626" y2="216" stroke="#f97316" stroke-width="2" marker-end="url(#ar-10-0401)"/>
<text x="495" y="236" font-size="11" font-weight="700" fill="#c2410c" text-anchor="middle">409 で出し直す</text>
<text x="495" y="254" font-size="11" fill="#475569" text-anchor="middle">B の入力は残す・hidden は version=2</text>
<text x="495" y="270" font-size="11" fill="#475569" text-anchor="middle">「最新のタイトルは…」のお知らせ</text>
<line x1="630" y1="310" x2="364" y2="310" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0401)"/>
<text x="495" y="302" font-size="11" fill="#0f172a" text-anchor="middle">④ B が確かめて、もう一度保存（version=2）</text>
<text x="348" y="314" font-size="11" fill="#047857" text-anchor="end">1行を更新 → version=3</text>
<line x1="364" y1="342" x2="626" y2="342" stroke="#10b981" stroke-width="2" marker-end="url(#ar-10-0401)"/>
<text x="495" y="362" font-size="11" font-weight="700" fill="#047857" text-anchor="middle">303 → 詳細（B の内容で上書き）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: 2つのタブの楽観的ロック。あとから古い version で保存した B は 409 になり、最新の内容を知ってから、自分で上書きを選ぶ</div>
</div>

保存の SQL と、1行も更新されなかったときの見分け方です（`src/repository/notes.rs` の `update`）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-10-0402" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="680" height="76" rx="8" fill="#0f172a"/>
<text x="40" y="44" font-size="12" fill="#e2e8f0" font-family="Consolas, monospace">UPDATE notes SET title = ?1, body = ?2, version = version + 1, …</text>
<text x="40" y="64" font-size="12" fill="#e2e8f0" font-family="Consolas, monospace"> WHERE id = ?3 AND user_id = ?4 AND <tspan fill="#fbbf24">version = ?5</tspan></text>
<text x="40" y="84" font-size="12" fill="#e2e8f0" font-family="Consolas, monospace"> RETURNING …</text>
<line x1="200" y1="96" x2="140" y2="130" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0402)"/>
<line x1="460" y1="96" x2="460" y2="130" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0402)"/>
<rect x="20" y="134" width="240" height="60" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="140" y="158" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">1行返った</text>
<text x="140" y="178" font-size="11" fill="#0f172a" text-anchor="middle">Ok(新しいノート) → 303</text>
<rect x="300" y="134" width="320" height="60" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="460" y="158" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">0行（何も返らない）</text>
<text x="460" y="178" font-size="11" fill="#0f172a" text-anchor="middle">id と持ち主だけで、もう一度探す（find）</text>
<line x1="400" y1="194" x2="330" y2="214" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0402)"/>
<line x1="520" y1="194" x2="590" y2="214" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0402)"/>
<text x="300" y="236" font-size="11" font-weight="700" fill="#c2410c" text-anchor="middle">ある → VersionMismatch（409）</text>
<text x="600" y="236" font-size="11" font-weight="700" fill="#b91c1c" text-anchor="middle">無い → NotFound（404）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: 保存の SQL。version が合ったときだけ1行が更新され、version が1つ進む。0行なら、id と持ち主だけで探し直して、版の違い（409）か、無いノート（404）かを見分ける</div>
</div>

### まず動かす最小コード

#### 例4-1: 編集のフォームを開き、hidden の version ごと送り返す

編集のフォームは、作成と同じ部品（`src/views.rs` の `note_form`）を使います。違いは、`version` があるときに hidden の入力欄を出すことです（抜き出しています）。

```rust
<form method="post" action=(action) class="note-form">
    if let Some(notice) = form.notice {
        <p class="form-notice">(notice)</p>
    }
    if let Some(version) = form.version {
        <input type="hidden" name="version" value=(version)>
    }
    // …（タイトルの input・本文の textarea・ボタン）
</form>
```

送られてきた値は、`src/routes/notes/id/edit.rs` の `EditForm` に入ります。

```rust
#[derive(Deserialize)]
pub struct EditForm {
    title: String,
    body: String,
    /// フォームを開いたときの version
    version: i64,
}
```

> **▼ このコードがやること（先に日本語で）:** alice がノートを1件作り、DB の行を表示します。次に、編集のフォームの HTML から、ブラウザが送り返す値（hidden の `version`・タイトル・本文）を読み取って表示します。1秒待ってから、タイトルだけを変えて、読み取った `version` ごと保存（POST）し、返事と、保存したあとの DB の行を表示します。

```rust
@@FILE:rustnote/tests/u04_edit.rs@@
```

```powershell
cargo test --test u04_edit -- --nocapture
```

```text
@@OUTGREP:u04_edit:^(?!   Compiling)@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: フォームには、今の値と version が入っている

- 2のフォームの値は、`version=Some("1")`・`title="牛乳を買う"`・`body="1本"` でした。編集の画面（`edit_page`）は、`find_own_note` で読んだノートを `NoteFormState::from_input` に入れ、`version: Some(note.version)` を足しています。入力欄に今の値が入っているので、直したい所だけを変えられます。
- `version` は hidden（画面に見えない入力欄）なので、利用者は気づきません。でも、ブラウザは、ほかの入力欄と同じように、この値も送り返します。**「どの版を見て直したか」**を、フォームが覚えて運んでいるのです。

##### 解説2: 保存すると、version が 1 進み、updated_at が変わる

- 3の保存は `303 See Other` で、`Location: /notes/1`（詳細）でした。作成と同じ PRG です（UNIT 02）。
- 4の DB では、タイトルが `牛乳を2本買う` に、`version` が `1` から `2` に、`updated_at` が1秒あとの時刻に変わりました。`UPDATE` の `version = version + 1` と `updated_at = datetime('now')` です。`datetime('now')` は秒までなので、テストでは1秒待ってから保存しています。
- 本文は送り返した `1本` のままです。HTML のフォームは、変えていない項目も、全部送ります。

### もう一歩: 実務に近い形

#### 例4-2: 2つのタブで、同じノートを直す

> **▼ このコードがやること（先に日本語で）:** 同じノートの編集のフォームを、2回開きます（タブ A とタブ B）。タブ A が先にタイトルを `牛乳を2本買う` にして保存し、次にタブ B が、古い `version` のまま `牛乳と卵を買う` にして保存します。タブ B の返事の、状態コード・お知らせの文・出し直したフォームの `version` とタイトルを表示します。最後に、タブ B が出し直したフォームで、もう一度保存します。それぞれのあとで、DB の行を表示します。

```rust
@@FILE:rustnote/tests/u04_conflict.rs@@
```

```powershell
cargo test --test u04_conflict -- --nocapture
```

```text
@@OUTGREP:u04_conflict:^(?!   Compiling)@@
```

##### 解説1: あとから古い version で保存すると 409

- 3のタブ B の保存は **`409 Conflict`** で、DB は `牛乳を2本買う`・`version=2` のままでした。タブ A の内容は消えていません。
- SQL の `AND version = ?5` に、タブ B は `1` を渡しました。DB の `version` は `2` なので、条件に合う行は無く、更新は 0 行です。リポジトリは、id と持ち主だけで探し直し、ノートはあるので `VersionMismatch` を返しました（図の2つ目）。
- お知らせは、`AppError::Conflict` の文（「ほかの画面で先に更新されました。読み込み直してください」）に、最新のタイトル（`牛乳を2本買う`）と、「もう一度『保存する』を押すと上書きする」ことを足した文です。

##### 解説2: 入力は残し、version だけを新しくする

- 出し直したフォームは、`title="牛乳と卵を買う"`（タブ B が書いた内容）と、`version=Some("2")`（最新の版）でした。入力を捨てて最新の内容に置き換えると、タブ B で書いた内容が消えてしまうからです。
- 4のもう一度の保存は `303` で、DB は `牛乳と卵を買う`・`version=3` になりました。今度は、タブ B が「最新は `牛乳を2本買う` だ」と**知ったうえで**、上書きを選んでいます。黙って消えるのと、確かめてから上書きするのとは、違います。
- 鍵をかける方法（悲観的ロック）を使わないのは、Web の画面では、フォームを開いたまま、タブを閉じたり、何時間も放っておいたりできるからです。鍵を外す時を、サーバーは知ることができません。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: UPDATE から version の条件を外した

`src/repository/notes.rs` の `update` の SQL を、`WHERE id = ?3 AND user_id = ?4` にした場合です（練習用のコピーで書き換え、確かめたら元に戻します）。例4-2のテストを、そのまま動かします。

```text
@@OUTGREP:u04_err_lost:^(?!   Compiling)@@
```

- **何が起きたか:** タブ B の保存も `303` で、DB は `牛乳と卵を買う`・`version=3` になりました。タブ A が保存した `牛乳を2本買う` は、**誰にも知らされずに消えました**。タブ A の人は「保存しました」を見たので、自分の内容が残っていると思っています。
- **気づきにくい理由:** エラーは1つも出ません。`version = version + 1` は残っているので、`version` も進み続けます。`params!` は5つの値を渡したままですが、使わない `?5` があっても、libsql はエラーにしませんでした（UNIT 03 のエラー1と同じです）。
- **学べること:** 楽観的ロックは、保存の SQL の `WHERE` に**版の条件があるときだけ**働きます。「同じノートを2つの画面で順に保存する」テストを残しておくと、この条件を消したときに気づけます。

#### エラー2: フォームから hidden の version を外した（400）

`note_form` から、`if let Some(version) = form.version { … }` の3行を消した場合です。例4-1のテストを動かします（フォームから読み取れた値だけを送ります）。

```text
@@OUTGREP:u04_err_hidden:^(?!   Compiling)@@
```

- **何が起きたか:** 2のフォームの値が `version=None` になり、送った POST は **`400 Bad Request`**「送信された内容を読み取れませんでした」でした。DB は変わっていません。
- **理由:** `EditForm` の `version: i64` は、必ず要る項目です。フォームに `version` が無いので、Topcoat の `Form` が、送られてきた値を `EditForm` に変えられませんでした（UNIT 02 のエラー2と同じ形です）。
- **もし `Option<i64>` にしていたら:** 400 にはならず、`version` の無い保存を、そのまま受け付ける作りになりがちです。そうすると、版を確かめずに上書きする道ができてしまいます。RustNote は、`version` を必ず要る項目にしているので、hidden を消したら、まず保存が止まって気づけます。

#### エラー3: `Option<i64>` の場所に `i64` を入れた（E0308）

409 のときに、フォームの version を最新にする行を、`form.version = latest.version;` と書いた場合です。

```text
@@OUTHEAD:u04_err_option:15@@
```

- **何が起きたか:** 「型が合いません。`Option<i64>` を期待しましたが、`i64` でした」です。`NoteFormState` の `version` は `Option<i64>` です。作成のフォームには `version` が無い（`None`）ので、`Option` にしています。
- **直し方:** `help:` のとおり `Some(latest.version)` と包みます。

### RustNote ではここで使う

- 編集は `src/routes/notes/id/edit.rs` の `edit_page`（GET）と `update`（POST）です。ファイルの先頭のコメントに、成功（303）・入力の問題（400）・version が古い（409）・他人や無い（404）の表があります。
- `update` は、入力の検査より**先に** `find_own_note` を呼んで、他人のノートを 404 にします（問3）。
- 保存の SQL は `src/repository/notes.rs` の `update` です。`version` を使った楽観的ロックの SQL そのものは、第7巻 UNIT 05 でも確かめています。
- `version` の列は、`migrations/0002_notes.sql` で `version INTEGER NOT NULL DEFAULT 1` として作っています。

### 練習問題

**問1:** 保存したあと、ブラウザの「戻る」で、保存の前の編集のフォームに戻り、別の内容でもう一度保存すると、どうなるでしょうか。

**問2:** タブ A は本文だけを `低脂肪を2本` に直して保存し、タブ B はタイトルだけを `牛乳を買う（急ぎ）` に直して保存します。タブ B は 409 になりますが、お知らせを読んだうえで、そのままもう一度保存します。DB の本文は、どうなるでしょうか。

**問3:** bob が、alice のノートの編集（`POST /notes/1/edit`）に、空のタイトルを送ると、400 と 404 のどちらになるでしょうか。

### 解答と解説

```rust
@@FILE:rustnote/tests/u04_q.rs@@
```

```powershell
cargo test --test u04_q -- --nocapture --test-threads=1
```

```text
@@OUTGREP:u04_q:^問@@
```

（問題の答えの行だけを抜き出しています）

**問1の解答:**

- 1回目は `303`、2回目は **`409 Conflict`** でした。「戻る」で開いたフォームの hidden は、保存の前の `version=1` のままだからです。DB は1回目の `牛乳を2本買う`・`version=2` のままです。
- 別のタブで直したときだけでなく、**同じタブの古い画面**から保存したときにも、楽観的ロックが守ってくれます。

**問2の解答:**

- DB の本文は `1本` に戻り、タブ A が保存した `低脂肪を2本` は消えました。タイトルは `牛乳を買う（急ぎ）`、`version=3` です。
- 理由は、409 のお知らせにあります。お知らせは「保存されている最新の**タイトル**は『牛乳を買う』です」と、タイトルだけを見せます。タブ A は本文だけを直したので、タイトルは元のままです。タブ B の人には、何が変わったのかが分かりません。出し直したフォームの本文は、タブ B が送った `1本` のままなので、そのまま保存すると、A の本文を上書きします。
- 409 で止めること自体は、うまく働いています。足りないのは、**何が変わったかを見せる**ことです。最新の本文もお知らせに出す、変わった項目に印を付ける、などの直し方があります。これは、RustNote に残る改善点です（RustNote のコードは変えていません）。

**問3の解答:**

- bob の空のタイトルは **`404 Not Found`** でした。正しいタイトル（`卵を買う`）でも 404、alice が無いノート（`/notes/999`）に空のタイトルを送っても 404 です。alice が自分のノートに空のタイトルを送ったときだけ、`400 Bad Request` になります。
- `update` が、入力の検査より先に `find_own_note` を呼んでいるからです。見せないノートの編集の画面は、どんな入力のときにも出しません。「自分のノートでなければ、何をしても 404」と、決まりが1つで済みます。

### まとめ

- 編集のフォームは、今の値と hidden の `version` を出し、保存の SQL は `version` が合ったときだけ1行を更新して、`version` を 1 進める。
- 古い `version` の保存は 0 行の更新になり、`VersionMismatch` → **409** で、入力を残し、最新の `version` とタイトルを見せて出し直す。もう一度保存すると、知ったうえでの上書きになる。`AND version = ?5` を外すと、エラーなく先の保存が消える（失われた更新）。
- `version` は必ず要る項目（`i64`）にして、hidden を忘れたら 400 で止まるようにする。409 のお知らせがタイトルだけなので、本文だけの変更に気づけないのは、RustNote に残る改善点。
