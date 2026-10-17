## UNIT 08: DTO・Serde・境界の変換

### この単元のゴール

- Serde（サーデ）の `#[derive(Serialize, Deserialize)]` で、構造体と JSON を行き来できる
- 外から来るデータを受け取る**入力 DTO**、検証済みの**ドメイン型**、外へ返す**出力 DTO** を分け、`TryFrom` / `From` で変換できる
- `#[serde(default)]`・`deny_unknown_fields`・`rename_all`・`try_from` などの属性を使い分けられる
- serde_json のエラーメッセージ（`missing field`・`invalid type`・`unknown field` など）を読んで、原因を特定できる

### たとえ話でつかむ

空港の入国審査を思い浮かべてください。

外国から来た人は、それぞれの国の書式の書類（パスポート・申告書）を持っています。審査官は、それを**そのままの形で受け取り**（ここではまだ信用しない）、内容を確かめてから、国内で通用する形（入国のスタンプ・在留カード）に**書き換えます**。国内では、その在留カードだけを見れば済みます。

逆に、出国するときに外へ渡す書類には、**相手に見せてよい情報だけ**を書きます。国内の内部の管理番号や、審査官のメモまで渡したりはしません。

- 入国時に受け取る外国の書類 ＝ **入力 DTO**（JSON やフォームを、文字列のまま受け取る型）
- 審査と書き換え ＝ **境界での変換**（`TryFrom` で検証して、ドメイン型にする）
- 国内で通用するカード ＝ **ドメイン型**（UNIT 01 の `NoteTitle` などを持つ、検証済みの型）
- 出国時に渡す書類 ＝ **出力 DTO**（見せてよい項目だけを持つ型）

> **用語:** DTO（ディー・ティー・オー：Data Transfer Object。外とのデータのやり取りのためだけに使う、入れ物の型）

<!-- -->

> **用語:** シリアライズ（serialize：プログラムの中の値を、JSON のような文字の並びに変換すること）／デシリアライズ（deserialize：その逆。文字の並びから値を作ること）

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 210" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-16" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="30" width="240" height="120" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="140" y="56" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">Rust の値（構造体）</text>
<text x="140" y="84" font-size="12" fill="#0f172a" text-anchor="middle">NoteJson {</text>
<text x="140" y="104" font-size="12" fill="#0f172a" text-anchor="middle">title: "牛乳を買う",</text>
<text x="140" y="124" font-size="12" fill="#0f172a" text-anchor="middle">done: false }</text>
<rect x="460" y="30" width="240" height="120" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="580" y="56" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">JSON の文字列</text>
<text x="580" y="94" font-size="12" fill="#0f172a" text-anchor="middle">{"title":"牛乳を買う",</text>
<text x="580" y="114" font-size="12" fill="#0f172a" text-anchor="middle">"done":false}</text>
<line x1="264" y1="66" x2="456" y2="66" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-16)"/>
<text x="360" y="58" font-size="12" fill="#475569" text-anchor="middle">Serialize（to_string）</text>
<line x1="456" y1="118" x2="264" y2="118" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-16)"/>
<text x="360" y="140" font-size="12" fill="#475569" text-anchor="middle">Deserialize（from_str）</text>
<text x="360" y="186" font-size="12" fill="#475569" text-anchor="middle">#[derive] で、フィールドの名前と型から変換のコードが自動で作られる</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-16: Serialize と Deserialize の向き</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-17" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="10" width="190" height="240" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<rect x="210" y="10" width="250" height="240" rx="10" fill="#fff7ed" stroke="#fdba74"/>
<rect x="470" y="10" width="240" height="240" rx="10" fill="#ecfdf5" stroke="#6ee7b7"/>
<text x="105" y="34" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">外の世界</text>
<text x="335" y="34" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">境界（DTO）</text>
<text x="590" y="34" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">アプリの中（ドメイン）</text>
<rect x="22" y="56" width="166" height="56" rx="8" fill="#ffffff" stroke="#64748b"/>
<text x="105" y="80" font-size="12" fill="#0f172a" text-anchor="middle">JSON・フォーム</text>
<text x="105" y="100" font-size="12" fill="#475569" text-anchor="middle">{"title":"  牛乳 "}</text>
<rect x="225" y="56" width="220" height="56" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="335" y="80" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">CreateNoteInput</text>
<text x="335" y="100" font-size="12" fill="#c2410c" text-anchor="middle">title: String（未検証）</text>
<rect x="485" y="56" width="210" height="56" rx="8" fill="#ffffff" stroke="#10b981"/>
<text x="590" y="80" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">NewNote</text>
<text x="590" y="100" font-size="12" fill="#047857" text-anchor="middle">title: NoteTitle（検証済み）</text>
<line x1="188" y1="84" x2="221" y2="84" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-17)"/>
<line x1="445" y1="84" x2="481" y2="84" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-17)"/>
<text x="463" y="130" font-size="12" fill="#475569" text-anchor="middle">↑ TryFrom（検証）</text>
<rect x="485" y="160" width="210" height="70" rx="8" fill="#ffffff" stroke="#10b981"/>
<text x="590" y="184" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">Note（保存済み）</text>
<text x="590" y="204" font-size="12" fill="#047857" text-anchor="middle">id, owner_id, title, body,</text>
<text x="590" y="222" font-size="12" fill="#047857" text-anchor="middle">done, version</text>
<rect x="225" y="166" width="220" height="56" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="335" y="190" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">NoteView（出力 DTO）</text>
<text x="335" y="210" font-size="12" fill="#c2410c" text-anchor="middle">id, title, body, isDone</text>
<rect x="22" y="166" width="166" height="56" rx="8" fill="#ffffff" stroke="#64748b"/>
<text x="105" y="190" font-size="12" fill="#0f172a" text-anchor="middle">JSON・HTML</text>
<text x="105" y="210" font-size="12" fill="#475569" text-anchor="middle">{"id":1,…}</text>
<line x1="485" y1="194" x2="449" y2="194" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-17)"/>
<line x1="225" y1="194" x2="192" y2="194" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-17)"/>
<text x="463" y="152" font-size="12" fill="#475569" text-anchor="middle">↓ From（選んで写す）</text>
<text x="360" y="282" font-size="12" fill="#475569" text-anchor="middle">owner_id・version・password_hash などは、出力 DTO に入れない限り外へ出ない</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-17: DTO とドメイン型の境界。入るときに検証し、出るときに選ぶ</div>
</div>

