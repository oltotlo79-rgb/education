## UNIT 01: 要件・ユーザーストーリー・受入条件

### この単元のゴール

- 「要件」「ユーザーストーリー」「受入条件」の違いを説明し、RustNote の機能を「誰が・何をしたい・なぜ」の形で書ける
- 受入条件を、うまくいく場合だけでなく、未ログイン・空の入力・他人の ID・存在しない ID・壊れた DB の場合まで、**ステータスコードや画面の文で確かめられる形**で書ける
- 「この版では作らないこと」（スコープの外）と、安全さ・量の上限のような機能以外の要件を、あいまいでない言葉で書ける
- 受入条件をテストの名前と1対1で対応させ、`cargo test` で確かめられる。テストが失敗したとき、間違っているのが「コード」なのか「受入条件」なのかを判断できる

### たとえ話でつかむ

家を建てるときのことを考えます。施主（家を頼む人）は、設計士にこう言いました。

「明るいリビングにしてください」

この一言だけで家を建て始めると、引き渡しの日にもめます。施主の思う「明るい」と、大工さんの思う「明るい」は違うからです。そこで設計士は、要望を**確かめられる条件**に書き直します。「リビングの南側に、幅 2m 以上の窓を2つ付ける」。こう書いてあれば、引き渡しの前に、巻き尺で測って「できている」「できていない」を誰でも判定できます。

さらに設計士は、「この家にはプールは付けない」のように、**作らないもの**も書いておきます。途中で「ついでにプールも」と言われたときに、話し合いのきっかけになるからです。「地震に強いこと（耐震等級3）」のように、間取りとは別の性質の約束も書きます。

ソフトウェアでも同じことをします。

| 家づくり | ソフトウェア |
|---|---|
| 施主の要望（「明るいリビング」） | **ユーザーストーリー**（「利用者として、ノートを書きたい」） |
| 設計士が書いた、測れる条件（「幅 2m 以上の窓」） | **受入条件**（「タイトルが空なら 400 を返し、入力した本文は消さない」） |
| 引き渡し前の検査表と、巻き尺での検査 | **テスト**と `cargo test` |
| 「プールは付けない」 | **スコープの外**（この版では作らないこと） |
| 「耐震等級3」 | **非機能要件**（安全さ・速さ・量の上限など、機能以外の約束） |
| 要望・条件・作らないもの、の全部 | **要件** |

> **用語:** 要件（ようけん：作るものが満たすべきことの全部。機能・機能以外の約束・作らないことを含む）

<!-- -->

> **用語:** ユーザーストーリー（user story：利用者の立場から「誰として・何をしたい・なぜ」を1文で書いたもの。例:「利用者として、思いついたことをすぐ書き留めたい。忘れないように」）

<!-- -->

> **用語:** 受入条件（うけいれじょうけん。acceptance criteria：ストーリーが「できた」と言える条件。「前提・操作・結果」の形で、誰が確かめても同じ答えになるように書く）

<!-- -->

> **用語:** 非機能要件（ひきのうようけん：何ができるか以外の約束。安全さ・速さ・量の上限・動いているかを確かめる方法など）

### 図で理解する

要件は、上から順に細かくしていきます。「誰のためのアプリか」を決め、その人がしたいことをストーリーにし、ストーリーごとに受入条件を書き、受入条件をテストにします。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-0101" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="40" width="152" height="56" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="90" y="64" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">ペルソナと目的</text>
<text x="90" y="84" font-size="11" fill="#475569" text-anchor="middle">誰のためのアプリか</text>
<rect x="194" y="40" width="152" height="56" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="270" y="64" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">ユーザーストーリー</text>
<text x="270" y="84" font-size="11" fill="#475569" text-anchor="middle">誰が・何を・なぜ</text>
<rect x="374" y="40" width="152" height="56" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="450" y="64" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">受入条件</text>
<text x="450" y="84" font-size="11" fill="#475569" text-anchor="middle">前提・操作・結果</text>
<rect x="554" y="40" width="152" height="56" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="630" y="64" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">テスト</text>
<text x="630" y="84" font-size="11" fill="#475569" text-anchor="middle">cargo test で確かめる</text>
<line x1="166" y1="68" x2="190" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0101)"/>
<line x1="346" y1="68" x2="370" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0101)"/>
<line x1="526" y1="68" x2="550" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0101)"/>
<text x="90" y="138" font-size="12" fill="#0f172a" text-anchor="middle">メモを残したい学生</text>
<text x="90" y="156" font-size="12" fill="#0f172a" text-anchor="middle">どの PC からでも</text>
<text x="270" y="138" font-size="12" fill="#0f172a" text-anchor="middle">利用者として</text>
<text x="270" y="156" font-size="12" fill="#0f172a" text-anchor="middle">ノートを書きたい</text>
<text x="450" y="138" font-size="12" fill="#0f172a" text-anchor="middle">タイトルが空なら</text>
<text x="450" y="156" font-size="12" fill="#0f172a" text-anchor="middle">400、本文は残す</text>
<text x="630" y="138" font-size="12" fill="#0f172a" text-anchor="middle">empty_title_is_400</text>
<text x="630" y="156" font-size="12" fill="#0f172a" text-anchor="middle">→ ok / FAILED</text>
<text x="14" y="118" font-size="11" fill="#64748b">RustNote の例:</text>
<path d="M630,166 V202 H450 V170" fill="none" stroke="#f97316" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#ar-09-0101)"/>
<text x="540" y="222" font-size="12" fill="#c2410c" text-anchor="middle">失敗したら: 条件とコードの</text>
<text x="540" y="239" font-size="12" fill="#c2410c" text-anchor="middle">どちらが正しいかを決めて直す</text>
<text x="200" y="222" font-size="12" fill="#475569" text-anchor="middle">左ほど大まかで、右ほど細かく、</text>
<text x="200" y="239" font-size="12" fill="#475569" text-anchor="middle">機械で確かめられる形になる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: 要件を細かくしていく流れ。ペルソナからストーリー、受入条件、テストへと進み、テストが失敗したら受入条件かコードのどちらかを直す</div>
</div>

