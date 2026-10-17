## UNIT 07: struct・impl・関連関数・メソッド

### この単元のゴール

- `struct` で名前付きのフィールドを持つ型を定義し、値（インスタンス）を作って読み書きできる
- `impl` ブロックに、**関連関数**（`Note::new` のように型から呼ぶ）と**メソッド**（`note.toggle()` のように値から呼ぶ）を書ける
- メソッドの `&self` / `&mut self` / `self` の違いを説明できる
- `#[derive(Debug)]` で中身を表示し、タプル構造体（newtype）で ID の取り違えを防げる

### たとえ話でつかむ

- **struct**（ストラクト：構造体）は「**申込用紙の書式**」です。「ID」「タイトル」「本文」「完了」と、記入欄（**フィールド**）の名前と種類が決まっています。
- **インスタンス**は、その書式に「**記入済みの用紙**」です。同じ書式から何枚でも作れ、1枚ずつ中身が違います。
- **impl**（インプル：implementation ＝ 実装）は、その用紙専用の「**窓口**」です。
  - 新しい用紙を**発行する窓口**が**関連関数**（`Note::new`）。まだ用紙がないので、型の名前に向かって頼みます。
  - 記入済みの用紙を持って行って**手続きしてもらう窓口**が**メソッド**（`note.toggle()`）。用紙そのもの（`self`）に対して働きます。
- メソッドの受け取り方は3種類あります。用紙を**見せるだけ**（`&self`）、**書き込んでもらう**（`&mut self`）、**渡してしまう**（`self`）です。