### まず動かす最小コード

serde と serde_json は「はじめに」で `cargo add` 済みです（serde 1.0.229 は `derive` 機能つき、serde_json は 1.0.154）。

#### 例8-1: 構造体と JSON を行き来する

> **▼ このコードがやること（先に日本語で）:** ノートを表す構造体を JSON の文字列に変換して、1行の形と整形した形で表示します。次に、JSON の文字列から構造体を作り、中のタイトルを取り出します。

<!--CODE:design_lab/src/bin/u08_serde.rs-->

```powershell
cargo run --bin u08_serde
```

**▼ 実行結果**

<!--OUT:u08_serde-->

#### ▼ コードを1つずつ分解して解説

##### 解説1: `#[derive(Serialize, Deserialize)]`

```rust
#[derive(Debug, Serialize, Deserialize)]
struct NoteJson {
    title: String,
    body: String,
    done: bool,
}
```

- `Serialize` を derive すると「この構造体を JSON などに変換できる」、`Deserialize` を derive すると「JSON などからこの構造体を作れる」ようになります（図4-16）。
- フィールドの名前（`title`）がそのまま JSON のキーになり、型（`String`・`bool`）に合わせて値が変換されます。
- serde 自体は「どう変換するか」の仕組みだけを持ち、**JSON という形式を扱うのは serde_json** です。同じ derive のまま、TOML（`toml` クレート）など別の形式にも変換できるのが serde の強みです。

##### 解説2: `to_string` / `to_string_pretty` / `from_str`

```rust
let text = serde_json::to_string(&note).unwrap();
```

```rust
let parsed: NoteJson = serde_json::from_str(input).unwrap();
```

