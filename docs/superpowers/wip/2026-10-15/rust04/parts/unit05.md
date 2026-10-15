## UNIT 05: エラー型・thiserror・文脈（anyhow との使い分け）

### この単元のゴール

- エラーを「文字列」ではなく **enum で分類**し、`Display` と `std::error::Error` を実装できる
- `thiserror` 2 系で同じことを短く書き、`#[from]` と `?` で層をまたいだ変換ができる
- アプリ全体のエラー `AppError` を作り、HTTP の 400 / 404 / 409 / 500 に対応づけられる。500 のときは中身を利用者に見せない
- `anyhow` で「文脈（何をしようとして失敗したか）」を積み重ね、thiserror との使い分けを説明できる

### たとえ話でつかむ

郵便物が届けられなかったとき、郵便局は封筒に「**あて所に尋ねあたりません**」「**料金不足**」「**受取拒否**」のような理由のハンコを押して返します。理由が分類されているから、送った人は「住所を直す」「切手を足す」と、**理由ごとに次の行動を決められます**。もし全部「届きませんでした」とだけ書いてあったら、どうしようもありません。

エラーの enum は、この「返送理由のハンコ」です。そして、窓口の係員は、局の中の細かい事情（「仕分け機が故障した」）をお客さんにそのまま言わず、「申し訳ありません、時間をおいてお越しください」と**言い換えます**。一方、局内の報告書には故障の詳細を書きます。

- 中の層（ドメイン・DB）: 細かい理由を正確に分類する（`TitleError`、`RepoError`）
- 真ん中の層（アプリ）: 利用者への対応の種類で分類し直す（`AppError`）
- 外の層（HTTP）: ステータスコードと優しい文章に言い換える。詳細はログへ