UNIT 06 ではノートを `(&str, bool)` のタプルで表しましたが、`.0` が何だったかを覚えておく必要がありました。struct なら `note.title` と**名前で**読めます。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-09" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="170" y="26" text-anchor="middle" font-size="15" font-weight="700" fill="#1e40af">struct Note（書式＝設計図）</text>
<rect x="30" y="40" width="280" height="200" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="50" y="76" font-size="14" font-family="Consolas, monospace" fill="#1e40af">id</text>
<rect x="130" y="58" width="160" height="28" rx="4" fill="#ffffff" stroke="#93c5fd" stroke-dasharray="4 3"/>
<text x="210" y="77" text-anchor="middle" font-size="12" fill="#64748b">u64</text>
<text x="50" y="122" font-size="14" font-family="Consolas, monospace" fill="#1e40af">title</text>
<rect x="130" y="104" width="160" height="28" rx="4" fill="#ffffff" stroke="#93c5fd" stroke-dasharray="4 3"/>
<text x="210" y="123" text-anchor="middle" font-size="12" fill="#64748b">String</text>
<text x="50" y="168" font-size="14" font-family="Consolas, monospace" fill="#1e40af">body</text>
<rect x="130" y="150" width="160" height="28" rx="4" fill="#ffffff" stroke="#93c5fd" stroke-dasharray="4 3"/>
<text x="210" y="169" text-anchor="middle" font-size="12" fill="#64748b">String</text>
<text x="50" y="214" font-size="14" font-family="Consolas, monospace" fill="#1e40af">done</text>
<rect x="130" y="196" width="160" height="28" rx="4" fill="#ffffff" stroke="#93c5fd" stroke-dasharray="4 3"/>
<text x="210" y="215" text-anchor="middle" font-size="12" fill="#64748b">bool</text>
<line x1="315" y1="140" x2="395" y2="140" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-09)"/>
<text x="355" y="128" text-anchor="middle" font-size="12" fill="#475569">記入する</text>
<text x="550" y="26" text-anchor="middle" font-size="15" font-weight="700" fill="#047857">let note = Note { ... }（記入済み）</text>
<rect x="400" y="40" width="300" height="200" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="420" y="76" font-size="14" font-family="Consolas, monospace" fill="#047857">id</text>
<rect x="500" y="58" width="180" height="28" rx="4" fill="#ffffff" stroke="#047857"/>
<text x="590" y="78" text-anchor="middle" font-size="14" fill="#0f172a">1</text>
<text x="420" y="122" font-size="14" font-family="Consolas, monospace" fill="#047857">title</text>
<rect x="500" y="104" width="180" height="28" rx="4" fill="#ffffff" stroke="#047857"/>
<text x="590" y="124" text-anchor="middle" font-size="14" fill="#0f172a">"牛乳を買う"</text>
<text x="420" y="168" font-size="14" font-family="Consolas, monospace" fill="#047857">body</text>
<rect x="500" y="150" width="180" height="28" rx="4" fill="#ffffff" stroke="#047857"/>
<text x="590" y="170" text-anchor="middle" font-size="14" fill="#0f172a">"低脂肪のもの"</text>
<text x="420" y="214" font-size="14" font-family="Consolas, monospace" fill="#047857">done</text>
<rect x="500" y="196" width="180" height="28" rx="4" fill="#ffffff" stroke="#047857"/>
<text x="590" y="216" text-anchor="middle" font-size="14" fill="#0f172a">false</text>
<text x="360" y="262" text-anchor="middle" font-size="12" fill="#334155">読むときは note.title、書き換えるときは note.done = true;（let mut が必要）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図7-1: struct は記入欄の名前と型を決めた書式。インスタンスはそれを埋めた1枚の用紙</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-10" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="24" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">impl Note { ... } の中身</text>
<rect x="20" y="44" width="200" height="200" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="120" y="72" text-anchor="middle" font-size="14" font-weight="700" fill="#c2410c">関連関数</text>
<text x="120" y="102" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#0f172a">Note::new(1, "..")</text>
<text x="120" y="132" text-anchor="middle" font-size="12" fill="#334155">self を受け取らない</text>
<text x="120" y="154" text-anchor="middle" font-size="12" fill="#334155">型の名前 + :: で呼ぶ</text>
<text x="120" y="186" text-anchor="middle" font-size="12" fill="#c2410c">用紙を発行する窓口</text>
<text x="120" y="208" text-anchor="middle" font-size="12" fill="#c2410c">（作る・変換する）</text>
<line x1="220" y1="144" x2="256" y2="144" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-10)"/>
<rect x="260" y="44" width="440" height="200" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="480" y="72" text-anchor="middle" font-size="14" font-weight="700" fill="#1e40af">メソッド（値 + . で呼ぶ）</text>
<rect x="276" y="88" width="408" height="44" rx="8" fill="#ffffff" stroke="#93c5fd"/>
<text x="290" y="115" font-size="14" font-family="Consolas, monospace" fill="#0f172a">note.summary()</text>
<text x="470" y="108" font-size="12" font-weight="700" fill="#1e40af">&amp;self</text>
<text x="470" y="124" font-size="12" fill="#334155">見せるだけ（読む）</text>
<rect x="276" y="138" width="408" height="44" rx="8" fill="#ffffff" stroke="#93c5fd"/>
<text x="290" y="165" font-size="14" font-family="Consolas, monospace" fill="#0f172a">note.toggle()</text>
<text x="470" y="158" font-size="12" font-weight="700" fill="#047857">&amp;mut self</text>
<text x="470" y="174" font-size="12" fill="#334155">書き込んでもらう（let mut 必要）</text>
<rect x="276" y="188" width="408" height="44" rx="8" fill="#ffffff" stroke="#93c5fd"/>
<text x="290" y="215" font-size="14" font-family="Consolas, monospace" fill="#0f172a">note.into_title()</text>
<text x="470" y="208" font-size="12" font-weight="700" fill="#c2410c">self</text>
<text x="470" y="224" font-size="12" fill="#334155">渡してしまう（以後 note は使えない）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図7-2: 関連関数は「型」に、メソッドは「値」に対して呼ぶ。self の受け取り方で、できることが変わる</div>
</div>

### まず動かす最小コード

#### 例1: struct を定義してインスタンスを作る

> **▼ このコードがやること（先に日本語で）:** ノートを表す `Note` 型を定義し、1枚のノートを作って、フィールドを1つずつ表示します。最後に、`{:?}` と `{:#?}` で中身をまとめて表示します。

