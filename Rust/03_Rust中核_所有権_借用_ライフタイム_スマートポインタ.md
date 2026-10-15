<div style="page-break-after: always; text-align: center; margin: 0; padding: 0;">
<img src="assets/cover.jpg" alt="表紙" style="width: 100%; max-height: 175mm; object-fit: cover; border-radius: 10px;">
<div style="margin-top: 14px; font-size: 11px; letter-spacing: 0.18em; color: #c2410c; font-weight: 700;">VOLUME 03 / RUST BACKEND MASTERY</div>
<div style="font-size: 26px; font-weight: 800; color: #0f172a; margin-top: 6px; line-height: 1.35;">Rust中核 — 所有権・借用・ライフタイム・スマートポインタ</div>
<div style="font-size: 13px; color: #475569; margin-top: 8px;">メモリの図をコマ送りで追いながら、Rust のコンパイラが何を守っているのかを自分の言葉で説明できるようになる</div>
</div>

# 第3巻: Rust中核 — 所有権・借用・ライフタイム・スマートポインタ

> この巻では、Rust（ラスト）をほかの言語と大きく違うものにしている**所有権（しょゆうけん：ある値を最後に片付ける責任を、どの変数が持っているか）**の仕組みを学びます。Rust を学ぶ人のほとんどが最初につまずくのがこの巻の内容です。でも安心してください。所有権のルールは、**メモリ（コンピュータが値を置いておく場所）の図**を1コマずつ動かして見れば、どれも「そうしないと危ないから」という理由がはっきり見えてきます。この巻では、コードの各行の時点でメモリがどうなっているかを、何十枚もの図で追いかけます。

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
| UNIT 07 | 借用は「最後に使った所」で終わる。借用を短くする書き方 | E0502, E0499 |
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.13s
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
| Rust | rustc 1.97.1 / cargo 1.97.1 | `rustc --version` で確認できます |
| edition | 2024 | `cargo new` で作ると自動で `edition = "2024"` になります |
| OS | Windows 11（ARM64）+ PowerShell | WSL / macOS / Linux でも同じコードが動きます |
| 外部クレート | 使いません | この巻のコードはすべて標準ライブラリ（`std`）だけで動きます |

> **注意:** この巻に出てくる「`String` の持ち手は 24 バイト」といった大きさは、**64 ビットのパソコン**での値です。いまのパソコンはほぼすべて 64 ビットなので、そのまま同じ数字になるはずです。また、`0x26904108da0` のような**メモリの場所（アドレス）**を表示する例もありますが、この数字は実行するたびに変わります。大事なのは数字そのものではなく、「2つが同じか、違うか」です。

## UNIT 01: スタック・ヒープ・スコープ・Drop

### この単元のゴール

- 値が置かれる2つの場所、**スタック**と**ヒープ**の違いを、例を挙げて説明できる
- `String` が「スタックにある持ち手（`ptr`・`len`・`cap`）」と「ヒープにある文字の本体」の2つでできていることを、図に描ける
- 変数が**スコープ**（`{ }` で囲まれた範囲）を抜けると、値が**1回だけ**片付けられる（**Drop**）こと、その順番が「作った順の逆」であることを確かめる
- `Drop` を使って「後片付けを絶対に忘れない」仕組み（RAII）を作れる

### たとえ話でつかむ

あなたは事務所で働いていて、机と倉庫を使って仕事をしています。

- **スタック = 机の上の書類トレイ** です。仕事（関数）を始めると、机の上に書類を積んでいきます。置ける書類は「大きさが決まっているもの」だけ。取り出しはとても速く、仕事が終わると机の上はまとめて片付けます。
- **ヒープ = 倉庫** です。大きさが途中で変わる荷物（長さの決まらない文章や、件数の決まらない名簿）は、倉庫の棚を借りてそこに置きます。倉庫を使うには受付で手続きが必要なので、机より少し時間がかかります。手続きが終わると「棚の場所の番号札」がもらえます。
- **`String` は「番号札を机に置き、本体は倉庫」** という使い方をしています。机の上には「番号札（`ptr`：場所）」「今入っている文字数（`len`：長さ）」「借りた棚の大きさ（`cap`：容量）」の3枚のメモだけを置きます。
- **持ち主 = 棚を借りた人** です。仕事が終わって机を片付けるとき、借りていた棚も必ず返します。これが **Drop（ドロップ：片付け）** です。

ここで、倉庫の棚の返し方には2種類の事故があります。

| 事故 | たとえ | プログラムで起きること |
|---|---|---|
| 返し忘れ | 誰も使っていない棚が借りっぱなし | **メモリリーク**（使わないメモリが増え続け、いつか足りなくなる） |
| 2回返す | すでに返した棚を、別の人がもう一度返す | **二重解放**（ほかの人が使い始めた棚を壊してしまう。セキュリティ事故の原因） |

C 言語では、プログラマが自分で「返す」命令（`free`）を書くので、この2つの事故がよく起きます。Java・JavaScript・Python は、**ガベージコレクタ**（GC：使われていないメモリを定期的に見回って回収する係）に任せて事故を防ぎますが、見回りのためにプログラムが少し止まることがあります。

**Rust は第3の方法を選びました。** 「この値の持ち主は誰か」をコンパイラが常に把握していて、**持ち主のスコープが終わる場所に、片付けの処理を自動で1回だけ入れる**のです。見回り係はいらず、返し忘れも二重返却も起きません。その代わり、「持ち主は誰か」をコンパイラが分かるように書く必要があります。それがこの巻で学ぶ所有権のルールです。

> **用語: スコープ** … 変数が使える範囲のことです。Rust では基本的に `{` から `}` までがスコープです。関数の本体も、`if` や `for` の `{ }` も、ただの `{ }` もスコープになります。

### 図で理解する

まず、整数 `n` と文字列 `s` を作った直後のメモリを見てみましょう。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-03-05" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="6" y="6" width="350" height="258" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="20" y="30" font-size="13" font-weight="700" fill="#0f172a" font-family="Consolas, Menlo, monospace">① let s = String::from("hello");</text>
<rect x="20" y="44" width="150" height="206" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="32" y="64" font-size="13" font-weight="700" fill="#1e40af">スタック</text>
<text x="34" y="98" font-size="14" font-weight="700" fill="#0f172a">n</text>
<rect x="54" y="80" width="104" height="28" rx="3" fill="#ffffff" stroke="#1e40af"/>
<text x="64" y="99" font-size="13" fill="#0f172a">5（i32）</text>
<text x="34" y="150" font-size="14" font-weight="700" fill="#0f172a">s</text>
<rect x="54" y="128" width="104" height="84" rx="3" fill="#ffffff" stroke="#1e40af"/>
<line x1="54" y1="156" x2="158" y2="156" stroke="#bfdbfe"/>
<line x1="54" y1="184" x2="158" y2="184" stroke="#bfdbfe"/>
<text x="64" y="147" font-size="13" fill="#0f172a">ptr</text>
<circle cx="146" cy="142" r="4" fill="#475569"/>
<text x="64" y="175" font-size="13" fill="#0f172a">len = 5</text>
<text x="64" y="203" font-size="13" fill="#0f172a">cap = 5</text>
<text x="95" y="238" text-anchor="middle" font-size="12" fill="#1e40af">持ち手 24 バイト</text>
<rect x="196" y="44" width="148" height="206" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="208" y="64" font-size="13" font-weight="700" fill="#c2410c">ヒープ</text>
<rect x="210" y="128" width="24" height="28" fill="#ffffff" stroke="#c2410c"/>
<rect x="234" y="128" width="24" height="28" fill="#ffffff" stroke="#c2410c"/>
<rect x="258" y="128" width="24" height="28" fill="#ffffff" stroke="#c2410c"/>
<rect x="282" y="128" width="24" height="28" fill="#ffffff" stroke="#c2410c"/>
<rect x="306" y="128" width="24" height="28" fill="#ffffff" stroke="#c2410c"/>
<text x="222" y="147" text-anchor="middle" font-size="13" fill="#0f172a">h</text>
<text x="246" y="147" text-anchor="middle" font-size="13" fill="#0f172a">e</text>
<text x="270" y="147" text-anchor="middle" font-size="13" fill="#0f172a">l</text>
<text x="294" y="147" text-anchor="middle" font-size="13" fill="#0f172a">l</text>
<text x="318" y="147" text-anchor="middle" font-size="13" fill="#0f172a">o</text>
<text x="270" y="178" text-anchor="middle" font-size="12" fill="#c2410c">本体 5 バイト</text>
<line x1="146" y1="142" x2="208" y2="142" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-05)"/>
<rect x="364" y="6" width="350" height="258" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="378" y="30" font-size="13" font-weight="700" fill="#0f172a" font-family="Consolas, Menlo, monospace">② s.push_str(", world");</text>
<rect x="378" y="44" width="150" height="206" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="390" y="64" font-size="13" font-weight="700" fill="#1e40af">スタック</text>
<text x="392" y="98" font-size="14" font-weight="700" fill="#0f172a">n</text>
<rect x="412" y="80" width="104" height="28" rx="3" fill="#ffffff" stroke="#1e40af"/>
<text x="422" y="99" font-size="13" fill="#0f172a">5（i32）</text>
<text x="392" y="150" font-size="14" font-weight="700" fill="#0f172a">s</text>
<rect x="412" y="128" width="104" height="84" rx="3" fill="#ffffff" stroke="#1e40af"/>
<line x1="412" y1="156" x2="516" y2="156" stroke="#bfdbfe"/>
<line x1="412" y1="184" x2="516" y2="184" stroke="#bfdbfe"/>
<text x="422" y="147" font-size="13" fill="#0f172a">ptr</text>
<circle cx="504" cy="142" r="4" fill="#475569"/>
<text x="422" y="175" font-size="13" fill="#c2410c" font-weight="700">len = 12</text>
<text x="422" y="203" font-size="13" fill="#c2410c" font-weight="700">cap = 12</text>
<text x="453" y="238" text-anchor="middle" font-size="12" fill="#1e40af">持ち手は 24 バイトのまま</text>
<rect x="554" y="44" width="148" height="206" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="566" y="64" font-size="13" font-weight="700" fill="#c2410c">ヒープ</text>
<rect x="566" y="128" width="126" height="28" fill="#ffffff" stroke="#c2410c"/>
<text x="629" y="147" text-anchor="middle" font-size="13" fill="#0f172a" font-family="Consolas, Menlo, monospace">hello, world</text>
<text x="629" y="178" text-anchor="middle" font-size="12" fill="#c2410c">本体 12 バイト</text>
<text x="629" y="200" text-anchor="middle" font-size="12" fill="#64748b">足りなければ広い場所へ</text>
<text x="629" y="216" text-anchor="middle" font-size="12" fill="#64748b">引っ越して ptr が変わる</text>
<line x1="504" y1="142" x2="564" y2="142" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-05)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-1: 整数はスタックにそのまま、String は「スタックの持ち手」と「ヒープの本体」に分かれて置かれる</div>
</div>

図から読み取ってほしいことは3つです。

1. `n`（`i32`）は大きさが4バイトと決まっているので、**値そのものがスタックに置かれます**。
2. `s`（`String`）は、スタックに**持ち手**（`ptr`・`len`・`cap` の3つ、合わせて24バイト）、ヒープに**文字の本体**があります。`ptr` は「本体がヒープのどこにあるか」を指す番号札です。
3. 文字を足しても、変わるのはヒープの本体と `len`・`cap` だけです。**持ち手の大きさは24バイトのまま**です。だから Rust は、`String` を「大きさの決まった値」としてスタックで扱えるのです。

| 置かれる場所 | どんな値か | 例 |
|---|---|---|
| スタック | 大きさがコンパイル時に決まっている値 | `i32`、`f64`、`bool`、`char`、`(i32, bool)`、`[u8; 4]`、`String` や `Vec` の**持ち手** |
| ヒープ | 実行してみないと大きさが分からない値 | `String` の文字の本体、`Vec` の要素の並び、`Box` の中身（UNIT 08） |

次に、スコープを抜けたときに何が起きるかを、コマ送りで見ます。これは後で動かす `u01_drop_order.rs` の動きをそのまま絵にしたものです。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="7" y="8" width="134" height="254" rx="8" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="18" y="30" font-size="13" font-weight="700" fill="#0f172a">① let _a</text>
<text x="18" y="48" font-size="12" fill="#475569">a を作る</text>
<rect x="19" y="60" width="110" height="150" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<rect x="31" y="172" width="86" height="30" rx="4" fill="#ffffff" stroke="#1e40af"/>
<text x="74" y="192" text-anchor="middle" font-size="13" fill="#0f172a">a</text>
<text x="18" y="232" font-size="12" fill="#047857">作成: a</text>
<rect x="150" y="8" width="134" height="254" rx="8" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="161" y="30" font-size="13" font-weight="700" fill="#0f172a">② 内側の { }</text>
<text x="161" y="48" font-size="12" fill="#475569">b と c を作る</text>
<rect x="162" y="60" width="110" height="150" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<rect x="174" y="172" width="86" height="30" rx="4" fill="#ffffff" stroke="#1e40af"/>
<text x="217" y="192" text-anchor="middle" font-size="13" fill="#0f172a">a</text>
<rect x="168" y="96" width="98" height="72" rx="4" fill="none" stroke="#10b981" stroke-dasharray="4 3"/>
<rect x="174" y="134" width="86" height="30" rx="4" fill="#ffffff" stroke="#1e40af"/>
<text x="217" y="154" text-anchor="middle" font-size="13" fill="#0f172a">b</text>
<rect x="174" y="100" width="86" height="30" rx="4" fill="#ffffff" stroke="#1e40af"/>
<text x="217" y="120" text-anchor="middle" font-size="13" fill="#0f172a">c</text>
<text x="217" y="88" text-anchor="middle" font-size="12" fill="#047857">内側のスコープ</text>
<text x="161" y="232" font-size="12" fill="#047857">作成: b</text>
<text x="161" y="250" font-size="12" fill="#047857">作成: c</text>
<rect x="293" y="8" width="134" height="254" rx="8" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="304" y="30" font-size="13" font-weight="700" fill="#0f172a">③ 内側の }</text>
<text x="304" y="48" font-size="12" fill="#475569">c → b の順に片付け</text>
<rect x="305" y="60" width="110" height="150" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<rect x="317" y="172" width="86" height="30" rx="4" fill="#ffffff" stroke="#1e40af"/>
<text x="360" y="192" text-anchor="middle" font-size="13" fill="#0f172a">a</text>
<rect x="317" y="134" width="86" height="30" rx="4" fill="#f1f5f9" stroke="#94a3b8" stroke-dasharray="5 3"/>
<text x="360" y="154" text-anchor="middle" font-size="13" fill="#94a3b8">b</text>
<text x="388" y="156" font-size="16" font-weight="700" fill="#dc2626">✕</text>
<text x="323" y="155" font-size="12" font-weight="700" fill="#c2410c">2</text>
<rect x="317" y="100" width="86" height="30" rx="4" fill="#f1f5f9" stroke="#94a3b8" stroke-dasharray="5 3"/>
<text x="360" y="120" text-anchor="middle" font-size="13" fill="#94a3b8">c</text>
<text x="388" y="122" font-size="16" font-weight="700" fill="#dc2626">✕</text>
<text x="323" y="121" font-size="12" font-weight="700" fill="#c2410c">1</text>
<text x="304" y="232" font-size="12" fill="#c2410c">片付け: c</text>
<text x="304" y="250" font-size="12" fill="#c2410c">片付け: b</text>
<rect x="436" y="8" width="134" height="254" rx="8" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="447" y="30" font-size="13" font-weight="700" fill="#0f172a">④ drop(d)</text>
<text x="447" y="48" font-size="12" fill="#475569">d をすぐ片付け</text>
<rect x="448" y="60" width="110" height="150" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<rect x="460" y="172" width="86" height="30" rx="4" fill="#ffffff" stroke="#1e40af"/>
<text x="503" y="192" text-anchor="middle" font-size="13" fill="#0f172a">a</text>
<rect x="460" y="134" width="86" height="30" rx="4" fill="#f1f5f9" stroke="#94a3b8" stroke-dasharray="5 3"/>
<text x="503" y="154" text-anchor="middle" font-size="13" fill="#94a3b8">d</text>
<text x="531" y="156" font-size="16" font-weight="700" fill="#dc2626">✕</text>
<text x="447" y="232" font-size="12" fill="#047857">作成: d</text>
<text x="447" y="250" font-size="12" fill="#c2410c">片付け: d</text>
<rect x="579" y="8" width="134" height="254" rx="8" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="590" y="30" font-size="13" font-weight="700" fill="#0f172a">⑤ main の }</text>
<text x="590" y="48" font-size="12" fill="#475569">最後に a を片付け</text>
<rect x="591" y="60" width="110" height="150" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<rect x="603" y="172" width="86" height="30" rx="4" fill="#f1f5f9" stroke="#94a3b8" stroke-dasharray="5 3"/>
<text x="646" y="192" text-anchor="middle" font-size="13" fill="#94a3b8">a</text>
<text x="674" y="194" font-size="16" font-weight="700" fill="#dc2626">✕</text>
<text x="590" y="232" font-size="12" fill="#475569">main 終了</text>
<text x="590" y="250" font-size="12" fill="#c2410c">片付け: a</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-2: スコープとDropのコマ送り。後から作った値ほど先に片付けられる（積み上げた書類を上から片付けるのと同じ）</div>
</div>

ポイントは、**片付けの順番が「作った順の逆」**になっていることです。後から作った値は、先に作った値を頼りにしているかもしれません（たとえば「接続」を作ってから、その接続を使う「問い合わせ」を作る）。逆順に片付ければ、頼りにしている相手が先に消えてしまう心配がありません。

### まず動かす最小コード

#### 例1: スタックとヒープを覗いてみる

> **▼ このコードがやること（先に日本語で）:** 整数と文字列を1つずつ作り、文字列の「長さ」と「借りている容量」を表示します。文字を足すと長さと容量が増えること、それでも `String` の持ち手の大きさ（24バイト）は変わらないことを確かめます。

`src/bin/u01_stack_heap.rs` という名前で保存します。

```rust
use std::mem::size_of;

fn main() {
    // 整数は大きさが決まっているので、値そのものがスタックに置かれる
    let n: i32 = 5;
    // String は「持ち手」(ptr・len・cap) がスタックに、文字の本体がヒープに置かれる
    let mut s = String::from("hello");

    println!("n = {n}");
    println!("s = {s} / len = {} / cap = {}", s.len(), s.capacity());

    // 文字を足すと、ヒープの本体が大きくなる
    s.push_str(", world");
    println!("s = {s} / len = {} / cap = {}", s.len(), s.capacity());

    // 持ち手の大きさは、中身の文字数に関係なくいつも同じ
    println!("i32 の大きさ: {} バイト", size_of::<i32>());
    println!("String の持ち手の大きさ: {} バイト", size_of::<String>());
}
```