> **用語:** ステータスコード（HTTP の返事に付ける3桁の番号。400 = 送られた内容が正しくない、404 = 見つからない、409 = 状態が食い違って処理できない、500 = サーバーの中の故障。第6巻で詳しく扱う）

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-10" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="110" y="24" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">ドメイン・保存先の層</text>
<text x="360" y="24" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">アプリの層</text>
<text x="610" y="24" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">HTTP の層</text>
<rect x="20" y="40" width="180" height="40" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="110" y="65" font-size="13" fill="#047857" text-anchor="middle">TitleError</text>
<rect x="20" y="95" width="180" height="40" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="110" y="120" font-size="13" fill="#047857" text-anchor="middle">EmailError</text>
<rect x="20" y="152" width="180" height="88" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="110" y="174" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">RepoError</text>
<text x="110" y="194" font-size="12" fill="#475569" text-anchor="middle">UniqueViolation</text>
<text x="110" y="212" font-size="12" fill="#475569" text-anchor="middle">VersionMismatch</text>
<text x="110" y="230" font-size="12" fill="#475569" text-anchor="middle">Connection</text>
<rect x="280" y="40" width="160" height="200" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="60" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">AppError</text>
<text x="360" y="90" font-size="12" fill="#1e40af" text-anchor="middle">InvalidTitle / Email</text>
<text x="360" y="130" font-size="12" fill="#1e40af" text-anchor="middle">NotFound</text>
<text x="360" y="170" font-size="12" fill="#1e40af" text-anchor="middle">EmailTaken / Conflict</text>
<text x="360" y="210" font-size="12" fill="#1e40af" text-anchor="middle">Internal</text>
<line x1="200" y1="60" x2="276" y2="83" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-10)"/>
<line x1="200" y1="115" x2="276" y2="90" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-10)"/>
<line x1="200" y1="190" x2="276" y2="168" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-10)"/>
<line x1="200" y1="225" x2="276" y2="208" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-10)"/>
<text x="238" y="146" font-size="12" fill="#475569" text-anchor="middle">? で</text>
<text x="238" y="162" font-size="12" fill="#475569" text-anchor="middle">変換</text>
<rect x="520" y="70" width="180" height="30" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="610" y="90" font-size="12" fill="#c2410c" text-anchor="middle">400 Bad Request</text>
<rect x="520" y="110" width="180" height="30" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="610" y="130" font-size="12" fill="#c2410c" text-anchor="middle">404 Not Found</text>
<rect x="520" y="150" width="180" height="30" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="610" y="170" font-size="12" fill="#c2410c" text-anchor="middle">409 Conflict</text>
<rect x="520" y="190" width="180" height="30" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="610" y="210" font-size="12" fill="#c2410c" text-anchor="middle">500 Internal Server Error</text>
<line x1="440" y1="85" x2="516" y2="85" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-10)"/>
<line x1="440" y1="125" x2="516" y2="125" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-10)"/>
<line x1="440" y1="165" x2="516" y2="165" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-10)"/>
<line x1="440" y1="205" x2="516" y2="205" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-10)"/>
<text x="478" y="58" font-size="12" fill="#475569" text-anchor="middle">status_code()</text>
<text x="360" y="272" font-size="12" fill="#475569" text-anchor="middle">画面には言い換えた文章（500 は詳細を隠す）、ログには source() で原因の連鎖を書く</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-10: エラーの層。中の層の細かい失敗を、アプリの分類を経て HTTP の返事に変える</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-11" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="10" width="340" height="230" rx="10" fill="#eff6ff" stroke="#3b82f6"/>
<text x="180" y="36" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">thiserror: 種類を見分けたい</text>
<rect x="40" y="52" width="280" height="96" rx="8" fill="#ffffff" stroke="#3b82f6"/>
<text x="180" y="74" font-size="12" fill="#0f172a" text-anchor="middle">enum AppError {</text>
<text x="180" y="94" font-size="12" fill="#0f172a" text-anchor="middle">NotFound, Conflict, Internal(..)</text>
<text x="180" y="114" font-size="12" fill="#0f172a" text-anchor="middle">}</text>
<text x="180" y="136" font-size="12" fill="#1e40af" text-anchor="middle">呼ぶ側が match で対応を変える</text>
<text x="180" y="180" font-size="12" fill="#1e40af" text-anchor="middle">向く場所: ドメイン型・ライブラリ・</text>
<text x="180" y="200" font-size="12" fill="#1e40af" text-anchor="middle">HTTP に変換する AppError</text>
<rect x="370" y="10" width="340" height="230" rx="10" fill="#fff7ed" stroke="#f97316"/>
<text x="540" y="36" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">anyhow: 報告できればよい</text>
<rect x="395" y="52" width="290" height="32" rx="6" fill="#ffffff" stroke="#f97316"/>
<text x="540" y="73" font-size="12" fill="#0f172a" text-anchor="middle">シードの投入に失敗しました</text>
<rect x="395" y="96" width="290" height="32" rx="6" fill="#ffffff" stroke="#f97316"/>
<text x="540" y="117" font-size="12" fill="#0f172a" text-anchor="middle">シードファイル … を読めませんでした</text>
<rect x="395" y="140" width="290" height="32" rx="6" fill="#ffffff" stroke="#f97316"/>
<text x="540" y="161" font-size="12" fill="#0f172a" text-anchor="middle">ファイルが見つかりません (os error 2)</text>
<line x1="540" y1="140" x2="540" y2="132" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-11)"/>
<line x1="540" y1="96" x2="540" y2="88" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-11)"/>
<text x="540" y="198" font-size="12" fill="#c2410c" text-anchor="middle">下から上へ「文脈」を積んで、まとめて表示</text>
<text x="540" y="220" font-size="12" fill="#c2410c" text-anchor="middle">向く場所: main・スクリプト・起動処理</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-11: thiserror は「分類するため」、anyhow は「何が起きたかを人に伝えるため」</div>
</div>

### まず動かす最小コード

まずはライブラリを使わずに、エラー型を手で書いてみます。thiserror が裏で何をしてくれるのかが分かるようになります。

#### 例5-1: TitleError に Display と Error を実装する

> **▼ このコードがやること（先に日本語で）:** タイトルの検査で起きる失敗を2種類に分けた型を作り、①利用者に見せる文章と、②開発者が調べるための表示の2通りで出力します。最後に、標準の「どんなエラーでも入る箱」にも入れられることを確かめます。

