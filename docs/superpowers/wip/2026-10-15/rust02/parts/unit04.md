## UNIT 04: if・loop・while・for・match

### この単元のゴール

- `if` / `else if` / `else` で条件分岐を書け、`if` を式として値を受け取れる
- `loop` / `while` / `for` の3つのくり返しを使い分けられ、`break` と `continue` を使える
- `match` で値を「形」ごとに振り分けられ、範囲・`|`・`_`・ガードを使える
- `match` が**すべての場合を網羅しているか**をコンパイラが調べる理由を説明できる

### たとえ話でつかむ

- **if** は「分かれ道の標識」です。「80点以上ならこっち、60点以上ならあっち、それ以外は向こう」と、上から順に標識を見て最初に合った道へ進みます。
- **loop** は「回転寿司のレーン」です。誰かが止める（`break`）まで、ずっと回り続けます。
- **while** は「お腹が空いている間は食べ続ける」です。毎回、食べる前に条件（まだ空いている？）を確かめます。
- **for** は「出席簿を上から順に読み上げる」です。名簿（範囲や配列）の最後まで来たら自然に終わります。
- **match** は「郵便局の仕分け棚」です。手紙（値）を宛先ごとの棚（腕）に振り分けます。**どの棚にも入らない手紙があってはいけない**ので、Rust は「全部の宛先に棚がありますか？」をコンパイル時に確かめます。棚が足りないときは「その他」の棚（`_`）を用意します。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-05" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="10" width="225" height="280" rx="12" fill="#fff7ed" stroke="#f97316"/>
<text x="122" y="38" text-anchor="middle" font-size="16" font-weight="700" fill="#c2410c">loop</text>
<rect x="47" y="70" width="150" height="40" rx="8" fill="#ffffff" stroke="#c2410c"/>
<text x="122" y="95" text-anchor="middle" font-size="13" fill="#0f172a">中身を実行</text>
<polygon points="122,140 192,170 122,200 52,170" fill="#ffffff" stroke="#c2410c"/>
<text x="122" y="175" text-anchor="middle" font-size="12" fill="#0f172a">break した？</text>
<line x1="122" y1="110" x2="122" y2="136" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-05)"/>
<path d="M52,170 L30,170 L30,90 L43,90" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-05)"/>
<text x="40" y="190" font-size="12" fill="#64748b">いいえ</text>
<line x1="122" y1="200" x2="122" y2="236" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-05)"/>
<text x="130" y="222" font-size="12" fill="#64748b">はい</text>
<text x="122" y="258" text-anchor="middle" font-size="13" fill="#c2410c">抜ける（値も返せる）</text>
<text x="122" y="280" text-anchor="middle" font-size="12" fill="#64748b">回数が決まっていないとき</text>
<rect x="247" y="10" width="225" height="280" rx="12" fill="#eff6ff" stroke="#3b82f6"/>
<text x="360" y="38" text-anchor="middle" font-size="16" font-weight="700" fill="#1e40af">while 条件</text>
<polygon points="360,60 430,90 360,120 290,90" fill="#ffffff" stroke="#1e40af"/>
<text x="360" y="95" text-anchor="middle" font-size="12" fill="#0f172a">条件は true？</text>
<line x1="360" y1="120" x2="360" y2="146" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-05)"/>
<text x="368" y="138" font-size="12" fill="#64748b">はい</text>
<rect x="285" y="150" width="150" height="40" rx="8" fill="#ffffff" stroke="#1e40af"/>
<text x="360" y="175" text-anchor="middle" font-size="13" fill="#0f172a">中身を実行</text>
<path d="M285,170 L265,170 L265,90 L286,90" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-05)"/>
<line x1="430" y1="90" x2="455" y2="90" stroke="#475569" stroke-width="2"/>
<line x1="455" y1="90" x2="455" y2="236" stroke="#475569" stroke-width="2"/>
<line x1="455" y1="236" x2="420" y2="236" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-05)"/>
<text x="440" y="80" font-size="12" fill="#64748b">いいえ</text>
<text x="360" y="242" text-anchor="middle" font-size="13" fill="#1e40af">抜ける</text>
<text x="360" y="280" text-anchor="middle" font-size="12" fill="#64748b">条件で止めたいとき</text>
<rect x="484" y="10" width="226" height="280" rx="12" fill="#ecfdf5" stroke="#10b981"/>
<text x="597" y="38" text-anchor="middle" font-size="16" font-weight="700" fill="#047857">for x in 並び</text>
<rect x="502" y="62" width="40" height="30" rx="4" fill="#ffffff" stroke="#047857"/>
<text x="522" y="82" text-anchor="middle" font-size="13" fill="#0f172a">1</text>
<rect x="552" y="62" width="40" height="30" rx="4" fill="#ffffff" stroke="#047857"/>
<text x="572" y="82" text-anchor="middle" font-size="13" fill="#0f172a">2</text>
<rect x="602" y="62" width="40" height="30" rx="4" fill="#ffffff" stroke="#047857"/>
<text x="622" y="82" text-anchor="middle" font-size="13" fill="#0f172a">3</text>
<rect x="652" y="62" width="40" height="30" rx="4" fill="#f8fafc" stroke="#94a3b8" stroke-dasharray="4 3"/>
<text x="672" y="82" text-anchor="middle" font-size="12" fill="#94a3b8">終</text>
<line x1="522" y1="92" x2="522" y2="146" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-05)"/>
<text x="530" y="124" font-size="12" fill="#64748b">1つ取り出して x に</text>
<rect x="502" y="150" width="190" height="40" rx="8" fill="#ffffff" stroke="#047857"/>
<text x="597" y="175" text-anchor="middle" font-size="13" fill="#0f172a">中身を実行 → 次の要素へ</text>
<text x="597" y="230" text-anchor="middle" font-size="13" fill="#047857">要素がなくなったら終わる</text>
<text x="597" y="280" text-anchor="middle" font-size="12" fill="#64748b">範囲・配列を順に処理するとき</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-1: 3つのくり返し。迷ったら for、条件で止めるなら while、それ以外は loop</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-06" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="24" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">match code { ... } に 404 を入れると</text>
<rect x="20" y="120" width="110" height="56" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="75" y="146" text-anchor="middle" font-size="12" fill="#c2410c">code の値</text>
<text x="75" y="166" text-anchor="middle" font-size="18" font-weight="700" fill="#c2410c">404</text>
<line x1="130" y1="148" x2="166" y2="148" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-06)"/>
<rect x="170" y="40" width="250" height="34" rx="6" fill="#f8fafc" stroke="#94a3b8"/>
<text x="185" y="62" font-size="14" font-family="Consolas, monospace" fill="#475569">200 =&gt;</text>
<text x="405" y="62" text-anchor="end" font-size="12" fill="#94a3b8">合わない ↓</text>
<rect x="170" y="80" width="250" height="34" rx="6" fill="#f8fafc" stroke="#94a3b8"/>
<text x="185" y="102" font-size="14" font-family="Consolas, monospace" fill="#475569">201 | 202 =&gt;</text>
<text x="405" y="102" text-anchor="end" font-size="12" fill="#94a3b8">合わない ↓</text>
<rect x="170" y="120" width="250" height="34" rx="6" fill="#f8fafc" stroke="#94a3b8"/>
<text x="185" y="142" font-size="14" font-family="Consolas, monospace" fill="#475569">300..=399 =&gt;</text>
<text x="405" y="142" text-anchor="end" font-size="12" fill="#94a3b8">合わない ↓</text>
<rect x="170" y="160" width="250" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="185" y="182" font-size="14" font-family="Consolas, monospace" font-weight="700" fill="#047857">404 =&gt;</text>
<text x="405" y="182" text-anchor="end" font-size="12" font-weight="700" fill="#047857">合った！</text>
<rect x="170" y="200" width="250" height="34" rx="6" fill="#ffffff" stroke="#e2e8f0" stroke-dasharray="4 3"/>
<text x="185" y="222" font-size="14" font-family="Consolas, monospace" fill="#94a3b8">400..=499 =&gt;</text>
<rect x="170" y="240" width="250" height="34" rx="6" fill="#ffffff" stroke="#e2e8f0" stroke-dasharray="4 3"/>
<text x="185" y="262" font-size="14" font-family="Consolas, monospace" fill="#94a3b8">_ =&gt;</text>
<text x="405" y="262" text-anchor="end" font-size="12" fill="#94a3b8">それ以外すべて</text>
<line x1="420" y1="177" x2="476" y2="177" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-06)"/>
<rect x="480" y="150" width="220" height="56" rx="10" fill="#ecfdf5" stroke="#047857" stroke-width="2"/>
<text x="590" y="174" text-anchor="middle" font-size="12" fill="#047857">match 全体の値</text>
<text x="590" y="195" text-anchor="middle" font-size="15" font-weight="700" fill="#047857">"見つかりません"</text>
<text x="590" y="236" text-anchor="middle" font-size="12" fill="#64748b">合った腕より下は調べない</text>
<text x="590" y="256" text-anchor="middle" font-size="12" fill="#64748b">（400..=499 にも合うが使われない）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-2: match は上から順に比べ、最初に合った腕だけを実行する。どの値もどこかの腕に合う必要がある</div>
</div>

