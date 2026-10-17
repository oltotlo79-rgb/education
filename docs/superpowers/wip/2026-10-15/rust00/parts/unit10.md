## UNIT 10: 学習サイクルと完成像

### この単元のゴール

- 「**予想 → 実行 → 観察 → 1か所変える → 説明・記録**」の学習サイクルを、自分で回せる
- `cargo test` で**テスト**を動かし、失敗したテストの出力（`left` と `right`）を読める
- 文字列の「**バイト数**」と「**文字数**」の違いが分かる（RustNote のタイトル検査につながる）
- この巻の10単元が、RustNote の1回のリクエストのどこにつながっているかを説明できる

### たとえ話でつかむ

プログラミングの上達は、**料理の練習**に似ています。

- レシピ（教材）を何冊読んでも、料理は上手になりません。上達する人は、作る前に「**たぶんこんな味になる**」と予想し（予想）、作って（実行）、味見をして（観察）、「**塩をひとつまみだけ**増やしたらどうなるか」を試し（1か所変える）、違いを言葉にして**レシピノートに書き留めます**（説明・記録）。
- 一度に塩と砂糖と火加減を全部変えると、何が効いたのか分かりません。**変えるのは1か所ずつ**です。
- 書き留めたレシピノートが、プログラミングでは**テスト**です。「このタイトルは正しい」「空のタイトルはだめ」を自動で確かめるコードを書いておくと、後でコードを変えたときに、前にできていたことが壊れていないかを**一瞬で確かめられます**。

> **用語:** **テスト** … プログラムが期待どおりに動くかを確かめるための、別の小さなプログラム。Rust では `#[test]` という印を付けた関数がテストになり、`cargo test` で全部まとめて実行できます。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 320" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-24" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<circle cx="360" cy="160" r="62" fill="#f8fafc" stroke="#cbd5e1" stroke-width="2"/>
<text x="360" y="154" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">1単元ごとに</text>
<text x="360" y="176" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">何周も回す</text>
<rect x="290" y="12" width="140" height="58" rx="12" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="36" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">① 予想する</text>
<text x="360" y="56" font-size="12" fill="#334155" text-anchor="middle">何が表示される？</text>
<rect x="520" y="86" width="150" height="58" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="595" y="110" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">② 実行する</text>
<text x="595" y="130" font-size="12" fill="#334155" text-anchor="middle">そのまま動かす</text>
<rect x="470" y="232" width="160" height="58" rx="12" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="550" y="256" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">③ 観察する</text>
<text x="550" y="276" font-size="12" fill="#334155" text-anchor="middle">予想と同じ？違う？</text>
<rect x="90" y="232" width="160" height="58" rx="12" fill="#fdf2f8" stroke="#db2777" stroke-width="2"/>
<text x="170" y="256" font-size="14" font-weight="700" fill="#be185d" text-anchor="middle">④ 1か所変える</text>
<text x="170" y="276" font-size="12" fill="#334155" text-anchor="middle">値や型を1つだけ</text>
<rect x="40" y="86" width="170" height="58" rx="12" fill="#fefce8" stroke="#ca8a04" stroke-width="2"/>
<text x="125" y="110" font-size="14" font-weight="700" fill="#a16207" text-anchor="middle">⑤ 説明・記録する</text>
<text x="125" y="130" font-size="12" fill="#334155" text-anchor="middle">言葉にしてテストに</text>
<path d="M432,40 C500,44 560,60 585,82" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-24)"/>
<path d="M600,146 C610,180 595,210 575,228" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-24)"/>
<path d="M468,262 L254,262" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-24)"/>
<path d="M140,230 C120,200 115,175 120,148" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-24)"/>
<path d="M150,84 C180,50 230,40 286,40" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-24)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図10-1: 学習サイクル。読んだ量ではなく、このサイクルを何周回したかで進み具合を測る</div>
</div>