<!--CODE:design_lab/src/bin/u05_manual.rs-->

```powershell
cargo run --bin u05_manual
```

**▼ 実行結果**

<!--OUT:u05_manual-->

#### ▼ コードを1つずつ分解して解説

##### 解説1: Display は「人に見せる文章」、Debug は「開発者向け」

```rust
impl fmt::Display for TitleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TitleError::Empty => write!(f, "タイトルを入力してください"),
```

- `Display` を実装すると、`{}` で表示したときの文章を決められます。エラーの `Display` は「何が悪かったか」を1文で、文末に句点を付けずに書くのが Rust の慣習です（複数のエラーをつないで表示するときに読みやすいため）。
- `#[derive(Debug)]` で作られる `{:?}` の表示（`TooLong { len: 81, max: 80 }`）は、**開発者が中身を正確に知るため**のものです。利用者の画面には出しません。

##### 解説2: `impl Error for TitleError {}`

```rust
impl Error for TitleError {}
```

- `std::error::Error` は「これはエラーを表す型です」という標準の trait です。`Debug` と `Display` を実装していることが条件で、メソッドはすべて既定の実装があるので、中身は空で構いません。
- 実装しておくと、`Box<dyn Error>`（どんなエラーでも入る箱。UNIT 04 の trait オブジェクト）に入れたり、後で見る `source()` で「原因のエラー」をたどったりできるようになります。

##### 解説3: 手で書くのは少し面倒

- エラーの種類が増えるたびに、`Display` の `match` にも1行ずつ足す必要があります。種類と文章が離れた場所に書かれるので、読みにくくもなります。
- これを `#[derive]` のように自動化してくれるのが **thiserror** です。

### もう一歩: 実務に近い形

#### 例5-2: thiserror で AppError を作り、HTTP の返事に変換する

> **▼ このコードがやること（先に日本語で）:** 入力の検査の失敗（タイトル・メール）、保存先の失敗（重複・版の食い違い・接続できない）、アプリとしての分類（見つからない・登録済み・競合・内部エラー）を、3つの層のエラー型に分けて書きます。「ノートの名前を変える」「登録する」という2つの処理でいろいろな失敗を起こし、それぞれが何番のステータスコードと、どんな文章になるかを表示します。

thiserror は「はじめに」で `cargo add thiserror` 済みです（版は 2.0.21）。

<!--CODE:design_lab/src/bin/u05_app_error.rs-->

```powershell
cargo run --bin u05_app_error
```

**▼ 実行結果**

<!--OUT:u05_app_error-->

##### 解説4: `#[derive(Error)]` と `#[error("...")]`

```rust
#[derive(Debug, Error, PartialEq, Eq)]
enum TitleError {
    #[error("タイトルを入力してください")]
    Empty,
    #[error("タイトルは{max}文字以内にしてください（現在{len}文字）")]
    TooLong { len: usize, max: usize },
}
```

- `#[derive(Error)]` が、例5-1 で手書きした `impl Display` と `impl Error` を自動で作ります。
- 各種類の上の `#[error("...")]` が `Display` の文章です。`{max}` `{len}` のように、**フィールドの名前を `{}` に書くと値が埋め込まれます**。タプルの形（`UniqueViolation(String)`）なら `{0}` で0番目のフィールドを埋め込みます。
- 種類と文章が隣り合うので、読みやすく、書き忘れも起きません。

##### 解説5: `#[from]` で `?` が変換してくれる

```rust
#[error(transparent)]
InvalidTitle(#[from] TitleError),
```

```rust
let title = parse_title(input)?;
```

- `#[from]` を付けると、thiserror が `impl From<TitleError> for AppError` を自動で作ります。
- `?` 演算子は、エラーを返すときに**自動で `From` による変換をする**ので、`Result<_, TitleError>` に `?` を付けるだけで `AppError::InvalidTitle(...)` になって返ります。
- `#[error(transparent)]` は「表示も原因も、中のエラーのものをそのまま使う」という意味です。だから 400 の画面には `TitleError` の文章「タイトルを入力してください」がそのまま出ています。