- 左の2つ（ペルソナ・ストーリー）は、人が読んで合意するためのものです。言葉はやわらかくてかまいません。
- 右の2つ（受入条件・テスト）は、**誰が確かめても同じ答えになる**ように書きます。「使いやすい」「ちゃんと動く」のような言葉は使わず、「303 で `/login` へ移る」「400 で『タイトルを入力してください』と出る」のように、目で見える結果で書きます。
- テストが失敗したら、すぐにコードを直すのではなく、まず**受入条件のほうが正しいか**を確かめます。受入条件の書き間違いということもあるからです（「よくあるエラー」のエラー1で体験します）。

1つのストーリーに対して、受入条件は1つでは足りません。うまくいく場合だけを書くと、「空のタイトルを送ったらどうなるか」「他人のノートを開いたらどうなるか」が決まらないまま作ることになります。次の図のように、4つの枠で考えると、抜けが見つかりやすくなります。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="360" y="22" font-size="14" font-weight="700" fill="#0f172a" text-anchor="middle">ストーリー「利用者として、ノートを書きたい」の受入条件</text>
<rect x="16" y="36" width="336" height="116" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="32" y="62" font-size="13" font-weight="700" fill="#047857">正常（うまくいく）</text>
<text x="32" y="88" font-size="12" fill="#0f172a">ログイン中にタイトルを入れて送る</text>
<text x="32" y="110" font-size="12" fill="#0f172a">→ 303 でそのノートの詳細へ移る</text>
<text x="32" y="132" font-size="12" fill="#0f172a">→「ノートを作成しました」が1回だけ出る</text>
<rect x="368" y="36" width="336" height="116" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="384" y="62" font-size="13" font-weight="700" fill="#1e40af">境界（ぎりぎりの値）</text>
<text x="384" y="88" font-size="12" fill="#0f172a">日本語 80 文字のタイトル → 作れる</text>
<text x="384" y="110" font-size="12" fill="#0f172a">81 文字 → 400「現在81文字」</text>
<text x="384" y="132" font-size="12" fill="#0f172a">空・空白だけ → 400、本文は残す</text>
<rect x="16" y="166" width="336" height="116" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="32" y="192" font-size="13" font-weight="700" fill="#c2410c">拒否（させない）</text>
<text x="32" y="218" font-size="12" fill="#0f172a">ログインしていない → 303 で /login へ</text>
<text x="32" y="240" font-size="12" fill="#0f172a">他人のノートを開く → 404</text>
<text x="32" y="262" font-size="12" fill="#0f172a">ほかのサイトから送られた → 403</text>
<rect x="368" y="166" width="336" height="116" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="384" y="192" font-size="13" font-weight="700" fill="#475569">障害（壊れたとき）</text>
<text x="384" y="218" font-size="12" fill="#0f172a">DB が故障した → 500</text>
<text x="384" y="240" font-size="12" fill="#0f172a">画面には原因の詳細を出さない</text>
<text x="384" y="262" font-size="12" fill="#0f172a">原因はサーバーのログに書く</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: 受入条件を考える4つの枠。うまくいく場合・ぎりぎりの値・させないこと・壊れたときを、1つのストーリーごとに書き出す</div>
</div>

- **正常:** いちばん普通の使い方です。ここだけは誰でも思いつきます。
- **境界:** 「80 文字まで」と決めたら、80 文字と 81 文字の両方を書きます。境目のすぐ内側と外側は、間違いがいちばん起きやすい場所です。日本語では「文字数」と「バイト数」が違うので、日本語で確かめることも大切です。
- **拒否:** してはいけない人・してはいけない操作を、どう断るかです。RustNote では、他人のノートは「403 Forbidden（禁止）」ではなく「404 Not Found（見つからない）」で断ると決めています。理由は、エラー1で説明します。
- **障害:** DB が止まったときのように、利用者のせいではない失敗です。画面に何を出し、何を出さないかを先に決めておきます。

### まず動かす最小コード

テストを書く前に、RustNote の要件を書き出します。これが、この単元で作る「設計書」です。中身は、完成した RustNote のソース（`README.md`・`src/`・`tests/`）を読んで、実際の動きに合わせてあります。

**ペルソナ（誰のためのアプリか）:** 授業や仕事で思いついたことを、自宅の PC でも学校の PC でも、ブラウザから書き留めたい人。自分のメモは自分だけが見られればよく、ほかの人と共有はしない。

> **用語:** ペルソナ（persona：アプリを使う人を、具体的な1人の人物として思い描いたもの。「誰のために作るか」がぶれないようにする）

**ユーザーストーリー:**

| 番号 | ストーリー（誰として・何をしたい・なぜ） |
|---|---|
| US-1 | はじめての利用者として、メールアドレスとパスワードで登録したい。自分専用のノートを使い始めるために |
| US-2 | 利用者として、ログインとログアウトをしたい。共用の PC でも、自分のノートをほかの人に見られないように |
| US-3 | 利用者として、タイトルと本文のノートを書きたい。思いついたことを忘れないように |
| US-4 | 利用者として、自分のノートを新しい順に一覧し、キーワードで探したい。前に書いたことをすぐ見つけるために |
| US-5 | 利用者として、ノートを直したい。内容が変わったときに書き換えるために |
| US-6 | 利用者として、ノートを完了にしたり消したりしたい。終わったことと、要らなくなったことを片付けるために |
| US-7 | 運用する人として、アプリと DB が動いているかを外から確かめたい。止まったことにすぐ気づくために |

**受入条件（US-3「ノートを書く」と、全部の画面に共通するもの）:**