この単元のコードで扱う「文字数」の落とし穴を、先に図で見ておきます。Rust の文字列は **UTF-8** という形でメモリに並んでいて、日本語の文字は1文字が **3バイト** になります。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 210" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="20" y="28" font-size="13" font-weight="700" fill="#334155">"Rust"</text>
<rect x="120" y="12" width="40" height="28" rx="4" fill="#eff6ff" stroke="#3b82f6"/>
<rect x="164" y="12" width="40" height="28" rx="4" fill="#eff6ff" stroke="#3b82f6"/>
<rect x="208" y="12" width="40" height="28" rx="4" fill="#eff6ff" stroke="#3b82f6"/>
<rect x="252" y="12" width="40" height="28" rx="4" fill="#eff6ff" stroke="#3b82f6"/>
<text x="140" y="31" font-size="13" fill="#1e40af" text-anchor="middle">R</text>
<text x="184" y="31" font-size="13" fill="#1e40af" text-anchor="middle">u</text>
<text x="228" y="31" font-size="13" fill="#1e40af" text-anchor="middle">s</text>
<text x="272" y="31" font-size="13" fill="#1e40af" text-anchor="middle">t</text>
<text x="310" y="31" font-size="12" fill="#334155">4 文字 ＝ 4 バイト（英数字は1文字1バイト）</text>
<text x="20" y="88" font-size="13" font-weight="700" fill="#334155">"牛乳を買う"</text>
<rect x="120" y="64" width="114" height="34" rx="4" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<rect x="238" y="64" width="114" height="34" rx="4" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<rect x="356" y="64" width="114" height="34" rx="4" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<rect x="474" y="64" width="114" height="34" rx="4" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<rect x="592" y="64" width="114" height="34" rx="4" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="177" y="87" font-size="15" font-weight="700" fill="#c2410c" text-anchor="middle">牛</text>
<text x="295" y="87" font-size="15" font-weight="700" fill="#c2410c" text-anchor="middle">乳</text>
<text x="413" y="87" font-size="15" font-weight="700" fill="#c2410c" text-anchor="middle">を</text>
<text x="531" y="87" font-size="15" font-weight="700" fill="#c2410c" text-anchor="middle">買</text>
<text x="649" y="87" font-size="15" font-weight="700" fill="#c2410c" text-anchor="middle">う</text>
<g fill="#ffffff" stroke="#94a3b8">
<rect x="120" y="104" width="36" height="22" rx="3"/><rect x="159" y="104" width="36" height="22" rx="3"/><rect x="198" y="104" width="36" height="22" rx="3"/>
<rect x="238" y="104" width="36" height="22" rx="3"/><rect x="277" y="104" width="36" height="22" rx="3"/><rect x="316" y="104" width="36" height="22" rx="3"/>
<rect x="356" y="104" width="36" height="22" rx="3"/><rect x="395" y="104" width="36" height="22" rx="3"/><rect x="434" y="104" width="36" height="22" rx="3"/>
<rect x="474" y="104" width="36" height="22" rx="3"/><rect x="513" y="104" width="36" height="22" rx="3"/><rect x="552" y="104" width="36" height="22" rx="3"/>
<rect x="592" y="104" width="36" height="22" rx="3"/><rect x="631" y="104" width="36" height="22" rx="3"/><rect x="670" y="104" width="36" height="22" rx="3"/>
</g>
<text x="120" y="146" font-size="12" fill="#334155">↑ 各文字の下の小さな箱が1バイト。5 文字 ＝ 15 バイト</text>
<rect x="20" y="160" width="330" height="40" rx="8" fill="#fef2f2" stroke="#dc2626"/>
<text x="34" y="185" font-size="13" fill="#b91c1c" font-family="Cascadia Mono, Consolas, monospace">title.len()           → 15（バイト数）</text>
<rect x="370" y="160" width="336" height="40" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="384" y="185" font-size="13" fill="#047857" font-family="Cascadia Mono, Consolas, monospace">title.chars().count() → 5（文字数）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図10-2: バイト数と文字数。「80文字まで」を len() で数えると、日本語は 27 文字目で上限を超えてしまう</div>
</div>

### まず動かす最小コード