##### 解説6: 変換のルールを自分で書く `impl From`

```rust
impl From<RepoError> for AppError {
    fn from(e: RepoError) -> Self {
        match e {
            RepoError::VersionMismatch => AppError::Conflict,
            other => AppError::Internal(other),
        }
    }
}
```

- 保存先の失敗は、種類によって意味が違います。「version が一致しない」は、ほかの画面で先に更新されたという**利用者に伝えるべき競合（409）**です。それ以外の DB の失敗は、利用者にはどうしようもない**内部エラー（500）**です。
- このように、変換に判断が要るときは `#[from]` ではなく `impl From` を手で書きます。それでも `?` で使える点は同じです（`db?;`）。
- 登録の「一意制約違反」は、`signup` の中で `map_err` を使い、その場で `EmailTaken` に言い換えています。同じ `UniqueViolation` でも、どの列か・どの処理かで意味が変わるので、**意味が分かる場所（ユースケース）で変換する**のです。

> **用語:** 一意制約（UNIQUE：DB で「この列に同じ値を2つ入れてはいけない」という決まり。RustNote の `users.email` に付いている）

##### 解説7: 500 の詳細は画面に出さず、ログに出す

```rust
fn public_message(&self) -> String {
    match self {
        AppError::Internal(_) => "時間をおいてもう一度お試しください".to_string(),
        other => other.to_string(),
    }
}
```

```rust
let mut cause = std::error::Error::source(&e);
while let Some(c) = cause {
    println!("    ログ（原因）: {c}");
    cause = c.source();
}
```

- DB のエラーメッセージには、テーブル名やサーバーの場所など、**攻撃者に知られたくない情報**が含まれることがあります。だから 500 の画面には、決まった文章だけを出します。
- 一方で、開発者は原因を知る必要があります。`#[source]` を付けたフィールドは `source()` で取り出せるので、`while let` で原因の原因…とたどってログに書きます。実行結果では「DB に接続できません ← 接続を拒否されました」と2段の原因が出ています。
- この `respond` 関数は HTTP の層の代わりです。本物の Web アプリでは、`AppError` から HTTP のレスポンスを作る部分（第6巻・第10巻）でこの考え方を使います。

#### 例5-3: anyhow で文脈を積み重ねる

thiserror は「エラーの**種類**を作る」道具でした。一方、プログラムの入口（`main`）や、DB に最初のデータを入れるスクリプトのような場所では、エラーの種類で分岐することはほとんどありません。必要なのは「**何をしようとして、なぜ失敗したか**」を人間に分かるように表示して終わることです。そのための道具が **anyhow** です（版は 1.0.104）。

> **▼ このコードがやること（先に日本語で）:** 1行に1件のタイトルが書かれたテキストファイルを読み、ノートとして投入する（ふりをする）小さなツールです。ファイルが無い・中身が空、のときは、「何をしようとして失敗したか」を重ねた分かりやすいエラーを表示して終了します。

<!--CODE:design_lab/src/bin/u05_anyhow.rs-->

`design_lab` フォルダに `seed.txt`（1行目「牛乳を買う」、2行目は空行、3行目「本を返す」）を作ってから実行します。`--` の後ろは、プログラムに渡す引数です。

```powershell
cargo run --bin u05_anyhow -- seed.txt
```

**▼ 実行結果（成功）**

<!--OUT:u05_anyhow_ok-->

存在しないファイルを指定すると、次のようになります。

```powershell
cargo run --bin u05_anyhow -- missing.txt
```

**▼ 実行結果（失敗）**

<!--OUT:u05_anyhow_missing-->

空白だけのファイル `empty.txt` を指定した場合（`-q` は cargo 自身の表示を省くオプション）:

```powershell
cargo run -q --bin u05_anyhow -- empty.txt
```

<!--OUT:u05_anyhow_empty-->

##### 解説8: `anyhow::Result` と `?`

```rust
use anyhow::{Context, Result, bail};
```

