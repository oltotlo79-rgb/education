## UNIT 05: String・str・UTF-8

### この単元のゴール

- 書き足せる文字列 `String` と、文字列を借りて読む `&str` の違いを説明できる
- 関数の引数は `&str` で受け取る、という基本形を使える
- 日本語の文字列で `len()`（バイト数）と `chars().count()`（文字数）が違う理由を UTF-8 で説明できる
- `s[0]` が書けない理由を知り、`chars()` や `get` で安全に文字を取り出せる

### たとえ話でつかむ

- **`String`** は「**自分のノート**」です。自分が持ち主なので、好きなだけページを書き足したり、消したりできます。
- **`&str`**（エスアンド・ストル、または「ストル参照」）は、ノートの一部を囲んだ「**のぞき窓**」です。窓からは中身を読めますが、窓は持ち主ではないので書き換えられません。ノート全体を囲むことも、1行だけを囲むこともできます。
- 文字列リテラル `"こんにちは"` は、「**本に印刷された文**」です。プログラムに最初から埋め込まれていて書き換えられないので、`&str`（のぞき窓）として扱います。

もう1つ、UTF-8（ユーティーエフエイト：文字をバイトの並びに変換する決まり）のたとえです。

- 文字列は、中では**バイト（0〜255 の数）の列**として保存されています。英字は1文字1バイトですが、日本語のひらがな・漢字は1文字3バイト、絵文字は4バイトを使います。
- 「1文字＝1マス」ではなく「**文字によって使うマス数が違う原稿用紙**」なので、「3マス目」が文字の途中ということが起こります。だから Rust は `s[0]` のような「○マス目の文字」という取り出し方をさせません。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-07" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="110" y="26" text-anchor="middle" font-size="14" font-weight="700" fill="#0f172a">スタック（変数そのもの）</text>
<text x="490" y="26" text-anchor="middle" font-size="14" font-weight="700" fill="#0f172a">ヒープ（文字のバイト本体）</text>
<text x="40" y="52" font-size="13" font-weight="700" fill="#1e40af">s: String</text>
<rect x="40" y="60" width="140" height="30" fill="#eff6ff" stroke="#1e40af"/>
<text x="110" y="80" text-anchor="middle" font-size="13" fill="#1e40af">ptr（本体の場所）</text>
<rect x="40" y="90" width="140" height="30" fill="#eff6ff" stroke="#1e40af"/>
<text x="110" y="110" text-anchor="middle" font-size="13" fill="#1e40af">len 10</text>
<rect x="40" y="120" width="140" height="30" fill="#eff6ff" stroke="#1e40af"/>
<text x="110" y="140" text-anchor="middle" font-size="13" fill="#1e40af">cap 10</text>
<text x="40" y="182" font-size="13" font-weight="700" fill="#c2410c">t: &amp;str = &amp;s[0..4]</text>
<rect x="40" y="190" width="140" height="30" fill="#fff7ed" stroke="#c2410c"/>
<text x="110" y="210" text-anchor="middle" font-size="13" fill="#c2410c">ptr（のぞく場所）</text>
<rect x="40" y="220" width="140" height="30" fill="#fff7ed" stroke="#c2410c"/>
<text x="110" y="240" text-anchor="middle" font-size="13" fill="#c2410c">len 4</text>
<rect x="310" y="110" width="36" height="40" fill="#ffffff" stroke="#475569"/>
<text x="328" y="136" text-anchor="middle" font-size="14" fill="#0f172a">R</text>
<rect x="346" y="110" width="36" height="40" fill="#ffffff" stroke="#475569"/>
<text x="364" y="136" text-anchor="middle" font-size="14" fill="#0f172a">u</text>
<rect x="382" y="110" width="36" height="40" fill="#ffffff" stroke="#475569"/>
<text x="400" y="136" text-anchor="middle" font-size="14" fill="#0f172a">s</text>
<rect x="418" y="110" width="36" height="40" fill="#ffffff" stroke="#475569"/>
<text x="436" y="136" text-anchor="middle" font-size="14" fill="#0f172a">t</text>
<rect x="454" y="110" width="108" height="40" fill="#f8fafc" stroke="#475569"/>
<line x1="490" y1="110" x2="490" y2="150" stroke="#cbd5e1" stroke-dasharray="3 3"/>
<line x1="526" y1="110" x2="526" y2="150" stroke="#cbd5e1" stroke-dasharray="3 3"/>
<text x="508" y="136" text-anchor="middle" font-size="15" fill="#0f172a">入</text>
<rect x="562" y="110" width="108" height="40" fill="#f8fafc" stroke="#475569"/>
<line x1="598" y1="110" x2="598" y2="150" stroke="#cbd5e1" stroke-dasharray="3 3"/>
<line x1="634" y1="110" x2="634" y2="150" stroke="#cbd5e1" stroke-dasharray="3 3"/>
<text x="616" y="136" text-anchor="middle" font-size="15" fill="#0f172a">門</text>
<path d="M310,100 L310,90 L670,90 L670,100" fill="none" stroke="#1e40af" stroke-width="2"/>
<text x="490" y="82" text-anchor="middle" font-size="12" fill="#1e40af">s が持っている 10 バイト（"Rust入門"）</text>
<path d="M310,160 L310,170 L454,170 L454,160" fill="none" stroke="#c2410c" stroke-width="2"/>
<text x="382" y="188" text-anchor="middle" font-size="12" fill="#c2410c">t がのぞく 4 バイト（"Rust"）</text>
<line x1="180" y1="75" x2="304" y2="118" stroke="#1e40af" stroke-width="2" marker-end="url(#ar-02-07)"/>
<line x1="180" y1="205" x2="304" y2="143" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-02-07)"/>
<text x="490" y="236" text-anchor="middle" font-size="13" fill="#334155">String = 持ち主。書き足すと本体を広げる（cap = 確保済みの広さ）</text>
<text x="490" y="260" text-anchor="middle" font-size="13" fill="#334155">&amp;str = のぞき窓（開始位置と長さだけ）。書き換えはできない</text>
<text x="490" y="284" text-anchor="middle" font-size="12" fill="#64748b">"Rust入門" は 4 + 3 + 3 = 10 バイト</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図5-1: String は本体（ヒープ上のバイト列）の持ち主、&amp;str はその一部をのぞく窓</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="360" y="24" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">"aあ🦀" を UTF-8 で保存すると 8 バイトになる</text>
<text x="40" y="70" text-anchor="end" font-size="12" fill="#64748b">文字</text>
<rect x="80" y="44" width="70" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="115" y="70" text-anchor="middle" font-size="16" fill="#1e40af">a</text>
<rect x="150" y="44" width="210" height="40" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="255" y="70" text-anchor="middle" font-size="16" fill="#c2410c">あ</text>
<rect x="360" y="44" width="280" height="40" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="500" y="71" text-anchor="middle" font-size="18" fill="#047857">🦀</text>
<text x="40" y="126" text-anchor="end" font-size="12" fill="#64748b">バイト</text>
<rect x="80" y="100" width="70" height="40" fill="#eff6ff" stroke="#3b82f6"/>
<text x="115" y="126" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#1e40af">61</text>
<rect x="150" y="100" width="70" height="40" fill="#fff7ed" stroke="#f97316"/>
<text x="185" y="126" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#c2410c">E3</text>
<rect x="220" y="100" width="70" height="40" fill="#fff7ed" stroke="#f97316"/>
<text x="255" y="126" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#c2410c">81</text>
<rect x="290" y="100" width="70" height="40" fill="#fff7ed" stroke="#f97316"/>
<text x="325" y="126" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#c2410c">82</text>
<rect x="360" y="100" width="70" height="40" fill="#ecfdf5" stroke="#10b981"/>
<text x="395" y="126" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#047857">F0</text>
<rect x="430" y="100" width="70" height="40" fill="#ecfdf5" stroke="#10b981"/>
<text x="465" y="126" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#047857">9F</text>
<rect x="500" y="100" width="70" height="40" fill="#ecfdf5" stroke="#10b981"/>
<text x="535" y="126" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#047857">A6</text>
<rect x="570" y="100" width="70" height="40" fill="#ecfdf5" stroke="#10b981"/>
<text x="605" y="126" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#047857">80</text>
<text x="40" y="162" text-anchor="end" font-size="12" fill="#64748b">位置</text>
<text x="115" y="162" text-anchor="middle" font-size="12" fill="#64748b">0</text>
<text x="185" y="162" text-anchor="middle" font-size="12" fill="#64748b">1</text>
<text x="255" y="162" text-anchor="middle" font-size="12" fill="#64748b">2</text>
<text x="325" y="162" text-anchor="middle" font-size="12" fill="#64748b">3</text>
<text x="395" y="162" text-anchor="middle" font-size="12" fill="#64748b">4</text>
<text x="465" y="162" text-anchor="middle" font-size="12" fill="#64748b">5</text>
<text x="535" y="162" text-anchor="middle" font-size="12" fill="#64748b">6</text>
<text x="605" y="162" text-anchor="middle" font-size="12" fill="#64748b">7</text>
<rect x="80" y="180" width="270" height="56" rx="8" fill="#f8fafc" stroke="#94a3b8"/>
<text x="215" y="204" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#0f172a">len() = 8</text>
<text x="215" y="224" text-anchor="middle" font-size="12" fill="#64748b">バイトの数</text>
<rect x="370" y="180" width="270" height="56" rx="8" fill="#f8fafc" stroke="#94a3b8"/>
<text x="505" y="204" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#0f172a">chars().count() = 3</text>
<text x="505" y="224" text-anchor="middle" font-size="12" fill="#64748b">文字の数</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図5-2: 英字は1バイト、ひらがなは3バイト、絵文字は4バイト。位置1〜3は「あ」の途中なので、そこでは切れない</div>
</div>