```powershell
cargo run --bin u01_stack_heap
```

**▼ 実行結果**

```text
n = 5
s = hello / len = 5 / cap = 5
s = hello, world / len = 12 / cap = 12
i32 の大きさ: 4 バイト
String の持ち手の大きさ: 24 バイト
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `size_of` を使えるようにする

```rust
use std::mem::size_of;
```

- `use` は「長い名前を短く呼べるようにする」宣言です。`std::mem::size_of` は標準ライブラリ（`std`）の `mem`（メモリ）モジュールにある関数で、**型の大きさ（バイト数）**を教えてくれます。
- 使うときは `size_of::<型>()` と書きます。`::<i32>` の部分は「どの型の大きさを調べるか」の指定です。

##### 解説2: `len` と `capacity`

```rust
println!("s = {s} / len = {} / cap = {}", s.len(), s.capacity());
```

- `s.len()` は**今入っている文字のバイト数**、`s.capacity()` は**ヒープに確保してある棚の大きさ**（容量）です。図1-1の `len` と `cap` にあたります。
- `String::from("hello")` で作った直後は、ぴったり5バイトの棚を借りています。

##### 解説3: 文字を足すと何が起きるか

```rust
s.push_str(", world");
```

- `push_str` は末尾に文字列を足すメソッドです。12バイトになると5バイトの棚では足りないので、`String` は**より大きな棚を借り直し**、中身を移してから古い棚を返します（その場で棚を広げられる場合もあります）。
- 実行結果では `cap = 12` になりました。どれだけ大きく借り直すかは標準ライブラリの内部の工夫で、版によって変わることがあります。数字そのものより「`len` が増え、必要なら `cap` も増える」ことを押さえてください。

##### 解説4: 持ち手の大きさは24バイト

```rust
println!("String の持ち手の大きさ: {} バイト", size_of::<String>());
```

- `String` の持ち手は「`ptr`（8バイト）＋`len`（8バイト）＋`cap`（8バイト）」で24バイトです。中身が1文字でも100万文字でも変わりません。
- これが「`String` 自体はスタックに置けるが、文字の本体はヒープ」という仕組みの正体です。

#### 例2: 片付けの順番を見る

> **▼ このコードがやること（先に日本語で）:** 作られたときと片付けられたときにメッセージを出す、実験用の型 `Noisy` を作ります。ブロック `{ }` の中と外で値を作り、どの順番で片付けられるかを画面で確かめます。図1-2のコマ送りそのものです。

`src/bin/u01_drop_order.rs`:

```rust
// 片付けられるときにメッセージを出す、実験用の型
struct Noisy {
    name: String,
}

impl Noisy {
    fn new(name: &str) -> Noisy {
        println!("  作成: {name}");
        Noisy {
            name: name.to_string(),
        }
    }
}

// Drop を実装すると「片付けのときに実行する処理」を書ける
impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  片付け: {}", self.name);
    }
}

fn main() {
    println!("main 開始");
    let _a = Noisy::new("a");
    {
        println!("内側のブロック開始");
        let _b = Noisy::new("b");
        let _c = Noisy::new("c");
        println!("内側のブロック終了");
    }
    let d = Noisy::new("d");
    // drop(値) で、スコープの終わりを待たずに今すぐ片付ける
    drop(d);
    println!("main 終了");
}
```

```powershell
cargo run --bin u01_drop_order
```

**▼ 実行結果**

```text
main 開始
  作成: a
内側のブロック開始
  作成: b
  作成: c
内側のブロック終了
  片付け: c
  片付け: b
  作成: d
  片付け: d
main 終了
  片付け: a
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `Drop` トレイトを実装する

```rust
impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  片付け: {}", self.name);
    }
}
```

- **トレイト**（trait：型に持たせる「約束ごと」のこと。第4巻で詳しく学びます）の1つ `Drop` を `Noisy` に実装しています。`Drop` を実装すると、**その値が片付けられる直前に `drop` メソッドが自動で呼ばれます**。
- `&mut self` は「自分自身を書き換えてよい借用」という意味です（UNIT 03 で学びます）。ここでは名前を表示するだけです。
- `drop` の中身が終わると、`Noisy` が持っている `name`（`String`）も続けて片付けられ、ヒープの本体が返されます。

##### 解説2: 内側のブロックの終わりで片付けられる

```rust
    {
        println!("内側のブロック開始");
        let _b = Noisy::new("b");
        let _c = Noisy::new("c");
        println!("内側のブロック終了");
    }
```

- `_b` と `_c` は内側の `{ }` のスコープで作られたので、**その `}` に来た時点で片付けられます**。`main` の終わりまで待ちません。
- 片付けの順番は `c` → `b`。作った順（`b` → `c`）の**逆**です。
- 変数名の先頭の `_` は「この変数は使わないけど、警告を出さないで」という合図です（使わない変数があるとコンパイラが警告を出すため）。

##### 解説3: `drop(d)` で今すぐ片付ける

```rust
    let d = Noisy::new("d");
    drop(d);
```

- `drop(値)` は標準ライブラリにある関数で、**値を受け取って、その場で片付けます**。「この先はもう使わないので、すぐ返したい」ときに使います（ファイルを閉じる、ロックを外すなど）。
- 実はこの `drop` 関数の中身は空っぽです。値を受け取った関数のスコープがすぐ終わるので、そこで片付けられる、という仕組みです。この「値を関数に渡すと持ち主が移る」話は UNIT 02 で詳しく見ます。

##### 解説4: `main` の終わり

- 最後に `main` の `}` で `_a` が片付けられます。`d` はすでに片付け済みなので、2回片付けられることはありません。**片付けはどの値もちょうど1回**です。

### もう一歩: 実務に近い形

`Drop` の本当の力は、「途中で `return` しても、エラーで抜けても、**必ず**片付けが走る」ことです。これを使った設計を **RAII**（アール・エー・アイ・アイ：Resource Acquisition Is Initialization の略。「資源の確保を値の作成と結びつけ、解放を値の片付けと結びつける」という考え方）と呼びます。

データベースの**トランザクション**（いくつかの書き込みを「全部成功」か「全部取り消し」のどちらかにまとめる仕組み）を真似した例で見てみましょう。

#### 例3: commit しないまま抜けたら自動で取り消す

> **▼ このコードがやること（先に日本語で）:** 「BEGIN（開始）」した取引を、最後に「COMMIT（確定）」しないまま関数を抜けたら、自動で「ROLLBACK（取り消し）」するようにします。タイトルが空のときは途中で `return` しますが、取り消しの処理を書き忘れる心配がありません。

`src/bin/u01_transaction.rs`:

```rust
// データベースの「トランザクション」を真似した型
struct Transaction {
    id: u32,
    committed: bool,
}

impl Transaction {
    fn begin(id: u32) -> Transaction {
        println!("[{id}] BEGIN");
        Transaction {
            id,
            committed: false,
        }
    }

    // self を丸ごと受け取るので、commit した後の tx はもう使えない
    fn commit(mut self) {
        println!("[{}] COMMIT", self.id);
        self.committed = true;
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        // commit されないまま片付けられたら、取り消す
        if !self.committed {
            println!("[{}] ROLLBACK", self.id);
        }
    }
}

fn save_note(id: u32, title: &str) -> Result<(), String> {
    let tx = Transaction::begin(id);
    if title.is_empty() {
        // ここで return しても、tx の片付け (drop) は自動で走る
        return Err(String::from("タイトルが空です"));
    }
    println!("[{id}] INSERT INTO notes ... ({title})");
    tx.commit();
    Ok(())
}

fn main() {
    println!("{:?}", save_note(1, "買い物メモ"));
    println!("{:?}", save_note(2, ""));
}
```

**▼ 実行結果**

```text
[1] BEGIN
[1] INSERT INTO notes ... (買い物メモ)
[1] COMMIT
Ok(())
[2] BEGIN
[2] ROLLBACK
Err("タイトルが空です")
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `commit(mut self)` は自分を丸ごと受け取る

```rust
    fn commit(mut self) {
        println!("[{}] COMMIT", self.id);
        self.committed = true;
    }
```

- 引数が `&self` ではなく `self` です。これは「メソッドを呼ぶと、`tx` という値そのものが `commit` の中へ渡される」という意味です。`commit` が終わると、渡された値はそこで片付けられます。
- 片付けのときには `committed` が `true` になっているので、`drop` の中の `ROLLBACK` は表示されません。
- そして `tx.commit()` の後で `tx` を使おうとすると、コンパイルエラーになります。「確定した取引をもう一度使う」というミスを、**型で防いでいる**わけです。

##### 解説2: 途中の `return` でも片付けが走る

```rust
    if title.is_empty() {
        return Err(String::from("タイトルが空です"));
    }
```

- `return` で関数を抜けると、`tx` のスコープも終わります。すると Rust が自動で `drop` を呼び、`committed` が `false` なので `ROLLBACK` が表示されます。
- もし `Drop` がなかったら、`return` するすべての場所に「取り消し」の処理を手で書く必要があります。1か所でも忘れると、中途半端な書き込みが残ってしまいます。

> **つまずきポイント: `let _ = 値;` はその場で片付けられる**
>
> 「使わない変数だから」と `let _ = ...;` と書くと、**名前のない値はその文の終わりですぐ片付けられます**。ロックやトランザクションを守る「ガード」の値でこれをやると、守ったつもりで何も守れていない、というバグになります。`_guard` のように `_` で始まる**名前**を付ければ、スコープの終わりまで生きます。

> **▼ このコードがやること（先に日本語で）:** `let _ = ...` と `let _guard = ...` で、片付けられるタイミングがどう違うかを確かめます。

`src/bin/u01_let_underscore.rs`:

```rust
struct Guard {
    name: String,
}

impl Drop for Guard {
    fn drop(&mut self) {
        println!("  解放: {}", self.name);
    }
}

fn main() {
    println!("1. let _ = ...");
    // 名前を付けないと、その場ですぐ片付けられる
    let _ = Guard {
        name: String::from("ガード1"),
    };
    println!("   ↑ もう解放されている");

    println!("2. let _guard = ...");
    // _ で始まる名前を付けると、スコープの終わりまで生きる
    let _guard = Guard {
        name: String::from("ガード2"),
    };
    println!("   ↑ まだ解放されていない");
    println!("main 終了");
}
```

**▼ 実行結果**

```text
1. let _ = ...
  解放: ガード1
   ↑ もう解放されている
2. let _guard = ...
   ↑ まだ解放されていない
main 終了
  解放: ガード2
```

### よくあるエラーと読み方

#### エラー1: `drop` メソッドを直接呼ぶ（E0040）

`Drop` を実装したからといって、`a.drop()` と自分で呼ぶことはできません。

```rust
struct Noisy;

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("片付け");
    }
}

fn main() {
    let a = Noisy;
    a.drop();
}
```

```text
error[E0040]: explicit use of destructor method
  --> src\bin\u01_err_explicit_drop.rs:11:7
   |
11 |     a.drop();
   |       ^^^^ explicit destructor calls not allowed
   |
help: consider using `drop` function
   |
11 -     a.drop();
11 +     drop(a);
   |

For more information about this error, try `rustc --explain E0040`.
error: could not compile `ownership-lab` (bin "u01_err_explicit_drop") due to 1 previous error
```

- **何が起きたか:** 「デストラクタ（片付け用のメソッド）を明示的に呼ぶことはできません」というエラーです。もし `a.drop()` を許すと、その後 `main` の終わりでもう一度自動で `drop` が呼ばれ、**片付けが2回**になってしまいます。
- **直し方:** `help:` の提案どおり、`drop(a)` と**関数**の方を使います。`drop(a)` は `a` そのものを受け取るので、その後 `a` は使えなくなり、自動の片付けも二重に起きません。

> **ヒント: エラー文の読み方** … `-->` の後ろが「ファイル名:行:列」です。`^^^^` が問題の場所、`-` の線が「関係する場所」を指します。`help:` には直し方の候補が、`-` 行（消す）と `+` 行（足す）の形で示されます。

#### エラー2: 片付けた後に使う（E0382）

```rust
fn main() {
    let s = String::from("hello");
    drop(s);
    println!("{s}");
}
```

```text
error[E0382]: borrow of moved value: `s`
 --> src\bin\u01_err_use_after_drop.rs:4:16
  |
2 |     let s = String::from("hello");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3 |     drop(s);
  |          - value moved here
4 |     println!("{s}");
  |                ^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     drop(s.clone());
  |           ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership-lab` (bin "u01_err_use_after_drop") due to 1 previous error
```

- **何が起きたか:** 3行目で `s` を `drop` 関数に渡した時点で、`s` の持ち主は `drop` 関数に移り（**move**）、そこで片付けられました。4行目で `s` を使おうとしても、もう `s` は何も持っていません。片付け済みの倉庫の棚を見に行くのと同じで、とても危険です。
- **読み方:** 下から読むと分かりやすいです。「4行目で借りようとしたが（`value borrowed here after move`）、3行目で移動済み（`value moved here`）。`String` は `Copy` を実装していないので代入や受け渡しで移動する（2行目の説明）」。
- **直し方:** 片付けた後で使う必要があるなら、`drop` する位置を後ろにずらします。この E0382 は Rust で最も多いエラーの1つなので、次の UNIT 02 で詳しく扱います。

### RustNote ではここで使う

この教材を通して作るメモアプリ **RustNote** でも、「スコープの終わりで自動で片付く」性質に何度も助けられます。

- **リクエストごとの値:** ログインフォームから届いたメールアドレスやパスワードの `String` は、リクエストを処理する関数（ハンドラ）が終わると自動で片付けられます。後片付けのコードは1行も書きません。
- **データベースのトランザクション:** 「ノートを保存して、更新日時も書き換える」のような複数の書き込みは、トランザクションでまとめます。DB ライブラリの多くは、例3のように「commit しないまま片付けられたら取り消す」設計になっています。RustNote で使う libSQL での具体的な書き方は第7巻で確認します。
- **ロック:** UNIT 10 で学ぶ `Mutex` の鍵も、「鍵を表す値（ガード）が片付けられたら鍵を返す」という RAII の仕組みでできています。

### 練習問題

**問題1（予想）** 次のプログラムの出力を、実行する前に紙に書いてください。

```rust
struct Noisy {
    name: String,
}

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.name);
    }
}

fn make(name: &str) -> Noisy {
    Noisy {
        name: name.to_string(),
    }
}

fn main() {
    let _x = make("x");
    {
        let _y = make("y");
        let z = make("z");
        drop(z);
        println!("block end");
    }
    let _w = make("w");
    println!("main end");
}
```

**問題2（作る）** 作成時に「ロック取得: 名前」、片付け時に「ロック解除: 名前」と表示する `LockGuard` 型を作ってください。そして、負の数を渡すと途中で `Err` を返す関数 `update(value: i32) -> Result<i32, String>` の先頭で `LockGuard` を作り、**どちらの場合も「ロック解除」が表示される**ことを確かめてください。

**問題3（説明）** `String` の中身が `"a"` でも、長い小説1冊分でも、`size_of::<String>()` が同じ 24 になるのはなぜですか。図1-1を使って説明してください。

### 解答と解説

**問題1の解答**（`src/bin/u01_ex1.rs` で確認）

```text
drop z
block end
drop y
main end
drop w
drop x
```

- `z` は `drop(z)` でその場で片付けられるので、`block end` より先に `drop z` が出ます。
- 内側のブロックの `}` で、残っていた `_y` が片付けられます。
- `main` の `}` では、後から作った `_w` が先、`_x` が最後です（作った順の逆）。

**問題2の解答例**（`src/bin/u01_ex2.rs`）

```rust
struct LockGuard {
    resource: String,
}

impl LockGuard {
    fn acquire(resource: &str) -> LockGuard {
        println!("ロック取得: {resource}");
        LockGuard {
            resource: resource.to_string(),
        }
    }
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        println!("ロック解除: {}", self.resource);
    }
}

fn update(value: i32) -> Result<i32, String> {
    let _guard = LockGuard::acquire("notes");
    if value < 0 {
        return Err(String::from("負の数は保存できません"));
    }
    println!("更新: {value}");
    Ok(value)
}

fn main() {
    println!("{:?}", update(5));
    println!("{:?}", update(-1));
}
```

**▼ 実行結果**

```text
ロック取得: notes
更新: 5
ロック解除: notes
Ok(5)
ロック取得: notes
ロック解除: notes
Err("負の数は保存できません")
```

- `let _guard` と**名前を付ける**のが大事です。`let _ =` にすると、取得した直後に解除されてしまいます（つまずきポイント参照）。
- `return Err(...)` で抜けた場合も、`_guard` のスコープが終わるので必ず「ロック解除」が表示されます。

**問題3の解答例**

`String` の値そのもの（スタックにある部分）は、「本体の場所 `ptr`」「長さ `len`」「容量 `cap`」の3つの数だけでできていて、64ビット環境ではそれぞれ8バイト、合計24バイトです。文字の本体はヒープにあり、`String` の値はその場所を指しているだけなので、中身が何文字でも持ち手の大きさは変わりません。

### まとめ

- 大きさの決まった値はスタック、大きさが変わる本体（`String` の文字など）はヒープに置かれ、スタックの持ち手がヒープを指す。
- 値はスコープの終わりで、作った順の逆に、ちょうど1回だけ片付けられる（Drop）。早く片付けたいときは `drop(値)`。
- `Drop` を使うと、途中で `return` しても後片付けが必ず走る（RAII）。ガードには `let _guard` のように名前を付ける。

## UNIT 02: Move・Copy・Clone

### この単元のゴール

- `let s2 = s1;` のような代入や関数呼び出しで、`String` の**持ち主が移る（move）**ことを図で説明できる
- move の後に元の変数を使うと E0382 になる理由を、「二重解放」という言葉を使って説明できる
- `i32` などの **Copy 型**と、`clone()` による**明示的な複製**の違い、それぞれの費用が分かる
- 関数に値を「渡す」「返す」ときに、持ち主がどう動くかを追える

### たとえ話でつかむ

あなたが図書館の本ではなく、**自分で買った本**を持っているとします。