- `anyhow::Result<T>` は `Result<T, anyhow::Error>` の別名です。`anyhow::Error` は、**`std::error::Error` を実装したどんなエラーでも**（`io::Error` でも自作の `TitleError` でも）`?` で受け取れる、何でも入る箱です。
- そのぶん、受け取った側は「どの種類のエラーか」を `match` で見分けにくくなります。だから、**種類で分岐したい場所には向きません**。

##### 解説9: `context` / `with_context` で文脈を足す

```rust
let text = fs::read_to_string(path)
    .with_context(|| format!("シードファイル {path} を読めませんでした"))?;
```

- 元のエラー「指定されたファイルが見つかりません。 (os error 2)」だけでは、**どのファイルを、何のために**読もうとしたのか分かりません。`with_context` で「シードファイル missing.txt を読めませんでした」という文脈を上に重ねます。
- `context("固定の文")` はいつも同じ文章のとき、`with_context(|| format!(...))` は文章を組み立てるときに使います。後者はクロージャなので、**失敗したときだけ** `format!` が実行され、成功したときの無駄がありません。
- `bail!("...")` は「この文章のエラーを作って、すぐ `return Err(...)` する」マクロです。

##### 解説10: `main` が `Result` を返すと何が起きるか

- `fn main() -> Result<()>` にすると、`main` の中で `?` が使えます。`Err` で終わると、Rust は `Error: ` に続けてエラーを表示し、**終了コード 1**（失敗）でプログラムを終えます。最後の行の `error: process didn't exit successfully ... (exit code: 1)` は、それを見た cargo の報告です。
- anyhow のエラーは、`Caused by:` の下に、積んだ文脈を上から順に番号付きで表示します（図4-11 の右側）。「シードの投入に失敗しました ← ファイルを読めませんでした ← ファイルが見つかりません」と、**外側の目的から内側の原因へ**読めます。

#### thiserror と anyhow の使い分け

| 場所 | 使うもの | 理由 |
|---|---|---|
| `NoteTitle::parse` などドメイン型の検査 | thiserror（または手書き） | 呼ぶ側が「空なのか長すぎるのか」で対応を変える |
| 保存先（Repository）の失敗 | thiserror | 「重複」「競合」「接続できない」を見分けて変換する |
| アプリ全体の `AppError` | thiserror | 種類ごとにステータスコードと文章を決める |
| `main`・起動処理・管理用スクリプト | anyhow | 種類で分岐せず、人に分かる文脈を付けて報告できればよい |

覚え方は「**プログラムが読むエラーは thiserror、人が読むエラーは anyhow**」です。両方を1つのプロジェクトで使うのは普通のことです。

### よくあるエラーと読み方

#### エラー1: E0277 `?` がエラーを変換できない

<!--CODE:errs/u05_err_from.rs-->

<!--OUT:u05_err_from-->

- **何が起きたか:** `signup` の戻り値のエラー型は `AppError` なのに、`parse_email(input)?` が返そうとしているのは `EmailError` です。`?` は `From` で変換しようとしますが、`From<EmailError> for AppError` がありません（`the trait From<EmailError> is not implemented for AppError`）。最後の `note` が「`?` は `From` trait で暗黙に変換する」と仕組みまで説明してくれています。
- **直し方:** `AppError` に `#[from] EmailError` を持つ種類を足すか（練習問題2）、その場で `.map_err(|_| AppError::...)?` と変換を書きます。

#### エラー2: 警告 unused `Result` ― 失敗を握りつぶしている

これはコンパイルエラーではなく**警告**（warning）なので、プログラムは動いてしまいます。だからこそ危険です。

<!--CODE:errs/u05_warn_unused.rs-->

<!--OUT:u05_warn_unused-->

- **何が起きたか:** `save_note("")` は `Err` を返しているのに、結果を見ずに捨てたため、画面には「保存しました！」と**嘘の表示**が出ています。`Result` には「使わずに捨てたら警告する」という印（`#[must_use]`）が付いているので、コンパイラが `unused Result that must be used` と教えてくれました。
- **直し方:** `save_note("")?;` で呼び出し元に失敗を返すか、`match` / `if let Err(e) = ...` で処理します。help の `let _ = ...` は「わざと無視する」という意思表示で、**本当に無視してよいときだけ**使います。
- `cargo clippy -- -D warnings` のように実行すると、警告もエラーとして扱われ、ビルドが止まります。チーム開発や CI（第11巻・第12巻）ではこの設定がよく使われます。

