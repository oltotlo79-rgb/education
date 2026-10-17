## UNIT 07: データベースと永続化

### この単元のゴール

- 変数の値はプロセスが終わると消え、ファイルやデータベースの中身は残ること（**永続化**）を体験する
- データベースの **テーブル（表）・行・列・主キー・制約** を説明できる
- 「ファイルに書くだけ」では足りなくなる理由と、データベースが引き受けてくれる仕事が分かる
- SQL の文と、制約違反のエラーを読める

### たとえ話でつかむ

データの置き場所を、**会社の事務所**にたとえます。

- **変数（メモリ）はホワイトボード**です。書くのも消すのも速いですが、退社するとき（プロセスの終了）に消されてしまい、停電（電源オフ）でも消えます。
- **ファイルはノートのメモ**です。閉じても残りますが、書き方は自分しだいです。何人もが同時に書き込むと字が重なり、目当ての1行を探すにも最初から読むしかありません。
- **データベース（DB）は台帳を管理する経理係**です。データを決まった形の**表**で整理し、「この列は必ず埋める」「メールアドレスは同じものを2つ登録しない」といった**ルール（制約）**を守らない記入は受け付けません。何人から同時に頼まれても順番に正しく処理し、「Hanako さんのノートだけ」のような探し物もすばやく見つけてくれます。

> **用語:** **永続化（えいぞくか）** … プログラムが終わっても、PC を再起動しても、データが残るように保存すること。
>
> **用語:** **データベース（DB）** … データを整理して保存し、探したり書き換えたりする仕事を引き受ける専用のプログラム（とそのデータ）。RustNote は **Turso（ターソ）** というサービスの **libSQL（リブエスキューエル）** というデータベースを使います。libSQL は、世界中で使われている **SQLite（エスキューライト）** の仲間です。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-17" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#047857"/></marker>
</defs>
<text x="20" y="24" font-size="13" font-weight="700" fill="#334155">プログラムを3回起動すると…</text>
<rect x="150" y="38" width="160" height="196" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="230" y="60" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">1回目</text>
<rect x="340" y="38" width="160" height="196" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="420" y="60" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">2回目</text>
<rect x="530" y="38" width="160" height="196" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="610" y="60" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">3回目</text>
<text x="20" y="104" font-size="13" font-weight="700" fill="#1e40af">メモリ</text>
<text x="20" y="122" font-size="12" fill="#64748b">（変数）</text>
<rect x="170" y="80" width="120" height="50" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="230" y="111" font-size="16" font-weight="700" fill="#1e40af" text-anchor="middle">0 → 1</text>
<rect x="360" y="80" width="120" height="50" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="420" y="111" font-size="16" font-weight="700" fill="#1e40af" text-anchor="middle">0 → 1</text>
<rect x="550" y="80" width="120" height="50" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="610" y="111" font-size="16" font-weight="700" fill="#1e40af" text-anchor="middle">0 → 1</text>
<text x="20" y="184" font-size="13" font-weight="700" fill="#047857">ストレージ</text>
<text x="20" y="202" font-size="12" fill="#64748b">（count.txt）</text>
<rect x="170" y="160" width="120" height="50" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="230" y="191" font-size="16" font-weight="700" fill="#047857" text-anchor="middle">0 → 1</text>
<rect x="360" y="160" width="120" height="50" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="420" y="191" font-size="16" font-weight="700" fill="#047857" text-anchor="middle">1 → 2</text>
<rect x="550" y="160" width="120" height="50" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="610" y="191" font-size="16" font-weight="700" fill="#047857" text-anchor="middle">2 → 3</text>
<line x1="292" y1="185" x2="356" y2="185" stroke="#047857" stroke-width="2" marker-end="url(#ar-00-17)"/>
<line x1="482" y1="185" x2="546" y2="185" stroke="#047857" stroke-width="2" marker-end="url(#ar-00-17)"/>
<text x="20" y="140" font-size="12" fill="#b91c1c">毎回 0 から</text>
<text x="360" y="258" font-size="12" fill="#334155" text-anchor="middle">メモリの値はプロセスの終わりに消える　／　緑の矢印 = ファイルの値を次の起動が引き継ぐ</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図7-1: メモリの値はプロセスが終わると消え、ストレージに書いた値は次の起動に引き継がれる</div>
</div>