- **move（移動）= 本を手渡す** ことです。友だちに本を渡したら、あなたの手にはもう本はありません。本はこの世に1冊のままで、持ち主が変わっただけです。手渡しは一瞬で終わります（本を書き写したりしません）。
- **Copy（コピー）= メモ用紙に書いた数字を書き写す** ことです。「42」と書いたメモを友だちも欲しがったら、別のメモに書き写して渡します。あなたのメモも手元に残ります。数字は小さいので書き写すのは一瞬です。
- **clone（クローン）= 本を丸ごとコピー機で複製する** ことです。分厚い本なら時間も紙代もかかります。だから「複製してね」と**わざわざ頼んだときだけ**行います。

では、なぜ `String` の代入は「書き写し」ではなく「手渡し」なのでしょうか。UNIT 01 で見たとおり、`String` は「スタックの持ち手（番号札）」と「ヒープの本体」でできています。代入で持ち手だけを書き写すと、**同じ倉庫の棚を指す番号札が2枚**できてしまいます。2人とも「自分が持ち主だ」と思っていたら、2人ともスコープの終わりで棚を返そうとして、**二重解放**が起きます。

そこで Rust は「番号札を書き写したら、**元の番号札は無効にする**」と決めました。これが move です。

### 図で理解する

`let s2 = s1;` の前後を、1行ずつコマ送りで見ます。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-03-06" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="4" y="6" width="232" height="278" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="14" y="26" font-size="13" font-weight="700" fill="#0f172a">①</text>
<text x="32" y="26" font-size="12" fill="#0f172a" font-family="Consolas, Menlo, monospace">let s1 = String::from(..);</text>
<rect x="14" y="40" width="110" height="200" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="22" y="58" font-size="12" font-weight="700" fill="#1e40af">スタック</text>
<text x="20" y="96" font-size="13" font-weight="700" fill="#0f172a">s1</text>
<rect x="44" y="72" width="72" height="66" rx="3" fill="#ffffff" stroke="#1e40af"/>
<line x1="44" y1="94" x2="116" y2="94" stroke="#bfdbfe"/>
<line x1="44" y1="116" x2="116" y2="116" stroke="#bfdbfe"/>
<text x="50" y="88" font-size="12" fill="#0f172a">ptr</text>
<circle cx="106" cy="83" r="3.5" fill="#475569"/>
<text x="50" y="110" font-size="12" fill="#0f172a">len 5</text>
<text x="50" y="132" font-size="12" fill="#0f172a">cap 5</text>
<rect x="134" y="40" width="92" height="200" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="142" y="58" font-size="12" font-weight="700" fill="#c2410c">ヒープ</text>
<rect x="148" y="122" width="66" height="26" fill="#ffffff" stroke="#c2410c"/>
<text x="181" y="140" text-anchor="middle" font-size="13" fill="#0f172a" font-family="Consolas, Menlo, monospace">hello</text>
<line x1="106" y1="83" x2="146" y2="128" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-06)"/>
<text x="120" y="262" text-anchor="middle" font-size="12" fill="#1e40af">持ち主は s1</text>
<rect x="244" y="6" width="232" height="278" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="254" y="26" font-size="13" font-weight="700" fill="#0f172a">②</text>
<text x="272" y="26" font-size="12" fill="#0f172a" font-family="Consolas, Menlo, monospace">let s2 = s1;</text>
<rect x="254" y="40" width="110" height="200" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="262" y="58" font-size="12" font-weight="700" fill="#1e40af">スタック</text>
<text x="260" y="96" font-size="13" font-weight="700" fill="#0f172a">s1</text>
<rect x="284" y="72" width="72" height="66" rx="3" fill="#ffffff" stroke="#1e40af"/>
<line x1="284" y1="94" x2="356" y2="94" stroke="#bfdbfe"/>
<line x1="284" y1="116" x2="356" y2="116" stroke="#bfdbfe"/>
<text x="290" y="88" font-size="12" fill="#0f172a">ptr</text>
<circle cx="346" cy="83" r="3.5" fill="#475569"/>
<text x="290" y="110" font-size="12" fill="#0f172a">len 5</text>
<text x="290" y="132" font-size="12" fill="#0f172a">cap 5</text>
<text x="260" y="186" font-size="13" font-weight="700" fill="#0f172a">s2</text>
<rect x="284" y="162" width="72" height="66" rx="3" fill="#ffffff" stroke="#1e40af"/>
<line x1="284" y1="184" x2="356" y2="184" stroke="#bfdbfe"/>
<line x1="284" y1="206" x2="356" y2="206" stroke="#bfdbfe"/>
<text x="290" y="178" font-size="12" fill="#0f172a">ptr</text>
<circle cx="346" cy="173" r="3.5" fill="#475569"/>
<text x="290" y="200" font-size="12" fill="#0f172a">len 5</text>
<text x="290" y="222" font-size="12" fill="#0f172a">cap 5</text>
<text x="320" y="154" text-anchor="middle" font-size="11" fill="#c2410c">↓ 24B をコピー</text>
<rect x="374" y="40" width="92" height="200" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="382" y="58" font-size="12" font-weight="700" fill="#c2410c">ヒープ</text>
<rect x="388" y="122" width="66" height="26" fill="#ffffff" stroke="#c2410c"/>
<text x="421" y="140" text-anchor="middle" font-size="13" fill="#0f172a" font-family="Consolas, Menlo, monospace">hello</text>
<line x1="346" y1="83" x2="386" y2="128" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-06)"/>
<line x1="346" y1="173" x2="386" y2="144" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-06)"/>
<text x="421" y="176" text-anchor="middle" font-size="11" fill="#c2410c">本体は1つ</text>
<text x="360" y="262" text-anchor="middle" font-size="12" fill="#c2410c">一瞬、2枚の札が同じ本体を指す</text>
<rect x="484" y="6" width="232" height="278" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="494" y="26" font-size="13" font-weight="700" fill="#0f172a">③</text>
<text x="512" y="26" font-size="12" fill="#0f172a">s1 を無効にする</text>
<rect x="494" y="40" width="110" height="200" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="502" y="58" font-size="12" font-weight="700" fill="#1e40af">スタック</text>
<text x="500" y="96" font-size="13" font-weight="700" fill="#94a3b8">s1</text>
<rect x="524" y="72" width="72" height="66" rx="3" fill="#f1f5f9" stroke="#94a3b8" stroke-dasharray="5 3"/>
<text x="560" y="102" text-anchor="middle" font-size="12" fill="#94a3b8">無効</text>
<text x="560" y="120" text-anchor="middle" font-size="11" fill="#94a3b8">（使えない）</text>
<text x="500" y="186" font-size="13" font-weight="700" fill="#0f172a">s2</text>
<rect x="524" y="162" width="72" height="66" rx="3" fill="#ffffff" stroke="#1e40af"/>
<line x1="524" y1="184" x2="596" y2="184" stroke="#bfdbfe"/>
<line x1="524" y1="206" x2="596" y2="206" stroke="#bfdbfe"/>
<text x="530" y="178" font-size="12" fill="#0f172a">ptr</text>
<circle cx="586" cy="173" r="3.5" fill="#475569"/>
<text x="530" y="200" font-size="12" fill="#0f172a">len 5</text>
<text x="530" y="222" font-size="12" fill="#0f172a">cap 5</text>
<rect x="614" y="40" width="92" height="200" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="622" y="58" font-size="12" font-weight="700" fill="#c2410c">ヒープ</text>
<rect x="628" y="122" width="66" height="26" fill="#ffffff" stroke="#c2410c"/>
<text x="661" y="140" text-anchor="middle" font-size="13" fill="#0f172a" font-family="Consolas, Menlo, monospace">hello</text>
<line x1="586" y1="173" x2="626" y2="144" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-06)"/>
<text x="600" y="262" text-anchor="middle" font-size="12" fill="#1e40af">持ち主は s2 だけ</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図2-1: move のコマ送り。コピーされるのは 24 バイトの持ち手だけで、ヒープの本体は動かない。元の s1 は無効になる</div>
</div>

もし③で `s1` を無効にしなかったら、どうなるでしょうか。左側がその「もしも」の世界、右側が `clone()` で本当に複製した場合です。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-03-07" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="4" y="6" width="352" height="258" rx="10" fill="#fef2f2" stroke="#fca5a5"/>
<text x="16" y="28" font-size="13" font-weight="700" fill="#b91c1c">もしも: s1 も s2 も有効なままだったら</text>
<rect x="16" y="42" width="120" height="150" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="24" y="60" font-size="12" font-weight="700" fill="#1e40af">スタック</text>
<text x="24" y="96" font-size="13" font-weight="700" fill="#0f172a">s1</text>
<rect x="50" y="78" width="74" height="28" rx="3" fill="#ffffff" stroke="#1e40af"/>
<text x="58" y="97" font-size="12" fill="#0f172a">ptr</text>
<circle cx="112" cy="92" r="3.5" fill="#475569"/>
<text x="24" y="156" font-size="13" font-weight="700" fill="#0f172a">s2</text>
<rect x="50" y="138" width="74" height="28" rx="3" fill="#ffffff" stroke="#1e40af"/>
<text x="58" y="157" font-size="12" fill="#0f172a">ptr</text>
<circle cx="112" cy="152" r="3.5" fill="#475569"/>
<rect x="196" y="42" width="148" height="150" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="204" y="60" font-size="12" font-weight="700" fill="#c2410c">ヒープ</text>
<rect x="236" y="108" width="70" height="26" fill="#ffffff" stroke="#c2410c"/>
<text x="271" y="126" text-anchor="middle" font-size="13" fill="#0f172a" font-family="Consolas, Menlo, monospace">hello</text>
<line x1="112" y1="92" x2="234" y2="116" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-07)"/>
<line x1="112" y1="152" x2="234" y2="128" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-07)"/>
<text x="316" y="128" font-size="22" font-weight="700" fill="#dc2626">✕</text>
<text x="16" y="216" font-size="12" fill="#b91c1c">スコープの終わりで s2 が本体を返す</text>
<text x="16" y="234" font-size="12" fill="#b91c1c">→ 続けて s1 も同じ本体を返す</text>
<text x="16" y="254" font-size="13" font-weight="700" fill="#b91c1c">→ 二重解放。Rust はこれをコンパイル時に禁止</text>
<rect x="364" y="6" width="352" height="258" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="376" y="28" font-size="13" font-weight="700" fill="#0f172a" font-family="Consolas, Menlo, monospace">let s3 = s2.clone();</text>
<rect x="376" y="42" width="120" height="150" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="384" y="60" font-size="12" font-weight="700" fill="#1e40af">スタック</text>
<text x="384" y="96" font-size="13" font-weight="700" fill="#0f172a">s2</text>
<rect x="410" y="78" width="74" height="28" rx="3" fill="#ffffff" stroke="#1e40af"/>
<text x="418" y="97" font-size="12" fill="#0f172a">ptr</text>
<circle cx="472" cy="92" r="3.5" fill="#475569"/>
<text x="384" y="156" font-size="13" font-weight="700" fill="#0f172a">s3</text>
<rect x="410" y="138" width="74" height="28" rx="3" fill="#ffffff" stroke="#1e40af"/>
<text x="418" y="157" font-size="12" fill="#0f172a">ptr</text>
<circle cx="472" cy="152" r="3.5" fill="#475569"/>
<rect x="556" y="42" width="148" height="150" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="564" y="60" font-size="12" font-weight="700" fill="#c2410c">ヒープ</text>
<rect x="596" y="78" width="70" height="26" fill="#ffffff" stroke="#c2410c"/>
<text x="631" y="96" text-anchor="middle" font-size="13" fill="#0f172a" font-family="Consolas, Menlo, monospace">hello</text>
<rect x="596" y="138" width="70" height="26" fill="#ffffff" stroke="#c2410c"/>
<text x="631" y="156" text-anchor="middle" font-size="13" fill="#0f172a" font-family="Consolas, Menlo, monospace">hello</text>
<line x1="472" y1="92" x2="594" y2="91" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-07)"/>
<line x1="472" y1="152" x2="594" y2="151" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-07)"/>
<text x="376" y="216" font-size="12" fill="#047857">本体ごと複製するので、ヒープに2つの本体</text>
<text x="376" y="234" font-size="12" fill="#047857">→ s2 も s3 もそれぞれ自分の本体の持ち主</text>
<text x="376" y="254" font-size="13" font-weight="700" fill="#047857">→ 安全。ただし複製の費用がかかる</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図2-2: 左は「move しなかったら」起きる二重解放。右は clone で本体ごと複製した状態</div>
</div>

それに対して `i32` のような値は、**値そのものがスタックにあるだけ**で、ヒープに本体を持っていません。書き写しても「同じ本体を2人が指す」ことが起きないので、代入しても元の変数は使えたままです。こういう型を **Copy 型**と呼びます。

| 分類 | 代入・関数に渡したとき | 主な型 |
|---|---|---|
| Copy 型 | 値が書き写され、元も使える | 整数（`i32`・`u64` など）、小数（`f64`）、`bool`、`char`、共有参照 `&T`、Copy 型だけでできたタプル・配列 |
| Copy でない型 | 持ち主が移り（move）、元は使えなくなる | `String`、`Vec<T>`、`Box<T>`、`HashMap`、可変参照 `&mut T`、`String` などを含む struct |

> **用語: Copy と Clone** … どちらも「複製できる」ことを表すトレイト（約束ごと）です。**Copy** は「書き写すだけで安全・安価に複製できる」型にだけ付けられ、代入のたびに自動で複製されます。**Clone** は「`.clone()` と書いたときに複製できる」型に付けられ、費用がかかるかもしれないので**自分で明示的に呼ぶ**必要があります。Copy 型はすべて Clone でもあります。

### まず動かす最小コード

#### 例1: move・clone・Copy の違いをアドレスで確かめる

> **▼ このコードがやること（先に日本語で）:** 文字列を move したとき、ヒープの本体の場所（アドレス）が変わらないことを表示して確かめます。次に clone して、本体が別の場所に作られることを見ます。最後に整数の代入では元の変数がそのまま使えることを確かめます。

`src/bin/u02_move_clone_copy.rs`:

```rust
fn main() {
    let s1 = String::from("hello");
    // as_ptr() は、ヒープにある本体の場所（アドレス）を返す
    println!("s1 の本体の場所: {:p}", s1.as_ptr());

    // s1 の持ち手が s2 へ移る（move）。本体は動かない
    let s2 = s1;
    println!("s2 の本体の場所: {:p}", s2.as_ptr());

    // clone は本体をまるごと複製する。場所が別になる
    let s3 = s2.clone();
    println!("s3 の本体の場所: {:p}", s3.as_ptr());
    println!("s2 = {s2}, s3 = {s3}");

    // i32 は Copy 型。代入しても x はそのまま使える
    let x = 42;
    let y = x;
    println!("x = {x}, y = {y}");
}
```

```powershell
cargo run --bin u02_move_clone_copy
```

**▼ 実行結果**（アドレスの数字は実行ごとに変わります）

```text
s1 の本体の場所: 0x26904108da0
s2 の本体の場所: 0x26904108da0
s3 の本体の場所: 0x26904108dc0
s2 = hello, s3 = hello
x = 42, y = 42
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `{:p}` でアドレスを表示する

```rust
println!("s1 の本体の場所: {:p}", s1.as_ptr());
```

- `as_ptr()` は、`String` の持ち手の `ptr`（ヒープの本体の場所）を取り出すメソッドです。
- `{:p}` は「ポインタ（場所）として16進数で表示する」という書式です。`0x` で始まる数字がメモリの場所を表します。

##### 解説2: move しても本体の場所は同じ

```rust
let s2 = s1;
println!("s2 の本体の場所: {:p}", s2.as_ptr());
```

- 実行結果の1行目と2行目のアドレスが**まったく同じ**です。図2-1のとおり、move でコピーされるのは24バイトの持ち手だけで、ヒープの本体は1バイトも動いていません。だから move は、文字列がどんなに長くても一瞬で終わります。
- この行より後で `s1` を使おうとすると E0382 になります（後の「よくあるエラー」で実際に見ます）。

##### 解説3: clone は本体を作り直す

```rust
let s3 = s2.clone();
```

- 3行目のアドレスは `...dc0` で、`s2` の本体（`...da0`）とは**別の場所**です。図2-2の右側のとおり、ヒープに新しい棚を借りて、文字を全部書き写しています。
- 5文字ならすぐですが、10万件のノートが入った `Vec` を `clone()` すると、10万件すべてを複製します。**clone は「費用を払ってでも、独立した複製が欲しい」ときにだけ使う**と覚えてください。

##### 解説4: Copy 型は代入しても元が残る

```rust
let x = 42;
let y = x;
println!("x = {x}, y = {y}");
```

- `i32` は Copy 型なので、`let y = x;` は「42 を書き写す」だけです。`x` も `y` もそれぞれ自分の 42 を持っていて、どちらも使えます。

#### 例2: 関数に渡す・関数から返す

move は代入だけでなく、**関数に値を渡したとき**と**関数から値を返したとき**にも起きます。

> **▼ このコードがやること（先に日本語で）:** `String` を受け取る関数に値を渡すと、呼んだ側ではもう使えなくなること、関数が `String` を返すと持ち主が呼んだ側に戻ってくることを確かめます。整数は渡しても呼んだ側で使い続けられます。

`src/bin/u02_move_fn.rs`:

```rust
// String を受け取る関数。呼んだ側から所有権が移ってくる
fn print_and_drop(s: String) {
    println!("受け取った: {s}");
    // ここで s のスコープが終わり、本体が片付けられる
}

// 作った String を呼び出し元へ返す。所有権も一緒に返る
fn make_greeting(name: &str) -> String {
    let greeting = format!("こんにちは、{name}さん");
    greeting
}

// 受け取って、手を加えて、返す
fn add_mark(mut s: String) -> String {
    s.push('!');
    s
}

// i32 は Copy 型なので、渡しても呼んだ側の値は残る
fn show_number(n: i32) {
    println!("数値: {n}");
}