| 番号 | 前提 | 操作 | 結果 |
|---|---|---|---|
| AC-01 | ログインしている | タイトル「牛乳を買う」で作成を送る | 303 でそのノートの詳細（`/notes/{id}`）へ移る。次の画面で「ノートを作成しました」が1回だけ出る |
| AC-02 | ログインしていない | ノートの一覧・詳細・編集を開く／作成・編集・完了の切り替え・削除を送る | どれも 303 でログイン画面（`/login`）へ移り、「ログインしてください」が出る |
| AC-03 | ログインしている | タイトルを空、または空白だけにして送る | 400 で「タイトルを入力してください」と出る。入力した本文は消えない |
| AC-04 | ログインしている | 81 文字のタイトルを送る | 400 で「現在81文字」と出る。80 文字（日本語でも）なら作れる |
| AC-05 | alice のノートがある | bob が、そのノートを開く・編集・完了の切り替え・削除をする | どれも 404。alice のノートは変わらない |
| AC-06 | ログインしている | `/notes/abc`・`/notes/0`・`/notes/999` のような、ありえない・存在しない ID を開く | 404 で「ページが見つかりません」と出る |
| AC-07 | ログインしている | ほかのサイトのフォームから、作成が送られてくる | 403 で断る |
| AC-08 | ログインしている | ログイン画面・登録画面を開く | 303 でノートの一覧（`/notes`）へ移る |
| AC-09 | DB が故障している | どの画面でも | 500 で「時間をおいてもう一度お試しください」とだけ出す。原因はサーバーのログに書く |

**非機能要件（機能以外の約束）:**

| 種類 | 約束 | RustNote で守っている場所 |
|---|---|---|
| 安全 | パスワードはそのまま保存せず、Argon2id のハッシュだけを保存する | `src/auth.rs` の `hash_password` |
| 安全 | ログインの失敗は、メールが登録されているかどうかで答えを変えない（どちらも 401 と同じ文） | `src/routes/login.rs` |
| 安全 | 入力された HTML は、文字として表示する（実行されない） | `view!` の自動エスケープ |
| 安全 | 500 のとき、画面には原因の詳細を出さない | `src/error.rs` の `public_message` |
| 量の上限 | 一覧は1ページ 20 件。タイトル 80 文字・本文 10,000 文字・パスワード 8〜128 文字・検索語 100 文字まで | `src/domain.rs` の定数 |
| 運用 | `GET /health` で、DB まで届くかを確かめられる（届けば 200、届かなければ 503） | `src/routes/health.rs` |

**スコープの外（この版では作らないこと）:** パスワードを忘れたときの再設定、メールアドレスの確認（確認メールの送信）、ノートの共有や公開、タグや画像の添付、退会の画面、ログインの試行回数の制限。どれも、RustNote のソースにはありません。

> **ヒント:** 「作らないこと」を書いておくのは、「作り忘れ」と「わざと作らない」を区別するためです。たとえば「ログインの試行回数の制限」は、本番で公開するなら考えたい機能です。書いてあれば、後で「次の版で足すかどうか」を話し合えます。

この表ができたら、受入条件を1つずつテストにします。最初は、いちばん短い AC-08 です。

#### 例1-1: 受入条件 AC-08 を、そのままテストにする

> **▼ このコードがやること（先に日本語で）:** 「ログインしている人が、ログイン画面と登録画面を開くと、303 でノートの一覧へ移る」という受入条件 AC-08 を、テストにします。テストの名前を `ac08_...` にして、受入条件の番号と1対1で対応させます。前提・操作・結果は、コメントにそのまま書き写します。

```rust
//! 受入条件（AC）を、そのままテストにしたもの。
//! テストの名前は「ac番号_条件の要約」にそろえ、受入条件の表と1対1で対応させる。
//! 実行: cargo test --test acceptance

mod common;

use common::{location, test_app};
use topcoat::router::StatusCode;

const PASSWORD: &str = "correct horse battery";

// AC-08
// 前提: ログインしている
// 操作: ログイン画面・登録画面を開く
// 結果: 303 でノート一覧（/notes）へ移動する
#[tokio::test]
async fn ac08_logged_in_user_skips_login_and_signup() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;

    for path in ["/login", "/signup"] {
        let response = app.get(path, Some(&alice)).await;
        assert_eq!(response.status(), StatusCode::SEE_OTHER, "{path}");
        assert_eq!(location(&response), "/notes", "{path}");
    }
}
```

コピーした `rustnote` フォルダの `tests/acceptance.rs` として保存して実行します。

```powershell
cargo test --test acceptance
```

**▼ 実行結果**

```text
   Compiling rustnote v0.1.0 (...\rustnote)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.05s
     Running tests\acceptance.rs (target\debug\deps\acceptance-0f2d784d56c401b2.exe)

running 1 test
test ac08_logged_in_user_skips_login_and_signup ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.48s
```

`ok` が出ました。完成した RustNote は、AC-08 を満たしています。

#### ▼ コードを1つずつ分解して解説

##### 解説1: テストの名前を、受入条件の番号にそろえる

```rust
async fn ac08_logged_in_user_skips_login_and_signup() {
```

- 名前の先頭を `ac08_` にしておくと、テストの結果の `test ac08_... ok` を見ただけで、「受入条件の表の AC-08 が守られている」と分かります。
- 後ろの `logged_in_user_skips_login_and_signup`（ログイン中の人は、ログインと登録を飛ばす）は、受入条件の要約です。Rust の関数名なので、英語の小文字と `_` で書きます。
- 受入条件の表を書き換えたら、テストの名前も一緒に直します。表とテストが1対1になっていれば、「表にあるのにテストがない条件」がすぐ見つかります（例1-2で、実際に探します）。

##### 解説2: 前提・操作・結果を、コメントとコードの両方に書く

```rust
// AC-08
// 前提: ログインしている
// 操作: ログイン画面・登録画面を開く
// 結果: 303 でノート一覧（/notes）へ移動する
```