データベースは、データを **テーブル（表）** の形でしまいます。表計算ソフトのシートによく似ています。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="24" y="26" font-size="14" font-weight="700" fill="#047857">notes テーブル（一部の列だけ）</text>
<rect x="24" y="40" width="80" height="34" fill="#047857"/>
<rect x="104" y="40" width="90" height="34" fill="#047857"/>
<rect x="194" y="40" width="230" height="34" fill="#047857"/>
<rect x="424" y="40" width="70" height="34" fill="#047857"/>
<rect x="494" y="40" width="200" height="34" fill="#047857"/>
<text x="64" y="62" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">id</text>
<text x="149" y="62" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">user_id</text>
<text x="309" y="62" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">title</text>
<text x="459" y="62" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">done</text>
<text x="594" y="62" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">updated_at</text>
<rect x="24" y="74" width="670" height="34" fill="#ffffff" stroke="#cbd5e1"/>
<rect x="24" y="108" width="670" height="34" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<rect x="24" y="142" width="670" height="34" fill="#ffffff" stroke="#cbd5e1"/>
<rect x="194" y="74" width="230" height="102" fill="none" stroke="#3b82f6" stroke-width="3"/>
<text x="64" y="96" font-size="13" fill="#0f172a" text-anchor="middle">1</text>
<text x="149" y="96" font-size="13" fill="#0f172a" text-anchor="middle">1</text>
<text x="206" y="96" font-size="13" fill="#0f172a">牛乳を買う</text>
<text x="459" y="96" font-size="13" fill="#0f172a" text-anchor="middle">0</text>
<text x="594" y="96" font-size="13" fill="#0f172a" text-anchor="middle">2026-10-15 09:12</text>
<text x="64" y="130" font-size="13" fill="#0f172a" text-anchor="middle">2</text>
<text x="149" y="130" font-size="13" fill="#0f172a" text-anchor="middle">1</text>
<text x="206" y="130" font-size="13" fill="#0f172a">Rust の本を読む</text>
<text x="459" y="130" font-size="13" fill="#0f172a" text-anchor="middle">1</text>
<text x="594" y="130" font-size="13" fill="#0f172a" text-anchor="middle">2026-10-14 21:40</text>
<text x="64" y="164" font-size="13" fill="#0f172a" text-anchor="middle">3</text>
<text x="149" y="164" font-size="13" fill="#0f172a" text-anchor="middle">2</text>
<text x="206" y="164" font-size="13" fill="#0f172a">歯医者の予約</text>
<text x="459" y="164" font-size="13" fill="#0f172a" text-anchor="middle">0</text>
<text x="594" y="164" font-size="13" fill="#0f172a" text-anchor="middle">2026-10-13 18:05</text>
<rect x="24" y="196" width="210" height="70" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="36" y="218" font-size="13" font-weight="700" fill="#c2410c">行（レコード）</text>
<text x="36" y="238" font-size="12" fill="#334155">ノート1件ぶんのデータ</text>
<text x="36" y="256" font-size="12" fill="#334155">（橙の枠がその1件）</text>
<rect x="250" y="196" width="210" height="70" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="262" y="218" font-size="13" font-weight="700" fill="#1e40af">列（カラム）</text>
<text x="262" y="238" font-size="12" fill="#334155">項目。型とルールを持つ</text>
<text x="262" y="256" font-size="12" fill="#334155">（青の枠が title 列）</text>
<rect x="476" y="196" width="218" height="70" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="488" y="218" font-size="13" font-weight="700" fill="#047857">主キー（id）</text>
<text x="488" y="238" font-size="12" fill="#334155">1件を特定する番号</text>
<text x="488" y="256" font-size="12" fill="#334155">同じ値は2つとない</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図7-2: テーブルの行と列。done は 0 = 未済み、1 = 済み（値は例）</div>
</div>