fn main() {
    let memo = String::from("メモ");
    print_and_drop(memo);

    let greeting = make_greeting("佐藤");
    let greeting = add_mark(greeting);
    println!("{greeting}");

    let n = 7;
    show_number(n);
    println!("n はまだ使える: {n}");
}
```

**▼ 実行結果**

```text
受け取った: メモ
こんにちは、佐藤さん!
数値: 7
n はまだ使える: 7
```

`print_and_drop(memo)` の前後で、メモリはこう変わります。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-03-08" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="4" y="6" width="232" height="238" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="14" y="26" font-size="13" font-weight="700" fill="#0f172a">① 呼ぶ直前</text>
<rect x="14" y="38" width="124" height="170" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="22" y="56" font-size="12" font-weight="700" fill="#1e40af">スタック</text>
<rect x="20" y="130" width="112" height="70" rx="6" fill="#dbeafe" stroke="#3b82f6"/>
<text x="28" y="148" font-size="12" font-weight="700" fill="#1e40af">main</text>
<text x="28" y="180" font-size="13" fill="#0f172a">memo</text>
<rect x="72" y="164" width="52" height="26" rx="3" fill="#ffffff" stroke="#1e40af"/>
<text x="80" y="182" font-size="12" fill="#0f172a">ptr</text>
<circle cx="114" cy="177" r="3.5" fill="#475569"/>
<rect x="148" y="38" width="80" height="170" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="156" y="56" font-size="12" font-weight="700" fill="#c2410c">ヒープ</text>
<rect x="160" y="164" width="56" height="26" fill="#ffffff" stroke="#c2410c"/>
<text x="188" y="182" text-anchor="middle" font-size="13" fill="#0f172a">メモ</text>
<line x1="114" y1="177" x2="158" y2="177" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-08)"/>
<text x="120" y="230" text-anchor="middle" font-size="12" fill="#1e40af">持ち主は main の memo</text>
<rect x="244" y="6" width="232" height="238" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="254" y="26" font-size="13" font-weight="700" fill="#0f172a">② 関数の中</text>
<rect x="254" y="38" width="124" height="170" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="262" y="56" font-size="12" font-weight="700" fill="#1e40af">スタック</text>
<rect x="260" y="64" width="112" height="62" rx="6" fill="#dcfce7" stroke="#10b981"/>
<text x="266" y="80" font-size="11" font-weight="700" fill="#047857">print_and_drop</text>
<text x="268" y="111" font-size="13" fill="#0f172a">s</text>
<rect x="312" y="94" width="52" height="26" rx="3" fill="#ffffff" stroke="#1e40af"/>
<text x="320" y="112" font-size="12" fill="#0f172a">ptr</text>
<circle cx="354" cy="107" r="3.5" fill="#475569"/>
<rect x="260" y="130" width="112" height="70" rx="6" fill="#dbeafe" stroke="#3b82f6"/>
<text x="268" y="148" font-size="12" font-weight="700" fill="#1e40af">main</text>
<text x="268" y="180" font-size="13" fill="#94a3b8">memo</text>
<rect x="312" y="164" width="52" height="26" rx="3" fill="#f1f5f9" stroke="#94a3b8" stroke-dasharray="5 3"/>
<text x="338" y="182" text-anchor="middle" font-size="12" fill="#94a3b8">無効</text>
<rect x="388" y="38" width="80" height="170" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="396" y="56" font-size="12" font-weight="700" fill="#c2410c">ヒープ</text>
<rect x="400" y="94" width="56" height="26" fill="#ffffff" stroke="#c2410c"/>
<text x="428" y="112" text-anchor="middle" font-size="13" fill="#0f172a">メモ</text>
<line x1="354" y1="107" x2="398" y2="107" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-08)"/>
<text x="360" y="230" text-anchor="middle" font-size="12" fill="#047857">持ち主は関数の引数 s</text>
<rect x="484" y="6" width="232" height="238" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="494" y="26" font-size="13" font-weight="700" fill="#0f172a">③ 関数から戻った</text>
<rect x="494" y="38" width="124" height="170" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="502" y="56" font-size="12" font-weight="700" fill="#1e40af">スタック</text>
<rect x="500" y="130" width="112" height="70" rx="6" fill="#dbeafe" stroke="#3b82f6"/>
<text x="508" y="148" font-size="12" font-weight="700" fill="#1e40af">main</text>
<text x="508" y="180" font-size="13" fill="#94a3b8">memo</text>
<rect x="552" y="164" width="52" height="26" rx="3" fill="#f1f5f9" stroke="#94a3b8" stroke-dasharray="5 3"/>
<text x="578" y="182" text-anchor="middle" font-size="12" fill="#94a3b8">無効</text>
<rect x="628" y="38" width="80" height="170" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="636" y="56" font-size="12" font-weight="700" fill="#c2410c">ヒープ</text>
<rect x="640" y="94" width="56" height="26" fill="#f1f5f9" stroke="#94a3b8" stroke-dasharray="5 3"/>
<text x="668" y="112" text-anchor="middle" font-size="13" fill="#94a3b8">メモ</text>
<text x="684" y="140" font-size="20" font-weight="700" fill="#dc2626">✕</text>
<text x="600" y="230" text-anchor="middle" font-size="12" fill="#c2410c">s のスコープ終了で本体を片付け</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図2-3: 関数に String を渡すと持ち主が引数へ移り、関数の終わりで片付けられる。main の memo は無効のまま</div>
</div>

#### ▼ コードを1つずつ分解して解説

##### 解説1: 引数が `String` なら「受け取って持ち主になる」

```rust
fn print_and_drop(s: String) {
    println!("受け取った: {s}");
}
```

- 引数の型が `String`（`&` が付いていない）なので、この関数は**値を丸ごと受け取ります**。呼んだ側の `memo` から、関数の `s` へ持ち主が移ります（図2-3の②）。
- 関数の `}` で `s` のスコープが終わるので、本体が片付けられます（③）。だから、`main` で `print_and_drop(memo);` の後に `memo` を使うと E0382 になります。

##### 解説2: 戻り値で持ち主を返す

```rust
fn make_greeting(name: &str) -> String {
    let greeting = format!("こんにちは、{name}さん");
    greeting
}
```

- `format!` は `println!` と同じ書き方で、表示する代わりに新しい `String` を作るマクロです。
- 最後の行の `greeting`（`;` なし）が戻り値です。関数の中で作った `String` の**持ち主が、呼んだ側の変数へ移ります**。だから関数の `}` で片付けられずに済みます。

##### 解説3: 受け取って、加工して、返す

```rust
fn add_mark(mut s: String) -> String {
    s.push('!');
    s
}
```

- `mut s` は「受け取った値を、関数の中で書き換えてよい」という意味です。
- `let greeting = add_mark(greeting);` のように、同じ名前で受け直すのは Rust でよく見る書き方です（第2巻で学んだシャドーイング）。

##### 解説4: Copy 型の引数

```rust
fn show_number(n: i32) {
    println!("数値: {n}");
}
```

- `i32` は Copy 型なので、関数には 7 という値が書き写されて渡されます。`main` の `n` はそのまま残るので、呼んだ後も使えます。

> **ヒント:** 「関数に渡したら使えなくなる」のは不便に感じるかもしれません。実務では、**読むだけの関数は値を受け取らず「借りる」**のが普通です（`&String` や `&str`）。その方法を次の UNIT 03 で学びます。

### もう一歩: 実務に近い形

RustNote のノート保存を小さく真似して、「move させたい値」と「Copy にしておきたい値」を使い分けてみます。

#### 例3: NewNote を保存処理へ move し、NoteId は Copy にする

> **▼ このコードがやること（先に日本語で）:** フォームから届いた「新しいノート」を保存処理へ丸ごと渡し（move）、保存処理は中の文字列をコピーせずにそのまま保存済みノートへ移します。保存結果の ID は小さな Copy 型にして、何度でも気軽に使えるようにします。最後に、一覧のスナップショット（ある時点の写し）を clone で作り、元を変えても写しは変わらないことを確かめます。

`src/bin/u02_note_store.rs`:

```rust
// 小さな ID は Copy にしておくと、何度でも気軽に渡せる
#[derive(Debug, Clone, Copy, PartialEq)]
struct NoteId(i64);

// フォームから届いた「これから保存するノート」
#[derive(Debug)]
struct NewNote {
    title: String,
    body: String,
}

// 保存済みのノート
#[derive(Clone)]
struct Note {
    id: NoteId,
    title: String,
    body: String,
    done: bool,
}

impl Note {
    // 中身を1行の文字列にまとめる（&self は「借りて読むだけ」の意味。UNIT 03 で詳しく）
    fn summary(&self) -> String {
        format!(
            "#{} {} / {} / done={}",
            self.id.0, self.title, self.body, self.done
        )
    }
}

struct NoteStore {
    notes: Vec<Note>,
    next_id: i64,
}

impl NoteStore {
    fn new() -> NoteStore {
        NoteStore {
            notes: Vec::new(),
            next_id: 1,
        }
    }

    // NewNote を丸ごと受け取る（所有権を引き取る）
    fn create(&mut self, new_note: NewNote) -> NoteId {
        let id = NoteId(self.next_id);
        self.next_id += 1;
        // new_note の中の String を、コピーせずにそのまま Note へ移す
        let note = Note {
            id,
            title: new_note.title,
            body: new_note.body,
            done: false,
        };
        self.notes.push(note);
        id
    }
}

fn main() {
    let mut store = NoteStore::new();
    let draft = NewNote {
        title: String::from("買い物"),
        body: String::from("牛乳と卵"),
    };
    let id = store.create(draft);

    // NoteId は Copy なので、代入しても id は使い続けられる
    let same = id;
    println!("{id:?} と {same:?} は同じ? {}", id == same);

    // 一覧を別の場所へ渡したいときは、費用を承知で clone する
    let snapshot = store.notes.clone();
    store.notes[0].done = true;
    println!("store:    {}", store.notes[0].summary());
    println!("snapshot: {}", snapshot[0].summary());
}
```

**▼ 実行結果**

```text
NoteId(1) と NoteId(1) は同じ? true
store:    #1 買い物 / 牛乳と卵 / done=true
snapshot: #1 買い物 / 牛乳と卵 / done=false
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `#[derive(Clone, Copy)]` で Copy 型にする

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct NoteId(i64);
```

- `#[derive(...)]` は「この型に、よく使う機能を自動で付けて」という指示です。`Clone` と `Copy` を付けると、`NoteId` は `i32` と同じように**代入しても元が残る**型になります。
- Copy にできるのは、**中身がすべて Copy 型**のときだけです。`NoteId` の中身は `i64` だけなので OK です。`PartialEq` は `==` で比べられるようにするためのものです。

##### 解説2: `create` は `NewNote` を丸ごと受け取る

```rust
    fn create(&mut self, new_note: NewNote) -> NoteId {
```

- `new_note: NewNote` と `&` なしで受け取っているので、呼んだ側の `draft` は move されます。保存したノートの下書きを、保存した後でうっかり使い回す、というミスを防げます。
- `&mut self` は「このメソッドは `store` の中身を書き換える」という意味です（UNIT 03 で学びます）。

##### 解説3: フィールドごとに move する

```rust
        let note = Note {
            id,
            title: new_note.title,
            body: new_note.body,
            done: false,
        };
```

- `title: new_note.title` は、`new_note` の中の `String` の**持ち手だけを** `note` へ移しています。文字の本体はコピーされません。
- `new_note` はもう持ち主ではないので、関数の終わりで二重に片付けられることもありません。Rust はこうした「部分的な move」も正しく追いかけてくれます。

##### 解説4: clone したスナップショットは独立している

```rust
    let snapshot = store.notes.clone();
    store.notes[0].done = true;
```

- `Vec<Note>` を clone すると、中の `Note` も1つずつ clone され、`String` の本体も全部複製されます。`snapshot` は元の一覧と**完全に独立**しているので、元の `done` を `true` にしても写しは `false` のままです。
- 便利ですが、件数が多いと重くなります。「本当に独立した写しが必要か？借りるだけで済まないか？」を先に考えましょう。

> **つまずきポイント: エラーが出たらとりあえず `.clone()` してしまう**
>
> E0382 のエラー文には、`help: consider cloning the value if the performance cost is acceptable`（性能の費用が許せるなら clone を検討して）と出ます。これを見て何でも `.clone()` で黙らせると、コンパイルは通りますが、毎回本体を複製する重いプログラムになります。まず「**この関数は値を持ち続ける必要があるか？読むだけなら借りればいいのでは？**」と考えてください。借り方は UNIT 03 と 04 で学びます。

### よくあるエラーと読み方

#### エラー1: move した後に使う（E0382）

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1} / {s2}");
}
```

```text
error[E0382]: borrow of moved value: `s1`
 --> src\bin\u02_err_use_after_move.rs:4:16
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |     println!("{s1} / {s2}");
  |                ^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership-lab` (bin "u02_err_use_after_move") due to 1 previous error
```

- **図と対応させると:** 3行目が図2-1の②→③、4行目は③で「無効」になった `s1` の箱を開けようとしています。
- **エラー文の3つの印:** `move occurs because ...`（なぜ move が起きるのか：`String` は Copy でないから）、`value moved here`（どこで移ったか）、`value borrowed here after move`（どこで使おうとしたか）。この3点セットは E0382 で毎回出るので、読み慣れておきましょう。
- **直し方:** ①`s1` を使い終わってから move する、②`s2` には借用 `&s1` を入れる（UNIT 03）、③本当に2つ必要なら `s1.clone()`。

#### エラー2: ループの中で move してしまう（E0382）

```rust
fn save(title: String) {
    println!("保存: {title}");
}

fn main() {
    let title = String::from("買い物");
    for _ in 0..2 {
        save(title);
    }
}
```

```text
error[E0382]: use of moved value: `title`
 --> src\bin\u02_err_move_in_loop.rs:8:14
  |
6 |     let title = String::from("買い物");
  |         ----- move occurs because `title` has type `String`, which does not implement the `Copy` trait
7 |     for _ in 0..2 {
  |     ------------- inside of this loop
8 |         save(title);
  |              ^^^^^ value moved here, in previous iteration of loop
  |
note: consider changing this parameter type in function `save` to borrow instead if owning the value isn't necessary
 --> src\bin\u02_err_move_in_loop.rs:1:16
  |
1 | fn save(title: String) {
  |    ----        ^^^^^^ this parameter takes ownership of the value
  |    |
  |    in this function
help: consider cloning the value if the performance cost is acceptable
  |
8 |         save(title.clone());
  |                   ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership-lab` (bin "u02_err_move_in_loop") due to 1 previous error
```

- **何が起きたか:** 1周目の `save(title)` で `title` は `save` へ移り、片付けられました。2周目にはもう渡すものがありません。`value moved here, in previous iteration of loop`（前の周回でここで移動した）がそれを言っています。
- **注目してほしい `note:`:** 「`save` が値を持ち続ける必要がないなら、引数を**借用**にしたら？」と提案しています。`save` は表示するだけなので、`fn save(title: &str)` にするのが正解です（練習問題2）。

#### エラー3: Vec の要素を取り出して move しようとする（E0507）

```rust
fn main() {
    let names = vec![String::from("佐藤"), String::from("鈴木")];
    let first = names[0];
    println!("{first}");
}
```

```text
error[E0507]: cannot move out of index of `Vec<String>`
 --> src\bin\u02_err_move_out_of_index.rs:3:17
  |
3 |     let first = names[0];
  |                 ^^^^^^^^ move occurs because value has type `String`, which does not implement the `Copy` trait
  |
help: consider borrowing here
  |
3 |     let first = &names[0];
  |                 +
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let first = names[0].clone();
  |                         ++++++++

For more information about this error, try `rustc --explain E0507`.
error: could not compile `ownership-lab` (bin "u02_err_move_out_of_index") due to 1 previous error
```

- **何が起きたか:** `names[0]` の `String` を `first` へ move すると、`Vec` の0番目に「無効な穴」が空いてしまいます。`Vec` は最後に全要素を片付けるので、穴の分を二重に片付けたり、穴を読んだりする危険があります。そこで Rust は、**添え字（インデックス）で中身を取り出して move することを禁止**しています。
- **直し方:** 読むだけなら `&names[0]`（借りる）、独立した値が欲しいなら `names[0].clone()`、`Vec` から本当に取り出したいなら `names.remove(0)` などのメソッドを使います（練習問題3）。

#### エラー4: String を含む struct を Copy にしようとする（E0204）

```rust
#[derive(Clone, Copy)]
struct NewNote {
    title: String,
}

fn main() {
    let a = NewNote {
        title: String::from("買い物"),
    };
    let b = a;
    println!("{} {}", a.title, b.title);
}
```

```text
error[E0204]: the trait `Copy` cannot be implemented for this type
 --> src\bin\u02_err_derive_copy.rs:2:8
  |
1 | #[derive(Clone, Copy)]
  |                 ---- in this derive macro expansion