- `serde_json::to_string(&値)` は1行の JSON を、`to_string_pretty` は人が読みやすいように改行と字下げをした JSON を返します。
- `serde_json::from_str(文字列)` は JSON を読んで値を作ります。どの型として読むのかを、`let parsed: NoteJson` のように変数の型で教えます（UNIT 03 の型推論。`from_str::<NoteJson>(...)` と書いても同じです）。
- どちらも失敗しうるので `Result` を返します。この例では正しいと分かっている値なので `unwrap()` しましたが、**外から来た JSON を読むときは、必ず失敗を扱います**（次の例）。

##### 解説3: `r#"..."#` は「中の `"` をそのまま書ける文字列」

```rust
let input = r#"{ "title": "本を返す", "body": "図書館へ", "done": true }"#;
```

- JSON には `"` がたくさん出てくるので、ふつうの文字列だと `\"` と書く必要があり読みにくくなります。`r#"` と `"#` で囲む**生文字列**（raw string）なら、中の `"` や `\` をそのまま書けます。

### もう一歩: 実務に近い形

#### 例8-2: 入力 DTO → ドメイン型 → 出力 DTO

> **▼ このコードがやること（先に日本語で）:** 「ノートを作る」API の入口を想定します。送られてきた JSON を、まず文字列のままの入力用の型で受け取り、次にタイトルを検査して検証済みの型に変換し、保存したことにして、最後に外に見せてよい項目だけを JSON にして返します。正しい入力・タイトルが空・項目が足りない・余計な項目がある・型が違う、の6通りを試します。

<!--CODE:design_lab/src/bin/u08_dto.rs-->

```powershell
cargo run --bin u08_dto
```

**▼ 実行結果**

<!--OUT:u08_dto-->

##### 解説4: 入力 DTO は「文字列のまま」受け取る

```rust
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateNoteInput {
    title: String,
    // 送られてこなければ空文字にする
    #[serde(default)]
    body: String,
}
```

- 入力 DTO の `title` は、`NoteTitle` ではなく**ただの `String`** です。ここでは「JSON として形が正しいか」だけを確かめ、中身の検査は次の段階で行います。こう分けると、「形式の誤り（400 形式エラー）」と「内容の誤り（400 入力エラー）」を区別でき、入力欄の横に出すメッセージも作りやすくなります。
- `#[serde(default)]` は「この項目が送られてこなかったら、型の既定値（`String` なら空文字）を使う」という意味です。実行結果の2つ目では、`body` がなくても成功しています。
- `#[serde(deny_unknown_fields)]` は「知らない項目が送られてきたらエラーにする」という意味です。5つ目の入力 `{"title": "牛乳", "owner_id": 1}` は、`unknown field owner_id` で断られました。

> **つまずきポイント:** `deny_unknown_fields` を付けないと、serde は知らない項目を**黙って無視**します。無視されるだけなら安全そうですが、「`owner_id` を送れば他人のノートとして作れるのでは？」と試す攻撃者に「受け付けたように見える」返事をしてしまいます。持ち主は**ログイン中のセッションから決める**もので、入力から受け取ってはいけません（第8巻）。入力 DTO に `owner_id` を書かないこと、知らない項目を断ることの両方で守ります。

##### 解説5: `TryFrom` で境界の変換を書く

```rust
impl TryFrom<CreateNoteInput> for NewNote {
    type Error = TitleError;

    fn try_from(input: CreateNoteInput) -> Result<Self, Self::Error> {
        let title = NoteTitle::parse(&input.title)?;
        Ok(NewNote {
            title,
            body: input.body.trim().to_string(),
        })
    }
}
```

- `TryFrom` は「失敗するかもしれない変換」を表す標準の trait です。実装すると `NewNote::try_from(input)` と書けます（`input.try_into()` とも書けます）。
- 変換の中で `NoteTitle::parse` を呼ぶので、`NewNote` ができた時点でタイトルは必ず検証済みです。UNIT 01 の「門番」が、ここで外の世界との境界に立っているわけです。
- 失敗の型（`type Error`）は UNIT 05 の `TitleError` です。実際の RustNote では、ここから `AppError` の 400 に変換されます。

##### 解説6: 出力 DTO は「見せてよいものだけ」

```rust
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NoteView {
    id: i64,
    title: String,
    body: String,
    is_done: bool,
}
```