### まず動かす最小コード

#### 例1: if / else if / else と if 式

> **▼ このコードがやること（先に日本語で）:** 点数に応じて3通りのメッセージを出し分けます。続いて、`if` の結果を変数に入れて「偶数か奇数か」を表示します。

```rust
@@CODE u04/a_if.rs@@
```

```powershell
cargo run
```

**▼ 実行結果**

```text
@@OUT u04/a_if@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 条件は上から順に、最初の1つだけ

```rust
if score >= 80 {
    println!("よくできました");
} else if score >= 60 {
    println!("合格です");
} else {
    println!("もう一歩です");
}
```

- 72 は `>= 80` に当てはまらず、`>= 60` に当てはまったので「合格です」だけが表示されました。**当てはまった時点で残りは調べません**。
- 条件のまわりに `( )` は不要です（付けると警告が出ます）。一方、`{ }` は1行でも省略できません。
- 比べる演算子は `==`（等しい）、`!=`（等しくない）、`<` `<=` `>` `>=` です。

##### 解説2: if 式の型はそろえる

```rust
let parity = if score % 2 == 0 { "偶数" } else { "奇数" };
```

- `score % 2` は2で割った余りです。0なら偶数です。
- `if` を式として使うときは、**すべての枝が同じ型の値**を返す必要があります（ここではどちらも文字列）。また、値を受け取るなら `else` も必須です（下の「よくあるエラー」で確かめます）。

#### 例2: loop と break で値を返す

> **▼ このコードがやること（先に日本語で）:** 1 から始めて2倍をくり返し、100 を超えた最初の数を見つけます。見つけた数を `break` で持ち帰ります。

```rust
@@CODE u04/b_loop.rs@@
```

**▼ 実行結果**

```text
@@OUT u04/b_loop@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `break 値`