#### 予想してから実行する: タイトルの文字数チェック（バグあり）

RustNote のノートのタイトルは「1文字以上80文字以下」と決めています。その検査をする関数を書いてみました。ただし、この版には**バグ**が1つ入っています。

> **▼ このコードがやること（先に日本語で）:** タイトルの文字数を数える関数と、「1〜80文字なら正しい」と判定する関数を作り、3つのタイトルで試して結果を表示します。さらに、正しく動いているかを自動で確かめる**テスト**を4つ用意します。

```rust
// タイトルの文字数を数える（この版にはバグがある）
fn title_length(title: &str) -> usize {
    title.len()
}

// 1 文字以上 80 文字以下なら true（正しいタイトル）
fn is_valid_title(title: &str) -> bool {
    let length = title_length(title);
    (1..=80).contains(&length)
}

fn main() {
    for title in ["Rust", "牛乳を買う", ""] {
        let length = title_length(title);
        let valid = is_valid_title(title);
        println!("{title:?}: 文字数 {length}, 正しい? {valid}");
    }
}

// ここから下は cargo test のときだけ使われる「テスト」
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_japanese_characters() {
        assert_eq!(title_length("牛乳を買う"), 5);
    }

    #[test]
    fn empty_title_is_invalid() {
        assert!(!is_valid_title(""));
    }

    #[test]
    fn eighty_characters_is_valid() {
        // 「あ」を 80 個並べた文字列を作る
        let title = "あ".repeat(80);
        assert!(is_valid_title(&title));
    }

    #[test]
    fn eighty_one_characters_is_invalid() {
        let title = "あ".repeat(81);
        assert!(!is_valid_title(&title));
    }
}
```

**① 予想:** `cargo run` すると、3行それぞれに何と表示されるでしょうか。特に `"牛乳を買う"` の文字数を予想してから、次に進んでください。

**② 実行:**

```powershell
cargo run -q
```

**▼ 実行結果**

```text
"Rust": 文字数 4, 正しい? true
"牛乳を買う": 文字数 15, 正しい? true
"": 文字数 0, 正しい? false
```

**③ 観察:** `"牛乳を買う"` は5文字なのに、**15** と表示されました。予想と違ったところが、学びのチャンスです。続けてテストも実行してみます。

```powershell
cargo test
```

**▼ 実行結果**

```text
   Compiling practice v0.1.0 (C:\Users\you\rust-study\practice)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.46s
     Running unittests src\main.rs (target\debug\deps\practice-7d4a8a329333a0ec.exe)

running 4 tests
test tests::eighty_one_characters_is_invalid ... ok
test tests::empty_title_is_invalid ... ok
test tests::eighty_characters_is_valid ... FAILED
test tests::counts_japanese_characters ... FAILED

failures:

---- tests::eighty_characters_is_valid stdout ----

thread 'tests::eighty_characters_is_valid' (54260) panicked at src\main.rs:39:9:
assertion failed: is_valid_title(&title)

---- tests::counts_japanese_characters stdout ----

thread 'tests::counts_japanese_characters' (3680) panicked at src\main.rs:27:9:
assertion `left == right` failed
  left: 15
 right: 5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::counts_japanese_characters
    tests::eighty_characters_is_valid

test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin practice`
```

#### ▼ コードと出力を1つずつ分解して解説

##### 解説1: バグの正体は `len()`

```rust
fn title_length(title: &str) -> usize {
    title.len()
}
```

- `len()` は文字列の **バイト数** を返します（図10-2）。英数字だけなら文字数と同じなので、`"Rust"` では正しく見えます。
- 日本語は1文字3バイトなので、`"牛乳を買う"` は 15 になりました。この関数のままだと、日本語のタイトルは **26文字（78バイト）までしか**付けられません。
- `usize` は「0 以上の整数で、長さや個数を表すのに使う型」です。

##### 解説2: テストの書き方

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_japanese_characters() {
        assert_eq!(title_length("牛乳を買う"), 5);
    }
    // ...
}
```