```rust
// #[derive(Debug)] を付けると、{:?} で中身を表示できるようになる
#[derive(Debug)]
struct Note {
    id: u64,
    title: String,
    body: String,
    done: bool,
}

fn main() {
    // 「フィールド名: 値」を並べて、Note の値（インスタンス）を作る
    let note = Note {
        id: 1,
        title: String::from("牛乳を買う"),
        body: String::from("低脂肪のもの"),
        done: false,
    };
    // 値.フィールド名 で取り出す
    println!("#{} {}（完了: {}）", note.id, note.title, note.done);
    println!("本文: {}", note.body);
    // {:?} は1行で、{:#?} は複数行で見やすく表示する
    println!("{note:?}");
    println!("{note:#?}");
}
```

```powershell
cargo run
```

**▼ 実行結果**

```text
#1 牛乳を買う（完了: false）
本文: 低脂肪のもの
Note { id: 1, title: "牛乳を買う", body: "低脂肪のもの", done: false }
Note {
    id: 1,
    title: "牛乳を買う",
    body: "低脂肪のもの",
    done: false,
}
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: struct の定義

```rust
struct Note {
    id: u64,
    title: String,
    body: String,
    done: bool,
}
```

- `struct 型名 { フィールド名: 型, ... }` で、新しい型を作ります。型名は大文字で始めるのが決まりです（`Note`、`NoteList`）。
- フィールドの文字列は `&str` ではなく `String` にしています。構造体は**自分の中身を自分で持つ**のが基本だからです（`&str` は借り物なので、貸し主より長く生きられないという制約が付きます。第3巻で学びます）。

##### 解説2: インスタンスの作成とフィールドの読み出し

```rust
let note = Note {
    id: 1,
    title: String::from("牛乳を買う"),
    body: String::from("低脂肪のもの"),
    done: false,
};
println!("#{} {}（完了: {}）", note.id, note.title, note.done);
```

- `型名 { フィールド名: 値, ... }` で値を作ります。**すべてのフィールドに値を入れる**必要があります（1つでも欠けるとエラー）。並び順は定義と違ってもかまいません。
- `note.title` のように `.フィールド名` で読み出します。

##### 解説3: `#[derive(Debug)]` と `{:?}` / `{:#?}`

- `#[derive(Debug)]` は「デバッグ用の表示方法を自動で作って」という指示（**属性**）です。これがあると `{:?}` で中身を表示できます。
- `{:#?}` は、フィールドを1行ずつ字下げして表示します。フィールドが多いときに読みやすくなります。

#### 例2: 書き換えと便利な書き方

> **▼ このコードがやること（先に日本語で）:** 関数でノートを作り、完了フラグを書き換えます。続いて、既存のノートの一部のフィールドだけを変えた新しいノートを作ります。

```rust
#[derive(Debug)]
struct Note {
    id: u64,
    title: String,
    done: bool,
}

// 引数と同じ名前のフィールドは、名前だけ書けばよい（省略記法）
fn make_note(id: u64, title: &str) -> Note {
    Note {
        id,
        title: title.to_string(),
        done: false,
    }
}

fn main() {
    // フィールドを書き換えるので mut を付ける（mut はインスタンス全体に付く）
    let mut note = make_note(1, "牛乳を買う");
    note.done = true;
    println!("{note:?}");

    // ..note は「残りのフィールドは note と同じ値にする」（構造体更新記法）
    let another = Note {
        id: 2,
        title: String::from("卵を買う"),
        ..note
    };
    println!(
        "#{} {}（完了: {}）",
        another.id, another.title, another.done
    );
}
```

**▼ 実行結果**

