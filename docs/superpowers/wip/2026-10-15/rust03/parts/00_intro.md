<div style="page-break-after: always; text-align: center; margin: 0; padding: 0;">
<img src="assets/cover.jpg" alt="表紙" style="width: 100%; max-height: 175mm; object-fit: cover; border-radius: 10px;">
<div style="margin-top: 14px; font-size: 11px; letter-spacing: 0.18em; color: #c2410c; font-weight: 700;">VOLUME 03 / RUST BACKEND MASTERY</div>
<div style="font-size: 26px; font-weight: 800; color: #0f172a; margin-top: 6px; line-height: 1.35;">Rust中核 — 所有権・借用・ライフタイム・スマートポインタ</div>
<div style="font-size: 13px; color: #475569; margin-top: 8px;">メモリの図をコマ送りで追いながら、Rust のコンパイラが何を守っているのかを自分の言葉で説明できるようになる</div>
</div>

# 第3巻: Rust中核 — 所有権・借用・ライフタイム・スマートポインタ

> この巻では、Rust（ラスト）をほかの言語と大きく違うものにしている**所有権（しょゆうけん：ある値を最後に片付ける責任を、どの変数が持っているか）**の仕組みを学びます。Rust を学ぶ人のほとんどが最初につまずくのがこの巻の内容です。でも安心してください。所有権のルールは、**メモリ（コンピュータが値を置いておく場所）の図**を1コマずつ動かして見れば、どれも「そうしないと危ないから」という理由がはっきり見えてきます。この巻では、コードの各行の時点でメモリがどうなっているかを、何十枚もの図で追いかけます。

<!-- -->

> **大事なポイント（先に伝えておくと安心）:**
> - **コンパイルエラーは敵ではありません。** Rust のコンパイラ（rustc：書いたコードを機械語に翻訳するプログラム）は、メモリの事故が起きそうな場所を、プログラムを動かす**前に**教えてくれます。この巻では、わざとエラーを出して、エラー文と図を見比べる練習を何度もします。
> - **ルールは3つだけです。** ①値の持ち主は常に1人、②持ち主がいなくなったら値は片付けられる、③借りるときは「読む人が何人でも」か「書く人が1人だけ」のどちらか。この巻の10単元は、この3つを色々な場面に当てはめていくだけです。
> - **暗記より図です。** 「なぜエラーになるのか」が分からなくなったら、紙にスタックとヒープの箱を描いて、矢印を引いてみてください。

## この巻で身に付けること

### 到達目標

この巻を読み終えると、次のことができるようになります。

- 値が**スタック**と**ヒープ**のどちらに置かれ、いつ片付けられるか（Drop）を図で説明できる
- **Move（移動）・Copy（複製）・Clone（明示的な複製）**の違いと、それぞれの費用を説明できる
- **共有参照 `&T`** と **可変参照 `&mut T`** のルールを守ったコードを書き、E0499 / E0502 / E0382 などのエラーを自分で直せる
- 関数の引数を `&str` や `&[T]` で受け取り、結果を `String` や `Vec<T>` で返す、という**借用を使った API 設計**ができる
- **ライフタイム注釈 `'a`** を「参照どうしの有効期間の関係」として読み書きでき、省略規則が使える場面を見分けられる
- `Box<T>`・`Rc<T>`・`RefCell<T>`・`Arc<T>`・`Mutex<T>` を、目的に応じて選べる
- `Send` と `Sync` が何を表しているかを説明し、スレッドをまたいで値を共有するコードを書ける

### 学習地図