2 | struct NewNote {
  |        ^^^^^^^
3 |     title: String,
  |     ------------- this field does not implement `Copy`

For more information about this error, try `rustc --explain E0204`.
error: could not compile `ownership-lab` (bin "u02_err_derive_copy") due to 1 previous error
```

- **何が起きたか:** `title` が `String`（Copy でない）なので、`NewNote` を書き写すと、図2-2の左側と同じ「1つの本体を2人が指す」状態になってしまいます。だから Copy は付けられません。
- **直し方:** `#[derive(Clone)]` だけにして、複製が必要なときは `a.clone()` と書きます。

### RustNote ではここで使う

- **`NewNote` は保存処理へ move:** 作成フォームの内容から作った `NewNote` は、`NoteRepository` の作成メソッドに丸ごと渡します。保存が終わった下書きを使い回すミスが、型のレベルで起きなくなります。
- **`UserId`・`NoteId` は Copy:** 中身が整数1つだけの ID 型は `#[derive(Clone, Copy)]` を付けます。ログ出力、権限チェック、URL 作成など、あちこちで気軽に渡せます。
- **`Email`・`NoteTitle` は Copy にしない:** 中に `String` を持つので Copy にはできません。必要なら `Clone` だけを付けます。
- **`clone()` は理由があるときだけ:** 一覧を返すときに全ノートを clone するようなコードは、件数が増えると遅くなります。RustNote では「借りて渡す」設計を基本にします。

### 練習問題

**問題1（予想）** 次のコードの出力を予想してください。また、`a` と `c` を最後の `println!` で使うとどうなるか答えてください。

```rust
fn shout(mut s: String) -> String {
    s.push_str("!!");
    s
}

fn main() {
    let a = String::from("hey");
    let b = shout(a);
    let c = b.clone();
    let d = shout(c);
    println!("{b} {d}");
}
```

**問題2（修正）** 「よくあるエラー2」のコードを、`clone()` を使わずにコンパイルが通るように直してください。ループの後で `title` をもう一度表示できるようにしてください。

**問題3（3つの方法）** `vec![String::from("佐藤"), String::from("鈴木")]` から先頭の名前を取り出す方法を、「借りる」「複製する」「Vec から取り出す」の3通りで書いてください。

### 解答と解説

**問題1の解答**（`src/bin/u02_ex1.rs`）

```text
hey!! hey!!!!
```

- `shout(a)` で `a` は move され、`"hey!!"` になった `String` が `b` に返ってきます。
- `b.clone()` で独立した `c` を作り、`shout(c)` でさらに `"!!"` が足されて `d` は `"hey!!!!"` です。`b` は clone の元なので無事です。
- 最後に `a` や `c` を使うと、どちらも `shout` に move 済みなので **E0382** になります。

**問題2の解答例**（`src/bin/u02_ex2.rs`）

```rust
fn save(title: &str) {
    println!("保存: {title}");
}

fn main() {
    let title = String::from("買い物");
    for _ in 0..2 {
        save(&title);
    }
    println!("最後に: {title}");
}
```

**▼ 実行結果**

```text
保存: 買い物
保存: 買い物
最後に: 買い物
```

- `save` の引数を `&str`（文字列を借りる型）にし、呼ぶ側は `&title` で貸します。`title` の持ち主は `main` のままなので、何度でも貸せて、最後にも使えます。

**問題3の解答例**（`src/bin/u02_ex3.rs`）

```rust
fn main() {
    let names = vec![String::from("佐藤"), String::from("鈴木")];
    // 1. 借りる（いちばん安い）
    let first: &String = &names[0];
    println!("借りた: {first}");
    // 2. 複製する（names は残る）
    let copied: String = names[0].clone();
    println!("複製した: {copied}");
    // 3. Vec ごと手放して、要素を取り出す
    let mut names = names;
    let taken: String = names.remove(0);
    println!("取り出した: {taken} / 残り: {names:?}");
}
```

**▼ 実行結果**

```text
借りた: 佐藤
複製した: 佐藤
取り出した: 佐藤 / 残り: ["鈴木"]
```

- `remove(0)` は要素を取り出して残りを前に詰めるので、`Vec` に穴が空きません。書き換えが必要なので `let mut names = names;` で `mut` な変数へ移しています。

### まとめ

- `String` などの代入・関数呼び出しは **move**。コピーされるのは持ち手だけで、元の変数は無効になる（二重解放を防ぐため）。
- `i32` などの **Copy 型**は書き写されるだけなので元も使える。`clone()` は本体ごと複製する、費用のかかる明示的な操作。
- E0382 が出たら「どこで移ったか（`value moved here`）」を探し、clone の前に「借りられないか」を考える。

## UNIT 03: 共有参照 &T と可変参照 &mut T

### この単元のゴール

- 値を move せずに「借りる」方法、**参照**（`&T` と `&mut T`）を使える
- 「**読む人は何人でも、書く人は1人だけ（そしてその間は誰も読めない）**」という借用のルールを説明できる
- このルールが**ダングリング参照**（もう存在しない場所を指す参照）を防いでいることを、`Vec` の引っ越しの図で説明できる
- E0499・E0502・E0596 のエラー文を読んで、自分で直せる

### たとえ話でつかむ

UNIT 02 の「本の持ち主」の話の続きです。友だちに本を**手渡す**（move）と、自分の手元から本がなくなってしまいました。でも、ちょっと読ませてあげたいだけなら、**貸せばいい**のです。

- **共有参照 `&T` = 閲覧のための貸し出し** です。本を机に開いて置き、「読むだけならどうぞ」と言います。何人が同時にのぞき込んでも問題ありません。誰も本を書き換えないからです。
- **可変参照 `&mut T` = 書き込みのための貸し出し** です。1人に赤ペンと一緒に本を渡し、「直していいよ」と言います。書き込み中は「**作業中**」の札が掛かり、ほかの人は**書くことも、読むことも**できません。
- **持ち主も、貸している間は本を勝手に処分できません。** 閲覧中の本を捨てたら、読んでいる人が困ります。

なぜ「書き込み中は読むのも禁止」なのでしょう。読んでいる途中でページが差し替えられたり、ページが丸ごと別の場所へ移されたりしたら、読んでいる人は**消えたページ**を見つめることになるからです。プログラムでは、これが「片付け済みのメモリを読む」という深刻なバグになります。後で実際に見てみましょう。

> **用語: 参照（さんしょう）と借用（しゃくよう）** … 参照は「値がある場所を指す、持ち主ではない値」です。`&x` と書くと `x` への参照が作られます。参照を作ることを**借用**と呼びます。借用は値を所有しないので、参照がなくなっても値は片付けられません。

### 図で理解する

`memo` を共有参照で何人かに貸している状態と、可変参照で1人に貸している状態を比べます。参照の矢印が、ヒープの本体ではなく**スタックの `memo`（持ち手）**を指していることにも注目してください。`&String` は「`String` という値そのもの」への参照だからです。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-03-09" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#10b981"/></marker>
<marker id="ar-03-10" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#f97316"/></marker>
<marker id="ar-03-11" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="4" y="6" width="352" height="268" rx="10" fill="#ecfdf5" stroke="#10b981"/>
<text x="16" y="28" font-size="14" font-weight="700" fill="#047857">&amp;T: 読む人は何人でも OK</text>
<rect x="16" y="40" width="200" height="190" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="24" y="58" font-size="12" font-weight="700" fill="#1e40af">スタック</text>
<text x="26" y="88" font-size="13" font-weight="700" fill="#0f172a">r1</text>
<rect x="52" y="72" width="44" height="24" rx="3" fill="#ffffff" stroke="#10b981"/>
<circle cx="84" cy="84" r="3.5" fill="#10b981"/>
<text x="112" y="88" font-size="13" font-weight="700" fill="#0f172a">r2</text>
<rect x="138" y="72" width="44" height="24" rx="3" fill="#ffffff" stroke="#10b981"/>
<circle cx="170" cy="84" r="3.5" fill="#10b981"/>
<text x="104" y="135" font-size="12" font-weight="700" fill="#047857">text</text>
<rect x="138" y="118" width="44" height="24" rx="3" fill="#ffffff" stroke="#10b981"/>
<circle cx="170" cy="130" r="3.5" fill="#10b981"/>
<text x="26" y="200" font-size="13" font-weight="700" fill="#0f172a">memo</text>
<rect x="74" y="176" width="96" height="40" rx="3" fill="#ffffff" stroke="#1e40af" stroke-width="2"/>
<text x="84" y="200" font-size="12" fill="#0f172a">ptr len cap</text>
<circle cx="160" cy="196" r="3.5" fill="#475569"/>
<line x1="84" y1="84" x2="96" y2="174" stroke="#10b981" stroke-width="2" marker-end="url(#ar-03-09)"/>
<line x1="170" y1="84" x2="126" y2="174" stroke="#10b981" stroke-width="2" marker-end="url(#ar-03-09)"/>
<line x1="170" y1="130" x2="150" y2="174" stroke="#10b981" stroke-width="2" marker-end="url(#ar-03-09)"/>
<rect x="232" y="40" width="112" height="190" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="240" y="58" font-size="12" font-weight="700" fill="#c2410c">ヒープ</text>
<rect x="242" y="182" width="94" height="28" fill="#ffffff" stroke="#c2410c"/>
<text x="289" y="201" text-anchor="middle" font-size="12" fill="#0f172a">牛乳を買う</text>
<line x1="160" y1="196" x2="240" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-11)"/>
<text x="180" y="252" text-anchor="middle" font-size="12" fill="#047857">全員が読むだけなので、何人いても内容は変わらない</text>
<rect x="364" y="6" width="352" height="268" rx="10" fill="#fff7ed" stroke="#f97316"/>
<text x="376" y="28" font-size="14" font-weight="700" fill="#c2410c">&amp;mut T: 書く人は1人だけ</text>
<rect x="376" y="40" width="200" height="190" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="384" y="58" font-size="12" font-weight="700" fill="#1e40af">スタック</text>
<text x="386" y="88" font-size="13" font-weight="700" fill="#94a3b8">r1</text>
<rect x="412" y="72" width="44" height="24" rx="3" fill="#f1f5f9" stroke="#94a3b8" stroke-dasharray="4 3"/>
<text x="462" y="90" font-size="16" font-weight="700" fill="#dc2626">✕</text>
<text x="482" y="88" font-size="11" fill="#b91c1c">読むのも不可</text>
<text x="386" y="132" font-size="12" fill="#c2410c">text（&amp;mut）</text>
<rect x="498" y="118" width="44" height="24" rx="3" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<circle cx="530" cy="130" r="3.5" fill="#f97316"/>
<text x="386" y="200" font-size="13" font-weight="700" fill="#0f172a">memo</text>
<rect x="434" y="176" width="96" height="40" rx="3" fill="#ffffff" stroke="#1e40af" stroke-width="2"/>
<text x="444" y="200" font-size="12" fill="#0f172a">ptr len cap</text>
<circle cx="520" cy="196" r="3.5" fill="#475569"/>
<line x1="530" y1="130" x2="496" y2="174" stroke="#f97316" stroke-width="2.5" marker-end="url(#ar-03-10)"/>
<rect x="592" y="40" width="112" height="190" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="600" y="58" font-size="12" font-weight="700" fill="#c2410c">ヒープ</text>
<rect x="602" y="168" width="94" height="44" fill="#ffffff" stroke="#c2410c"/>
<text x="649" y="186" text-anchor="middle" font-size="12" fill="#0f172a">牛乳を買う</text>
<text x="649" y="204" text-anchor="middle" font-size="11" fill="#c2410c">（編集済み）</text>
<line x1="520" y1="196" x2="600" y2="194" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-11)"/>
<text x="540" y="252" text-anchor="middle" font-size="12" fill="#c2410c">書いている間は、ほかの借用は1つも作れない</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図3-1: 緑の &amp;T は何本でも同時に作れる（text は関数の引数）。橙の &amp;mut T がある間は、ほかの参照は（読むためのものも）作れない</div>
</div>

このルールを表にまとめると、次のようになります。「今ある借用」の行と「新しくしたいこと」の列が交わるマスを見てください。

| 今ある借用 ＼ 新しくしたいこと | `&x` で読む借用を作る | `&mut x` で書く借用を作る | 持ち主が `x` を書き換える・move する |
|---|---|---|---|
| 借用なし | OK | OK（`x` が `mut` なら） | OK |
| `&x` が1つ以上ある | **OK** | NG（E0502） | NG |
| `&mut x` が1つある | NG（E0502） | NG（E0499） | NG |

「今ある借用」とは、**作られていて、この後もまだ使われる予定の借用**のことです。もう使われない借用は、自動で返却されたものとして扱われます（UNIT 07 で詳しく学びます）。

### まず動かす最小コード

#### 例1: 借りて読む・借りて書く

> **▼ このコードがやること（先に日本語で）:** メモの文字列を、読むだけの関数には `&` で、書き換える関数には `&mut` で貸します。どちらの場合も、持ち主は `main` の `memo` のままなので、関数を呼んだ後も `memo` を使い続けられます。最後に、整数への可変参照を通して値を1増やします。

`src/bin/u03_borrow_basic.rs`:

```rust
// &String は「String を借りて読むだけ」という意味
fn count_chars(text: &String) -> usize {
    text.chars().count()
}

// &mut String は「String を借りて書き換える」という意味
fn add_suffix(text: &mut String) {
    text.push_str("（編集済み）");
}

fn main() {
    let mut memo = String::from("牛乳を買う");

    // 共有参照（読むための借用）は、同時にいくつ作ってもよい
    let r1 = &memo;
    let r2 = &memo;
    println!("r1 = {r1}, r2 = {r2}");
    println!("文字数 = {}", count_chars(&memo));

    // 可変参照（書くための借用）で、中身を書き換える
    add_suffix(&mut memo);
    println!("memo = {memo}");

    let mut count = 10;
    let c = &mut count;
    // 参照の「先にある値」を変えるときは * を付ける
    *c += 1;
    println!("count = {count}");
}
```

```powershell
cargo run --bin u03_borrow_basic
```

**▼ 実行結果**

```text
r1 = 牛乳を買う, r2 = 牛乳を買う
文字数 = 5
memo = 牛乳を買う（編集済み）
count = 11
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 引数の型 `&String` と `&mut String`

```rust
fn count_chars(text: &String) -> usize {
    text.chars().count()
}

fn add_suffix(text: &mut String) {
    text.push_str("（編集済み）");
}
```

- `&String` は「`String` への共有参照」、`&mut String` は「`String` への可変参照」という**型**です。関数の引数の型を見るだけで、「この関数は読むだけ」「この関数は書き換える」が分かります。
- `text.chars().count()` は文字数を数えます（第2巻で学んだとおり、`len()` はバイト数なので日本語では文字数と一致しません）。
- 参照に対して `.` でメソッドを呼ぶと、Rust が自動で参照の先の値をたどってくれます（**自動参照外し**）。だから `(*text).push_str(...)` と書かなくても `text.push_str(...)` で動きます。

##### 解説2: 共有参照は同時にいくつでも

```rust
    let r1 = &memo;
    let r2 = &memo;
    println!("r1 = {r1}, r2 = {r2}");
    println!("文字数 = {}", count_chars(&memo));
```

- `&memo` で `memo` への共有参照を作ります。`r1`・`r2`・関数の引数 `text` の3つが同時に `memo` を指していても、全員が読むだけなので問題ありません（図3-1の左）。
- 共有参照 `&T` は **Copy 型**です。参照（場所を指す8バイトの値）を書き写しても、`memo` が2つになるわけではないからです。

##### 解説3: 可変参照で書き換える

```rust
    add_suffix(&mut memo);
    println!("memo = {memo}");
```

- `&mut memo` で可変参照を作って渡します。可変参照を作るには、**元の変数が `let mut` で宣言されている**必要があります。
- 関数の中で `push_str` した結果は、`memo` 本体に反映されています。`memo` を move していないので、呼んだ後も `memo` を使えます。
- この時点で `r1` と `r2` はもう使われないので、`&mut memo` を作っても衝突しません（UNIT 07 で学ぶ NLL のおかげです）。

##### 解説4: `*` で参照の先の値を変える

```rust
    let mut count = 10;
    let c = &mut count;
    *c += 1;
```

- `c` は「`count` の場所」を持っている参照です。`c += 1` と書くと「場所に1を足す」という意味不明な命令になってしまうので、`*c` で「**参照の先にある値**」を取り出してから 1 を足します。この `*` を**参照外し**（デリファレンス）と呼びます。
- メソッド呼び出し（解説1）では自動で参照外ししてくれますが、`+=` や `=` で値そのものを書き換えるときは `*` が必要です。

> **ヒント:** このコードを `cargo clippy`（Rust の公式のコード点検ツール）にかけると、「`&String` ではなく `&str` にしましょう」という提案が出ます。理由は UNIT 04 で説明します。この単元では、借用の仕組みを見やすくするために、あえて `&String` を使っています。

### もう一歩: 実務に近い形

#### 例2: ノートを読む関数と書き換える関数

> **▼ このコードがやること（先に日本語で）:** ノートを表示用の文字列にする関数（読むだけ）と、完了状態を切り替える関数・タイトルを変える関数（書き換える）を作ります。関数の引数の型が `&Note` か `&mut Note` かで、役割が一目で分かるようにします。

`src/bin/u03_note_fns.rs`:

```rust
struct Note {
    title: String,
    done: bool,
}

// 読むだけなので &Note を受け取る
fn describe(note: &Note) -> String {
    let mark = if note.done { "[x]" } else { "[ ]" };
    format!("{mark} {}", note.title)
}

// 書き換えるので &mut Note を受け取る
fn toggle(note: &mut Note) {
    note.done = !note.done;
}

// 書き換える。新しいタイトルは読むだけなので &str で受け取る
fn rename(note: &mut Note, new_title: &str) {
    note.title = new_title.to_string();
}

fn main() {
    let mut note = Note {
        title: String::from("牛乳を買う"),
        done: false,
    };
    println!("{}", describe(&note));

    toggle(&mut note);
    rename(&mut note, "牛乳と卵を買う");
    println!("{}", describe(&note));

    // 可変参照を変数に入れておき、続けて使うこともできる
    let editor = &mut note;
    toggle(editor);
    editor.title.push_str("（急ぎ）");
    println!("{}", describe(&note));
}
```

**▼ 実行結果**

```text
[ ] 牛乳を買う
[x] 牛乳と卵を買う
[ ] 牛乳と卵を買う（急ぎ）
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 参照からフィールドを読む・書く

```rust
fn toggle(note: &mut Note) {
    note.done = !note.done;
}
```

- `note.done` は、参照 `note` の先にある `Note` の `done` フィールドです。フィールドへのアクセスも、メソッドと同じく自動で参照外しされるので `(*note).done` と書く必要はありません。
- `!` は真偽値を反転する演算子です。`&mut Note` で借りているので、フィールドに代入できます。

##### 解説2: `rename` の2つの引数

```rust
fn rename(note: &mut Note, new_title: &str) {
    note.title = new_title.to_string();
}
```

- ノートは書き換えるので `&mut Note`、新しいタイトルは読むだけなので `&str`（文字列への共有参照）です。
- `new_title.to_string()` で、借りた文字列から**新しい `String`（自分の本体を持つ値）**を作り、`note.title` に入れます。古い `title` の `String` は、上書きされた瞬間に片付けられます。

##### 解説3: 可変参照を変数に入れて使い回す

```rust
    let editor = &mut note;
    toggle(editor);
    editor.title.push_str("（急ぎ）");
    println!("{}", describe(&note));
```

- `editor` は `&mut Note` 型の変数です。`toggle(editor)` と渡した後も `editor` を使えるのは、関数に渡すときに「`editor` からさらに一時的に借りる」**再借用**が自動で行われるからです（UNIT 07 で詳しく見ます）。
- 最後の `describe(&note)` の時点では、`editor` はもう使われないので、`note` を共有参照で借りられます。

#### 例3: なぜ「書く人は1人だけ」なのか — Vec の引っ越しを見る

ルールの理由を、実際のメモリで確かめてみましょう。`Vec` は容量（`cap`）が足りなくなると、**ヒープの別の場所へ本体ごと引っ越す**ことがあります。

> **▼ このコードがやること（先に日本語で）:** `Vec` に要素を1つずつ足していき、本体の場所（アドレス）が変わったときだけ表示します。容量が増えるときに、本体が別の場所へ移ることがあるのを確かめます。