- 受入条件の表の1行を、そのままコメントに写しています。コードの順番も、コメントと同じにそろえます。
- **前提**は `sign_up`（登録すると、ログインした状態になる）、**操作**は `app.get(path, ...)`、**結果**は2つの `assert_eq!` です。
- この「前提・操作・結果」の書き方は、英語では Given（前提）・When（操作）・Then（結果）と呼ばれ、受入条件を書くときの定番の形です。

##### 解説3: tests/common/mod.rs の道具を使う

```rust
mod common;

use common::{location, test_app};
```

- `tests/common/mod.rs` は、完成した RustNote に最初から入っている、テスト用の道具箱です。`mod common;` と書くと、このファイルから使えるようになります。
- `test_app()` は、一時フォルダに空の DB を作ってマイグレーションを適用し、RustNote のルーター（URL ごとの処理の一覧）を組み立てます。テストごとに別の DB なので、ほかのテストの影響を受けません。
- `app.sign_up(メール, パスワード)` は、`POST /signup` を送って登録し、返ってきたログイン用のクッキー（`__Host-session=...`）を文字列で返します。`app.get(URL, クッキー)` は GET のリクエストを送ります。`location(&response)` は、リダイレクト先（`Location` ヘッダー）を取り出します。
- この道具は、Topcoat の `Router::handle` に、リクエストを**直接**渡しています。ポートを開いてサーバーを起動しないので、速く、何個のテストを同時に動かしてもぶつかりません。

##### 解説4: #[tokio::test] と、assert_eq! の3つ目の引数

```rust
#[tokio::test]
async fn ac08_logged_in_user_skips_login_and_signup() {
```

```rust
        assert_eq!(response.status(), StatusCode::SEE_OTHER, "{path}");
```

- RustNote の処理は `async` なので、テストの関数も `async fn` にして、`#[tokio::test]` を付けます。`#[tokio::main]` のテスト版で、テストの外側に Tokio のランタイムを用意してくれます（第5巻 UNIT 01・02）。
- `StatusCode::SEE_OTHER` は 303 です。数字の `303` と書くより、意味が名前で分かります。
- `assert_eq!` の3つ目の `"{path}"` は、失敗したときに表示する言葉です。ループの中で2つの URL を試しているので、どちらで失敗したのかが分かるように、URL を表示させています。

### もう一歩: 実務に近い形

受入条件を全部、新しいテストとして書く必要はありません。RustNote には、すでに `tests/http.rs` に 14 個の HTTP のテストがあります。まず、**今あるテストと受入条件の表を突き合わせて**、足りないところだけを足します。

#### 例1-2: 今あるテストの一覧を出して、受入条件と突き合わせる

> **▼ このコードがやること（先に日本語で）:** `tests/http.rs` のテストを実行せずに、名前の一覧だけを表示します。その一覧と受入条件の表を見比べて、どの受入条件がどのテストで確かめられているか、確かめられていないのはどれかを調べます。

```powershell
cargo test --test http -- --list
```

**▼ 実行結果**

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running tests\http.rs (target\debug\deps\http-9c37f549b35c0170.exe)
create_note_follows_prg_and_escapes_html: test
cross_site_post_is_403: test
edit_with_stale_version_is_409: test
empty_title_is_400: test
flash_message_is_shown_once: test
health_returns_ok_json: test
invalid_ids_and_unknown_urls_are_404: test
list_searches_and_paginates: test
login_and_logout: test
notes_require_login: test
other_users_notes_are_404: test
signup_validates_input_and_rejects_duplicates: test
toggle_and_delete_redirect: test
top_page_shows_links_when_logged_out: test

14 tests, 0 benchmarks
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: --list で、テストを実行せずに名前だけを出す

- `--list` は、テストを実行せずに名前の一覧だけを出す指定です。`--` の後ろに書くので、Cargo ではなく、テストの実行ファイルへの指定です。最後の `14 tests` が数です。
- 名前の後ろの `: test` は「ふつうのテスト」という印です（ほかに、速さを測る benchmark があります。`0 benchmarks` がその数です）。

##### 解説2: 受入条件の表と突き合わせる

テストの名前と中身（`tests/http.rs`）を読んで、受入条件の表と突き合わせると、次のようになります。

| 受入条件 | 確かめているテスト | 足りないところ |
|---|---|---|
| AC-01 作成すると詳細へ・お知らせは1回 | `create_note_follows_prg_and_escapes_html`・`flash_message_is_shown_once` | なし |
| AC-02 未ログインはログイン画面へ | `notes_require_login` | **一覧（GET）と作成（POST）しか試していない**。詳細・編集・完了の切り替え・削除が無い |
| AC-03 空・空白だけは 400 | `empty_title_is_400` | なし（空白3つの `title=+++` で試している） |
| AC-04 81 文字は 400 | `empty_title_is_400` と、ユニットテストの `note_title_counts_characters_not_bytes` | なし |
| AC-05 他人のノートは 404 | `other_users_notes_are_404` | なし |
| AC-06 ありえない ID は 404 | `invalid_ids_and_unknown_urls_are_404` | なし |
| AC-07 ほかのサイトからは 403 | `cross_site_post_is_403` | なし |
| AC-08 ログイン中はログイン画面を飛ばす | （無し） | **テストが無い**（例1-1で足した） |
| AC-09 DB の故障は 500 | ユニットテストの `status_codes_and_public_messages` | HTTP の形では試していない |

##### 解説3: 表にすると、穴が見つかる

- 表を作ってみると、**AC-02 は半分しか確かめられていない**ことが分かりました。`notes_require_login` は `/notes` の GET と POST だけを試していて、`/notes/1` や `/notes/1/edit` などは試していません。
- AC-09 は、「500 のときに原因を画面に出さない」ことを、`src/error.rs` のユニットテストで確かめています。HTTP のテストで DB をわざと壊すのは手間がかかるので、この巻では、この形のままにします。どこまでをテストで確かめるかも、設計で決めることの1つです。