テーブルどうしは、列の値でつながります。`notes` の `user_id` 列には「そのノートの持ち主の `users.id`」が入っています。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 220" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-18" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#c2410c"/></marker>
</defs>
<text x="24" y="24" font-size="14" font-weight="700" fill="#1e40af">users</text>
<rect x="24" y="34" width="60" height="30" fill="#1e40af"/>
<rect x="84" y="34" width="200" height="30" fill="#1e40af"/>
<text x="54" y="54" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">id</text>
<text x="184" y="54" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">email（UNIQUE）</text>
<rect x="24" y="64" width="260" height="30" fill="#ffffff" stroke="#cbd5e1"/>
<rect x="24" y="94" width="260" height="30" fill="#ffffff" stroke="#cbd5e1"/>
<text x="54" y="84" font-size="13" fill="#0f172a" text-anchor="middle">1</text>
<text x="96" y="84" font-size="13" fill="#0f172a">hanako@example.com</text>
<text x="54" y="114" font-size="13" fill="#0f172a" text-anchor="middle">2</text>
<text x="96" y="114" font-size="13" fill="#0f172a">taro@example.com</text>
<text x="404" y="24" font-size="14" font-weight="700" fill="#047857">notes</text>
<rect x="404" y="34" width="50" height="30" fill="#047857"/>
<rect x="454" y="34" width="80" height="30" fill="#047857"/>
<rect x="534" y="34" width="166" height="30" fill="#047857"/>
<text x="429" y="54" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">id</text>
<text x="494" y="54" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">user_id</text>
<text x="617" y="54" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">title</text>
<rect x="404" y="64" width="296" height="30" fill="#ffffff" stroke="#cbd5e1"/>
<rect x="404" y="94" width="296" height="30" fill="#ffffff" stroke="#cbd5e1"/>
<rect x="404" y="124" width="296" height="30" fill="#ffffff" stroke="#cbd5e1"/>
<text x="429" y="84" font-size="13" fill="#0f172a" text-anchor="middle">1</text>
<text x="494" y="84" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">1</text>
<text x="546" y="84" font-size="13" fill="#0f172a">牛乳を買う</text>
<text x="429" y="114" font-size="13" fill="#0f172a" text-anchor="middle">2</text>
<text x="494" y="114" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">1</text>
<text x="546" y="114" font-size="13" fill="#0f172a">Rust の本を読む</text>
<text x="429" y="144" font-size="13" fill="#0f172a" text-anchor="middle">3</text>
<text x="494" y="144" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">2</text>
<text x="546" y="144" font-size="13" fill="#0f172a">歯医者の予約</text>
<path d="M400,79 C360,79 330,77 290,77" fill="none" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-18)"/>
<path d="M400,109 C350,109 340,83 290,83" fill="none" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-18)"/>
<path d="M400,139 C350,139 340,109 290,109" fill="none" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-18)"/>
<text x="344" y="168" font-size="12" fill="#c2410c" text-anchor="middle">user_id の値で</text>
<text x="344" y="184" font-size="12" fill="#c2410c" text-anchor="middle">持ち主を指す</text>
<text x="24" y="148" font-size="12" fill="#334155">UNIQUE = 同じメールアドレスは</text>
<text x="24" y="164" font-size="12" fill="#334155">2回登録できない</text>
<text x="24" y="210" font-size="12" fill="#334155">Hanako（1番）のノートは 1・2、Taro（2番）のノートは 3</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図7-3: users と notes のつながり。user_id が「持ち主」を表す</div>
</div>

### まず動かす最小コード

#### 消えるカウンターと、消えないカウンター

> **▼ このコードがやること（先に日本語で）:** 2つのカウンター（数を数える仕組み）を同時に1増やします。1つはメモリの中だけ、もう1つは `count.txt` というファイルに保存します。プログラムを何回か起動して、どちらが前回の続きから数えるかを比べます。

```rust
use std::fs;

fn main() {
    // (1) メモリの中だけのカウンター。プログラムが始まるたびに 0 から
    let mut in_memory = 0;
    in_memory += 1;

    // (2) ファイルに保存するカウンター。前回の値を読み、1 足して書き戻す
    //     count.txt が無い（初回）ときは "0" として扱う
    let saved = fs::read_to_string("count.txt").unwrap_or(String::from("0"));
    let mut on_disk: u32 = saved.trim().parse().unwrap();
    on_disk += 1;
    fs::write("count.txt", on_disk.to_string()).unwrap();

    println!("メモリのカウンター  : {in_memory}");
    println!("ファイルのカウンター: {on_disk}");
}
```

```powershell
cargo run -q
cargo run -q
cargo run -q
cat count.txt
```

**▼ 実行結果**

```text
メモリのカウンター  : 1
ファイルのカウンター: 1
メモリのカウンター  : 1
ファイルのカウンター: 2
メモリのカウンター  : 1
ファイルのカウンター: 3
3
```

メモリのカウンターは毎回 1 に戻り、ファイルのカウンターは 1→2→3 と増えました。PC を再起動してからもう一度実行しても、ファイルのカウンターは 4 になります。