- `#[cfg(test)]` は「`cargo test` のときだけコンパイルする」という印です。ふだんの `cargo run` では、テストは実行ファイルに含まれません。
- `mod tests { ... }` はテストをまとめる入れ物（**モジュール**）、`use super::*;` は「外側で作った関数をここで使えるようにする」という宣言です。
- `#[test]` を付けた関数が、テスト1つ分です。
- `assert_eq!(実際の値, 期待する値)` は「2つが等しいはず。違ったらテスト失敗」、`assert!(条件)` は「条件が true のはず」という確認です。`!` は「〜でない」です。

##### 解説3: テストの結果の読み方

```text
test tests::eighty_characters_is_valid ... FAILED
test tests::counts_japanese_characters ... FAILED
```

- テストごとに `ok`（成功）か `FAILED`（失敗）が並びます。実行の順番は毎回変わることがあります（いくつかのテストを同時に動かしているため）。
- 最後の `test result: FAILED. 2 passed; 2 failed;` が集計です。

```text
thread 'tests::counts_japanese_characters' (3680) panicked at src\main.rs:27:9:
assertion `left == right` failed
  left: 15
 right: 5
```

- 失敗したテストの詳しい理由です。`src\main.rs:27:9` が失敗した `assert_eq!` の場所です。
- `left` は `assert_eq!` の**左側（実際の値）**、`right` は**右側（期待した値）**です。「15 になったが、5 を期待していた」と読みます。
- `eighty_characters_is_valid` は `assertion failed: is_valid_title(&title)`、つまり「80文字のタイトルが正しいと判定されなかった」です（240バイトなので 80 を超えた）。

##### 解説4: うまくいったテストにも意味がある

- `eighty_one_characters_is_invalid`（81文字はだめ）は、バグがあっても**たまたま**成功しています（243バイトも 80 を超えるので）。
- テストは「成功した＝正しい」とは限りません。**失敗するはずのことが、ちゃんと失敗するか**も確かめるために、境目（0文字・1文字・80文字・81文字）をいくつも用意しておくのがコツです。

### もう一歩: 実務に近い形

#### ④ 1か所だけ変えて、⑤ テストで確かめる

直すのは `title_length` の**1行だけ**です。

```rust
// タイトルの文字数を数える（文字の個数で数える）
fn title_length(title: &str) -> usize {
    title.chars().count()
}
```

ほかの部分（`is_valid_title`・`main`・テスト）は、そのままにします。

```powershell
cargo run -q
cargo test
```

**▼ 実行結果**

```text
"Rust": 文字数 4, 正しい? true
"牛乳を買う": 文字数 5, 正しい? true
"": 文字数 0, 正しい? false
```