- 保存済みの `Note` には `owner_id` や `version` もありますが、`NoteView` には入れていません。**`Note` に `Serialize` を付けて、そのまま外に出さない**のがポイントです。後で `Note` に内部用の項目を足しても、うっかり外に漏れることがありません。
- `#[serde(rename_all = "camelCase")]` は、Rust の `is_done`（スネークケース）を JSON では `isDone`（キャメルケース）にする指定です。JavaScript の世界ではキャメルケースが一般的なので、相手に合わせられます。1つの項目だけ名前を変えたいときは `#[serde(rename = "...")]` を使います。
- `From<&Note> for NoteView` で、「`Note` から見せる項目を選んで写す」変換を1か所にまとめています。

##### 解説7: エラーメッセージの「line 1 column 30」

- serde_json のエラーには、問題が見つかった**行と列**が付きます。列はバイト単位で数えるので、日本語（1文字3バイト）を含むと、見た目の文字数より大きな数になります。
- このメッセージは開発者向けです。本物のアプリでは、このまま画面に出さず、「入力の形式が正しくありません」のような文章に言い換えて返し、詳細はログに書きます（UNIT 05）。

### よくあるエラーと読み方

#### エラー1: serde_json の読み込み失敗（実行時のエラー）

serde_json は、JSON の誤りを実行時に `Err` で返します。よくある誤りを並べて、メッセージを確かめてみましょう。

<!--CODE:design_lab/src/bin/u08_errors.rs-->

<!--OUT:u08_errors-->

| メッセージ | 意味 | よくある原因 |
|---|---|---|
| `invalid type: string "yes", expected a boolean` | 真偽値が来るべき所に文字列が来た | フォームの値を `"true"` のように文字列で送った |
| `trailing comma` | 最後の項目の後ろに余計な `,` | 手書きの JSON の書き間違い |
| `EOF while parsing a value` / `an object` | 途中で文字列が終わった（EOF：End Of File） | 送信の途中で切れた・空の本文 |
| `missing field title` | 必須の項目がない | 項目名のつづり違い（`titel` など） |
| `unknown field owner_id, expected title or body` | 知らない項目がある（`deny_unknown_fields` のとき） | 余計な項目・つづり違い |

- 5つ目の `["牛乳", false]` は、エラーにならずに読めてしまいました。serde は、構造体を「項目を順番に並べた配列」の形でも読めるようになっているためです。外から来るデータは、思ったより多くの形を受け付けることがあるので、**読んだ後の検証（境界での変換）**が大切です。
- 最後の空文字列は、`column 0` の `EOF while parsing a value` です。リクエストの本文が空だったときに出会います。

#### エラー2: E0277 フィールドの型が Serialize を実装していない

<!--CODE:errs/u08_err_serialize.rs-->

<!--OUT:u08_err_serialize-->

- **何が起きたか:** `NoteView` に `#[derive(Serialize)]` を付けましたが、フィールドの `NoteTitle` が `Serialize` を実装していません。構造体を JSON にするには、**すべてのフィールドが** JSON にできる必要があります。
- エラーは長いですが、大事なのは最初の2行と `help: the trait Serialize is not implemented for NoteTitle`、そして `note: for local types consider adding #[derive(serde::Serialize)] to your NoteTitle type`（自分の型なら derive を足しては？）です。`the following other types implement trait Serialize` 以下の一覧や、serde_core の中のファイルの場所は、読み飛ばしてかまいません。
- **直し方:** ①`NoteTitle` に `#[derive(Serialize)]` を付ける（中の `String` がそのまま出力されます）、または ②出力 DTO のフィールドを `title: String` にして、変換のときに `note.title.as_str().to_string()` で写す（例8-2 の方法）。ドメイン型を外の形式から切り離しておきたいなら ② が素直です。

### RustNote ではここで使う

- RustNote の画面はフォーム（HTML の `<form>`）でデータを送ります。フォームの値の受け取り方は第6巻で学びますが、**受け取った後**は、この単元と同じく「文字列のまま受け取る入力 DTO → `TryFrom` で検証 → ドメイン型」の順に変換します。
- 登録フォームの入力 DTO はメールアドレスとパスワードを `String` で受け取り、`Email::parse` で検証します。パスワードはそのまま保存せず、Argon2id でハッシュにします（第8巻）。
- `users` テーブルの行を表す型には `password_hash` があるので、**`Serialize` を付けず**、画面や API に出す型（出力 DTO）を別に作ります（練習問題1）。
- ノートの作成・編集では、持ち主（`user_id`）は入力からではなく、ログイン中のセッションから決めます。