#### 例1-3: 足りなかった AC-02 を、テストで埋める

> **▼ このコードがやること（先に日本語で）:** alice が書いたノートを1件用意してから、ログインしていない状態で、詳細と編集の画面を開き（GET）、完了の切り替えと削除を送ります（POST）。どれも 303 でログイン画面へ移ることを確かめます。例1-1のファイルに、AC-02 のテストを足した全体です。

```rust
//! 受入条件（AC）を、そのままテストにしたもの。
//! テストの名前は「ac番号_条件の要約」にそろえ、受入条件の表と1対1で対応させる。
//! 実行: cargo test --test acceptance

mod common;

use common::{location, test_app};
use topcoat::router::StatusCode;

const PASSWORD: &str = "correct horse battery";

// AC-02
// 前提: ログインしていない（alice のノートが1件ある）
// 操作: ノートの詳細・編集を開く／完了の切り替え・削除を送る
// 結果: どれも 303 でログイン画面（/login）へ移動する
#[tokio::test]
async fn ac02_logged_out_user_is_sent_to_login() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let url = app.create_note(&alice, "alice のノート", "").await;

    // 画面を開く（GET）
    for path in [url.clone(), format!("{url}/edit")] {
        let response = app.get(&path, None).await;
        assert_eq!(response.status(), StatusCode::SEE_OTHER, "GET {path}");
        assert_eq!(location(&response), "/login", "GET {path}");
    }
    // 操作を送る（POST）
    for path in [format!("{url}/toggle"), format!("{url}/delete")] {
        let response = app.post_form(&path, "", None).await;
        assert_eq!(response.status(), StatusCode::SEE_OTHER, "POST {path}");
        assert_eq!(location(&response), "/login", "POST {path}");
    }
}

// AC-08
// 前提: ログインしている
// 操作: ログイン画面・登録画面を開く
// 結果: 303 でノート一覧（/notes）へ移動する
#[tokio::test]
async fn ac08_logged_in_user_skips_login_and_signup() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;

    for path in ["/login", "/signup"] {
        let response = app.get(path, Some(&alice)).await;
        assert_eq!(response.status(), StatusCode::SEE_OTHER, "{path}");
        assert_eq!(location(&response), "/notes", "{path}");
    }
}
```

`tests/acceptance.rs` をこの内容に書き換えて、もう一度実行します。

```powershell
cargo test --test acceptance
```

**▼ 実行結果**

```text
   Compiling rustnote v0.1.0 (...\rustnote)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.25s
     Running tests\acceptance.rs (target\debug\deps\acceptance-0f2d784d56c401b2.exe)

running 2 tests
test ac08_logged_in_user_skips_login_and_signup ... ok
test ac02_logged_out_user_is_sent_to_login ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.49s
```

2つとも `ok` です。テストは同時に動くので、表示の順番はファイルに書いた順と違うことがあります。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 前提を作るのは、別の人（alice）

```rust
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let url = app.create_note(&alice, "alice のノート", "").await;
```

- 「ログインしていない人が `/notes/1` を開く」を試すには、まず `/notes/1` が**本当にある**必要があります。存在しないノートなら、ログインしていてもいなくても 404 になるかもしれず、ログインの確認を試したことになりません。
- そこで、alice が登録してノートを1件作ります。`create_note` は `POST /notes` を送り、できたノートの URL（`/notes/1`）を返します。
- この後の操作では、クッキーに `None` を渡します。つまり「誰のクッキーも持っていない人」として、alice のノートの URL を開いています。

##### 解説2: GET と POST を、両方とも試す

```rust
    for path in [url.clone(), format!("{url}/edit")] {
        let response = app.get(&path, None).await;
```

```rust
    for path in [format!("{url}/toggle"), format!("{url}/delete")] {
        let response = app.post_form(&path, "", None).await;
```

- 画面を**見る**のは GET、ノートを**変える**のは POST です（UNIT 02 でくわしく扱います）。「ログインしていなければ断る」は、見る操作にも変える操作にも必要なので、両方を試します。
- `post_form(URL, フォームの中身, クッキー)` は、フォームの POST を送ります。完了の切り替えと削除は、送る項目が無いので、中身は空の `""` です。
- `url.clone()` は、`url` を後でもう一度使うためのコピーです。配列の中に `url` をそのまま入れると、`url` の持ち主が配列に移ってしまい、次の `format!("{url}/edit")` で使えなくなるからです（第3巻）。

##### 解説3: 失敗したときの言葉に、メソッドと URL を入れる

```rust
        assert_eq!(location(&response), "/login", "GET {path}");
```

- 4つの URL を2つのループで試しているので、失敗したときに「どの操作で」失敗したのかが分からないと困ります。3つ目の引数に `"GET {path}"` や `"POST {path}"` を入れておくと、失敗したときに `GET /notes/1/edit` のように表示されます。

### よくあるエラーと読み方

#### エラー1: 受入条件の書き間違い（他人のノートを「403」と書いた）

受入条件を書いた人が、「他人のノートは開けないのだから、403 Forbidden（禁止）だろう」と考えて、AC-05 を次のように書いたとします。例1-3の `tests/acceptance.rs` の最後に、このテストを足して実行します。

```rust

// AC-05（書き間違えた版）
// 前提: alice と bob が登録済みで、alice のノートが1件ある
// 操作: bob が alice のノートの詳細を開く
// 結果: 403 Forbidden になる
#[tokio::test]
async fn ac05_other_users_note_is_forbidden() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let bob = app.sign_up("bob@example.com", PASSWORD).await;
    let url = app.create_note(&alice, "alice の秘密", "").await;

    let response = app.get(&url, Some(&bob)).await;
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}
```