```text
   Compiling practice v0.1.0 (C:\Users\you\rust-study\practice)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running unittests src\main.rs (target\debug\deps\practice-7d4a8a329333a0ec.exe)

running 4 tests
test tests::counts_japanese_characters ... ok
test tests::empty_title_is_invalid ... ok
test tests::eighty_one_characters_is_invalid ... ok
test tests::eighty_characters_is_valid ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `chars().count()` で文字を数える

```rust
title.chars().count()
```

- `.chars()` は文字列を**1文字ずつ**取り出します（バイトではなく文字単位）。
- `.count()` は、取り出した個数を数えます。`"牛乳を買う"` なら 5 です。
- データベースの SQLite（libSQL）の `length()` も文字数を数えるので、RustNote ではアプリ側も DB 側も「文字数で 1〜80」にそろえています。

##### 解説2: `contains` で範囲を確かめる

```rust
(1..=80).contains(&length)
```

- `1..=80` は「1 以上 80 以下」の範囲です（UNIT 01 の `for` でも使いました）。
- `.contains(&length)` は「`length` がこの範囲に入っているか」を true / false で返します。`length >= 1 && length <= 80` と同じ意味ですが、範囲が一目で分かります。

##### 解説3: テストは「直したこと」を固定する

- 今回は、テストのおかげで「直ったこと」を**4つの角度から一度に**確かめられました。
- もっと大事なのはこれからです。後で誰かが「`len()` のほうが速いのでは」と書き換えたら、`cargo test` がすぐに `left: 15, right: 5` で知らせてくれます。**一度直したバグが二度と戻らない**ようにするのがテストの役目です。
- 学習でも同じです。分かったことをテストの形で残すと、自分の理解の「記録」にもなります。

### よくあるエラーと読み方

#### テストの失敗を読む

ここまでに見た、テストの失敗の出力をまとめます。

| 出力 | 意味 | 次にすること |
|---|---|---|
| `test 名前 ... FAILED` | そのテストが失敗した | 下の `---- 名前 stdout ----` を読む |
| `panicked at src\main.rs:27:9:` | 失敗した確認の場所 | その行の `assert` を見る |
| ``assertion `left == right` failed`` と `left:` / `right:` | `assert_eq!` で値が違った | 左（実際）と右（期待）を比べる |
| `assertion failed: 式` | `assert!` の条件が false だった | その式がなぜ false か考える |
| `test result: FAILED. 2 passed; 2 failed;` | 集計 | 全部 ok になるまで直す |
| ``error: test failed, to rerun pass `--bin practice` `` | Cargo のまとめ | 気にしなくてよい（再実行の方法の案内） |

> **つまずきポイント:** テストが失敗したとき、**テストのほうを書き換えて**通そうとしたくなることがあります。たとえば `assert_eq!(title_length("牛乳を買う"), 15);` に変えれば通りますが、それは「バグを正しいことにしてしまう」直し方です。失敗したら、まず「**期待した値（right）のほうが正しいか**」を考え、正しければ**コードのほう**を直します。

### RustNote ではここで使う

- RustNote では、タイトルを `String` のままではなく **`NoteTitle`** という専用の型にして、作るときに必ず `NoteTitle::parse` で「1〜80文字か」を検査します（上限は `MAX_TITLE_CHARS = 80` という定数）。文字数は、この単元と同じ `chars().count()` で数えます。検査を通らないタイトルは、そもそも `NoteTitle` として存在できないので、「検査を忘れる」事故が起こりません（第4巻・第9巻）。
- この単元のようなテストを、第11巻ではたくさん書きます。関数のテスト、HTTP のリクエストを送ってレスポンスを確かめるテスト、「他人のノートは 404」を確かめるセキュリティのテストまで用意します。

この巻の10単元は、RustNote の**1回のリクエスト**の中に全部登場します。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 360" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-25" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="22" font-size="14" font-weight="700" fill="#0f172a" text-anchor="middle">「牛乳を買う」というノートを作成ボタンで送ったとき</text>
<rect x="14" y="36" width="216" height="62" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="122" y="60" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">① POST /notes を送る</text>
<text x="122" y="82" font-size="12" fill="#334155" text-anchor="middle">URL・HTTP・Cookie（U05）</text>
<rect x="252" y="36" width="216" height="62" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="60" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">② 8080 番に届く</text>
<text x="360" y="82" font-size="12" fill="#334155" text-anchor="middle">Fly.io・ポート（U06）</text>
<rect x="490" y="36" width="216" height="62" rx="10" fill="#f8fafc" stroke="#475569" stroke-width="2"/>
<text x="598" y="60" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">③ Rust のプロセスが処理</text>
<text x="598" y="82" font-size="12" fill="#334155" text-anchor="middle">OS・コンパイル済み（U01・U04）</text>
<rect x="490" y="134" width="216" height="62" rx="10" fill="#fdf2f8" stroke="#db2777" stroke-width="2"/>
<text x="598" y="158" font-size="13" font-weight="700" fill="#be185d" text-anchor="middle">④ 誰か調べる</text>
<text x="598" y="180" font-size="12" fill="#334155" text-anchor="middle">セッション・認証（U08）</text>
<rect x="252" y="134" width="216" height="62" rx="10" fill="#fefce8" stroke="#ca8a04" stroke-width="2"/>
<text x="360" y="158" font-size="13" font-weight="700" fill="#a16207" text-anchor="middle">⑤ タイトルを検査</text>
<text x="360" y="180" font-size="12" fill="#334155" text-anchor="middle">1〜80文字・テスト（U10）</text>
<rect x="14" y="134" width="216" height="62" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="122" y="158" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">⑥ DB に INSERT</text>
<text x="122" y="180" font-size="12" fill="#334155" text-anchor="middle">テーブル・制約（U07）</text>
<rect x="14" y="232" width="216" height="62" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="122" y="256" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">⑦ 303 で一覧へ</text>
<text x="122" y="278" font-size="12" fill="#334155" text-anchor="middle">ステータスコード（U05）</text>
<rect x="252" y="232" width="216" height="62" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="256" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">⑧ GET /notes</text>
<text x="360" y="278" font-size="12" fill="#334155" text-anchor="middle">自分のノートだけ（U08）</text>
<rect x="490" y="232" width="216" height="62" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="598" y="256" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">⑨ 一覧が表示される</text>
<text x="598" y="278" font-size="12" fill="#334155" text-anchor="middle">HTML を描く（U05）</text>
<line x1="232" y1="67" x2="248" y2="67" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-25)"/>
<line x1="470" y1="67" x2="486" y2="67" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-25)"/>
<line x1="598" y1="100" x2="598" y2="130" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-25)"/>
<line x1="488" y1="165" x2="472" y2="165" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-25)"/>
<line x1="250" y1="165" x2="234" y2="165" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-25)"/>
<line x1="122" y1="198" x2="122" y2="228" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-25)"/>
<line x1="232" y1="263" x2="248" y2="263" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-25)"/>
<line x1="470" y1="263" x2="486" y2="263" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-25)"/>
<text x="360" y="326" font-size="12" fill="#64748b" text-anchor="middle">途中でエラーが出たら U09 の読み方で、ファイルやコマンドで迷ったら U02・U03 に戻る</text>
<text x="360" y="348" font-size="12" fill="#64748b" text-anchor="middle">（U は UNIT の略）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図10-3: RustNote の1回のノート作成と、この巻の単元の対応</div>
</div>

### 練習問題

**問題1（予想してから実行）:** 「もう一歩」で直した版に、次のテストを追加して `cargo test` すると、成功するでしょうか、失敗するでしょうか。予想してから実行してください。

```rust
    #[test]
    fn spaces_only_title_is_invalid() {
        assert!(!is_valid_title("   "));
    }