```text
Note { id: 1, title: "牛乳を買う", done: true }
#2 卵を買う（完了: true）
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: フィールド初期化の省略記法

```rust
fn make_note(id: u64, title: &str) -> Note {
    Note {
        id,
        title: title.to_string(),
        done: false,
    }
}
```

- `id,` は `id: id,` の省略形です。引数や変数の名前がフィールド名と同じときに使えます。

##### 解説2: `mut` はインスタンス全体に付く

- `let mut note = ...;` とすると、`note.done = true;` のようにフィールドを書き換えられます。
- 「`title` だけ書き換え可能」のように**フィールドごとに `mut` を付けることはできません**。書き換えを制限したいときは、後で学ぶメソッドと公開範囲（UNIT 10 の `pub`）で守ります。

##### 解説3: 構造体更新記法 `..note`

```rust
let another = Note {
    id: 2,
    title: String::from("卵を買う"),
    ..note
};
```

- `..note` は「書かなかったフィールドは `note` と同じ値にする」という意味です。ここでは `done: true` が引き継がれました。

#### 例3: impl でメソッドと関連関数を作る

> **▼ このコードがやること（先に日本語で）:** `Note` に「作る」「1行の要約を返す」「完了を切り替える」「タイトルを取り出す」の4つの機能を付けて、順に使います。

```rust
struct Note {
    id: u64,
    title: String,
    done: bool,
}

// impl ブロックに、Note 専用の関数をまとめる
impl Note {
    // 関連関数: self を受け取らない。Note::new(...) と呼ぶ。値を「作る」窓口によく使う
    fn new(id: u64, title: &str) -> Self {
        Self {
            id,
            title: title.trim().to_string(),
            done: false,
        }
    }

    // &self: 読むだけのメソッド
    fn summary(&self) -> String {
        let mark = if self.done { "[x]" } else { "[ ]" };
        format!("{mark} #{} {}", self.id, self.title)
    }

    // &mut self: 中身を書き換えるメソッド
    fn toggle(&mut self) {
        self.done = !self.done;
    }

    // self: インスタンスを使い切って、中身を取り出すメソッド
    fn into_title(self) -> String {
        self.title
    }
}

fn main() {
    let mut note = Note::new(1, "  牛乳を買う ");
    println!("{}", note.summary());
    note.toggle();
    println!("{}", note.summary());
    let title = note.into_title();
    println!("取り出したタイトル: {title}");
}
```

**▼ 実行結果**

```text
[ ] #1 牛乳を買う
[x] #1 牛乳を買う
取り出したタイトル: 牛乳を買う
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 関連関数 `new` と `Self`

```rust
fn new(id: u64, title: &str) -> Self {
    Self {
        id,
        title: title.trim().to_string(),
        done: false,
    }
}
```

- `self` を受け取らない関数が**関連関数**です。`Note::new(1, "...")` と**型の名前と `::`** で呼びます。`String::from` や `Vec::new` も同じ仕組みです。
- `Self`（大文字）は「この impl の型」、ここでは `Note` の別名です。
- `new` という名前は決まりではなく慣習です。作るときに空白を取り除くなど、**作る時点で中身を整える**のに向いています。

##### 解説2: `&self` と `&mut self`

```rust
fn summary(&self) -> String { ... }
fn toggle(&mut self) {
    self.done = !self.done;
}
```

- メソッドの最初の引数は `self`（その値自身）です。`note.summary()` と呼ぶと、`note` が `self` として渡されます。
- `&self` は「読むために借りる」、`&mut self` は「書き換えるために借りる」です。`toggle` を呼ぶには、呼ぶ側の `note` が `let mut` である必要があります。
- メソッドの中では `self.done` のように、`self.` でフィールドに触れます。

##### 解説3: `self` を受け取るメソッド

```rust
fn into_title(self) -> String {
    self.title
}
```

- `&` のない `self` は、**インスタンスそのものを受け取って使い切る**メソッドです。`note.into_title()` の後、`note` はもう使えません。
- 中身を取り出して別の形に変える（`into_〜`）ときに使います。この「使い切る」仕組みは第3巻の所有権で詳しく学びます。

#### 例4: タプル構造体（newtype）で ID を区別する

> **▼ このコードがやること（先に日本語で）:** 中身はどちらも数の「ノートの ID」と「ユーザーの ID」を、別々の型として作ります。ノートの ID を受け取る関数に、正しい型を渡して呼び出します。