`src/bin/u03_realloc.rs`:

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let mut place = v.as_ptr();
    println!("最初    : cap = {:>3}, 本体の場所 = {:p}", v.capacity(), place);
    for i in 4..=100 {
        v.push(i);
        // 本体の場所が変わったときだけ表示する
        if v.as_ptr() != place {
            place = v.as_ptr();
            println!("引っ越し: cap = {:>3}, 本体の場所 = {:p}", v.capacity(), place);
        }
    }
}
```

**▼ 実行結果**（アドレスは実行ごとに変わります）

```text
最初    : cap =   3, 本体の場所 = 0x22ac5108d70
引っ越し: cap =  24, 本体の場所 = 0x22ac510bbb0
引っ越し: cap =  96, 本体の場所 = 0x22ac5111060
引っ越し: cap = 192, 本体の場所 = 0x22ac5117a10
```

- `{:>3}` は「3文字分の幅で右寄せして表示する」という書式です。
- 容量は 3 → 6 → 12 → 24 → 48 → 96 → 192 と増えていきますが、6・12・48 のときは**その場で棚を広げられた**ので場所が変わらず、24・96・192 のときは**別の場所へ引っ越し**ました。どちらになるかは、そのときのヒープの空き具合しだいで、プログラムを書く人には予測できません。

では、`v[0]` を借りている最中に `push` して、本体が引っ越したらどうなるでしょうか。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-03-12" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
<marker id="ar-03-13" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#10b981"/></marker>
<marker id="ar-03-14" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#dc2626"/></marker>
</defs>
<rect x="4" y="6" width="232" height="268" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="14" y="26" font-size="13" font-weight="700" fill="#0f172a">①</text>
<text x="32" y="26" font-size="12" fill="#0f172a" font-family="Consolas, Menlo, monospace">let first = &amp;v[0];</text>
<rect x="14" y="40" width="84" height="180" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="20" y="58" font-size="12" font-weight="700" fill="#1e40af">スタック</text>
<text x="20" y="88" font-size="12" font-weight="700" fill="#0f172a">v</text>
<rect x="34" y="74" width="56" height="24" rx="3" fill="#ffffff" stroke="#1e40af"/>
<text x="40" y="91" font-size="11" fill="#0f172a">ptr</text>
<circle cx="80" cy="86" r="3.5" fill="#475569"/>
<text x="20" y="160" font-size="11" font-weight="700" fill="#0f172a">first</text>
<rect x="52" y="146" width="38" height="24" rx="3" fill="#ffffff" stroke="#10b981"/>
<circle cx="80" cy="158" r="3.5" fill="#10b981"/>
<rect x="108" y="40" width="120" height="180" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="116" y="58" font-size="12" font-weight="700" fill="#c2410c">ヒープ</text>
<rect x="124" y="98" width="30" height="26" fill="#ffffff" stroke="#c2410c"/>
<rect x="154" y="98" width="30" height="26" fill="#ffffff" stroke="#c2410c"/>
<rect x="184" y="98" width="30" height="26" fill="#ffffff" stroke="#c2410c"/>
<text x="139" y="116" text-anchor="middle" font-size="13" fill="#0f172a">1</text>
<text x="169" y="116" text-anchor="middle" font-size="13" fill="#0f172a">2</text>
<text x="199" y="116" text-anchor="middle" font-size="13" fill="#0f172a">3</text>
<line x1="80" y1="86" x2="122" y2="104" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-12)"/>
<line x1="80" y1="158" x2="134" y2="126" stroke="#10b981" stroke-width="2" marker-end="url(#ar-03-13)"/>
<text x="120" y="244" text-anchor="middle" font-size="12" fill="#047857">first は本体の0番目を指す</text>
<text x="120" y="262" text-anchor="middle" font-size="12" fill="#047857">（cap = 3 で満杯）</text>
<rect x="244" y="6" width="232" height="268" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="254" y="26" font-size="13" font-weight="700" fill="#0f172a">②</text>
<text x="272" y="26" font-size="12" fill="#0f172a" font-family="Consolas, Menlo, monospace">v.push(4);</text>
<rect x="254" y="40" width="84" height="180" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="260" y="58" font-size="12" font-weight="700" fill="#1e40af">スタック</text>
<text x="260" y="88" font-size="12" font-weight="700" fill="#0f172a">v</text>
<rect x="274" y="74" width="56" height="24" rx="3" fill="#ffffff" stroke="#1e40af"/>
<text x="280" y="91" font-size="11" fill="#0f172a">ptr</text>
<circle cx="320" cy="86" r="3.5" fill="#475569"/>
<text x="260" y="160" font-size="11" font-weight="700" fill="#0f172a">first</text>
<rect x="292" y="146" width="38" height="24" rx="3" fill="#ffffff" stroke="#10b981"/>
<circle cx="320" cy="158" r="3.5" fill="#10b981"/>
<rect x="348" y="40" width="120" height="180" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="356" y="58" font-size="12" font-weight="700" fill="#c2410c">ヒープ</text>
<rect x="356" y="70" width="26" height="24" fill="#ffffff" stroke="#c2410c"/>
<rect x="382" y="70" width="26" height="24" fill="#ffffff" stroke="#c2410c"/>
<rect x="408" y="70" width="26" height="24" fill="#ffffff" stroke="#c2410c"/>
<rect x="434" y="70" width="26" height="24" fill="#ffffff" stroke="#c2410c"/>
<text x="369" y="87" text-anchor="middle" font-size="12" fill="#0f172a">1</text>
<text x="395" y="87" text-anchor="middle" font-size="12" fill="#0f172a">2</text>
<text x="421" y="87" text-anchor="middle" font-size="12" fill="#0f172a">3</text>
<text x="447" y="87" text-anchor="middle" font-size="12" fill="#c2410c">4</text>
<text x="408" y="110" text-anchor="middle" font-size="11" fill="#c2410c">新しい場所（cap 6）</text>
<rect x="364" y="150" width="30" height="26" fill="#f1f5f9" stroke="#94a3b8" stroke-dasharray="4 3"/>
<rect x="394" y="150" width="30" height="26" fill="#f1f5f9" stroke="#94a3b8" stroke-dasharray="4 3"/>
<rect x="424" y="150" width="30" height="26" fill="#f1f5f9" stroke="#94a3b8" stroke-dasharray="4 3"/>
<text x="409" y="194" text-anchor="middle" font-size="11" fill="#64748b">古い場所は返却済み</text>
<text x="446" y="172" font-size="16" font-weight="700" fill="#dc2626">✕</text>
<line x1="320" y1="86" x2="354" y2="82" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-12)"/>
<line x1="320" y1="158" x2="362" y2="162" stroke="#10b981" stroke-width="2" marker-end="url(#ar-03-13)"/>
<text x="360" y="244" text-anchor="middle" font-size="12" fill="#c2410c">本体が引っ越し、v.ptr は新しい場所へ</text>
<text x="360" y="262" text-anchor="middle" font-size="12" fill="#c2410c">first は古い場所を指したまま</text>
<rect x="484" y="6" width="232" height="268" rx="10" fill="#fef2f2" stroke="#fca5a5"/>
<text x="494" y="26" font-size="13" font-weight="700" fill="#0f172a">③</text>
<text x="512" y="26" font-size="12" fill="#0f172a" font-family="Consolas, Menlo, monospace">println!("{first}");</text>
<rect x="494" y="40" width="84" height="180" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="500" y="58" font-size="12" font-weight="700" fill="#1e40af">スタック</text>
<text x="500" y="160" font-size="11" font-weight="700" fill="#0f172a">first</text>
<rect x="532" y="146" width="38" height="24" rx="3" fill="#ffffff" stroke="#dc2626"/>
<circle cx="560" cy="158" r="3.5" fill="#dc2626"/>
<rect x="588" y="40" width="120" height="180" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="596" y="58" font-size="12" font-weight="700" fill="#c2410c">ヒープ</text>
<rect x="604" y="150" width="30" height="26" fill="#f1f5f9" stroke="#94a3b8" stroke-dasharray="4 3"/>
<rect x="634" y="150" width="30" height="26" fill="#f1f5f9" stroke="#94a3b8" stroke-dasharray="4 3"/>
<rect x="664" y="150" width="30" height="26" fill="#f1f5f9" stroke="#94a3b8" stroke-dasharray="4 3"/>
<text x="649" y="132" text-anchor="middle" font-size="22" font-weight="700" fill="#dc2626">?</text>
<line x1="560" y1="158" x2="602" y2="162" stroke="#dc2626" stroke-width="2" marker-end="url(#ar-03-14)"/>
<text x="600" y="244" text-anchor="middle" font-size="12" font-weight="700" fill="#b91c1c">返却済みの場所を読む</text>
<text x="600" y="262" text-anchor="middle" font-size="12" fill="#b91c1c">＝ダングリング参照（Rust は禁止）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図3-2: 借りている最中に push すると、本体の引っ越しで参照が「返却済みの場所」を指してしまう。Rust はこれをコンパイル時に止める</div>
</div>

`push` は `Vec` を書き換えるので、`&mut v` を必要とします。`first` という共有参照がまだ使われる予定なのに `&mut v` を作ろうとすると、借用のルール（表の2行目）に反するので、Rust は **E0502** でコンパイルを止めます。引っ越しが起きるかどうかを知らなくても、ルールさえ守っていれば③の事故は絶対に起きない、というのがポイントです。

### よくあるエラーと読み方

#### エラー1: 可変参照を2つ同時に作る（E0499）

```rust
fn main() {
    let mut s = String::from("メモ");
    let w1 = &mut s;
    let w2 = &mut s;
    w1.push_str("A");
    w2.push_str("B");
    println!("{s}");
}
```

```text
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src\bin\u03_err_two_mut.rs:4:14
  |
3 |     let w1 = &mut s;
  |              ------ first mutable borrow occurs here
4 |     let w2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
5 |     w1.push_str("A");
  |     -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership-lab` (bin "u03_err_two_mut") due to 1 previous error
```

- **何が起きたか:** 「`s` を可変で同時に2回以上借りることはできません」。3行目で1人目（`w1`）に赤ペンを渡したのに、4行目で2人目（`w2`）にも渡そうとしています。
- **3つの印を読む:** `first mutable borrow occurs here`（1つ目の可変借用）、`second mutable borrow occurs here`（2つ目）、`first borrow later used here`（1つ目が**後でまだ使われる**場所）。3つ目が重要で、「1つ目の借用がまだ終わっていない」ことの証拠です。
- **直し方:** 1つ目を使い終わってから2つ目を作ります。`w1.push_str("A");` を `let w2 = ...` より前に移せば、`w1` はそこで使い終わるのでコンパイルが通ります。そもそも2つの変数に分ける必要がなければ、`s.push_str("A"); s.push_str("B");` と直接書くのが一番簡単です。

#### エラー2: 借りている最中に書き換える（E0502）

図3-2のコードです。

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    println!("first = {first}");
}
```

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src\bin\u03_err_vec_push.rs:4:5
  |
3 |     let first = &v[0];
  |                  - immutable borrow occurs here
4 |     v.push(4);
  |     ^^^^^^^^^ mutable borrow occurs here
5 |     println!("first = {first}");
  |                        ----- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership-lab` (bin "u03_err_vec_push") due to 1 previous error
```

- **何が起きたか:** 「`v` は共有で借りられているので、可変で借りることはできません」。英語の `immutable borrow` は共有参照、`mutable borrow` は可変参照のことです。
- **図と対応させると:** 3行目が図3-2の①、4行目が②、5行目が③です。③で `first` を使う予定があるので、②の時点で `first` の借用はまだ終わっていません。
- **直し方:** ①`first` を使い終わってから `push` する（5行目と4行目を入れ替える）、②借用ではなく値をコピーしておく（`let first = v[0];`。`i32` は Copy なので move の問題も起きません）。

#### エラー3: 書き込み中に読む（E0502 の逆向き）

```rust
fn main() {
    let mut score = 10;
    let writer = &mut score;
    println!("score = {score}");
    *writer += 1;
}
```

```text
error[E0502]: cannot borrow `score` as immutable because it is also borrowed as mutable
 --> src\bin\u03_err_read_while_write.rs:4:24
  |
3 |     let writer = &mut score;
  |                  ---------- mutable borrow occurs here
4 |     println!("score = {score}");
  |                        ^^^^^ immutable borrow occurs here
5 |     *writer += 1;
  |     ------------ mutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership-lab` (bin "u03_err_read_while_write") due to 1 previous error
```

- **何が起きたか:** 今度は「可変で借りられているので、共有で借りることはできません」。`println!` は値を表示するために、`score` を共有参照で借ります。書き込み中の本は、持ち主自身でも読めません。
- **直し方:** 書き込み（5行目）を済ませてから表示します。

#### エラー4: `mut` でない変数を可変で借りる（E0596）

```rust
fn add_suffix(text: &mut String) {
    text.push_str("!");
}

fn main() {
    let memo = String::from("メモ");
    add_suffix(&mut memo);
    println!("{memo}");
}
```

```text
error[E0596]: cannot borrow `memo` as mutable, as it is not declared as mutable
 --> src\bin\u03_err_not_mut.rs:7:16
  |
7 |     add_suffix(&mut memo);
  |                ^^^^^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
6 |     let mut memo = String::from("メモ");
  |         +++

For more information about this error, try `rustc --explain E0596`.
error: could not compile `ownership-lab` (bin "u03_err_not_mut") due to 1 previous error
```

- **何が起きたか:** 「`memo` は `mut` と宣言されていないので、可変で借りられません」。持ち主自身が「書き換えません」と宣言した値を、他人に書き換えさせることはできません。
- **直し方:** `help:` のとおり `let mut memo` にします。

### RustNote ではここで使う

- **関数の引数の型が「役割の説明書」になる:** RustNote のコードでは、ノートを画面用の HTML にする関数は `&Note`（読むだけ）、編集フォームの内容を反映する関数は `&mut Note`（書き換える）を受け取ります。関数の中身を読まなくても、引数の型だけで「この関数はノートを変更するか」が分かります。
- **共有状態は「読む人がたくさん」:** RustNote のサーバーは、たくさんのリクエストを同時に処理します。設定（`Config`）などの共有された値は、全員が `&` で読むだけにしておけば安全です。書き換えが必要な共有状態の扱い方は UNIT 10 で学びます。

### 練習問題

**問題1（予想）** 次のコードはコンパイルできるでしょうか。できるなら出力を、できないならエラー番号を答えてください。

```rust
fn main() {
    let mut text = String::from("abc");
    let a = &text;
    let b = &text;
    println!("{a} {b}");
    let c = &mut text;
    c.push('d');
    println!("{text}");
}
```

**問題2（作る）** `Vec<i32>` の要素をすべて2倍にする関数 `double_all` と、要素の合計を返す関数 `sum` を作ってください。`double_all` は書き換え、`sum` は読むだけなので、引数の型をそれに合わせてください。`vec![1, 2, 3]` で試すと `[2, 4, 6] 合計 = 12` と表示されるようにします。

**問題3（説明）** 「よくあるエラー2」で、`first` が `i32` への参照ではなく `i32` の値（`let first = v[0];`）ならコンパイルが通るのはなぜですか。図3-2を使って説明してください。

### 解答と解説

**問題1の解答**（`src/bin/u03_ex1.rs`）: コンパイルできます。

```text
abc abc
abcd
```

- `a` と `b` は5行目の `println!` を最後に使われなくなるので、6行目で `&mut text` を作った時点では共有借用はもう終わっています。
- `c` も7行目が最後の使用なので、8行目で `text` を読めます。借用は「**最後に使われた場所**」で終わる、という考え方は UNIT 07 で詳しく扱います。

**問題2の解答例**（`src/bin/u03_ex2.rs`）

```rust
fn double_all(values: &mut Vec<i32>) {
    for v in values.iter_mut() {
        *v *= 2;
    }
}

fn sum(values: &Vec<i32>) -> i32 {
    let mut total = 0;
    for v in values {
        total += v;
    }
    total
}

fn main() {
    let mut values = vec![1, 2, 3];
    double_all(&mut values);
    println!("{values:?} 合計 = {}", sum(&values));
}
```

**▼ 実行結果**

```text
[2, 4, 6] 合計 = 12
```

- `values.iter_mut()` は、要素を1つずつ**可変参照**（`&mut i32`）で取り出します。`*v *= 2` の `*` は、参照の先の整数を書き換えるための参照外しです。
- `sum` の `for v in values` では、`values` が `&Vec<i32>` なので、要素は共有参照 `&i32` で取り出されます。`total += v` は、`i32` に `&i32` を足す書き方が標準ライブラリで用意されているので、そのまま書けます。
- UNIT 04 を学ぶと、この2つの関数の引数は `&mut [i32]` と `&[i32]` にした方がよいことが分かります（clippy もそう提案します）。

**問題3の解答例**

`let first = v[0];` は、`i32` が Copy 型なので、ヒープの0番目にある `1` という値を**スタックの `first` に書き写す**だけです。`first` はヒープを指していないので、`push` で本体が引っ越しても `first` の値は影響を受けません。借用が1つもないので、`push` に必要な `&mut v` も問題なく作れます。

### まとめ

- `&T` は読むための借用で何個でも同時に作れる。`&mut T` は書くための借用で、ある間はほかの借用（読む借用も）を作れない。
- このルールのおかげで、`Vec` の引っ越しなどで「返却済みの場所を指す参照（ダングリング参照）」が生まれない。
- E0499 は「書く人が2人」、E0502 は「読む人と書く人が同時」。`later used here` が指す行が、借用がまだ終わっていない証拠。

## UNIT 04: スライスと借用 API の設計

### この単元のゴール

- `&str` と `&[T]` が「**どこから・何個分**」を表す借用（スライス）であることを、図で説明できる
- `&String` より `&str`、`&Vec<T>` より `&[T]` を引数にすると、呼び出し側が楽になる理由が分かる
- 「引数は借りる（`&str` / `&[T]`）、新しく作った結果は所有して返す（`String` / `Vec<T>`）」という API 設計の基本形を使える
- 日本語の文字列を切り出すときの落とし穴（文字の境界）を避けられる

### たとえ話でつかむ

分厚い料理の本があるとします。友だちに「卵料理のところだけ見て」と頼むとき、本をまるごと渡す必要はありません。**付箋**に「120ページから、5ページ分」と書いて本に貼れば十分です。

- **スライス = 「始まりの場所」と「長さ」を書いた付箋** です。本そのもの（持ち主）は別にいて、付箋は本の一部分を指しているだけです。
- `&str` は「文字列の一部分（または全部）を指す付箋」、`&[T]` は「並び（配列や `Vec`）の一部分を指す付箋」です。
- 付箋は本より長生きできません。本が捨てられたら、「120ページから5ページ分」という付箋は意味を失います。これは UNIT 03 で学んだ借用のルールそのものです。