### RustNote ではここで使う

RustNote の `AppError` は、この単元の `AppError` とほぼ同じ形で、次のように HTTP の返事に対応づけます。

| AppError の種類 | ステータス | RustNote で起きる場面 |
|---|---|---|
| 入力の検証エラー（タイトル・メールなど） | 400 | 空のタイトルで作成、形式の違うメールで登録 |
| `NotFound` | 404 | 存在しないノート、**他人のノート**（「ある」こと自体を隠すため 403 ではなく 404） |
| `EmailTaken` / `Conflict` | 409 | 登録済みのメール、ほかのタブで先に編集された（`notes.version` が合わない） |
| `Internal` | 500 | DB に接続できない、想定外の失敗。画面には詳細を出さず、ログに原因を書く |

フォームの検証エラーは、実際の画面では 400 を返す代わりに「入力欄の横にメッセージを出して、同じフォームを表示し直す」ことも多いです。その形は第10巻で扱います。`main` での起動時のエラー（設定が読めない・DB に接続できない）には anyhow を使います。

### 練習問題

**問題1:** `AppError` に「ログインしていない」を表す `Unauthorized`（文章「ログインしてください」、ステータス 401）を追加してください。`status_code` の `match` に書き足すまでコンパイルが通らないことも確かめてください。

**問題2:** 「よくあるエラー」のエラー1（E0277）を、thiserror の `#[from]` を使って直してください。`signup("alice")` が失敗したときの `{}` と `{:?}` の表示も確かめてください。

**問題3:** 次のエラーには thiserror と anyhow のどちらが向いているでしょうか。

- (a) `Email::parse` の失敗
- (b) RustNote の `main` で、起動時に DB へ接続できなかった
- (c) ノートの保存で起きた「version の食い違い」

### 解答と解説

**解答1:**

<!--CODE:design_lab/src/bin/u05_ex1.rs-->

**▼ 実行結果**

<!--OUT:u05_ex1-->

- `match` は、enum のすべての種類を扱わないとコンパイルエラーになります（網羅性チェック）。`Unauthorized` を足して `status_code` の `401` の行を書き忘れると、次のように教えてくれます。

<!--OUT:u05_err_e0004-->

- `non-exhaustive patterns`（すべての場合を網羅していない）の `E0004` です。だから、エラーの種類を増やしても**ステータスコードの決め忘れが起きません**。ここで `_ => 500` のような「その他」の腕を書いてしまうと、このチェックが働かなくなるので注意しましょう。

**解答2:**

<!--CODE:design_lab/src/bin/u05_ex2.rs-->

**▼ 実行結果**

<!--OUT:u05_ex2-->

- `{}` では `#[error(transparent)]` により中の `EmailError` の文章が、`{:?}` では `InvalidEmail(EmailError)` という構造がそのまま表示されています。

**解答3:**

- (a) **thiserror**（または手書きの enum）。呼ぶ側が失敗の理由を見て、入力欄にメッセージを出すなど対応を変えるからです。
- (b) **anyhow**。起動時の失敗は種類で分岐せず、「DB に接続できませんでした（URL: …）」のような文脈を付けて表示し、終了すればよいからです。
- (c) **thiserror**。「version の食い違い」は 409 に変換すべき特別な失敗で、プログラムが種類を見分ける必要があるからです。

### まとめ

- エラーは enum で分類し、`Display`（人向けの文章）と `std::error::Error` を実装します。thiserror の `#[derive(Error)]` と `#[error("...")]` で短く書けます。
- `#[from]` や `impl From` があれば、`?` が層をまたいでエラーを変換します。`AppError` でステータスコードを決め、500 の詳細は画面に出さずログへ書きます。
- 種類で分岐したいエラーは thiserror、人に報告できればよいエラーは anyhow（`context` で文脈を積む）を使います。