10個の単元は、大きく3つのまとまりに分かれています。左から順に進むと、「誰が持つか」→「いつまで借りられるか」→「どうやって共有するか」と、問いが少しずつ広がっていきます。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 370" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-03-01" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="10" width="216" height="300" rx="12" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>
<text x="118" y="36" text-anchor="middle" font-size="15" font-weight="700" fill="#1e40af">第1部 所有権の基本</text>
<text x="118" y="56" text-anchor="middle" font-size="12" fill="#1e40af">問い: 誰が持つ？</text>
<rect x="24" y="72" width="188" height="40" rx="8" fill="#ffffff" stroke="#3b82f6"/>
<text x="36" y="97" font-size="13" fill="#0f172a">01 スタック・ヒープ・Drop</text>
<rect x="24" y="120" width="188" height="40" rx="8" fill="#ffffff" stroke="#3b82f6"/>
<text x="36" y="145" font-size="13" fill="#0f172a">02 Move・Copy・Clone</text>
<rect x="252" y="10" width="216" height="300" rx="12" fill="#ecfdf5" stroke="#10b981" stroke-width="1.5"/>
<text x="360" y="36" text-anchor="middle" font-size="15" font-weight="700" fill="#047857">第2部 借用とライフタイム</text>
<text x="360" y="56" text-anchor="middle" font-size="12" fill="#047857">問い: いつまで借りられる？</text>
<rect x="266" y="72" width="188" height="40" rx="8" fill="#ffffff" stroke="#10b981"/>
<text x="278" y="97" font-size="13" fill="#0f172a">03 &amp;T と &amp;mut T</text>
<rect x="266" y="120" width="188" height="40" rx="8" fill="#ffffff" stroke="#10b981"/>
<text x="278" y="145" font-size="13" fill="#0f172a">04 スライスと API 設計</text>
<rect x="266" y="168" width="188" height="40" rx="8" fill="#ffffff" stroke="#10b981"/>
<text x="278" y="193" font-size="13" fill="#0f172a">05 ライフタイム注釈</text>
<rect x="266" y="216" width="188" height="40" rx="8" fill="#ffffff" stroke="#10b981"/>
<text x="278" y="241" font-size="13" fill="#0f172a">06 参照を持つ struct</text>
<rect x="266" y="264" width="188" height="40" rx="8" fill="#ffffff" stroke="#10b981"/>
<text x="278" y="289" font-size="13" fill="#0f172a">07 NLL・再借用</text>
<rect x="494" y="10" width="216" height="300" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="1.5"/>
<text x="602" y="36" text-anchor="middle" font-size="15" font-weight="700" fill="#c2410c">第3部 スマートポインタ</text>
<text x="602" y="56" text-anchor="middle" font-size="12" fill="#c2410c">問い: どう共有する？</text>
<rect x="508" y="72" width="188" height="40" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="520" y="97" font-size="13" fill="#0f172a">08 Box・再帰型・dyn</text>
<rect x="508" y="120" width="188" height="40" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="520" y="145" font-size="13" fill="#0f172a">09 Rc・RefCell</text>
<rect x="508" y="168" width="188" height="40" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="520" y="193" font-size="13" fill="#0f172a">10 Arc・Mutex・Send/Sync</text>
<line x1="226" y1="140" x2="250" y2="140" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-01)"/>
<line x1="468" y1="140" x2="492" y2="140" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-01)"/>
<rect x="10" y="326" width="700" height="36" rx="8" fill="#f8fafc" stroke="#94a3b8"/>
<text x="360" y="349" text-anchor="middle" font-size="13" fill="#334155">この先へ: 第4巻 trait とエラー型 ／ 第5巻 async と Tokio ／ 第6巻 Topcoat の共有状態</text>
<line x1="602" y1="310" x2="602" y2="324" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-01)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図0-1: 第3巻の学習地図。第1部で「持ち主」、第2部で「借用の期限」、第3部で「共有のしかた」を学ぶ</div>
</div>

| 単元 | 一言でいうと | 主に出てくるエラー |
|---|---|---|
| UNIT 01 | 値はスタックかヒープに置かれ、持ち主のスコープが終わると1回だけ片付けられる | E0040 |
| UNIT 02 | 代入や関数呼び出しで、持ち主が移る（move） | E0382, E0507, E0204 |
| UNIT 03 | 借りるときは「読む人が何人でも」か「書く人が1人だけ」 | E0499, E0502, E0596 |
| UNIT 04 | `&str` と `&[T]` は「場所＋長さ」の借用。関数はこれで受け取る | E0502, E0277, E0308 |
| UNIT 05 | `'a` は「この参照はあの参照と同じ間だけ有効」という関係の印 | E0106, E0597, E0515, E0621 |
| UNIT 06 | 参照を持つ struct は、借り元より長生きできない。迷ったら所有へ切り替える | E0106, E0597, E0716, E0521 |
| UNIT 07 | 借用は「最後に使った所」で終わる。借用を短くする書き方 | E0502, E0499, E0505 |
| UNIT 08 | `Box` でヒープに置くと、再帰型や `dyn Trait` が扱える | E0072, E0782, E0277 |
| UNIT 09 | `Rc` で持ち主を複数に、`RefCell` で借用チェックを実行時に | E0596, 実行時 panic |
| UNIT 10 | `Arc` と `Mutex` でスレッドをまたいで共有する | E0373, E0277, E0382, E0594 |

### 前提知識（先に読んでおく巻）