#### ▼ コードを1つずつ分解して解説

##### 解説1: メモリのカウンター

```rust
let mut in_memory = 0;
in_memory += 1;
```

- 変数 `in_memory` はメモリ上にあり、**プロセスが終わると OS に片付けられます**。次に起動したときは、また `0` から始まります。

##### 解説2: 前回の値を読む

```rust
let saved = fs::read_to_string("count.txt").unwrap_or(String::from("0"));
let mut on_disk: u32 = saved.trim().parse().unwrap();
```

- `count.txt` を読みます。初回はファイルが無くて失敗するので、`unwrap_or` で `"0"` を使います。
- ファイルの中身は**文字**の `"3"` なので、`.parse()` で**数値**の `3` に変換します。`.trim()` は前後の空白や改行を取り除きます。
- `: u32` は「0 以上の整数として扱う」という型の指定です。これがあるので、`parse` は何に変換すればよいか分かります。

##### 解説3: 新しい値を書き戻す

```rust
on_disk += 1;
fs::write("count.txt", on_disk.to_string()).unwrap();
```

- 1 増やした数を `.to_string()` で文字に戻して、ファイルに上書きします。
- 「読む → 変える → 書き戻す」は、データを保存するプログラムの基本の形です。データベースを使っても、この流れは変わりません。

### もう一歩: 実務に近い形

#### ファイルで「表」を作ってみる

ノートの一覧を、1行1件・タブ区切りのファイル（**TSV**）で保存してみます。データベースの表を、手作りで真似したものです。

> **▼ このコードがやること（先に日本語で）:** 起動時にタイトルが渡されたら、`notes.tsv` の最後に「id・持ち主の番号・タイトル・済みかどうか」を1行追加します。そのあと、ファイルの全行を表の形で表示します。

```rust
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let file = "notes.tsv";

    // 引数が渡されたら、それをタイトルとして 1 行（1 件）追加する
    if let Some(title) = env::args().nth(1) {
        // 今ある行数 + 1 を、新しい行の id にする
        let existing = fs::read_to_string(file).unwrap_or_default();
        let next_id = existing.lines().count() + 1;

        // ファイルの末尾に追記する（無ければ作る）
        let mut f = OpenOptions::new().create(true).append(true).open(file)?;
        // 列の順番: id, user_id, title, done（タブ文字 \t で区切る）
        writeln!(f, "{next_id}\t1\t{title}\tfalse")?;
    }

    // ファイルの全行を、表（テーブル）の形で表示する
    let text = fs::read_to_string(file).unwrap_or_default();
    println!("id | user_id | done  | title");
    for line in text.lines() {
        let cols: Vec<&str> = line.split('\t').collect();
        println!(
            "{:<2} | {:<7} | {:<5} | {}",
            cols[0], cols[1], cols[3], cols[2]
        );
    }
    Ok(())
}
```

```powershell
cargo run -q -- 牛乳を買う
cargo run -q -- "Rust の本を読む"
cat notes.tsv
```

**▼ 実行結果**

```text
id | user_id | done  | title
1  | 1       | false | 牛乳を買う
id | user_id | done  | title
1  | 1       | false | 牛乳を買う
2  | 1       | false | Rust の本を読む
1	1	牛乳を買う	false
2	1	Rust の本を読む	false
```

最後の2行が `notes.tsv` の中身そのものです。列の間の空白に見える部分は、タブ文字です。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 追記モードで開く

```rust
let mut f = OpenOptions::new().create(true).append(true).open(file)?;
writeln!(f, "{next_id}\t1\t{title}\tfalse")?;
```

- `fs::write` は上書きでしたが、`OpenOptions` で `append(true)`（末尾に追記）を指定すると、前の行を残したまま書き足せます。`create(true)` は「無ければ作る」です。
- `writeln!` は、`println!` の「画面の代わりにファイルへ書く」版です。`\t` はタブ文字です。

##### 解説2: 1行を列に分ける

```rust
let cols: Vec<&str> = line.split('\t').collect();
```

- `split('\t')` でタブごとに区切り、`.collect()` で部品を並べた **`Vec`（ベクタ：伸び縮みする並び）** にまとめます。
- `cols[0]` が id、`cols[2]` がタイトルのように、番号で列を取り出せます。

##### 解説3: このやり方の限界

手作りの表は動きますが、本物のアプリにするには次の問題があります。