- `loop { ... }` は、`break` に出会うまで中身を何度でもくり返します。
- `break n;` は「ループを抜け、`n` を `loop` 式全体の値にする」という意味です。だから `let result = loop { ... };` と受け取れます。2, 4, 8, 16, 32, 64, 128 と進み、128 で抜けました。
- 「何回くり返すか前もって分からない」処理（再試行、入力待ちなど）に向いています。

#### 例3: while で条件の間くり返す

> **▼ このコードがやること（先に日本語で）:** 残り回数を表示しながら1ずつ減らし、0 になったらくり返しを終えて「発射！」と表示します。

```rust
@@CODE u04/c_while.rs@@
```

**▼ 実行結果**

```text
@@OUT u04/c_while@@
```

- 毎回、中身を実行する**前に**条件 `remaining > 0` を調べます。最初から条件が `false` なら一度も実行されません。
- 条件の中の変数（`remaining`）を中で変えないと、永遠に終わらないので注意しましょう（止めるときは `Ctrl + C`）。

#### 例4: for で範囲や配列を順に処理する

> **▼ このコードがやること（先に日本語で）:** 数の範囲を順番に、逆順に、飛ばしながら取り出して表示します。最後に配列の果物を1つずつ表示します。

```rust
@@CODE u04/d_for.rs@@
```