### 練習問題

**問題1:** DB の行に近い `User { id: i64, email: String, password_hash: String }` と、外に返してよい項目だけを持つ出力 DTO `UserView` を作ってください。`From<&User> for UserView` を実装し、`UserView` を JSON にして、`password_hash` が含まれないことを確かめてください（`User` には `Serialize` を付けないこと）。

**問題2:** 例8-2 では「入力 DTO を受け取ってから `TryFrom` で検証」しました。別の方法として、serde の `#[serde(try_from = "String")]` を `NoteTitle` に付けると、**JSON を読むのと同時に**検証できます。`TryFrom<String> for NoteTitle` を実装し、`{"title": " 牛乳 "}` と `{"title": "  "}` を読んでみてください。

**問題3:** ある人が「型を増やすのは面倒なので、DB の行・入力・出力のすべてに、`Serialize` と `Deserialize` を付けた1つの `Note` 構造体（`id`・`owner_id`・`title`・`body`・`done`・`version` を持つ）を使い回そう」と提案しています。どんな問題が起きうるか、入力と出力の両方の向きについて答えてください。

### 解答と解説

**解答1:**

<!--CODE:design_lab/src/bin/u08_ex1.rs-->

**▼ 実行結果**

<!--OUT:u08_ex1-->

- `User` に `Serialize` が付いていないので、`serde_json::to_string(&user)` と書くと**コンパイルエラーになります**。「パスワードのハッシュを JSON に出してしまう」事故を、型の段階で防げています。`#[serde(skip_serializing)]` で1つの項目だけ出さない方法もありますが、付け忘れたら漏れてしまうので、型を分けるほうが安全です。
- 例の `"$argon2id$v=19$..."` は、ハッシュの文字列の形を示すための仮の値です。

**解答2:**

<!--CODE:design_lab/src/bin/u08_ex2.rs-->

**▼ 実行結果**

<!--OUT:u08_ex2-->

- `#[serde(try_from = "String")]` は、「いったん `String` として読み、`NoteTitle::try_from` を通してから入れる」という指定です。検証に失敗すると、その文章がそのまま serde_json のエラーになります（`タイトルを入力してください at line 1 column 15`）。
- 便利ですが、「形式の誤り」と「内容の誤り」が同じ種類のエラーになり、どの項目が悪かったのかを入力欄ごとに出し分けにくくなります。フォームの検証では例8-2 の書き方、設定ファイルのように「間違っていたら止まればよい」読み込みでは `try_from` の書き方、と使い分けるとよいでしょう。

**解答3:**

- **入力の向き:** `owner_id` や `version`、`id` まで外から受け取れてしまいます。悪意のある利用者が `owner_id` を書き換えて送れば、他人のノートとして作ったり、他人のノートを上書きしたりできるかもしれません（一括代入の脆弱性。mass assignment）。また、タイトルが `String` のままなので、検証を忘れた経路から不正な値が入ります。
- **出力の向き:** 内部用の `owner_id` や `version` が外に出ます。後で内部用の項目（たとえば管理用のメモ）を `Note` に足すと、それも自動で外に出てしまいます。
- 型を分けるのは手間に見えますが、「何を受け取り、何を見せるか」が型に書かれるので、レビューでも一目で確かめられます。

### まとめ

- `#[derive(Serialize, Deserialize)]` と serde_json の `to_string` / `from_str` で、構造体と JSON を行き来できます。外から来た JSON は必ず `Result` で失敗を扱います。
- 入力 DTO（文字列のまま）→ `TryFrom` で検証 → ドメイン型 → `From` で選んで写す → 出力 DTO、の順に変換し、境界で検証と選別を行います。
- `deny_unknown_fields` で余計な項目を断り、パスワードのハッシュのような秘密は、出力 DTO に入れないことで外に出しません。