```rust
// タプル構造体: フィールドに名前がなく、.0 で取り出す
// 中身は同じ i64 でも型が別なので、取り違えをコンパイラが見つけてくれる
#[derive(Debug, Clone, Copy, PartialEq)]
struct NoteId(i64);

#[derive(Debug, Clone, Copy, PartialEq)]
struct UserId(i64);

fn find_note(id: NoteId) -> String {
    format!("ノート {} を探します", id.0)
}

fn main() {
    let note_id = NoteId(7);
    let user_id = UserId(7);
    println!("{}", find_note(note_id));
    println!("{note_id:?} と {user_id:?}");
    // PartialEq を derive したので == で比べられる
    println!("同じ? {}", note_id == NoteId(7));
}
```

**▼ 実行結果**

```text
ノート 7 を探します
NoteId(7) と UserId(7)
同じ? true
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: タプル構造体

```rust
struct NoteId(i64);
```

- フィールドに名前のない構造体を**タプル構造体**と呼びます。中身は `id.0` で取り出します。
- このように、既存の型（`i64`）を1つだけ包んで**新しい意味の型**にする使い方を **newtype**（ニュータイプ）パターンと呼びます。`find_note` に `UserId` を渡すと、コンパイルエラーで止めてくれます（下の「よくあるエラー」）。

##### 解説2: `derive` でまとめて機能を付ける

- `#[derive(Debug, Clone, Copy, PartialEq)]` は、表示（`Debug`）、複製（`Clone` / `Copy`）、`==` での比較（`PartialEq`）を自動で付けます。
- `Copy` を付けると、`i64` のように**代入や関数呼び出しでコピーされる**型になり、渡した後も元の変数を使えます。中身が数だけの小さな型に向いています。

### もう一歩: 実務に近い形

1枚のノートだけでなく、**ノートの一覧**と「次に振る ID」をまとめた構造体を作り、追加・切り替え・集計をメソッドにします。データと、それを正しく扱う操作をひとまとめにするのが struct + impl の目的です。

#### ノートの一覧を管理する構造体

> **▼ このコードがやること（先に日本語で）:** 空の一覧にノートを2件追加し、1件目を完了にします。存在しない ID の切り替えは失敗として `false` を返します。最後に一覧と完了件数を表示します。

```rust
struct Note {
    id: u64,
    title: String,
    done: bool,
}

impl Note {
    fn summary(&self) -> String {
        let mark = if self.done { "[x]" } else { "[ ]" };
        format!("{mark} #{} {}", self.id, self.title)
    }
}

// ノートの一覧と「次に振る番号」をまとめて持つ構造体
struct NoteList {
    notes: Vec<Note>,
    next_id: u64,
}

impl NoteList {
    fn new() -> Self {
        Self {
            notes: Vec::new(),
            next_id: 1,
        }
    }

    // ノートを足して、振った ID を返す
    fn add(&mut self, title: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.notes.push(Note {
            id,
            title: title.to_string(),
            done: false,
        });
        id
    }

    // ID が見つかれば完了を切り替えて true、見つからなければ false
    fn toggle(&mut self, id: u64) -> bool {
        for note in &mut self.notes {
            if note.id == id {
                note.done = !note.done;
                return true;
            }
        }
        false
    }

    fn done_count(&self) -> usize {
        let mut count = 0;
        for note in &self.notes {
            if note.done {
                count += 1;
            }
        }
        count
    }

    fn print(&self) {
        for note in &self.notes {
            println!("{}", note.summary());
        }
    }
}

fn main() {
    let mut list = NoteList::new();
    let milk = list.add("牛乳を買う");
    list.add("Rust の勉強");
    list.toggle(milk);
    println!("ID 99 の切り替え: {}", list.toggle(99));
    list.print();
    println!("完了: {} / {}", list.done_count(), list.notes.len());
}
```

**▼ 実行結果**