- **第1巻（環境構築）**: `cargo new` でプロジェクトを作り、`cargo run` で動かせること。
- **第2巻（Rust基礎）**: 変数と `mut`、`String` と `&str` の見た目の違い、`Vec`、`struct` と `impl`、`enum` と `Option` / `Result`、`for` 文、クロージャ（`|x| x + 1` のような小さな関数）を一度見たことがあること。
- 第0巻の「メモリとプロセス」の話を読んでいると、スタックとヒープの説明がすっと入ります（読んでいなくても、この巻だけで分かるように書いています）。

### この巻の進め方

1単元ごとに、次の4ステップを回します。

1. **予想する** — コードを実行する前に「何が表示されるか」「どこでエラーになるか」を予想します。
2. **実行する** — 実際に動かして、本書の実行結果と見比べます。
3. **エラーを読む** — わざとエラーになるコードを動かし、エラー文の矢印（`^^^^` や `----`）がどの行を指しているかを確かめます。
4. **図で確かめる** — エラーの理由を、スタックとヒープの図で説明できるか試します。

#### 練習用プロジェクトを1つ作る

この巻のコードは、すべて1つの Cargo プロジェクトの中で動かします。PowerShell（Windows のターミナル）で次のコマンドを実行してください。

```powershell
# 練習用のプロジェクトを作る
cargo new ownership-lab
cd ownership-lab
# 複数の小さなプログラムを置くためのフォルダを作る
mkdir src\bin
```

> **WSL / macOS / Linux の場合:** `mkdir src\bin` の代わりに `mkdir -p src/bin` と入力します。ほかのコマンドは同じです。

Cargo には「`src/bin/` フォルダに置いた `.rs` ファイルは、それぞれ別のプログラムとして扱う」という決まりがあります。この巻では、コード例ごとに名前を付けて `src/bin/` に保存し、`--bin 名前` で選んで実行します。

```text
ownership-lab/
├── Cargo.toml
└── src/
    ├── main.rs
    └── bin/
        ├── u01_stack_heap.rs    ← cargo run --bin u01_stack_heap で実行
        ├── u01_drop_order.rs    ← cargo run --bin u01_drop_order で実行
        └── ……（単元ごとに増えていく）
```

たとえば UNIT 01 の最初の例 `src/bin/u01_stack_heap.rs` を実行すると、画面には次のように表示されます。

```text
PS C:\Users\...\ownership-lab> cargo run --bin u01_stack_heap
   Compiling ownership-lab v0.1.0 (C:\Users\...\ownership-lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target\debug\u01_stack_heap.exe`