付箋のいいところは、**本の種類を問わない**ことです。図書館の本でも、自分の本でも、雑誌の切り抜きでも、「どこから・何ページ分」と書けば同じ形の付箋になります。Rust でも、`String` の中身も、文字列リテラル（`"hello"`）も、`String` の一部分も、すべて同じ `&str` として扱えます。

> **用語: スライス** … 連続して並んだデータの一部分（または全部）を借りて見るための型です。`&str`（文字列スライス）と `&[T]`（配列スライス）の2つをよく使います。`[T]` や `str` そのものは「長さが決まっていない並び」なので、必ず `&` を付けた参照の形で使います。

### 図で理解する

`let owned = String::from("hello rust world");` から、一部分 `&owned[6..10]` を切り出したときのメモリです。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-03-15" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
<marker id="ar-03-16" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#10b981"/></marker>
</defs>
<rect x="6" y="6" width="236" height="278" rx="10" fill="#eff6ff" stroke="#3b82f6"/>
<text x="16" y="28" font-size="13" font-weight="700" fill="#1e40af">スタック</text>
<text x="16" y="62" font-size="12" font-weight="700" fill="#0f172a">owned</text>
<text x="16" y="78" font-size="11" fill="#475569">String 24B</text>
<rect x="96" y="42" width="120" height="72" rx="3" fill="#ffffff" stroke="#1e40af"/>
<line x1="96" y1="66" x2="216" y2="66" stroke="#bfdbfe"/>
<line x1="96" y1="90" x2="216" y2="90" stroke="#bfdbfe"/>
<text x="104" y="59" font-size="12" fill="#0f172a">ptr</text>
<circle cx="204" cy="54" r="3.5" fill="#475569"/>
<text x="104" y="83" font-size="12" fill="#0f172a">len = 16</text>
<text x="104" y="107" font-size="12" fill="#0f172a">cap = 16</text>
<text x="16" y="160" font-size="12" font-weight="700" fill="#0f172a">part</text>
<text x="16" y="176" font-size="11" fill="#475569">&amp;str 16B</text>
<rect x="96" y="142" width="120" height="48" rx="3" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<line x1="96" y1="166" x2="216" y2="166" stroke="#a7f3d0"/>
<text x="104" y="159" font-size="12" fill="#0f172a">ptr</text>
<circle cx="204" cy="154" r="3.5" fill="#10b981"/>
<text x="104" y="183" font-size="12" fill="#0f172a">len = 4</text>
<text x="16" y="236" font-size="12" font-weight="700" fill="#0f172a">r</text>
<text x="16" y="252" font-size="11" fill="#475569">&amp;String 8B</text>
<rect x="96" y="226" width="120" height="26" rx="3" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="104" y="244" font-size="12" fill="#0f172a">ptr</text>
<circle cx="204" cy="239" r="3.5" fill="#10b981"/>
<path d="M204,239 C232,239 232,102 218,100" fill="none" stroke="#10b981" stroke-width="2" marker-end="url(#ar-03-16)"/>
<rect x="262" y="6" width="452" height="278" rx="10" fill="#fff7ed" stroke="#f97316"/>
<text x="272" y="28" font-size="13" font-weight="700" fill="#c2410c">ヒープ（owned の本体 16 バイト）</text>
<rect x="282" y="40" width="25" height="30" fill="#ffffff" stroke="#c2410c"/>
<rect x="307" y="40" width="25" height="30" fill="#ffffff" stroke="#c2410c"/>
<rect x="332" y="40" width="25" height="30" fill="#ffffff" stroke="#c2410c"/>
<rect x="357" y="40" width="25" height="30" fill="#ffffff" stroke="#c2410c"/>
<rect x="382" y="40" width="25" height="30" fill="#ffffff" stroke="#c2410c"/>
<rect x="407" y="40" width="25" height="30" fill="#ffffff" stroke="#c2410c"/>
<rect x="432" y="40" width="25" height="30" fill="#d1fae5" stroke="#047857"/>
<rect x="457" y="40" width="25" height="30" fill="#d1fae5" stroke="#047857"/>
<rect x="482" y="40" width="25" height="30" fill="#d1fae5" stroke="#047857"/>
<rect x="507" y="40" width="25" height="30" fill="#d1fae5" stroke="#047857"/>
<rect x="532" y="40" width="25" height="30" fill="#ffffff" stroke="#c2410c"/>
<rect x="557" y="40" width="25" height="30" fill="#ffffff" stroke="#c2410c"/>
<rect x="582" y="40" width="25" height="30" fill="#ffffff" stroke="#c2410c"/>
<rect x="607" y="40" width="25" height="30" fill="#ffffff" stroke="#c2410c"/>
<rect x="632" y="40" width="25" height="30" fill="#ffffff" stroke="#c2410c"/>
<rect x="657" y="40" width="25" height="30" fill="#ffffff" stroke="#c2410c"/>
<text x="294" y="60" text-anchor="middle" font-size="13" fill="#0f172a" font-family="Consolas, Menlo, monospace">h</text>
<text x="319" y="60" text-anchor="middle" font-size="13" fill="#0f172a" font-family="Consolas, Menlo, monospace">e</text>
<text x="344" y="60" text-anchor="middle" font-size="13" fill="#0f172a" font-family="Consolas, Menlo, monospace">l</text>
<text x="369" y="60" text-anchor="middle" font-size="13" fill="#0f172a" font-family="Consolas, Menlo, monospace">l</text>
<text x="394" y="60" text-anchor="middle" font-size="13" fill="#0f172a" font-family="Consolas, Menlo, monospace">o</text>
<text x="419" y="60" text-anchor="middle" font-size="13" fill="#64748b" font-family="Consolas, Menlo, monospace">␣</text>
<text x="444" y="60" text-anchor="middle" font-size="13" fill="#047857" font-weight="700" font-family="Consolas, Menlo, monospace">r</text>
<text x="469" y="60" text-anchor="middle" font-size="13" fill="#047857" font-weight="700" font-family="Consolas, Menlo, monospace">u</text>
<text x="494" y="60" text-anchor="middle" font-size="13" fill="#047857" font-weight="700" font-family="Consolas, Menlo, monospace">s</text>
<text x="519" y="60" text-anchor="middle" font-size="13" fill="#047857" font-weight="700" font-family="Consolas, Menlo, monospace">t</text>
<text x="544" y="60" text-anchor="middle" font-size="13" fill="#64748b" font-family="Consolas, Menlo, monospace">␣</text>
<text x="569" y="60" text-anchor="middle" font-size="13" fill="#0f172a" font-family="Consolas, Menlo, monospace">w</text>
<text x="594" y="60" text-anchor="middle" font-size="13" fill="#0f172a" font-family="Consolas, Menlo, monospace">o</text>
<text x="619" y="60" text-anchor="middle" font-size="13" fill="#0f172a" font-family="Consolas, Menlo, monospace">r</text>
<text x="644" y="60" text-anchor="middle" font-size="13" fill="#0f172a" font-family="Consolas, Menlo, monospace">l</text>
<text x="669" y="60" text-anchor="middle" font-size="13" fill="#0f172a" font-family="Consolas, Menlo, monospace">d</text>
<text x="294" y="86" text-anchor="middle" font-size="12" fill="#64748b">0</text>
<text x="444" y="86" text-anchor="middle" font-size="12" fill="#047857">6</text>
<text x="544" y="86" text-anchor="middle" font-size="12" fill="#047857">10</text>
<text x="669" y="86" text-anchor="middle" font-size="12" fill="#64748b">15</text>
<path d="M432,96 L432,104 L557,104 L557,96" fill="none" stroke="#047857" stroke-width="1.5"/>
<text x="494" y="122" text-anchor="middle" font-size="12" fill="#047857">part = &amp;owned[6..10]（6 から 4 バイト分）</text>
<line x1="204" y1="54" x2="280" y2="54" stroke="#475569" stroke-width="2" marker-end="url(#ar-03-15)"/>
<path d="M204,154 C330,154 420,150 442,74" fill="none" stroke="#10b981" stroke-width="2" marker-end="url(#ar-03-16)"/>
<rect x="282" y="196" width="420" height="78" rx="8" fill="#ffffff" stroke="#cbd5e1"/>
<text x="296" y="218" font-size="12" fill="#0f172a">・&amp;str（part）は「場所 + 長さ」の 2 つ組 = 16 バイト</text>
<text x="296" y="238" font-size="12" fill="#0f172a">・&amp;String（r）は「String の持ち手の場所」だけ = 8 バイト</text>
<text x="296" y="258" font-size="12" fill="#0f172a">・part はヒープの途中を直接指す。コピーは 1 バイトも起きない</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-1: &amp;str は本体の途中を「場所＋長さ」で直接指す（ファットポインタ）。&amp;String は String の持ち手を指すだけ</div>
</div>

図のポイントは2つです。

1. `part`（`&str`）は、ヒープの本体の**6バイト目を直接指す場所**と、**長さ 4** の2つを持っています。このように「場所＋追加の情報」を持つ参照を**ファットポインタ**（太いポインタ）と呼びます。文字のコピーは1バイトも起きていません。
2. `r`（`&String`）は、スタックにある `owned`（持ち手）を指しているだけです。中身を読むには「`r` → `owned` → ヒープ」と2回たどる必要があり、しかも「`String` 全体」しか指せません。

`&str` なら「`String` 全体」も「`String` の一部分」も「文字列リテラル」も同じ形で指せます。だから、**文字列を読むだけの関数の引数は `&String` ではなく `&str` にする**のが Rust の定石です。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-03-17" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#10b981"/></marker>
</defs>
<rect x="6" y="6" width="350" height="218" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="18" y="28" font-size="13" font-weight="700" fill="#0f172a">文字列を読む関数は &amp;str で受ける</text>
<rect x="18" y="44" width="150" height="34" rx="6" fill="#ffffff" stroke="#3b82f6"/>
<text x="93" y="66" text-anchor="middle" font-size="12" fill="#0f172a" font-family="Consolas, Menlo, monospace">&amp;owned（String）</text>
<rect x="18" y="92" width="150" height="34" rx="6" fill="#ffffff" stroke="#3b82f6"/>
<text x="93" y="114" text-anchor="middle" font-size="12" fill="#0f172a" font-family="Consolas, Menlo, monospace">"good morning"</text>
<rect x="18" y="140" width="150" height="34" rx="6" fill="#ffffff" stroke="#3b82f6"/>
<text x="93" y="162" text-anchor="middle" font-size="12" fill="#0f172a" font-family="Consolas, Menlo, monospace">&amp;owned[6..10]</text>
<rect x="214" y="80" width="130" height="58" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="279" y="104" text-anchor="middle" font-size="12" fill="#047857" font-family="Consolas, Menlo, monospace">first_word(</text>
<text x="279" y="124" text-anchor="middle" font-size="12" fill="#047857" font-family="Consolas, Menlo, monospace">text: &amp;str)</text>
<line x1="168" y1="61" x2="212" y2="96" stroke="#10b981" stroke-width="2" marker-end="url(#ar-03-17)"/>
<line x1="168" y1="109" x2="212" y2="109" stroke="#10b981" stroke-width="2" marker-end="url(#ar-03-17)"/>
<line x1="168" y1="157" x2="212" y2="122" stroke="#10b981" stroke-width="2" marker-end="url(#ar-03-17)"/>
<text x="181" y="204" text-anchor="middle" font-size="12" fill="#047857">どれも同じ「場所 + 長さ」として渡せる</text>
<rect x="364" y="6" width="350" height="218" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="376" y="28" font-size="13" font-weight="700" fill="#0f172a">並びを読む関数は &amp;[T] で受ける</text>
<rect x="376" y="44" width="150" height="34" rx="6" fill="#ffffff" stroke="#3b82f6"/>
<text x="451" y="66" text-anchor="middle" font-size="12" fill="#0f172a" font-family="Consolas, Menlo, monospace">&amp;array（配列）</text>
<rect x="376" y="92" width="150" height="34" rx="6" fill="#ffffff" stroke="#3b82f6"/>
<text x="451" y="114" text-anchor="middle" font-size="12" fill="#0f172a" font-family="Consolas, Menlo, monospace">&amp;vec（Vec）</text>
<rect x="376" y="140" width="150" height="34" rx="6" fill="#ffffff" stroke="#3b82f6"/>
<text x="451" y="162" text-anchor="middle" font-size="12" fill="#0f172a" font-family="Consolas, Menlo, monospace">&amp;vec[1..3]</text>
<rect x="572" y="80" width="130" height="58" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="637" y="104" text-anchor="middle" font-size="12" fill="#047857" font-family="Consolas, Menlo, monospace">total(</text>
<text x="637" y="124" text-anchor="middle" font-size="12" fill="#047857" font-family="Consolas, Menlo, monospace">scores: &amp;[i32])</text>
<line x1="526" y1="61" x2="570" y2="96" stroke="#10b981" stroke-width="2" marker-end="url(#ar-03-17)"/>
<line x1="526" y1="109" x2="570" y2="109" stroke="#10b981" stroke-width="2" marker-end="url(#ar-03-17)"/>
<line x1="526" y1="157" x2="570" y2="122" stroke="#10b981" stroke-width="2" marker-end="url(#ar-03-17)"/>
<text x="539" y="204" text-anchor="middle" font-size="12" fill="#047857">配列も Vec もその一部も受け取れる</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-2: 引数をスライスにすると、呼ぶ側はいろいろな形の値をそのまま渡せる</div>
</div>

### まず動かす最小コード

#### 例1: `&str` と `&[i32]` を受け取る関数

> **▼ このコードがやること（先に日本語で）:** 文字列の最初の単語を返す関数と、整数の並びの合計を返す関数を作ります。どちらも引数をスライスにしたので、`String`・文字列リテラル・配列・`Vec`・その一部分を、区別なく渡せることを確かめます。最後に、参照の大きさを表示して、スライスが「場所＋長さ」の2つ組であることを確かめます。

`src/bin/u04_slices.rs`:

```rust
use std::mem::size_of;

// &str を受け取れば、String からも文字列リテラルからも呼べる
fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(end) => &text[..end],
        None => text,
    }
}

// &[i32] を受け取れば、配列からも Vec からも、その一部分からも呼べる
fn total(scores: &[i32]) -> i32 {
    let mut sum = 0;
    for s in scores {
        sum += s;
    }
    sum
}

fn main() {
    let owned = String::from("hello rust world");
    let literal = "good morning";
    println!("{}", first_word(&owned));
    println!("{}", first_word(literal));

    // String の一部分（6バイト目から10バイト目の手前まで）を借りる
    let part = &owned[6..10];
    println!("part = {part} / len = {}", part.len());

    let array = [90, 75, 60];
    let vec = vec![10, 20, 30, 40];
    println!("配列の合計 = {}", total(&array));
    println!("Vec の合計 = {}", total(&vec));
    println!("Vec の一部の合計 = {}", total(&vec[1..3]));

    // スライス参照は「場所 + 長さ」の2つ組なので、ふつうの参照の2倍の大きさ
    println!("&String の大きさ: {} バイト", size_of::<&String>());
    println!("&str の大きさ: {} バイト", size_of::<&str>());
    println!("&[i32] の大きさ: {} バイト", size_of::<&[i32]>());
}
```

```powershell
cargo run --bin u04_slices
```

**▼ 実行結果**

```text
hello
good
part = rust / len = 4
配列の合計 = 225
Vec の合計 = 100
Vec の一部の合計 = 50
&String の大きさ: 8 バイト
&str の大きさ: 16 バイト
&[i32] の大きさ: 16 バイト
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 範囲を指定して切り出す `&text[..end]`

```rust
fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(end) => &text[..end],
        None => text,
    }
}
```

- `text.find(' ')` は、最初の空白が**何バイト目にあるか**を `Option<usize>` で返します。見つからなければ `None` です。
- `&text[..end]` は「先頭から `end` バイト目の手前まで」の部分を借りる書き方です。`[a..b]` は「a 以上 b 未満」、`[..b]` は「先頭から b 未満」、`[a..]` は「a から最後まで」を表します。
- 戻り値の `&str` は、**引数の `text` の一部分を指しています**。新しい文字列は作っていません。図4-1の `part` と同じ形です。

##### 解説2: `&owned` が `&str` として渡せる理由

```rust
    println!("{}", first_word(&owned));
    println!("{}", first_word(literal));
```

- `&owned` の型は `&String` ですが、`&str` を期待する引数に渡すと、Rust が自動で `&str` に変換してくれます。これを**参照外し型強制**（Deref coercion）と呼びます。「`String` は `str` を中に持っているので、`&String` から `&str` を作れる」という仕組みです。
- `literal` は文字列リテラルで、最初から `&str` 型です。プログラムの中に埋め込まれた文字列を指しています（UNIT 06 で詳しく見ます）。

##### 解説3: `&[i32]` は配列も `Vec` も受け取れる

```rust
    println!("配列の合計 = {}", total(&array));
    println!("Vec の合計 = {}", total(&vec));
    println!("Vec の一部の合計 = {}", total(&vec[1..3]));
```

- `&array` は `&[i32; 3]`（長さ3の配列への参照）、`&vec` は `&Vec<i32>` ですが、どちらも `&[i32]` に自動で変換されます。
- `&vec[1..3]` は `Vec` の1番目と2番目（`20` と `30`）だけを指すスライスで、合計は 50 です。
- もし引数を `&Vec<i32>` にしていたら、配列や `Vec` の一部分は渡せません（わざわざ `Vec` を作り直す必要が出てきます）。

##### 解説4: スライスの大きさは16バイト

```rust
    println!("&str の大きさ: {} バイト", size_of::<&str>());
```

- `&String` は場所だけの8バイト、`&str` と `&[i32]` は「場所＋長さ」で16バイトです。図4-1で見たファットポインタの正体です。

### もう一歩: 実務に近い形

関数の引数と戻り値の型は、次の表を目安に選びます。

| やりたいこと | 引数の型 | 戻り値の型 | 例 |
|---|---|---|---|
| 文字列を読むだけ | `&str` | なし／数値など | 文字数を数える、検証する |
| 文字列の一部分を返す | `&str` | `&str` | 最初の単語、先頭 N 文字 |
| 文字列から新しい文字列を作る | `&str` | `String` | 小文字にする、前後の空白を消す |
| 受け取った文字列を保存する | `String` | なし | struct のフィールドに入れる |
| 並びを読むだけ | `&[T]` | なし／数値など | 合計、件数、検索 |
| 並びの中身を書き換える（件数は変えない） | `&mut [T]` | なし | 全部2倍、並べ替え |
| 並びに追加・削除する | `&mut Vec<T>` | なし | `push`、`remove` |

この表に沿って、RustNote で使いそうな小さな関数をいくつか作ってみます。

#### 例2: 借りて受け取り、必要なら所有して返す

> **▼ このコードがやること（先に日本語で）:** フォームから届いたメールアドレスの前後の空白を消して小文字にする関数、ノートのタイトルを検証する関数、本文の先頭だけを借りて返す関数、完了済みの件数を数える関数を作ります。どの関数も引数は借用で受け取り、新しく作ったものだけを所有した値で返します。

`src/bin/u04_api_design.rs`:

```rust
// 入力は読むだけなので &str、結果は新しく作るので String
fn normalize_email(input: &str) -> String {
    input.trim().to_lowercase()
}