```

**問題2:** 問題1で失敗した場合は、`title_length` の1行だけを変えて、5つのテストがすべて成功するようにしてください。

**問題3:** 図10-1 の学習サイクルを、この単元のどの作業が①〜⑤のどれに当たるか、対応づけて説明してください。

### 解答と解説

**問題1の解答:** **失敗します**。空白3つの `"   "` は、`chars().count()` で数えると3文字なので、「1〜80文字」の範囲に入り、正しいタイトルと判定されてしまうからです（出力の最初の3行と最後の1行は省略しています）。

```text
running 5 tests
test tests::counts_japanese_characters ... ok
test tests::eighty_characters_is_valid ... ok
test tests::eighty_one_characters_is_invalid ... ok
test tests::empty_title_is_invalid ... ok
test tests::spaces_only_title_is_invalid ... FAILED

failures:

---- tests::spaces_only_title_is_invalid stdout ----

thread 'tests::spaces_only_title_is_invalid' (53800) panicked at src\main.rs:50:9:
assertion failed: !is_valid_title("   ")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::spaces_only_title_is_invalid

test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

このように、**先に「こうあるべき」というテストを書いて、失敗するのを確かめてから直す**やり方は、実務でもよく使われます。

**問題2の解答:** 数える前に、前後の空白を `trim()` で取り除きます。

```rust
// タイトルの文字数を数える（前後の空白を取り除いてから数える）
fn title_length(title: &str) -> usize {
    title.trim().chars().count()
}
```

**▼ 実行結果**（`cargo test`）