n = 5
s = hello / len = 5 / cap = 5
s = hello, world / len = 12 / cap = 12
i32 の大きさ: 4 バイト
String の持ち手の大きさ: 24 バイト
```

最初の3行（`Compiling` / `Finished` / `Running`）は Cargo 自身のメッセージです。**以降の「▼ 実行結果」では、この3行を省いてプログラムの出力だけを載せます。** また、エラーの例では、Cargo が出すエラー文をそのまま載せます。Windows ではファイルの場所が `src\bin\…`（バックスラッシュ区切り）と表示されますが、WSL / macOS / Linux では `src/bin/…` になります。意味は同じです。

> **ヒント:** エラー番号（`E0382` など）が出たら、`rustc --explain E0382` を実行すると、そのエラーの英語の解説と例が読めます。この巻では主なエラーを日本語で解説していきます。

#### メモリ図の読み方

この巻の図は、どれも同じ約束で描いています。最初にここで読み方を覚えてしまいましょう。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-03-02" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
<marker id="ar-03-03" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#10b981"/></marker>
<marker id="ar-03-04" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#f97316"/></marker>
</defs>
<rect x="10" y="10" width="300" height="150" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>
<text x="24" y="34" font-size="14" font-weight="700" fill="#1e40af">スタック（関数ごとの作業台）</text>
<text x="40" y="84" font-size="14" font-weight="700" fill="#0f172a">s</text>
<rect x="60" y="60" width="150" height="84" rx="4" fill="#ffffff" stroke="#1e40af"/>
<line x1="60" y1="88" x2="210" y2="88" stroke="#bfdbfe"/>
<line x1="60" y1="116" x2="210" y2="116" stroke="#bfdbfe"/>
<text x="72" y="79" font-size="13" fill="#0f172a">ptr（場所）</text>
<text x="72" y="107" font-size="13" fill="#0f172a">len = 5</text>
<text x="72" y="135" font-size="13" fill="#0f172a">cap = 5</text>
<circle cx="196" cy="74" r="4" fill="#475569"/>
<rect x="410" y="10" width="300" height="150" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="1.5"/>
<text x="424" y="34" font-size="14" font-weight="700" fill="#c2410c">ヒープ（大きさ自由の倉庫）</text>
<rect x="450" y="58" width="40" height="34" fill="#ffffff" stroke="#c2410c"/>
<rect x="490" y="58" width="40" height="34" fill="#ffffff" stroke="#c2410c"/>
<rect x="530" y="58" width="40" height="34" fill="#ffffff" stroke="#c2410c"/>
<rect x="570" y="58" width="40" height="34" fill="#ffffff" stroke="#c2410c"/>
<rect x="610" y="58" width="40" height="34" fill="#ffffff" stroke="#c2410c"/>
<text x="470" y="81" text-anchor="middle" font-size="14" fill="#0f172a">h</text>
<text x="510" y="81" text-anchor="middle" font-size="14" fill="#0f172a">e</text>
<text x="550" y="81" text-anchor="middle" font-size="14" fill="#0f172a">l</text>
<text x="590" y="81" text-anchor="middle" font-size="14" fill="#0f172a">l</text>
<text x="630" y="81" text-anchor="middle" font-size="14" fill="#0f172a">o</text>
<text x="550" y="118" text-anchor="middle" font-size="12" fill="#c2410c">文字の本体（5バイト）</text>
<path d="M196,74 C300,74 360,75 448,75" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-02)"/>
<text x="330" y="66" text-anchor="middle" font-size="12" fill="#475569">場所を指す</text>
<rect x="10" y="176" width="226" height="54" rx="6" fill="#f1f5f9" stroke="#94a3b8" stroke-dasharray="6 4"/>
<text x="24" y="200" font-size="13" fill="#64748b">点線・灰色の箱</text>
<text x="24" y="220" font-size="12" fill="#64748b">= 無効になった変数（使えない）</text>
<line x1="256" y1="194" x2="316" y2="194" stroke="#10b981" stroke-width="2.5" marker-end="url(#ar-03-03)"/>
<text x="326" y="199" font-size="13" fill="#047857">緑の矢印 = &amp;T（読むための借用）</text>
<line x1="256" y1="222" x2="316" y2="222" stroke="#f97316" stroke-width="2.5" marker-end="url(#ar-03-04)"/>
<text x="326" y="227" font-size="13" fill="#c2410c">橙の矢印 = &amp;mut T（書くための借用）</text>
<text x="40" y="270" font-size="22" font-weight="700" fill="#dc2626">✕</text>
<text x="68" y="268" font-size="13" fill="#334155">= 片付け済みのメモリ、または「ここを指したら危ない」場所</text>
<text x="40" y="292" font-size="12" fill="#64748b">①②③… = コマ送りの順番（コードを1行ずつ実行したときの様子）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図0-2: メモリ図の読み方。左の青がスタック、右の橙がヒープ。灰色の矢印は「この場所を指している」を表す</div>
</div>

### 使うツールと版

| 項目 | この巻で使うもの | 補足 |
|---|---|---|
| Rust | 1.98 以上（本書の動作確認は rustc 1.99.0 / cargo 1.99.0） | `rustc --version` で確認できます。古い場合は `rustup update` で更新します |
| edition | 2024 | `cargo new` で作ると自動で `edition = "2024"` になります |
| OS | Windows 11（ARM64）+ PowerShell | WSL / macOS / Linux でも同じコードが動きます |
| 外部クレート | 使いません | この巻のコードはすべて標準ライブラリ（`std`）だけで動きます |

シリーズ全体の基準を「Rust 1.98 以上」にしているのは、第6巻以降で使う Web フレームワーク Topcoat 0.10 が 1.98 以上を必要とするためです。実行結果とエラー文は、すべて 1.99.0 で実際に動かしたものを載せています。

> **ヒント: 本書と同じ版で試したいとき** … `ownership-lab` フォルダの中に `rust-toolchain.toml` という名前のファイルを作り、下の2行を書いて保存します。すると、このフォルダの中で `cargo` を実行したときだけ、指定した版の Rust が使われます。パソコン全体の既定の版は変わりません。その版が入っていないと言われたら、`rustup toolchain install 1.99.0` で入れてください。

```toml
[toolchain]
channel = "1.99.0"
```

> **注意:** この巻に出てくる「`String` の持ち手は 24 バイト」といった大きさは、**64 ビットのパソコン**での値です。いまのパソコンはほぼすべて 64 ビットなので、そのまま同じ数字になるはずです。また、`0x23628108d90` のような**メモリの場所（アドレス）**を表示する例もありますが、この数字は実行するたびに変わります。大事なのは数字そのものではなく、「2つが同じか、違うか」です。