| | `String` | `&str` |
|---|---|---|
| 役割 | 文字列の**持ち主** | 文字列を**借りて読む窓** |
| 書き足し・変更 | できる（`mut` が必要） | できない |
| 作り方の例 | `String::from("..")`、`"..".to_string()`、`format!(..)` | `"..."`（リテラル）、`&s`、`&s[0..4]` |
| よく使う場所 | 構造体のフィールド、関数の戻り値 | **関数の引数** |

### まず動かす最小コード

#### 例1: String を作って書き足す

> **▼ このコードがやること（先に日本語で）:** 文字列リテラルから `String` を作り、後ろに文字列や1文字を書き足して表示します。`format!` と `+` で文字列をつなぐ方法も試します。

```rust
@@CODE u05/a_string.rs@@
```

```powershell
cargo run
```

**▼ 実行結果**

```text
@@OUT u05/a_string@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: リテラルは `&str`、作ったものは `String`

```rust
let greeting: &str = "こんにちは";
let mut message = String::from(greeting);
```

- `"こんにちは"` はプログラムの中に埋め込まれた文字列で、型は `&str` です。書き換えはできません。
- `String::from(...)` で、その内容をコピーした**自分の** `String` を作ります。書き足すので `mut` を付けています。

##### 解説2: `push_str` と `push`

```rust
message.push_str("、Rust");
message.push('！');
```

- `push_str` は文字列（`&str`）を、`push` は1文字（`char`、シングルクォート）を末尾に足します。
- 足すたびに、`String` は必要なら本体の置き場所を広げます（図5-1 の `cap`）。

##### 解説3: `format!` と `+`

```rust
let line = format!("{name}さん、{message}");
let b = a + "入門";
```

- `format!` は `println!` と同じ書き方で、表示する代わりに**新しい `String` を作ります**。いくつもの値を組み合わせるときは、これがいちばん読みやすい方法です。
- `+` は「左の `String` に右の `&str` を書き足したもの」を返します。このとき左の `a` は**使い切られて**、以後は使えなくなります（理由は第3巻の「所有権」で学びます）。迷ったら `format!` を使いましょう。

#### 例2: バイト数と文字数

> **▼ このコードがやること（先に日本語で）:** 英字・ひらがな・絵文字が1つずつ入った文字列で、「バイト数」と「文字数」を比べます。1文字ずつ、何バイトを使っているかも表示します。

```rust
@@CODE u05/b_len_chars.rs@@
```

**▼ 実行結果**

```text
@@OUT u05/b_len_chars@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `len()` はバイト数