| 困りごと | 手作りファイルでは | データベースなら |
|---|---|---|
| 2人が同時に追加した | id が重なったり、行が混ざったりする | 順番に処理して、id も自動で重ならない |
| タイトルにタブが含まれていた | 列がずれて表が壊れる | 値の区切りを DB が管理するので壊れない |
| 「Hanako のノートだけ」を探す | 全行を読んで1行ずつ調べる | 条件を書くだけで、索引を使って速く探す |
| 空のタイトル、同じメールの二重登録 | 自分で全部チェックする | **制約**として書いておけば DB が断る |
| 途中で電源が切れた | 書きかけの半端な行が残る | 「全部やるか、何もしないか」を保証できる（**トランザクション**） |

#### 本物のデータベースなら: SQL の雰囲気

データベースには、**SQL（エスキューエル）** という言葉でお願いします。同じことを SQL で書くと次のようになります。ここは**見るだけ**で大丈夫です（第7巻で、Turso の道具を使って実際に動かします）。

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email TEXT NOT NULL UNIQUE
);
CREATE TABLE notes (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    title TEXT NOT NULL CHECK (length(title) BETWEEN 1 AND 80),
    done INTEGER NOT NULL DEFAULT 0
);
INSERT INTO users (email) VALUES ('hanako@example.com');
INSERT INTO users (email) VALUES ('taro@example.com');
INSERT INTO notes (user_id, title) VALUES (1, '牛乳を買う');
INSERT INTO notes (user_id, title) VALUES (1, 'Rust の本を読む');
INSERT INTO notes (user_id, title) VALUES (2, '歯医者の予約');
SELECT id, title, done FROM notes WHERE user_id = 1;
```

**▼ 実行結果**（SQLite 3.50 のコマンドラインツールで実行し、列を ` | ` で区切って表示したもの）

```text
id | title | done
1 | 牛乳を買う | 0
2 | Rust の本を読む | 0
```

- `CREATE TABLE` は表を作る文です。列の名前・型（`INTEGER` は整数、`TEXT` は文字）・ルールを書きます。
- `PRIMARY KEY` は主キー、`NOT NULL` は「必ず値を入れる」、`UNIQUE` は「同じ値を2つ入れない」、`CHECK (...)` は「この条件を満たす値だけ入れる」、`REFERENCES users(id)` は「`users` の `id` のどれかを指す」というルールです。
- `INSERT INTO ... VALUES ...` は行を追加する文、`SELECT ... FROM ... WHERE ...` は「条件に合う行を探して見せて」という文です。`WHERE user_id = 1` と書くだけで、Hanako のノートだけが返ってきます。

### よくあるエラーと読み方

#### 保存されたデータが壊れていた

最小コードの `count.txt` の中身を、手で `abc` に書き換えてから実行します。

```powershell
Set-Content count.txt "abc"
cargo run
```

**▼ 実行結果**

```text
thread 'main' (9276) panicked at src\main.rs:11:49:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\practice.exe` (exit code: 101)
```

**何が起きたか:** 11行目の `parse().unwrap()` で、`"abc"` を数値に変換できずにパニックしました（UNIT 04 と同じ `ParseIntError`）。保存されたデータは、**いつも自分の思ったとおりの形とは限らない**のです。

**直し方:** 今回は `count.txt` を消すか、数字に直せば動きます。本物のアプリでは、読み込んだデータが壊れていても止まらずにエラーとして扱う書き方をします。データベースを使うと、そもそも型や制約に合わないデータが入りにくくなります。

#### データベースが記入を断った（制約違反）

上の SQL の続きで、ルールに違反する記入を試します。

```sql
INSERT INTO users (email) VALUES ('hanako@example.com');
```

```text
Runtime error near line 1: UNIQUE constraint failed: users.email (19)
```

```sql
INSERT INTO notes (user_id, title) VALUES (1, '');
```

```text
Runtime error near line 1: CHECK constraint failed: length(title) BETWEEN 1 AND 80 (19)
```

**何が起きたか:**

- 1つ目は「`users.email` の **UNIQUE 制約**に失敗」。同じメールアドレスがすでに登録されているので断られました。
- 2つ目は「**CHECK 制約** `length(title) BETWEEN 1 AND 80` に失敗」。空のタイトル（0文字）は 1〜80 文字の範囲外です。
- 末尾の `(19)` は、SQLite の「制約違反」を表すエラー番号です。