```text
running 5 tests
test tests::counts_japanese_characters ... ok
test tests::eighty_characters_is_valid ... ok
test tests::eighty_one_characters_is_invalid ... ok
test tests::empty_title_is_invalid ... ok
test tests::spaces_only_title_is_invalid ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

なお、空白だけのタイトルを認めるかどうかは、アプリの**決めごと（仕様）**です。ここでは「認めない」と決めた場合の練習です。RustNote での扱いは第9巻で要件として決めます。

**問題3の解答（例）:** ①「`"牛乳を買う"` は 5 と出るはず」と予想した → ② `cargo run` と `cargo test` を実行した → ③ 15 と表示され、テストが2つ失敗したのを見た → ④ `len()` を `chars().count()` の1か所だけ変えた → ⑤「`len()` はバイト数、`chars().count()` は文字数」と説明できるようになり、テストとして残した。問題1・2は、2周目のサイクルです。

### まとめ

- 「予想 → 実行 → 観察 → 1か所変える → 説明・記録」を回すほど、理解が深くなります。
- `len()` は**バイト数**、`chars().count()` は**文字数**です。日本語を扱うときは特に注意します。
- テストは、分かったこと・直したことを**固定する記録**です。失敗したら `left`（実際）と `right`（期待）を比べ、まずコードを疑います。

## この先の学び方

### 巻の順番

基本は第0巻から第13巻まで、**番号順**に読みます。それぞれの巻が、前の巻の知識を使うように作られているからです。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 72 720 176" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-26" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<line x1="30" y1="120" x2="700" y2="120" stroke="#475569" stroke-width="3" marker-end="url(#ar-00-26)"/>
<circle cx="70" cy="120" r="9" fill="#f97316"/>
<circle cx="200" cy="120" r="9" fill="#3b82f6"/>
<circle cx="330" cy="120" r="9" fill="#3b82f6"/>
<circle cx="450" cy="120" r="9" fill="#f97316"/>
<circle cx="560" cy="120" r="9" fill="#10b981"/>
<circle cx="660" cy="120" r="9" fill="#10b981"/>
<text x="70" y="96" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">第0・1巻</text>
<text x="70" y="150" font-size="12" fill="#334155" text-anchor="middle">hello が動く</text>
<text x="70" y="168" font-size="12" fill="#334155" text-anchor="middle">道具がそろう</text>
<text x="200" y="96" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">第2〜3巻</text>
<text x="200" y="150" font-size="12" fill="#334155" text-anchor="middle">小さなプログラム</text>
<text x="200" y="168" font-size="12" fill="#334155" text-anchor="middle">が書ける</text>
<text x="330" y="96" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">第4〜5巻</text>
<text x="330" y="150" font-size="12" fill="#334155" text-anchor="middle">大きめのプログラム</text>
<text x="330" y="168" font-size="12" fill="#334155" text-anchor="middle">を組み立てられる</text>
<text x="450" y="96" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">第6〜8巻</text>
<text x="450" y="150" font-size="12" fill="#334155" text-anchor="middle">ページ・DB・</text>
<text x="450" y="168" font-size="12" fill="#334155" text-anchor="middle">ログインが作れる</text>
<text x="560" y="96" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">第9〜11巻</text>
<text x="560" y="150" font-size="12" fill="#334155" text-anchor="middle">RustNote が</text>
<text x="560" y="168" font-size="12" fill="#334155" text-anchor="middle">手元で完成</text>
<text x="660" y="96" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">第12〜13巻</text>
<text x="660" y="150" font-size="12" fill="#334155" text-anchor="middle">世界に公開</text>
<text x="660" y="168" font-size="12" fill="#334155" text-anchor="middle">全体を読み通す</text>
<rect x="150" y="196" width="420" height="40" rx="8" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="360" y="221" font-size="12" fill="#334155" text-anchor="middle">第3巻（所有権）は難所。1回で分からなくても先へ進み、後で戻ってよい</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図10-4: 全14巻のマイルストーン。各段階で「できるようになること」</div>
</div>

- **第1巻（環境構築）は飛ばさない**でください。道具の版がそろっていないと、教材と違うエラーが出て、原因探しに時間を取られます。
- **第3巻（所有権・借用）は、多くの人がつまずく難所**です。1回で全部分からなくても大丈夫です。第4巻以降で何度も出てくるうちに分かってくるので、7割分かったら先へ進み、後で戻ってきましょう。
- 第13巻には RustNote の**完全なコード**が載っています。第9〜10巻で手が止まったら、答え合わせに使ってかまいません。ただし、まず自分で書いてから見るほうが、ずっと身に付きます。

### つまずいたときの調べ方

困ったときは、次の順番で調べると、たいていの問題は自分で解決できます。

1. **エラーメッセージを最初から読む** … UNIT 09 の読み方で、`-->` の場所と `help:` を確かめる。
2. **今いる場所と版を確かめる** … `Get-Location`（UNIT 02・03）、`cargo --version`。教材と同じ版か。
3. **最小の形にする** … 動かないコードから、関係なさそうな行を1つずつ消して、エラーが出る最小のコードを作る。これだけで原因が見えることが多い。
4. **公式の説明を読む** … `rustc --explain エラー番号`、Rust の公式の入門書「The Rust Programming Language」（日本語訳もあります）、クレートの説明サイト docs.rs、Topcoat の公式ドキュメント。
5. **エラー全文で検索する** … エラーの1行目（`error[E0308]: mismatched types` など）をそのまま検索する。自分のユーザー名などは消してから。
6. **人や AI に聞く** … 下の「AI の上手な使い方」のとおりに聞く。

> **ヒント:** 調べて解決したことは、短いメモに残しておきましょう。「何をしたら・どんなエラーが出て・何が原因で・どう直したか」の4行で十分です。同じ問題に2回目に出会ったとき、そのメモが一番の味方になります。

### Claude Code など AI の上手な使い方

Claude Code（クロード コード）のような AI のアシスタントは、Rust の学習でもとても頼りになります。ただし、**使い方しだいで、上達を大きく助けることも、逆に妨げることもあります**。

**上手な聞き方:** 次の4つをそろえて伝えると、的確な答えが返ってきやすくなります。

```text
【やりたいこと】RustNote のタイトルを 1〜80 文字に制限したい
【したこと】title.len() で長さを調べて、80 以下なら OK にした
【起きたこと】日本語のタイトルが 27 文字で「長すぎる」と言われる
【聞きたいこと】原因と、正しい数え方を、理由も含めて教えてください
```

- **エラーは全文を貼る** … 「エラーが出ました」だけでは、AI も推測するしかありません。`error[...]` から最後の行まで、そのまま貼ります。
- **「直して」より「なぜ？」と聞く** … 直ったコードだけをもらうと、次に同じ問題が起きたとき自分で直せません。「なぜこのエラーになるのか」「なぜその直し方でよいのか」を説明してもらいましょう。
- **答えは必ず自分で動かして確かめる** … AI の答えは、もっともらしいのに間違っていることがあります。特に Topcoat のような新しいフレームワークは、古い版の書き方を答えてしまうことがあります。`cargo check`・`cargo test` で確かめ、教材や公式ドキュメントと見比べます。
- **まず自分で考えてから聞く** … UNIT 10 のサイクルの①予想と③観察を自分でしてから聞くと、答えの意味がよく分かります。
- **秘密を貼らない** … パスワード、API キー、`.env` ファイルの中身、本物の利用者のデータは、AI にも検索にも貼りません。

> **注意:** AI に書いてもらったコードを、意味が分からないまま RustNote に入れるのはやめましょう。認証や認可（UNIT 08）のような安全に関わる部分は特にです。「このコードを1行ずつ説明して」と頼み、説明できるようになってから使います。

### この巻のあとにすること

この巻はこれで終わりです。次は**第1巻（環境構築）**で、Windows に PowerShell・Git・VS Code・Rust などの道具をそろえます。道具がそろったら、この巻に戻ってきて、各単元のコードを**実際に動かしながら**もう一度読んでみてください。1回目には見えなかったことが、きっと見えてきます。