- `len()` は**バイト数**を返します。`a`(1) + `あ`(3) + `🦀`(4) = 8 です（図5-2）。
- 「文字数」が欲しいときは `chars().count()` を使います。`chars()` は文字列を `char`（1文字）の並びとして取り出し、`count()` はその個数を数えます。

##### 解説2: `{:02X?}` でバイトを見る

- `as_bytes()` は、文字列の中身をバイトの並びとして見せます。`{:02X?}` は「16進数・大文字・2桁で、並びとして表示する」書式です。
- `E3 81 82` が「あ」、`F0 9F A6 80` が「🦀」（Rust のマスコット、カニの Ferris）です。

> **用語:** **UTF-8** … Unicode の文字番号を、1〜4バイトのバイト列に変換する決まり。英数字は ASCII（昔からある英語用の文字コード）と同じ1バイトなので、互換性が高く、Web やファイルの標準になっています。Rust の `String` と `&str` は**必ず正しい UTF-8** であることが保証されています。

#### 例3: 関数の引数は &str で受け取る

> **▼ このコードがやること（先に日本語で）:** 文字列を大文字にして「！」を付ける関数を作り、`String` と文字列リテラルの両方から呼び出します。

```rust
@@CODE u05/c_str_param.rs@@
```

**▼ 実行結果**