**▼ 実行結果**

```text
@@OUT u04/d_for@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 範囲 `0..3` と `1..=3`

- `0..3` は「0 から 3 の**手前**まで」（0, 1, 2）、`1..=3` は「1 から 3 **まで**」（1, 2, 3）です。`=` の有無で終わりを含むかが変わります。
- 「0 から n 個」なら `0..n`、「1 から n まで」なら `1..=n` と覚えると迷いません。

##### 解説2: `.rev()` と `.step_by()`

- `(0..10).rev()` は 9, 8, …, 0 の逆順、`.step_by(3)` は「3つおき」です。組み合わせると 9, 6, 3, 0 になりました。
- このように、範囲の後ろに `.` で加工をつなげられます。この仕組み（イテレータ）は UNIT 10 で詳しく学びます。

##### 解説3: 配列を for で回す

- `for fruit in fruits` は、配列の要素を先頭から1つずつ `fruit` に入れてくり返します。添え字（何番目か）を自分で数える必要がないので、**数え間違いによるバグが起きません**。
- Rust では、`while` で添え字を数えるより `for` を使うのが基本です。

#### 例5: match で値を振り分ける

> **▼ このコードがやること（先に日本語で）:** HTTP のステータスコード（サーバーの返事の番号）を、意味を表す日本語に振り分けます。1つの値・複数の値・範囲・それ以外の4種類の書き方を使います。

```rust
@@CODE u04/e_match.rs@@
```

**▼ 実行結果**

```text
@@OUT u04/e_match@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 腕（アーム）の書き方

```rust
let label = match code {
    200 => "成功",
    201 | 202 => "受け付けました",
    300..=399 => "リダイレクト",
    _ => "不明なコード",
};
```

- `パターン => 値,` の1行を**腕**（アーム）と呼びます。`match` は上から順に比べ、最初に合った腕の値が `match` 全体の値になります（図4-2）。
- `201 | 202` は「201 または 202」、`300..=399` は「300 以上 399 以下」、`_` は「それ以外すべて」です。
- 404 は `404 =>` に合うので「見つかりません」。その下の `400..=499` にも当てはまりますが、もう調べません。だから**細かい条件を上、大まかな条件を下**に書きます。418 は `404` に合わず `400..=499` に合いました。

##### 解説2: なぜ `_` が必要なのか

- `code` は整数なので、999 のような想定外の値も入り得ます。Rust の `match` は**すべての値がどこかの腕に合うこと（網羅性）**を求めるので、最後に `_` を置いて「それ以外」を引き受けます。
- 網羅していないとコンパイルエラーになります（下の「よくあるエラー」）。これのおかげで「場合分けの書き忘れ」が実行前に見つかります。

#### 例6: ガード付きの match とラベル付き break

> **▼ このコードがやること（先に日本語で）:** 気温を「氷点下」「真夏日」などに分類します。範囲では書きにくい条件は `if` を添えて書きます。後半は、2重のループの内側から外側のループごと抜け出します。

```rust
@@CODE u04/f_guard_label.rs@@
```

**▼ 実行結果**