```text
ID 99 の切り替え: false
[x] #1 牛乳を買う
[ ] #2 Rust の勉強
完了: 1 / 2
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: ID の採番をメソッドの中に閉じ込める

```rust
fn add(&mut self, title: &str) -> u64 {
    let id = self.next_id;
    self.next_id += 1;
    self.notes.push(Note { ... });
    id
}
```

- 呼ぶ側は `list.add("...")` と書くだけで、ID が重ならないことが保証されます。`next_id` を外から自由に書き換えられると、この保証が崩れます（UNIT 10 の `pub` で外から隠す方法を学びます）。

##### 解説2: 見つからない場合も戻り値で知らせる

```rust
fn toggle(&mut self, id: u64) -> bool {
    for note in &mut self.notes {
        if note.id == id {
            note.done = !note.done;
            return true;
        }
    }
    false
}
```

- `for note in &mut self.notes` で、Vec の要素を書き換えられる形で1つずつ借ります（UNIT 06 の例6）。見つかったら書き換えて早期リターンします。
- 最後まで見つからなければ `false` を返します。「見つからない」を `bool` で表すのは素朴な方法で、UNIT 08 の `Option`、UNIT 09 の `Result` を使うと、もっと情報を持たせられます。

### よくあるエラーと読み方

#### エラー1: E0063 フィールドが足りない

```rust
struct Note {
    id: u64,
    title: String,
    done: bool,
}

fn main() {
    let note = Note {
        id: 1,
        title: String::from("牛乳を買う"),
    };
    println!("{} {} {}", note.id, note.title, note.done);
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0063]: missing field `done` in initializer of `Note`
 --> src\main.rs:8:16
  |