```text
@@OUT u05/c_str_param@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `&str` で受け取ると、どちらからも呼べる

```rust
fn shout(text: &str) -> String {
```

- 引数を `&str` にしておくと、`String` は `&owned` と書いて、リテラルはそのまま渡せます。`String` を `&` で貸すと、自動で `&str` として扱われます。
- 逆に引数を `String` にすると、リテラルを渡すたびに `.to_string()` が必要になり、呼び出し側の `String` も手放すことになります。**読むだけの関数は `&str` で受け取る**のが基本です。
- 戻り値は新しく作った文字列なので `String` です。「受け取りは `&str`、作って返すなら `String`」と覚えましょう。

#### 例4: スライスで一部分を取り出す

> **▼ このコードがやること（先に日本語で）:** 英字と日本語の文字列から、先頭の一部分を取り出します。バイトの位置で切る方法と、文字数で切る方法、切ってよい位置かを確かめる方法を比べます。

```rust
@@CODE u05/d_slice.rs@@
```

**▼ 実行結果**

```text
@@OUT u05/d_slice@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `&s[開始..終了]` の数字はバイト

- `&ascii[0..4]` は「0バイト目から4バイト目の手前まで」で、英字なら4文字です。
- 日本語は1文字3バイトなので、2文字欲しいなら `0..6` と書く必要があります。**文字の途中の位置を指定すると panic します**（下の「よくあるエラー」）。

##### 解説2: 文字数で切るなら `chars().take(n)`

```rust
let first_three: String = japanese.chars().take(3).collect();
```

- `chars()` で1文字ずつにして、`take(3)` で先頭3文字だけを取り、`collect()` で `String` に集めます。どんな文字でも安全です。
- `collect()` は「何に集めるか」を型注釈（`: String`）から読み取ります。

##### 解説3: `get` は失敗を `None` で返す

- `japanese.get(0..2)` は、2バイト目が文字の途中なので `None`（取り出せない）を返しました。`0..3` なら `Some("日")` です。
- 外から来た位置で切り出すときは、panic しない `get` を使うと安全です。

#### 例5: よく使う文字列メソッド

> **▼ このコードがやること（先に日本語で）:** カンマ区切りのタグの文字列を、空白の除去・小文字化・検索・置き換え・分割で加工します。最後に改行を含む文字列の行数を数えます。

```rust
@@CODE u05/e_methods.rs@@
```

**▼ 実行結果**

```text
@@OUT u05/e_methods@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 元の文字列は変わらない

- `trim` `to_lowercase` `replace` などは、**元の文字列を書き換えずに**、結果を新しく返します。`raw` は最後まで最初のままです。
- `trim` は `&str`（元の一部をのぞく窓）を、`to_lowercase` と `replace` は新しい `String` を返します。

##### 解説2: `split` と `lines`

- `split(',')` は `,` で区切った部分を順に取り出します。`"  DB  "` のように各部分に空白が残るので、`trim()` と組み合わせるのが定番です。
- `lines()` は改行で区切ります。`"\n"` は文字列の中で改行を表す書き方（エスケープ）です。

| メソッド | 何をするか | 結果の型 |
|---|---|---|
| `trim()` | 前後の空白を除く | `&str` |
| `to_lowercase()` / `to_uppercase()` | 小文字 / 大文字にする | `String` |
| `contains(x)` / `starts_with(x)` / `ends_with(x)` | 含むか / 始まるか / 終わるか | `bool` |
| `replace(a, b)` | a を b に置き換える | `String` |
| `split(x)` / `lines()` / `split_whitespace()` | 区切って順に取り出す | 部分の並び |
| `parse::<T>()` | 数などへ変換する | 成功か失敗（UNIT 09） |
| `chars()` / `chars().count()` | 1文字ずつ / 文字数 | 文字の並び / `usize` |

### もう一歩: 実務に近い形

一覧画面では、長いタイトルを省略して表示したり、入力された余分な空白をまとめたりします。どちらも「文字数」で考えるのがポイントです。

#### タイトルの省略表示と空白の整理

> **▼ このコードがやること（先に日本語で）:** タイトルが10文字を超えていたら10文字で切って「…」を付けます。また、全角スペースやタブを含む連続した空白を、半角スペース1つにまとめます。

```rust
@@CODE u05/f_preview.rs@@
```

**▼ 実行結果**

```text
@@OUT u05/f_preview@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 文字数で比べ、文字数で切る

```rust
if title.chars().count() <= max {
    return title.to_string();
}
let head: String = title.chars().take(max).collect();
```

- `len()` で比べたり `&title[0..max]` で切ったりすると、日本語では文字の途中で切れて panic します。**比べるのも切るのも `chars()`** にそろえます。
- 短いときは早期リターンで、そのまま `String` にして返しています。

##### 解説2: `split_whitespace` と `join`

```rust
text.split_whitespace().collect::<Vec<&str>>().join(" ")
```

- `split_whitespace()` は、半角・全角スペースやタブなど**あらゆる空白**で区切り、空の部分は捨てます。
- `collect::<Vec<&str>>()` で部分を `Vec`（UNIT 06）に集め、`join(" ")` で半角スペースをはさんで1つの `String` につなぎます。`::<...>` は「何に集めるか」を直接指定する書き方です。

### よくあるエラーと読み方

#### エラー1: E0277 文字列に s[0] と書いた

```rust
@@CODE u05/err1_index.rs@@
```

```text
@@ERR u05/err1_index@@
```

- **読み方:** 「`str` 型は整数で添え字アクセスできません」。`string indices are ranges of usize` は「文字列の添え字は（1つの数ではなく）範囲です」、`note: you can use .chars().nth() or .bytes().nth()` は「`.chars().nth()` か `.bytes().nth()` を使えます」という提案です。
- **何が起きたか:** UTF-8 では「0番目」が1バイトなのか文字なのかがあいまいで、日本語なら文字の途中にもなり得ます。そこで Rust は `s[0]` を禁止しています。
- **直し方:** 最初の文字が欲しいなら `s.chars().next()`、n番目なら `s.chars().nth(n)` を使います。結果は「ないかもしれない」ので `Option<char>` です（UNIT 08）。

#### エラー2: 文字の途中でスライスして panic

```rust
@@CODE u05/err2_slice_panic.rs@@
```

**▼ 実行結果**

```text
@@RUN u05/err2_slice_panic@@
```

- **読み方:** `end byte index 2 is not a char boundary; it is inside '日' (bytes 0..3 of string)` は「終わりの位置 2 は文字の区切りではありません。'日'（0〜3バイト目）の途中です」。
- **何が起きたか:** コンパイルは通りますが、**実行したときに** panic します。位置が実行中に決まることもあるので、コンパイラには見つけられません。
- **直し方:** 文字数で切るなら `title.chars().take(2).collect::<String>()`、バイト位置が正しいか分からないなら `title.get(0..2)`（`None` が返る）を使います。

#### エラー3: E0308 String が欲しいところに &str を渡した

```rust
@@CODE u05/err3_string.rs@@
```

```text
@@ERR u05/err3_string@@
```

- **読み方:** `expected String, found &str`。型注釈で `String` と決めたのに、リテラル（`&str`）を入れようとしました。
- **直し方:** `help` のとおり `"Rust".to_string()`（または `String::from("Rust")`）で `String` を作ります。

#### エラー4: E0369 &str どうしを + でつないだ

```rust
@@CODE u05/err4_add.rs@@
```

```text
@@ERR u05/err4_add@@
```

- **読み方:** `cannot add &str to &str` と `note: string concatenation requires an owned String on the left`（文字列の連結には、左側に持ち主の `String` が必要）。
- **直し方:** `help` の `.to_owned()`（`to_string()` と同じく `String` を作る）でもよいですが、`format!("{}{}", "Rust", "入門")` と書くほうが読みやすくおすすめです。

### RustNote ではここで使う

RustNote のタイトルは「1〜80**文字**」です。ここで `len()` を使うと、日本語のタイトルは1文字3バイトなので、**27文字（81バイト）になった時点で拒否されてしまいます**。必ず `chars().count()` で数えます。

> **▼ このコードがやること（先に日本語で）:** 「あ」を80文字並べたタイトル（ちょうど上限）について、`len()` と `chars().count()` で判定結果がどう違うかを確かめます。

```rust
@@CODE u05/g_rustnote.rs@@
```

**▼ 実行結果**

```text
@@OUT u05/g_rustnote@@
```

- `len()` で判定すると、規則上は正しい80文字のタイトルを「長すぎる」と誤って拒否してしまいます。
- RustNote のデータベース（libSQL / SQLite）でも `CHECK (length(title) BETWEEN 1 AND 80)` のように文字数を制限します。SQLite の `length()` は文字列の**文字数**を数えるので、Rust 側の `chars().count()` と同じ基準になります。画面・Rust・データベースで数え方をそろえることが大切です。

> **注意:** `chars().count()` が数えるのは Unicode の「文字番号」の数です。家族の絵文字のように、見た目は1文字でも複数の番号でできている文字があります。

```rust
@@CODE u05/h_emoji.rs@@
```

**▼ 実行結果**

```text
@@OUT u05/h_emoji@@
```

> 見た目どおりの「1文字」（書記素クラスタ）で数えるには `unicode-segmentation` という外部クレートが必要です。RustNote では、データベースの `length()` とそろえるために `chars().count()` を採用します。

### 練習問題

**問題1.** `"こんにちは"` と `"Hello"` のそれぞれについて、バイト数と文字数を表示してください。

**問題2.** 文字列の中に、指定した文字が何回出てくるかを数える関数 `fn count_char(text: &str, target: char) -> usize` を、`for` と `chars()` を使って書いてください。

**問題3.** `"Rust入門"` を逆順にした文字列 `門入tsuR` を作って表示してください（ヒント: `chars()`、`.rev()`、`collect()`）。

### 解答と解説

**解答1.**

```rust
@@CODE u05/ex1.rs@@
```

**▼ 実行結果**

```text
@@OUT u05/ex1@@
```

- ひらがなは1文字3バイトなので 5 × 3 = 15 バイトです。英字は文字数とバイト数が一致します。

**解答2.**

```rust
@@CODE u05/ex2.rs@@
```

**▼ 実行結果**

```text
@@OUT u05/ex2@@
```

- `char` どうしは `==` で比べられます。数え上げる変数 `count` だけを `mut` にしています。UNIT 10 では、同じ処理を `text.chars().filter(|c| *c == target).count()` と1行で書く方法を学びます。

**解答3.**

```rust
@@CODE u05/ex3.rs@@
```

**▼ 実行結果**

```text
@@OUT u05/ex3@@
```

- バイト単位で逆順にすると日本語が壊れますが、`chars()` で文字単位にしてから `.rev()` で逆順にすれば安全です。

### まとめ

- `String` は書き足せる文字列の持ち主、`&str` はそれを借りて読む窓。読むだけの関数の引数は `&str`、作って返すなら `String`。
- 文字列は UTF-8 のバイト列。日本語は1文字3バイトなので、`len()`（バイト数）と `chars().count()`（文字数）は違う。
- `s[0]` は書けない。文字は `chars()` で取り出し、切り出しは文字数（`chars().take(n)`）か、安全な `get` で行う。