```text
@@OUT u04/f_guard_label@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: マッチガード `t if 条件`

```rust
t if t < 0 => "氷点下",
```

- `t` は「どんな値にも合い、その値に `t` という名前を付ける」パターンです。後ろの `if t < 0` が追加の条件（**ガード**）で、両方が満たされたときだけこの腕が選ばれます。

##### 解説2: ラベル付き break

```rust
'outer: for row in 1..=3 {
    for col in 1..=3 {
        if row * col == 4 {
            break 'outer;
        }
    }
}
```

- ふつうの `break` は**いちばん内側の**ループだけを抜けます。`'outer:` のように `'` で始まる名前（ラベル）を外側のループに付けておくと、`break 'outer;` で外側まで一気に抜けられます。
- 同じように `continue 'outer;` と書くと「外側のループの次の回へ進む」になります。

### もう一歩: 実務に近い形

バックエンドでは、外部のサービスへの問い合わせが一時的に失敗することがあります。そのとき「成功するまで、上限回数まで再試行する」処理を、`loop` と `match` と `continue` で書いてみます。

#### 一時的な失敗を再試行する

> **▼ このコードがやること（先に日本語で）:** にせ物の問い合わせ関数を最大5回呼びます。503（一時的に使えない）なら再試行、2xx なら成功、それ以外ならあきらめて、最終結果を表示します。

```rust
@@CODE u04/g_retry.rs@@
```

**▼ 実行結果**