struct NoteTitle(String);

impl NoteTitle {
    // 検証に通ったら、所有する String を中に持った NoteTitle を返す
    fn parse(input: &str) -> Result<NoteTitle, String> {
        let trimmed = input.trim();
        let count = trimmed.chars().count();
        if count == 0 || count > 80 {
            return Err(format!("タイトルは1〜80文字です（今は{count}文字）"));
        }
        Ok(NoteTitle(trimmed.to_string()))
    }

    // 中の文字列を借りて見せる
    fn as_str(&self) -> &str {
        &self.0
    }
}

// 本文の先頭 max_chars 文字ぶんを「借りて」返す。新しい String は作らない
fn preview(body: &str, max_chars: usize) -> &str {
    match body.char_indices().nth(max_chars) {
        Some((byte_pos, _)) => &body[..byte_pos],
        None => body,
    }
}

// 完了フラグの一覧を借りて、true の数を数える
fn count_done(flags: &[bool]) -> usize {
    flags.iter().filter(|done| **done).count()
}

fn main() {
    let form_email = String::from("  Alice@Example.COM ");
    println!("[{}]", normalize_email(&form_email));

    for input in ["  買い物リスト  ", "   "] {
        match NoteTitle::parse(input) {
            Ok(title) => println!("OK: [{}]", title.as_str()),
            Err(message) => println!("NG: {message}"),
        }
    }

    let body = String::from("牛乳、卵、パン、バター、ヨーグルト");
    println!("{}…", preview(&body, 8));
    println!("{}", preview("短い", 8));

    let flags = vec![true, false, true, true];
    println!("完了 {} 件", count_done(&flags));
    println!("先頭2件のうち完了 {} 件", count_done(&flags[..2]));
}
```

**▼ 実行結果**

```text
[alice@example.com]
OK: [買い物リスト]
NG: タイトルは1〜80文字です（今は0文字）
牛乳、卵、パン、…
短い
完了 3 件
先頭2件のうち完了 1 件
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `&str` を受けて `String` を返す

```rust
fn normalize_email(input: &str) -> String {
    input.trim().to_lowercase()
}
```

- `trim()` は前後の空白を除いた部分を `&str` で返します（新しい文字列は作らず、`input` の一部分を指すだけ）。
- `to_lowercase()` は小文字にした**新しい `String`** を作ります。文字が変わるので、元の文字列の一部分を指すだけでは表せないからです。
- 「入力は借りる、作った結果は所有して返す」の典型です。呼ぶ側は `String` でもリテラルでも渡せて、結果は自由に保存できます。

##### 解説2: 検証して所有型に入れる `NoteTitle::parse`

```rust
    fn parse(input: &str) -> Result<NoteTitle, String> {
        let trimmed = input.trim();
        let count = trimmed.chars().count();
        if count == 0 || count > 80 {
            return Err(format!("タイトルは1〜80文字です（今は{count}文字）"));
        }
        Ok(NoteTitle(trimmed.to_string()))
    }
```

- 入力は `&str` で借り、検証が通ったときだけ `to_string()` で `String` を作って `NoteTitle` に入れます。検証に失敗したときは、無駄な文字列のコピーが起きません。
- 文字数は `chars().count()` で数えます。`len()` はバイト数なので、日本語のタイトルでは 3 倍近い数字になってしまいます。
- `as_str(&self) -> &str` は、中の `String` を `&str` として貸し出すメソッドです。`&self.0` は `&String` ですが、戻り値の型が `&str` なので自動で変換されます。

##### 解説3: 日本語でも安全に先頭を切り出す `preview`

```rust
fn preview(body: &str, max_chars: usize) -> &str {
    match body.char_indices().nth(max_chars) {
        Some((byte_pos, _)) => &body[..byte_pos],
        None => body,
    }
}
```

- `char_indices()` は「（その文字が始まるバイト位置, 文字）」の組を順に取り出します。`.nth(8)` で9文字目（0から数えて8番目）の組を取ると、その `byte_pos` は「8文字ぶんがちょうど終わる位置」です。
- その位置までを `&body[..byte_pos]` で借りて返します。文字の境目で切っているので、日本語でも安全です（後の「よくあるエラー2」で、境目でない所を切ると何が起きるかを見ます）。
- 本文が8文字以下なら `nth` が `None` を返すので、本文全体をそのまま返します。

##### 解説4: `&[bool]` で並びを借りて数える

```rust
fn count_done(flags: &[bool]) -> usize {
    flags.iter().filter(|done| **done).count()
}
```

- `flags.iter()` は要素を `&bool` で1つずつ取り出し、`filter` は条件に合うものだけを残します。`filter` のクロージャには「要素への参照」がさらに参照で渡されるので、`done` は `&&bool` です。`**done` で2回参照外しして `bool` を取り出しています。
- `&flags[..2]` のように一部分だけを渡しても動きます。引数をスライスにしておいた効果です。

#### 日本語の文字列と「文字の境目」

`&str` の範囲指定 `[a..b]` は**バイト位置**で指定します。UTF-8（ユーティーエフ・エイト：世界中の文字をバイトの並びで表す方式。Rust の文字列はすべてこれ）では、英数字は1文字1バイトですが、ひらがなや漢字は1文字3バイトです。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 170" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="6" y="6" width="708" height="158" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="20" y="30" font-size="13" font-weight="700" fill="#0f172a">"あいう" のバイトの並び（len = 9、文字数 = 3）</text>
<rect x="120" y="48" width="160" height="40" fill="#eff6ff" stroke="#3b82f6"/>
<rect x="280" y="48" width="160" height="40" fill="#ecfdf5" stroke="#10b981"/>
<rect x="440" y="48" width="160" height="40" fill="#fff7ed" stroke="#f97316"/>
<line x1="173" y1="48" x2="173" y2="88" stroke="#93c5fd" stroke-dasharray="3 3"/>
<line x1="227" y1="48" x2="227" y2="88" stroke="#93c5fd" stroke-dasharray="3 3"/>
<line x1="333" y1="48" x2="333" y2="88" stroke="#6ee7b7" stroke-dasharray="3 3"/>
<line x1="387" y1="48" x2="387" y2="88" stroke="#6ee7b7" stroke-dasharray="3 3"/>
<line x1="493" y1="48" x2="493" y2="88" stroke="#fdba74" stroke-dasharray="3 3"/>
<line x1="547" y1="48" x2="547" y2="88" stroke="#fdba74" stroke-dasharray="3 3"/>
<text x="200" y="74" text-anchor="middle" font-size="16" font-weight="700" fill="#1e40af">あ</text>
<text x="360" y="74" text-anchor="middle" font-size="16" font-weight="700" fill="#047857">い</text>
<text x="520" y="74" text-anchor="middle" font-size="16" font-weight="700" fill="#c2410c">う</text>
<text x="120" y="106" text-anchor="middle" font-size="12" fill="#047857">0 ✓</text>
<text x="173" y="106" text-anchor="middle" font-size="12" fill="#dc2626">1 ✕</text>
<text x="227" y="106" text-anchor="middle" font-size="12" fill="#dc2626">2 ✕</text>
<text x="280" y="106" text-anchor="middle" font-size="12" fill="#047857">3 ✓</text>
<text x="440" y="106" text-anchor="middle" font-size="12" fill="#047857">6 ✓</text>
<text x="600" y="106" text-anchor="middle" font-size="12" fill="#047857">9 ✓</text>
<text x="20" y="134" font-size="12" fill="#047857">✓ 文字の境目: &amp;s[0..3] = "あ"、&amp;s[3..9] = "いう" のように切れる</text>
<text x="20" y="154" font-size="12" fill="#dc2626">✕ 文字の途中: &amp;s[0..1] は「あ」の3バイトの途中で切ろうとするので、実行時に panic する</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-3: 日本語は1文字3バイト。範囲指定は必ず文字の境目（0・3・6・9）で行う</div>
</div>

> **▼ このコードがやること（先に日本語で）:** `"あいう"` のバイト数と文字数を比べ、境目で正しく切り出せることと、各文字が何バイト目から始まるかを表示します。

`src/bin/u04_japanese.rs`:

```rust
fn main() {
    let s = "あいう";
    // len() は「バイト数」。日本語のひらがなは1文字3バイト
    println!("len = {}", s.len());
    println!("文字数 = {}", s.chars().count());
    // 0〜3バイト目は「あ」の1文字ぶん
    println!("&s[0..3] = {}", &s[0..3]);
    // 文字の区切りと、その位置（バイト）を順に取り出す
    for (byte_pos, ch) in s.char_indices() {
        println!("{byte_pos} バイト目から: {ch}");
    }
}
```

**▼ 実行結果**

```text
len = 9
文字数 = 3
&s[0..3] = あ
0 バイト目から: あ
3 バイト目から: い
6 バイト目から: う
```

- ユーザーが入力した文字列を「先頭○文字」で切りたいときは、数字を直接書かず、例2の `preview` のように `char_indices()` で境目を探してから切り出しましょう。

### よくあるエラーと読み方

#### エラー1: スライスを持ったまま元の String を書き換える（E0502）

```rust
fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(end) => &text[..end],
        None => text,
    }
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("word = {word}");
}
```

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src\bin\u04_err_clear.rs:11:5
   |
10 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
11 |     s.clear();
   |     ^^^^^^^^^ mutable borrow occurs here
12 |     println!("word = {word}");
   |                       ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership-lab` (bin "u04_err_clear") due to 1 previous error
```

- **何が起きたか:** `word` は `s` の本体の一部分を指す付箋です（図4-1の `part` と同じ形）。`s.clear()` で中身を空にすると、`word` の「0バイト目から5バイト分」は存在しない文字を指すことになります。
- **注目点:** `first_word(&s)` の戻り値 `word` を使っている限り、**`&s` の借用も続いている**とコンパイラは判断しています。関数の戻り値が引数を借りていることを、コンパイラは関数の型（`&str` を受けて `&str` を返す）から読み取っています。その仕組みは UNIT 05 で学びます。
- **直し方:** `word` を使い終わってから `clear` するか、`clear` の後も残しておきたいなら `let word = first_word(&s).to_string();` で所有した `String` にしておきます。

#### エラー2: 文字の途中で切る（実行時 panic）

```rust
fn main() {
    let s = String::from("あいう");
    let head = &s[0..1];
    println!("{head}");
}
```

これはコンパイルは通りますが、実行すると止まります。

```text
thread 'main' (552) panicked at src\bin\u04_err_boundary.rs:3:18:
end byte index 1 is not a char boundary; it is inside 'あ' (bytes 0..3 of string)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- **何が起きたか:** 「終わりのバイト位置 1 は文字の境目ではありません。'あ'（0〜3バイト）の内側です」。図4-3の ✕ の位置で切ろうとしました。`(552)` はスレッドの番号で、実行ごとに変わります。
- **panic（パニック）とは:** プログラムを続けると危険な状態になったときに、その場で処理を止める仕組みです。コンパイル時ではなく**実行時**に起きる点に注意してください。範囲の数字は実行してみないと分からないことが多いので、コンパイラにはチェックできません。
- **直し方:** 文字単位で扱うなら `chars()` や `char_indices()` を使います。どうしても数字で切りたいときは、`s.get(0..1)` を使うと、境目でない場合に panic せず `None` が返ります。

#### エラー3: `str` をそのまま受け取ろうとする（E0277）

```rust
fn shout(text: str) {
    println!("{}!", &text);
}

fn main() {}
```

```text
error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> src\bin\u04_err_str_by_value.rs:1:16
  |
1 | fn shout(text: str) {
  |                ^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `str`
help: function arguments must have a statically known size, borrowed types always have a known size
  |
1 | fn shout(text: &str) {
  |                +

For more information about this error, try `rustc --explain E0277`.
error: could not compile `ownership-lab` (bin "u04_err_str_by_value") due to 1 previous error
```

- **何が起きたか:** 「`str` の大きさはコンパイル時に分かりません」。`str` は「何バイトあるか分からない文字の並び」そのものなので、スタック（大きさの決まった値しか置けない机）に置けません。
- **直し方:** `help:` のとおり `&str` にします。参照なら「場所＋長さ」の16バイトと大きさが決まっているので、引数にできます。`[T]` も同じ理由で、必ず `&[T]` の形で使います。

#### エラー4: `&str` の所に `String` を渡す（E0308）

```rust
fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(end) => &text[..end],
        None => text,
    }
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(s);
    println!("{word}");
}
```

```text
error[E0308]: mismatched types
  --> src\bin\u04_err_mismatch.rs:10:27
   |
10 |     let word = first_word(s);
   |                ---------- ^ expected `&str`, found `String`
   |                |
   |                arguments to this function are incorrect
   |
note: function defined here
  --> src\bin\u04_err_mismatch.rs:1:4
   |
 1 | fn first_word(text: &str) -> &str {
   |    ^^^^^^^^^^ ----------
help: consider borrowing here
   |
10 |     let word = first_word(&s);
   |                           +

For more information about this error, try `rustc --explain E0308`.
error: could not compile `ownership-lab` (bin "u04_err_mismatch") due to 1 previous error
```

- **何が起きたか:** 「型が合いません。`&str` を期待したのに `String` が来ました」。参照外し型強制は `&String` → `&str` の変換なので、`&` を付け忘れた `String` そのものは変換されません。
- **直し方:** `help:` のとおり `&s` で貸します。

### RustNote ではここで使う

RustNote の関数は、この単元の表に沿って設計します。

- **リポジトリ（データベースを読み書きする係）の関数は `&str` を受けて所有型を返す:** たとえば「メールアドレスでユーザーを探す」関数は、メールアドレスを `&str` で借りて受け取り、見つかったユーザーは所有した値（`String` を中に持つ struct）で返します。データベースから読んだ値は、関数が終わった後も使うので、借用では返せないからです。
- **`NoteTitle::parse(&str)`:** 例2と同じ形で、フォームの入力を借りて検証し、通ったら `NoteTitle`（中に `String`）を返します。
- **検索語 `?q=`:** URL から取り出した検索語は `&str` のまま検索関数に渡し、不要なコピーを作りません。
- **一覧の表示:** ノートの一覧を HTML にする関数は `&[Note]` を受け取ります。全件でも、ページ送りで切り出した一部分でも、同じ関数で表示できます。

### 練習問題

**問題1** 文字列の最後の単語を返す関数 `last_word(text: &str) -> &str` を作ってください。空白がなければ文字列全体を返します（ヒント: 後ろから探す `rfind`）。

**問題2** 整数のスライスを受け取り、最大値を `Option<i32>` で返す関数 `max_score(scores: &[i32]) -> Option<i32>` を作ってください。空のスライスなら `None` を返します。`Vec` 全体、`Vec` の一部分、空の配列 `&[]` で試してください。

**問題3** 名前の各単語の頭文字をつなげた文字列を返す関数 `initials(name: &str)` を作ってください（`"Grace Brewster Hopper"` → `"GBH"`）。また、戻り値の型を `&str` ではなく `String` にしなければならない理由を説明してください。

### 解答と解説

**問題1の解答例**（`src/bin/u04_ex1.rs`）

```rust
fn last_word(text: &str) -> &str {
    match text.rfind(' ') {
        Some(pos) => &text[pos + 1..],
        None => text,
    }
}

fn main() {
    let owned = String::from("hello rust world");
    println!("{}", last_word(&owned));
    println!("{}", last_word("single"));
}
```

**▼ 実行結果**

```text
world
single
```

- `rfind(' ')` は最後の空白のバイト位置を返します。空白そのものは含めたくないので、`pos + 1` から最後まで（`[pos + 1..]`）を借ります。空白は1バイトなので、`pos + 1` は必ず文字の境目です。

**問題2の解答例**（`src/bin/u04_ex2.rs`）

```rust
fn max_score(scores: &[i32]) -> Option<i32> {
    let mut best: Option<i32> = None;
    for &s in scores {
        best = match best {
            Some(b) if b >= s => Some(b),
            _ => Some(s),
        };
    }
    best
}

fn main() {
    let scores = vec![72, 95, 88];
    println!("{:?}", max_score(&scores));
    println!("{:?}", max_score(&scores[..1]));
    println!("{:?}", max_score(&[]));
}
```

**▼ 実行結果**

```text
Some(95)
Some(72)
None
```

- `for &s in scores` の `&s` は、取り出した `&i32` から「参照を外した `i32`」を `s` に入れるパターンです（`i32` は Copy なので取り出せます）。
- `match` の `Some(b) if b >= s` は「今の最大値 `b` が `s` 以上なら、そのまま」という条件付きのパターンです。それ以外（まだ `None`、または `s` の方が大きい）なら `Some(s)` にします。
- なお、標準ライブラリには `scores.iter().max()` という便利なメソッドもあります（戻り値は `Option<&i32>`）。

**問題3の解答例**（`src/bin/u04_ex3.rs`）

```rust
fn initials(name: &str) -> String {
    let mut result = String::new();
    for word in name.split_whitespace() {
        if let Some(first) = word.chars().next() {
            result.push(first);
        }
    }
    result
}

fn main() {
    println!("{}", initials("Grace Brewster Hopper"));
    println!("{}", initials("山田 太郎"));
}
```

**▼ 実行結果**

```text
GBH
山太
```

- 戻り値を `String` にする理由: 頭文字の `G`・`B`・`H` は元の文字列の中で**離れた場所**にあり、「場所＋長さ」の1つの付箋では指せません。新しい並びを作る必要があるので、所有した `String` で返します。「元の文字列のひと続きの一部分」を返すときだけ `&str` を返せる、と覚えておきましょう。

### まとめ

- `&str` と `&[T]` は「場所＋長さ」で本体の一部分を指す借用（ファットポインタ）。コピーは起きない。
- 読むだけの引数は `&String` / `&Vec<T>` ではなく `&str` / `&[T]` にすると、どんな形の値でも渡せる。新しく作った結果は `String` / `Vec<T>` で返す。
- `&s[a..b]` はバイト位置。日本語は1文字3バイトなので、`chars()` や `char_indices()` で境目を探して切る。