```text
   Compiling rustnote v0.1.0 (...\rustnote)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.92s
     Running tests\acceptance.rs (target\debug\deps\acceptance-0f2d784d56c401b2.exe)

running 3 tests
test ac08_logged_in_user_skips_login_and_signup ... ok
test ac02_logged_out_user_is_sent_to_login ... ok
test ac05_other_users_note_is_forbidden ... FAILED

failures:

---- ac05_other_users_note_is_forbidden stdout ----
POST /signup -> 303 See Other (394.9415ms)
POST /signup -> 303 See Other (438.6321ms)
POST /notes -> 303 See Other (4.4981ms)
GET /notes/1 -> 404 Not Found (2.862ms)

thread 'ac05_other_users_note_is_forbidden' (68436) panicked at tests\acceptance.rs:64:5:
assertion `left == right` failed
  left: 404
 right: 403
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    ac05_other_users_note_is_forbidden

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.87s

error: test failed, to rerun pass `--test acceptance`
```

- **読む順番:** まず最後の `test result: FAILED. 2 passed; 1 failed` で、3つのうち1つが失敗したと分かります。次に `failures:` の下の `---- ac05_other_users_note_is_forbidden stdout ----` が、失敗したテストの記録です。
- `POST /signup -> 303 See Other (394.9415ms)` のような行は、RustNote のアクセスログです（`src/routes.rs` の `access_log` が、リクエストごとに1行書きます）。テストが失敗すると、そのテストの間に出た表示がまとめて見られます。最後の `GET /notes/1 -> 404 Not Found` で、bob の GET が 404 になったことが分かります。
- `panicked at tests\acceptance.rs:64:5` は「64 行目の5文字目で止まった」、`assertion left == right failed` は「`assert_eq!` の左と右が違った」という意味です。`left: 404` が実際の値（`response.status()`）、`right: 403` が期待した値です。
- **ここで考えること:** コードが間違っているのか、受入条件が間違っているのか。RustNote は、他人のノートを**わざと** 404 にしています（`src/routes/notes/id.rs` の先頭のコメントに「他人のノートも 404（『ある』こと自体を知らせないため、403 にはしない）」と書いてあります）。403 を返すと、「その番号のノートは存在する。ただ、あなたには見せない」と教えてしまいます。番号を 1, 2, 3… と変えて試せば、どの番号のノートが存在するか、アプリ全体でノートが何件くらいあるかまで分かってしまいます。404 なら、「無い」のか「他人のもの」なのかを区別できません。
- **直し方:** 間違っていたのは受入条件です。受入条件の表の AC-05 を「404」に直し、テストも名前ごと直します。

```rust

// AC-05
// 前提: alice と bob が登録済みで、alice のノートが1件ある
// 操作: bob が alice のノートの詳細を開く
// 結果: 404 Not Found になる（ノートが「ある」ことも知らせない）
#[tokio::test]
async fn ac05_other_users_note_is_not_found() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let bob = app.sign_up("bob@example.com", PASSWORD).await;
    let url = app.create_note(&alice, "alice の秘密", "").await;

    let response = app.get(&url, Some(&bob)).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
```

```text
   Compiling rustnote v0.1.0 (...\rustnote)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.10s
     Running tests\acceptance.rs (target\debug\deps\acceptance-0f2d784d56c401b2.exe)

running 3 tests
test ac08_logged_in_user_skips_login_and_signup ... ok
test ac02_logged_out_user_is_sent_to_login ... ok
test ac05_other_users_note_is_not_found ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.02s
```

> **注意:** テストが失敗したとき、テストの期待する値を、実際の値に**黙って**合わせてはいけません。それでは、本当にコードが間違っていたときも、テストを通してしまいます。このエラー1のように、「なぜその値が正しいのか」を受入条件の段階で説明できたときだけ、受入条件とテストを直します。

#### エラー2: テストのファイル名の打ち間違い（no test target named）

```powershell
cargo test --test acceptanse
```

```text
error: no test target named `acceptanse` in default-run packages

help: a target with a similar name exists: `acceptance`
```

- **何が起きたか:** 「`acceptanse` という名前のテストのまとまりはありません」というエラーです。`--test` の後ろには、`tests/` の中のファイル名（`.rs` を除いたもの）を書きます。
- **直し方:** `help:` が「似た名前の `acceptance` があります」と教えてくれています。`cargo test --test acceptance` に直します。

#### エラー3: async fn に #[test] を付けた

`#[tokio::test]` の代わりに、ふつうのテストの `#[test]` を付けてしまった場合です（`tests/e01_test_attr.rs` に保存して試します）。

```rust
mod common;

use common::{location, test_app};
use topcoat::router::StatusCode;

// #[tokio::test] ではなく #[test] を付けてしまった
#[test]
async fn ac08_logged_in_user_skips_login_and_signup() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", "correct horse battery").await;
    let response = app.get("/login", Some(&alice)).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&response), "/notes");
}
```

```powershell
cargo test --test e01_test_attr
```

```text
   Compiling rustnote v0.1.0 (...\rustnote)
error: async functions cannot be used for tests
  --> tests\e01_test_attr.rs:8:1
   |
 8 |   async fn ac08_logged_in_user_skips_login_and_signup() {
   |   ^----
   |   |
   |  _`async` because of this
   | |
 9 | |     let app = test_app().await;
10 | |     let alice = app.sign_up("alice@example.com", "correct horse battery").await;
11 | |     let response = app.get("/login", Some(&alice)).await;
12 | |     assert_eq!(response.status(), StatusCode::SEE_OTHER);
13 | |     assert_eq!(location(&response), "/notes");
14 | | }
   | |_^

error: could not compile `rustnote` (test "e01_test_attr") due to 1 previous error
```

- **何が起きたか:** 「async 関数はテストに使えません」というエラーです。`#[test]` は、ふつうの関数を呼ぶだけで、async 関数が返す Future を動かす係（ランタイム）を用意しません（第5巻 UNIT 01）。`async because of this` の線が、原因の `async` を指しています。
- **直し方:** `#[test]` を `#[tokio::test]` に直します。確かめ終わったら、`tests/e01_test_attr.rs` は消しておきます。