```text
@@OUT u04/g_retry@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: match の腕から break / continue する

```rust
200..=299 => break format!("{attempt} 回目で成功（{status}）"),
503 if attempt < MAX_ATTEMPTS => {
    println!("{attempt} 回目: {status} のため再試行します");
    continue;
}
_ => break format!("{attempt} 回目で中止（{status}）"),
```

- 腕の右側には `{ }` で複数の文も書けます。`continue` は「この回の残りを飛ばして、次の回へ進む」です。
- 成功・中止のどちらの腕も `break 文字列` なので、`loop` 全体の値は `String` にそろいます。
- 503 でも試行回数が上限に達していればガードが成り立たないので、最後の `_` の腕（中止）に落ちます。これで**無限ループを防いでいます**。

> **注意:** 本物の再試行では、すぐにやり直すのではなく少し待ち時間を入れます（第5巻の async で扱います）。

### よくあるエラーと読み方

#### エラー1: E0308 if と else の型が違う

```rust
@@CODE u04/err1_if_types.rs@@
```

```text
@@ERR u04/err1_if_types@@
```

- **読み方:** 「`if` と `else` の型が合いません」。`expected because of this` が指す `"正の数"`（文字列）を見て「文字列が来るはずだ」と判断したのに、`else` 側は整数でした。
- **直し方:** どちらも同じ型にします。例えば `else { "0以下" }` にします。

#### エラー2: E0317 値として使う if に else がない

```rust
@@CODE u04/err2_if_no_else.rs@@
```

```text
@@ERR u04/err2_if_no_else@@
```

- **読み方:** 「`if` に `else` がないかもしれません」。`note` にあるとおり、`else` のない `if` は、条件が `false` のとき `()` になります。すると `label` が文字列なのか `()` なのか決まりません。
- **直し方:** `else { "0以下" }` を足して、どちらの場合も文字列になるようにします。

#### エラー3: E0004 match がすべての場合を網羅していない

```rust
@@CODE u04/err3_e0004.rs@@
```

```text
@@ERR u04/err3_e0004@@
```

- **読み方:** `non-exhaustive patterns`（網羅されていないパターン）は「この `match` では扱われていない値があります」。具体的に `0_u16..=199_u16`（0〜199）、`300_u16..=399_u16`（300〜399）、`600_u16..=u16::MAX`（600〜最大値）が抜けていると教えてくれています。
- **直し方:** 抜けている範囲に意味があるなら腕を足し、なければ最後に `_ => "その他",` を足します。`help` が提案している `todo!()` は「あとで書く」という仮置きで、**実行してそこに来ると panic します**。そのまま残さないようにしましょう。

#### エラー4: E0308 条件に整数を書いた

```rust
@@CODE u04/err4_if_int.rs@@
```

```text
@@ERR u04/err4_if_int@@
```

- **何が起きたか:** JavaScript や Python では「0 以外の数は真」とみなされますが、Rust の `if` の条件は**必ず `bool`** です。
- **直し方:** 意図をそのまま比較で書きます: `if unread > 0 { ... }`。

> **つまずきポイント:** 「`if x` で済むのに面倒」と感じるかもしれません。しかし `if count` が「0 でない」なのか「存在する」なのかは読む人によって解釈が分かれます。`unread > 0` と書けば誤解の余地がありません。

### RustNote ではここで使う

RustNote のノート一覧は20件ずつページに分け、ページの下に「← 前へ」「次へ →」のリンクを出します。どちらのリンクを出すかは「前のページがあるか」「次のページがあるか」の2つの `bool` の組み合わせで決まるので、`match` で4通りすべてを書き切ります。

> **▼ このコードがやること（先に日本語で）:** ノートが45件あるときのページ数を計算し、各ページにどのページ送りリンクを出すかを表示します。

```rust
@@CODE u04/h_rustnote.rs@@
```

**▼ 実行結果**

```text
@@OUT u04/h_rustnote@@
```

- `total_notes.div_ceil(PER_PAGE)` は「割り算して、端数を切り上げる」メソッドです。45 ÷ 20 = 2.25 → 3 ページです。
- `(has_prev, has_next)` は2つの値を組にした**タプル**です（UNIT 06）。`match` で組のまま比べると、`true` / `false` の4通りを表のように並べて書けます。1つでも書き忘れると E0004 で教えてもらえるので、**表示の出し忘れが起きません**。

### 練習問題

**問題1.** 1 から 15 までの数について、3 の倍数なら `Fizz`、5 の倍数なら `Buzz`、両方の倍数なら `FizzBuzz`、それ以外は数そのものを、空白区切りで1行に表示してください。`match` と `(n % 3, n % 5)` の組を使ってみましょう。

**問題2.** 1 + 2 + 3 + … と足していき、合計が初めて 100 以上になるのは何まで足したときかを、`loop` と `break 値` で求めてください。

**問題3.** 「よくあるエラー」のエラー3のコードを、`code` が 302 のとき `その他` と表示されるように直してください。

### 解答と解説

**解答1.**

```rust
@@CODE u04/ex1.rs@@
```

**▼ 実行結果**

```text
@@OUT u04/ex1@@
```

- `(0, 0)` を**いちばん上**に書くのがポイントです。`(0, _)` を先に書くと、15 も `Fizz` になってしまいます。`_` は「何でもよい」なので、`(0, _)` は「3 で割り切れれば、5 の余りは何でもよい」です。
- `.to_string()` は数や文字列から `String` を作るメソッドです。すべての腕を `String` にそろえるために使っています（UNIT 05）。

**解答2.**

```rust
@@CODE u04/ex2.rs@@
```

**▼ 実行結果**

```text
@@OUT u04/ex2@@
```

**解答3.** 最後に `_` の腕を足します。

```rust
@@CODE u04/ex3.rs@@
```

**▼ 実行結果**

```text
@@OUT u04/ex3@@
```

### まとめ

- `if` は式なので値を返せる。そのときは全部の枝の型をそろえ、`else` も書く。条件は必ず `bool`。
- くり返しは、順に処理するなら `for`、条件で止めるなら `while`、それ以外は `loop`（`break 値` で結果を返せる）。
- `match` は上から順に比べ、最初に合った腕を実行する。すべての場合を網羅する必要があり、書き忘れはコンパイラが E0004 で教えてくれる。