8 |     let note = Note {
  |                ^^^^ missing `done`

For more information about this error, try `rustc --explain E0063`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** `missing field done in initializer of Note`（`Note` を作るときに `done` が足りません）。
- **何が起きたか:** Rust の構造体は、**すべてのフィールドに値が入っていないと作れません**。「入れ忘れた欄」が後から問題を起こすことがないようにするためです。
- **直し方:** `done: false,` を足します。決まった初期値があるなら `Note::new` のような関連関数にまとめておくと、入れ忘れが起きません。

#### エラー2: E0596 mut でない値に &mut self のメソッドを呼んだ

```rust
struct Note {
    done: bool,
}

impl Note {
    fn toggle(&mut self) {
        self.done = !self.done;
    }
}

fn main() {
    let note = Note { done: false };
    note.toggle();
    println!("{}", note.done);
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0596]: cannot borrow `note` as mutable, as it is not declared as mutable
  --> src\main.rs:13:5
   |
13 |     note.toggle();
   |     ^^^^ cannot borrow as mutable
   |
help: consider changing this to be mutable
   |
12 |     let mut note = Note { done: false };
   |         +++

For more information about this error, try `rustc --explain E0596`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** 「`note` は mut と宣言されていないので、書き換えのために借りることはできません」。指しているのは `note.toggle()` の `note` です。
- **直し方:** `let mut note = ...;` にします。メソッドが `&mut self` なら、呼ぶ側にも `mut` が必要です。

#### エラー3: E0277 Debug を derive していない

```rust
struct Note {
    title: String,
}

fn main() {
    let note = Note {
        title: String::from("牛乳を買う"),
    };
    println!("{note:?}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0277]: `Note` doesn't implement `Debug`
 --> src\main.rs:9:15
  |
9 |     println!("{note:?}");
  |               ^^^^^^^^ `Note` cannot be formatted using `{:?}` because it doesn't implement `Debug`
  |
  = help: the trait `Debug` is not implemented for `Note`
help: consider annotating `Note` with `#[derive(Debug)]`
  |
1 + #[derive(Debug)]
2 | struct Note {
  |

For more information about this error, try `rustc --explain E0277`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** 「`Note` は `Debug` を実装していないので、`{:?}` で表示できません」。`help` が `#[derive(Debug)]` を付けるよう具体的に提案しています（`+` の行が追加する行です）。
- **直し方:** 構造体の定義の上に `#[derive(Debug)]` を書きます。

#### エラー4: E0308 newtype の取り違え

```rust
struct NoteId(i64);
struct UserId(i64);

fn find_note(id: NoteId) -> i64 {
    id.0
}

fn main() {
    let user_id = UserId(7);
    println!("{}", find_note(user_id));
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0308]: mismatched types
  --> src\main.rs:10:30
   |
10 |     println!("{}", find_note(user_id));
   |                    --------- ^^^^^^^ expected `NoteId`, found `UserId`
   |                    |
   |                    arguments to this function are incorrect
   |
note: function defined here
  --> src\main.rs:4:4
   |
 4 | fn find_note(id: NoteId) -> i64 {
   |    ^^^^^^^^^ ----------

For more information about this error, try `rustc --explain E0308`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** `expected NoteId, found UserId`。`arguments to this function are incorrect`（この関数への引数が正しくありません）。
- **何が起きたか:** どちらも中身は `i64` ですが、型が違うので混ぜられません。もし両方とも生の `i64` だったら、ユーザー ID でノートを探す**バグがそのまま動いてしまう**ところでした。これが newtype の効き目です。
- **直し方:** 正しい種類の ID（`NoteId(7)`）を渡します。

#### エラー5: E0599 メソッド名のつづり間違い

```rust
struct Note {
    title: String,
}

impl Note {
    fn summary(&self) -> String {
        format!("ノート: {}", self.title)
    }
}

fn main() {
    let note = Note {
        title: String::from("牛乳を買う"),
    };
    println!("{}", note.summery());
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0599]: no method named `summery` found for struct `Note` in the current scope
  --> src\main.rs:15:25
   |
 1 | struct Note {
   | ----------- method `summery` not found for this struct
...
15 |     println!("{}", note.summery());
   |                         ^^^^^^^
   |
help: there is a method `summary` with a similar name
   |
15 -     println!("{}", note.summery());
15 +     println!("{}", note.summary());
   |

For more information about this error, try `rustc --explain E0599`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** 「`Note` に `summery` というメソッドは見つかりません」。`help: there is a method summary with a similar name`（似た名前の `summary` ならあります）と、正しい候補を教えてくれます。`-` の行が今の行、`+` の行が直した後の行です。
- **直し方:** `note.summary()` に直します。

### RustNote ではここで使う

RustNote の `Note` の素朴版です。本物はデータベースの `notes` テーブル（`id`、`user_id`、`title`、`body`、`done` など）と対応します。ID は取り違えを防ぐため `NoteId` / `UserId` という `i64` の newtype にします。「完了の切り替え」（`POST /notes/{id}/toggle`）と「持ち主の確認」（他人のノートは見せない）をメソッドにしておきます。

> **▼ このコードがやること（先に日本語で）:** alice さんのノートを1件作り、1行の見出しを表示してから完了に切り替えます。alice さんと bob さんのそれぞれについて持ち主かどうかを調べ、最後に中身をすべて表示します。

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct UserId(i64);

#[derive(Debug, Clone, Copy, PartialEq)]
struct NoteId(i64);

#[derive(Debug)]
struct Note {
    id: NoteId,
    user_id: UserId,
    title: String,
    body: String,
    done: bool,
}

impl Note {
    fn new(id: NoteId, user_id: UserId, title: &str, body: &str) -> Self {
        Self {
            id,
            user_id,
            title: title.trim().to_string(),
            body: body.to_string(),
            done: false,
        }
    }

    // 一覧に出す1行
    fn headline(&self) -> String {
        let mark = if self.done { "[x]" } else { "[ ]" };
        let chars = self.body.chars().count();
        format!("{mark} #{} {}（本文 {chars} 文字）", self.id.0, self.title)
    }

    // POST /notes/{id}/toggle で使う: 完了と未完了を切り替える
    fn toggle_done(&mut self) {
        self.done = !self.done;
    }

    // 持ち主かどうか（他人のノートは見せない）
    fn is_owned_by(&self, user: UserId) -> bool {
        self.user_id == user
    }
}

fn main() {
    let alice = UserId(1);
    let bob = UserId(2);
    let mut note = Note::new(NoteId(10), alice, " 牛乳を買う ", "低脂肪のもの");
    println!("{}", note.headline());
    note.toggle_done();
    println!("{}", note.headline());
    println!("alice のノート? {}", note.is_owned_by(alice));
    println!("bob のノート? {}", note.is_owned_by(bob));
    println!("{note:#?}");
}
```

**▼ 実行結果**

```text
[ ] #10 牛乳を買う（本文 6 文字）
[x] #10 牛乳を買う（本文 6 文字）
alice のノート? true
bob のノート? false
Note {
    id: NoteId(
        10,
    ),
    user_id: UserId(
        1,
    ),
    title: "牛乳を買う",
    body: "低脂肪のもの",
    done: true,
}
```

- `is_owned_by` のように「**持ち主かどうか**」を型のメソッドとして用意しておくと、画面ごとに比較の書き方がばらつきません。RustNote では、持ち主でなければ「見つかりません（404）」として扱います（第8巻）。
- `NoteId(10)` が `{:#?}` で複数行に分かれて表示されているのは、タプル構造体の中身も字下げして表示するためです。
- 今のままでは、空のタイトルや81文字のタイトルの `Note` も作れてしまいます。UNIT 09 で「検証に通ったタイトルしか入らない型」`NoteTitle` を作り、この穴をふさぎます。

### 練習問題

**問題1.** `title: String` だけを持つ `Note` に、タイトルが10文字を超えるなら `true` を返すメソッド `is_long(&self) -> bool` を追加してください（日本語でも正しく数えること）。

**問題2.** 数を数える構造体 `Counter`（フィールド `count: u32`）を作り、`new()`（0 から始める）、`increment(&mut self)`（1 増やす）、`get(&self) -> u32`（今の値を返す）を実装して、3回増やした結果を表示してください。

**問題3.** 「よくあるエラー」のエラー1のコードを、コンパイルできるように直してください。

### 解答と解説

**解答1.**

```rust
struct Note {
    title: String,
}

impl Note {
    // タイトルが 10 文字を超えるなら true
    fn is_long(&self) -> bool {
        self.title.chars().count() > 10
    }
}

fn main() {
    let short = Note {
        title: String::from("牛乳を買う"),
    };
    let long = Note {
        title: String::from("週末にまとめて片付ける部屋の大掃除"),
    };
    println!("{}: {}", short.title, short.is_long());
    println!("{}: {}", long.title, long.is_long());
}
```

**▼ 実行結果**

```text
牛乳を買う: false
週末にまとめて片付ける部屋の大掃除: true
```

- 文字数は UNIT 05 で学んだとおり `chars().count()` で数えます。`len()` だと「牛乳を買う」（15バイト）まで長いと判定されてしまいます。

**解答2.**

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn get(&self) -> u32 {
        self.count
    }
}

fn main() {
    let mut counter = Counter::new();
    for _ in 0..3 {
        counter.increment();
    }
    println!("count = {}", counter.get());
}
```

**▼ 実行結果**

```text
count = 3
```

- `for _ in 0..3` の `_` は「回数だけが大事で、番号は使わない」という意味です。
- `count` を直接 `counter.count += 1` と書き換えることもできますが、メソッドにしておけば「1ずつしか増えない」というルールを守らせやすくなります。

**解答3.**

```rust
struct Note {
    id: u64,
    title: String,
    done: bool,
}

fn main() {
    let note = Note {
        id: 1,
        title: String::from("牛乳を買う"),
        done: false,
    };
    println!("{} {} {}", note.id, note.title, note.done);
}
```

**▼ 実行結果**

```text
1 牛乳を買う false
```

### まとめ

- `struct` は名前付きのフィールドを持つ型。インスタンスはすべてのフィールドを埋めて作り、書き換えるなら `let mut`。
- `impl` には、型から `::` で呼ぶ関連関数（`new` など）と、値から `.` で呼ぶメソッドを書く。`&self` は読む、`&mut self` は書き換える、`self` は使い切る。
- `#[derive(Debug)]` で `{:?}` 表示ができる。`struct NoteId(i64);` のような newtype で、同じ中身の値の取り違えを型で防ぐ。