#### エラー4: mod common; の書き忘れ（E0432）

```rust
// mod common; を書き忘れた
use common::{location, test_app};
use topcoat::router::StatusCode;

#[tokio::test]
async fn ac08_logged_in_user_skips_login_and_signup() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", "correct horse battery").await;
    let response = app.get("/login", Some(&alice)).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&response), "/notes");
}
```

`tests/e01_no_mod.rs` に保存して、`cargo test --test e01_no_mod` を実行します。

```text
   Compiling rustnote v0.1.0 (...\rustnote)
error[E0432]: unresolved import `common`
 --> tests\e01_no_mod.rs:2:5
  |
2 | use common::{location, test_app};
  |     ^^^^^^ use of unresolved module or unlinked crate `common`
  |
help: to make use of source file tests\common\mod.rs, use `mod common` in this file to declare the module
  |
2 + mod common;
  |

For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustnote` (test "e01_no_mod") due to 1 previous error
```

- **何が起きたか:** E0432 は「`use` で取り込もうとしたものが見つからない」というエラーです。`tests/` の中のファイルは、1つずつが別のプログラム（クレート）としてコンパイルされます。`tests/common/mod.rs` は、`mod common;` と書いたファイルからしか見えません。
- **直し方:** `help:` のとおり、ファイルの先頭に `mod common;` を足します。確かめ終わったら、`tests/e01_no_mod.rs` は消しておきます。

#### エラー5（エラーが出ない間違い）: 絞り込みの名前を間違えて、1件も実行していない

`cargo test` では、`--test` の後ろにもう1つ言葉を書くと、名前にその言葉を含むテストだけを実行できます。ところが、存在しない名前を書いても、エラーになりません。

```powershell
cargo test --test acceptance ac99
```

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running tests\acceptance.rs (target\debug\deps\acceptance-0f2d784d56c401b2.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s
```

- **何が起きたか:** 結果は `ok` ですが、`running 0 tests`（0件を実行）、`3 filtered out`（3件を絞り込みで外した）と出ています。名前に `ac99` を含むテストが1つも無かったので、何も確かめずに「成功」と表示されました。
- **見落とさない工夫:** 結果の行は、`ok` だけでなく、**`passed` の数**も見ます。「AC-05 を確かめたはず」なのに `0 passed` なら、絞り込みの言葉が間違っています。

### RustNote ではここで使う

完成した RustNote の `README.md` には、「できること」の箇条書きと、「URL の一覧」「エラーのときのステータス」の表があります。これは、この単元で作ったユーザーストーリーと受入条件を、短くまとめたものと同じ中身です。そして、受入条件の多くは、`tests/http.rs` のテストとして書かれています。

たとえば AC-05（他人のノートは 404）は、`tests/http.rs` の `other_users_notes_are_404` で確かめています。この単元のエラー1で直した版より、ずっと広く試しています。

```rust
#[tokio::test]
async fn other_users_notes_are_404() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let bob = app.sign_up("bob@example.com", PASSWORD).await;
    let url = app.create_note(&alice, "alice の秘密", "").await;

    // bob からは、見る・編集・切り替え・削除のどれも 404
    assert_eq!(
        app.get(&url, Some(&bob)).await.status(),
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        app.get(&format!("{url}/edit"), Some(&bob)).await.status(),
        StatusCode::NOT_FOUND
    );
    let response = app
        .post_form(
            &format!("{url}/edit"),
            "title=x&body=&version=1",
            Some(&bob),
        )
        .await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let response = app
        .post_form(&format!("{url}/toggle"), "", Some(&bob))
        .await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let response = app
        .post_form(&format!("{url}/delete"), "", Some(&bob))
        .await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // alice のノートは無事
    let html = body_text(app.get(&url, Some(&alice)).await).await;
    assert!(html.contains("alice の秘密"));
}
```

- 見る（詳細・編集の画面）と、変える（保存・完了の切り替え・削除）の5つの操作を、すべて bob から試しています。受入条件の「開く・編集・完了の切り替え・削除をする」を、そのまま全部コードにした形です。
- 最後に、alice 本人からはノートが見えて、中身も変わっていないことを確かめています。「断る」だけでなく、「断ったあとも、本人のデータは無事」までが受入条件です。

RustNote のエラーの決まりも、`src/error.rs` の先頭のコメントに、表で書いてあります（説明に関係する部分を抜き出しています）。

```rust
//! | AppError | ステータス | 起きる場面 |
//! |---|---|---|
//! | `InvalidTitle` など入力の検証エラー | 400 | 空のタイトル、形式の違うメール |
//! | `Unauthorized` | 401 | ログインしていない（通常はログイン画面へのリダイレクトで扱う） |
//! | `NotFound` | 404 | 存在しないノート、**他人のノート**（あること自体を隠す） |
//! | `EmailTaken` / `Conflict` | 409 | 登録済みのメール、ほかの画面で先に更新された |
//! | `Internal` | 500 | DB の故障など。画面には詳細を出さず、ログに原因を書く |
```

- 受入条件で決めた「どんなときに、どの番号で答えるか」が、コードの中のエラーの種類（`AppError`）と1対1で対応しています。受入条件を先に決めておいたから、エラーの種類も迷わずに決められた、ということです。

### 練習問題

**問1:** ストーリー US-5「利用者として、ノートを直したい」の受入条件を、図の4つの枠（正常・境界・拒否・障害）で書いてください。番号は AC-11 から振り、「前提・操作・結果」の形にします。RustNote の編集の画面（`src/routes/notes/id/edit.rs`）の動きに合わせてください。ヒント: 編集のフォームには、開いたときのノートの版の番号（`version`）が隠れて入っています。