**直し方:** これは「DB がちゃんと仕事をした」結果です。アプリ側では、この失敗を受け取って「そのメールアドレスはすでに登録されています」「タイトルは1〜80文字で入力してください」のような、利用者に分かる言葉に直して画面に表示します。

> **つまずきポイント:** 「アプリ側でチェックしているから、DB の制約はいらない」と考えがちですが、両方に書くのが基本です。アプリのチェックは**利用者に親切なメッセージを出すため**、DB の制約は**どんな経路からでも間違ったデータを入れさせない最後の砦**です。

### RustNote ではここで使う

- RustNote のデータは、次の4つのテーブルにしまいます（第7巻・第9巻で作ります）。

| テーブル | 中身 | 主な列 |
|---|---|---|
| `users` | 登録した人 | `id`、`email`（UNIQUE）、`password_hash`、`created_at` |
| `notes` | ノート | `id`、`user_id`（→ `users.id`）、`title`（1〜80文字）、`body`、`done`、`version`、`created_at`、`updated_at` |
| `sessions` | ログイン中の状態 | `token_hash`、`user_id`、`expires_at`、`created_at` |
| `schema_migrations` | 表の作り変えの記録 | `version`、`applied_at` |

- 開発中はデータベースを**自分の PC のファイル**として持ち、公開後はインターネット上の **Turso** を使います。プログラムからの使い方はほぼ同じです。
- テーブルの形を後から変えるときは、変更の手順を SQL のファイル（**マイグレーション**）として残し、`schema_migrations` に「どこまで適用したか」を記録します（第7巻）。

### 練習問題

**問題1:** 最小コードを改造して、`cargo run -q -- reset` と実行したときは `count.txt` を消して「カウンターを 0 に戻しました」と表示するようにしてください。

**問題2:** 「本物のデータベースなら」の SQL の表に対して、次の SQL を実行すると何が返ってくるでしょうか。

```sql
SELECT title FROM notes WHERE user_id = 2;
```

**問題3:** RustNote で次のデータを保存するとき、「メモリだけでよい」か「データベースに保存すべき」かを答えてください。

- a. ユーザーのメールアドレス
- b. 今まさに処理しているリクエストで送られてきたタイトルの文字列
- c. ノートの「済み（done）」の状態
- d. ログイン中であることの記録（セッション）

### 解答と解説

**問題1の解答:**

```rust
use std::env;
use std::fs;

fn main() {
    // 1 つ目の引数が "reset" なら、ファイルを消して 0 に戻す
    if env::args().nth(1).as_deref() == Some("reset") {
        // ファイルが無い場合のエラーは気にしないので、結果を捨てる
        let _ = fs::remove_file("count.txt");
        println!("カウンターを 0 に戻しました");
        return;
    }

    let saved = fs::read_to_string("count.txt").unwrap_or(String::from("0"));
    let mut on_disk: u32 = saved.trim().parse().unwrap();
    on_disk += 1;
    fs::write("count.txt", on_disk.to_string()).unwrap();
    println!("ファイルのカウンター: {on_disk}");
}
```

```powershell
cargo run -q
cargo run -q
cargo run -q -- reset
cargo run -q
```

**▼ 実行結果**

```text
ファイルのカウンター: 1
ファイルのカウンター: 2
カウンターを 0 に戻しました
ファイルのカウンター: 1
```

`.as_deref()` は、「`Option<String>`（あるかもしれない文字列）」を、`"reset"` と比べられる形（`Option<&str>`）に変える道具です。`return` で `main` を途中で終えています。`let _ = ...` は「結果を使わずに捨てる」という意味です。

**問題2の解答:** `user_id` が 2 の行は Taro の「歯医者の予約」だけなので、次の1行が返ってきます。

```text
歯医者の予約
```

**問題3の解答:** a・c・d は**データベース**です。サーバーが再起動しても残っていないと困るからです（d が消えると、全員がログアウトしてしまいます）。b は**メモリだけでよい**です。検査してデータベースに書き込んだら、その文字列はもう不要です。

### まとめ

- 変数（メモリ）の値はプロセスの終了で消えます。残したいデータはストレージに**永続化**します。
- データベースはデータを**テーブル（行と列）**で整理し、主キー・UNIQUE・CHECK などの**制約**で間違ったデータを断ってくれます。
- 手作りのファイルでは、同時書き込み・検索・壊れたデータに弱いので、Web アプリではデータベースを使います。