**問2:** ストーリー US-2 に、次の受入条件を足します。これを `tests/acceptance.rs` のテストにしてください。テストの名前は `ac10_` で始めます。

| 番号 | 前提 | 操作 | 結果 |
|---|---|---|---|
| AC-10 | alice がログインしていて、ノートが1件ある | ログアウトしたあと、ログアウトする前のクッキーで、ノートの詳細を開く | 303 でログイン画面（`/login`）へ移る（古いクッキーはもう使えない） |

**問3:** 次の4つの要件は、あいまいで、誰が確かめても同じ答えになるとは言えません。確かめられる形に書き直してください。

1. 一覧は見やすくする
2. 一覧はすぐに表示される
3. パスワードは安全に扱う
4. 他人のノートは見られない

### 解答と解説

**問1の解答:**

| 番号 | 枠 | 前提 | 操作 | 結果 |
|---|---|---|---|---|
| AC-11 | 正常 | alice が自分のノート（version 1）の編集画面を開いた | タイトルを書き換えて「保存する」 | 303 でそのノートの詳細へ移り、「ノートを保存しました」が出る。version は 2 になる |
| AC-12 | 境界 | 同上 | タイトルを空にする／81 文字にして保存する | 400 で編集のフォームを出し直す。入力した内容は残り、保存はされない |
| AC-13 | 拒否 | alice のノートがある | bob が、そのノートの編集画面を開く／保存を送る | どちらも 404 |
| AC-14 | 拒否 | 2つのタブで同じノート（version 1）の編集画面を開き、片方で先に保存した（version 2 になった） | もう片方のタブで保存する | 409 で「ほかの画面で先に更新されました」と出し、最新の version でフォームを出し直す。上書きはしない |
| AC-15 | 障害 | DB が故障している | 保存する | 500 で「時間をおいてもう一度お試しください」とだけ出す |

- AC-11〜AC-14 は、`src/routes/notes/id/edit.rs` の先頭のコメントにある表（成功は 303、タイトルの問題は 400、version が古いと 409、他人のノートは 404）と同じです。`tests/http.rs` の `edit_with_stale_version_is_409` と `other_users_notes_are_404` が確かめています。
- AC-14 は、正常・拒否のどちらの枠にも入れにくい「2人（2つの画面）が同時に直した」場面です。こういう場面は、4つの枠で考えても見落としやすいので、「同時に操作したら？」も自分に問いかける習慣をつけましょう。この仕組み（楽観的ロック）は、第7巻 UNIT 05 で学びました。

**問2の解答:**

```rust

// AC-10
// 前提: alice がログインしていて、ノートが1件ある
// 操作: ログアウトしたあと、ログアウト前のクッキーでノートの詳細を開く
// 結果: 303 でログイン画面（/login）へ移動する（古いクッキーはもう使えない）
#[tokio::test]
async fn ac10_old_cookie_cannot_open_note_after_logout() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", PASSWORD).await;
    let url = app.create_note(&alice, "買い物", "").await;

    let response = app.post_form("/logout", "", Some(&alice)).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);

    let response = app.get(&url, Some(&alice)).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&response), "/login");
}
```

`tests/acceptance.rs` の最後に足して、名前に `ac10` を含むテストだけを実行した結果です。

```powershell
cargo test --test acceptance ac10
```

```text
   Compiling rustnote v0.1.0 (...\rustnote)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.17s
     Running tests\acceptance.rs (target\debug\deps\acceptance-0f2d784d56c401b2.exe)

running 1 test
test ac10_old_cookie_cannot_open_note_after_logout ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.53s
```

- `1 passed`・`3 filtered out` なので、AC-10 のテストが1件だけ実行されて成功したと分かります（エラー5の見方です）。
- ログアウト（`POST /logout`）は、ブラウザのクッキーを消すだけでなく、DB の `sessions` 表の記録も消します（`src/auth.rs` の `log_out`）。だから、ログアウトの前にこっそりクッキーを写し取っておいても、もう使えません。クッキーを消すだけの作りだと、このテストは失敗します。

**問3の解答（例）:**

1. 「見やすく」→「一覧は、更新した日時の新しい順に並べ、1ページに 20 件まで出す。21 件目からは次のページに分け、『1 / 2 ページ（全21件）』のように今の位置と総数を出す」
2. 「すぐに」→ 時間で書きます。たとえば「ノートが 1,000 件ある人でも、一覧の1ページ目を 0.5 秒以内に返す」。RustNote の `README.md` には、時間の目標は書かれていません。目標を決めたら、測り方（どの PC で、何件のデータで、何回測るか）も一緒に決めます。
3. 「安全に」→「パスワードはそのまま保存せず、Argon2id のハッシュだけを保存する」「8 文字以上・128 文字以内」「ログインに失敗したときは、メールが登録されているかどうかで答えを変えない」のように、何をするかを書きます。
4. 「見られない」→ AC-05 の形（「bob が alice のノートを開く・編集・完了の切り替え・削除をすると、どれも 404。alice のノートは変わらない」）にします。「見る」だけでなく「変える」操作も入れるのがポイントです。

- 書き直すときのコツは、「それを確かめるテストを書けるか？」と自分に聞くことです。テストを書けない要件は、まだあいまいです。

### まとめ

- 要件は「ペルソナ → ユーザーストーリー → 受入条件 → テスト」の順に細かくする。受入条件は「前提・操作・結果」の形で、ステータスコードや画面の文のように、誰が確かめても同じ答えになる言葉で書く。
- 受入条件は、正常・境界・拒否・障害の4つの枠で考え、「作らないこと」と機能以外の約束も書き出す。テストの名前を受入条件の番号にそろえると、表とテストを突き合わせて抜けを探せる。
- テストが失敗したら、まず受入条件が正しいかを確かめる。RustNote は他人のノートを、存在を隠すために 403 ではなく 404 で断る。
