## UNIT 10: CI・依存・供給網・事故の対応

### この単元のゴール

- RustNote の CI（`ci.yml`）の3つの手順（`cargo fmt --check`・`cargo clippy -- -D warnings`・`cargo test --locked`）を手元で動かし、どれか1つでも失敗したら止まる理由を言える
- `Cargo.lock` と `--locked` で、毎回同じ版の部品を使う理由を、失敗の例で説明できる
- cargo-audit で、使っている部品（依存クレート）の、知られている弱点（RustSec の勧告）を調べ、どの部品から来ているかを `cargo tree -i` でたどれる
- 直せない弱点に、使わない機能を外す・理由と見直す日を書いて受け入れる、のように手を打てる
- 部品の弱点が見つかった日の、事故の対応の順番を言える

### たとえ話でつかむ

パン屋の、毎朝の仕事を考えます。

- **開店前の点検（CI）:** 毎朝、同じ順に、オーブンの温度・計りの目盛り・レジの釣り銭を確かめます。どれか1つでもおかしければ、開店しません。人によって点検の仕方が変わらないよう、**手順を紙に書いて**、毎回その通りにします。
- **材料の仕入れ（依存）:** 小麦粉もバターも、ほかの会社から仕入れます。自分で作っていない物が、パンのほとんどです（RustNote の部品は 367 個）。
- **仕入れの記録（Cargo.lock）:** 「どの会社の、いつの、どのロットの小麦粉か」を記録しておけば、同じパンを何度でも焼けます。勝手に違う袋に替わらないように、記録どおりでなければ焼かない、と決めます（`--locked`）。
- **回収のお知らせ（RustSec）:** 「このロットのバターに問題があった」という知らせが届くことがあります。毎日、知らせを見て（cargo audit）、自分の店の材料に当てはまるかを調べます。
- **すぐ替えられないとき:** 代わりのバターがまだ無いなら、そのバターを使うパンだけを減らしたり、「何日まで様子を見る」と記録したりします。

| パン屋 | RustNote |
|---|---|
| 開店前の点検の手順 | CI（`ci.yml` の fmt・clippy・test） |
| ほかの会社の材料 | 依存クレート（crates.io の部品） |
| 仕入れの記録 | `Cargo.lock`（版と、中身のハッシュ） |
| 記録どおりでなければ焼かない | `--locked` |
| 回収のお知らせ | RustSec の勧告（cargo-audit が読む） |
| 材料をたどる | `cargo tree -i`（その部品を、だれが使っているか） |
| 使うパンを減らす | 使わない機能（feature）を外す |
| 様子を見ると記録する | `audit.toml` の `ignore`（理由と見直す日） |

> **用語:** CI（Continuous Integration：変更を入れるたびに、決まった確かめ（整形・警告・テスト）を、自動で毎回動かすこと。GitHub Actions などで動かす）

<!-- -->

> **用語:** 供給網（サプライチェーン：自分のプログラムが使う部品と、その部品が使う部品…のつながり。どこか1つに弱点や悪意のある変更が入ると、自分のプログラムにも入る）

<!-- -->

> **用語:** RustSec（ラストセック：Rust のクレートの、知られている弱点や、手入れが止まったことの知らせ（勧告）を集めたデータベース。勧告には `RUSTSEC-2026-0258` のような番号が付く）

<!-- -->

> **用語:** feature（フィーチャー：クレートの機能のスイッチ。使わない機能を外すと、その機能のための部品も組み立てなくなる）

### 図で理解する

RustNote の CI の手順と、この単元で足す確かめです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 210" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-11-1001" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="40" width="100" height="56" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="60" y="64" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">push・PR</text>
<text x="60" y="84" font-size="10" fill="#475569" text-anchor="middle">変更を入れる</text>
<rect x="140" y="40" width="120" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="200" y="64" font-size="12" font-weight="700" fill="#1d4ed8" text-anchor="middle">fmt --check</text>
<text x="200" y="84" font-size="10" fill="#0f172a" text-anchor="middle">形がそろっているか</text>
<rect x="290" y="40" width="140" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="64" font-size="12" font-weight="700" fill="#1d4ed8" text-anchor="middle">clippy -D warnings</text>
<text x="360" y="84" font-size="10" fill="#0f172a" text-anchor="middle">警告も失敗にする</text>
<rect x="460" y="40" width="120" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="520" y="64" font-size="12" font-weight="700" fill="#1d4ed8" text-anchor="middle">test --locked</text>
<text x="520" y="84" font-size="10" fill="#0f172a" text-anchor="middle">記録どおりの版で</text>
<rect x="610" y="40" width="100" height="56" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="660" y="64" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">docker</text>
<text x="660" y="84" font-size="10" fill="#0f172a" text-anchor="middle">build（第12巻）</text>
<line x1="112" y1="68" x2="136" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-1001)"/>
<line x1="262" y1="68" x2="286" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-1001)"/>
<line x1="432" y1="68" x2="456" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-1001)"/>
<line x1="582" y1="68" x2="606" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-1001)"/>
<text x="360" y="124" font-size="11" fill="#b91c1c" text-anchor="middle">どれか1つが失敗したら、そこで止まり、先へ進まない（エラー1〜3）</text>
<rect x="140" y="140" width="440" height="56" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="164" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">足すとよい: cargo audit（毎日、決まった時刻に）</text>
<text x="360" y="184" font-size="10" fill="#0f172a" text-anchor="middle">コードを変えなくても、新しい弱点の知らせは毎日増える（問1）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: CI の手順。変更のたびに同じ確かめを同じ順で動かす。部品の弱点の確かめは、変更が無い日にも動かす</div>
</div>

RustNote の部品のうち、弱点の知らせがあったものと、そこまでのつながりです（例10-2の `cargo tree -i`）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-11-1002" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="100" width="100" height="44" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="60" y="127" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">rustnote</text>
<rect x="140" y="100" width="110" height="44" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="195" y="120" font-size="11" font-weight="700" fill="#0f172a" text-anchor="middle">libsql 0.10.0</text>
<text x="195" y="136" font-size="9" fill="#475569" text-anchor="middle">（今の最新）</text>
<rect x="290" y="30" width="150" height="44" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="365" y="50" font-size="11" fill="#0f172a" text-anchor="middle">hyper-rustls 0.25.0</text>
<text x="365" y="66" font-size="9" fill="#475569" text-anchor="middle">feature: tls</text>
<rect x="290" y="100" width="150" height="44" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="365" y="120" font-size="11" fill="#0f172a" text-anchor="middle">hyper 0.14.32</text>
<text x="365" y="136" font-size="9" fill="#475569" text-anchor="middle">feature: remote ほか</text>
<rect x="290" y="170" width="150" height="44" rx="8" fill="#f1f5f9" stroke="#94a3b8" stroke-width="2" stroke-dasharray="5 3"/>
<text x="365" y="190" font-size="11" fill="#475569" text-anchor="middle">tonic・axum・bincode</text>
<text x="365" y="206" font-size="9" fill="#475569" text-anchor="middle">feature: replication（外せた）</text>
<rect x="480" y="20" width="230" height="64" rx="8" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="595" y="42" font-size="11" font-weight="700" fill="#b91c1c" text-anchor="middle">rustls 0.22.4（2件）</text>
<text x="595" y="60" font-size="11" font-weight="700" fill="#b91c1c" text-anchor="middle">rustls-webpki 0.102.8（5件）</text>
<text x="595" y="76" font-size="9" fill="#0f172a" text-anchor="middle">直るのは 0.23.46・0.103.16 以上</text>
<rect x="480" y="100" width="230" height="44" rx="8" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="595" y="120" font-size="11" font-weight="700" fill="#b91c1c" text-anchor="middle">h2 0.3.27（1件）</text>
<text x="595" y="136" font-size="9" fill="#0f172a" text-anchor="middle">直るのは 0.4.16 以上</text>
<line x1="112" y1="122" x2="136" y2="122" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-1002)"/>
<line x1="252" y1="112" x2="286" y2="56" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-1002)"/>
<line x1="252" y1="122" x2="286" y2="122" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-1002)"/>
<line x1="252" y1="132" x2="286" y2="188" stroke="#94a3b8" stroke-width="2" stroke-dasharray="5 3" marker-end="url(#ar-11-1002)"/>
<line x1="442" y1="52" x2="476" y2="52" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-1002)"/>
<line x1="442" y1="122" x2="476" y2="122" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-1002)"/>
<text x="360" y="240" font-size="11" fill="#475569" text-anchor="middle">どれも、版を大きく上げないと直らない。決めるのは libsql の側なので、RustNote は待つしかない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: 弱点の知らせがあった部品のつながり。どれも libsql の、Turso への接続（TLS と HTTP/2）の部品。replication の機能を外すと tonic などは消えるが、弱点の 8 件は残った</div>
</div>

### まず動かす最小コード

#### 例10-1: CI の手順を、手元で動かす

RustNote には、GitHub Actions の設定の見本 `ci.yml` があります（リポジトリの `.github/workflows/` にコピーすると動きます）。

```yaml
# GitHub Actions の CI 設定の例。
# 使うときは、リポジトリの .github/workflows/ci.yml にコピーする。
# RustNote のフォルダがリポジトリの一番上なら、working-directory と workspaces の行を消す。

name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  test:
    runs-on: ubuntu-26.04
    defaults:
      run:
        working-directory: Rust/rustnote
    steps:
      - uses: actions/checkout@v7

      # rust-toolchain.toml と同じ 1.99.1 を入れる
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: 1.99.1
          components: rustfmt, clippy

      # 依存クレートのビルド結果をキャッシュして、2 回目から速くする
      - uses: Swatinem/rust-cache@v2
        with:
          workspaces: Rust/rustnote

      - name: 整形の確認
        run: cargo fmt --check

      - name: Clippy（警告もエラーとして扱う）
        run: cargo clippy --all-targets --locked -- -D warnings

      - name: テスト
        run: cargo test --locked

  docker:
    # テストが通ったら、コンテナイメージがビルドできるかも確かめる
    needs: test
    runs-on: ubuntu-26.04
    steps:
      - uses: actions/checkout@v7
      - name: docker build
        run: docker build -t rustnote Rust/rustnote
```

> **▼ このコマンドがやること（先に日本語で）:** CI の `test` の仕事と同じ3つの手順を、教材と同じ RustNote のコピーで、同じ順に動かします。①`cargo fmt --check`（整形が決まりどおりか。直さずに、違いがあれば失敗）、②`cargo clippy --all-targets --locked -- -D warnings`（テストも含めて Clippy にかけ、警告も失敗にする）、③`cargo test --locked`（`Cargo.lock` の版のまま、全部のテスト）。

```powershell
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

**▼ 実行結果**（①は何も表示せず、`exit=0` で終わります）

```text

    Checking rustnote v0.1.0 (...\rustnote)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 32.05s
     Running unittests src\lib.rs (target\debug\deps\rustnote-c39f4ee352b39312.exe)
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running unittests src\main.rs (target\debug\deps\rustnote-009228cf007e52ec.exe)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests\http.rs (target\debug\deps\http-afc265ac134dcb62.exe)
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.17s
     Running tests\repository.rs (target\debug\deps\repository-74d08b56e783b6bc.exe)
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s
   Doc-tests rustnote
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

- 3つとも `exit=0` で通りました。テストは、ユニットテスト 19 個・`tests/http.rs` 14 個・`tests/repository.rs` 8 個です（`src/main.rs` と、ドキュメントの中の例（Doc-tests）は 0 個）。
- CI は、**人ではなく機械が、毎回、同じ手順で**確かめます。「手元では通ったのに」が無くなり、どの変更で壊れたかも分かります。

### もう一歩: 実務に近い形

#### 例10-2: cargo-audit で、部品の弱点を調べる

cargo-audit は、`Cargo.lock` に書かれた全部の部品と版を、RustSec のデータベースと照らし合わせます。

```powershell
cargo install cargo-audit --version 0.22.2 --locked
cargo audit
```

**▼ 実行結果**（抜き出しています）

```text
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 1395 security advisories (from ...\advisory-db)
    Scanning Cargo.lock for vulnerabilities (367 crate dependencies)
Crate:     h2
Version:   0.3.27
Title:     h2 unbounded empty DATA frames
Date:      2026-08-17
ID:        RUSTSEC-2026-0258
Solution:  Upgrade to >=0.4.16
Crate:     rustls
Version:   0.22.4
Title:     `Debug` impls for client session values expose TLS secrets
Date:      2026-10-14
ID:        RUSTSEC-2026-0346
Solution:  Upgrade to >=0.23.46
Crate:     rustls
Version:   0.22.4
Title:     Unauthenticated ServerHello can downgrade later connections to classical key exchange
Date:      2026-10-14
ID:        RUSTSEC-2026-0349
Solution:  Upgrade to >=0.23.46
Crate:     rustls-webpki
Version:   0.102.8
Title:     CRLs not considered authoritative by Distribution Point due to faulty matching logic
Date:      2026-03-20
ID:        RUSTSEC-2026-0049
Solution:  Upgrade to >=0.103.10
Crate:     rustls-webpki
Version:   0.102.8
Title:     Name constraints for URI names were incorrectly accepted
Date:      2026-04-14
ID:        RUSTSEC-2026-0098
Solution:  Upgrade to >=0.103.12, <0.104.0-alpha.1 OR >=0.104.0-alpha.6
Crate:     rustls-webpki
Version:   0.102.8
Title:     Name constraints were accepted for certificates asserting a wildcard name
Date:      2026-04-14
ID:        RUSTSEC-2026-0099
Solution:  Upgrade to >=0.103.12, <0.104.0-alpha.1 OR >=0.104.0-alpha.6
Crate:     rustls-webpki
Version:   0.102.8
Title:     Reachable panic in certificate revocation list parsing
Date:      2026-04-22
ID:        RUSTSEC-2026-0104
Solution:  Upgrade to >=0.103.13, <0.104.0-alpha.1 OR >=0.104.0-alpha.7
Crate:     rustls-webpki
Version:   0.102.8
Title:     Certificate verification may accept distrusted intermediates
Date:      2026-10-20
ID:        RUSTSEC-2026-0382
Solution:  Upgrade to >=0.103.16, <0.104.0-alpha.1 OR >=0.104.0-alpha.8
Crate:     bincode
Version:   1.3.3
Warning:   unmaintained
Title:     Bincode is unmaintained
Date:      2025-12-16
ID:        RUSTSEC-2025-0141
Crate:     rustls-pemfile
Version:   2.2.0
Warning:   unmaintained
Title:     rustls-pemfile is unmaintained
Date:      2025-11-28
ID:        RUSTSEC-2025-0134
error: 8 vulnerabilities found!
warning: 2 allowed warnings found
```

- `Scanning Cargo.lock for vulnerabilities (367 crate dependencies)` のとおり、RustNote は、自分で書いた部品のほかに、367 個の部品を使っています。
- 弱点（vulnerability）が **8 件**、手入れが止まった（unmaintained）という知らせが 2 件ありました。`h2`（HTTP/2）・`rustls`（TLS）・`rustls-webpki`（証明書の確かめ）です。`Solution` は、どれも「`0.4.16` 以上」「`0.23.46` 以上」「`0.103.16` 以上」のように、**次の大きな版**でした。

どの部品が、これを使っているかをたどります。

```powershell
cargo tree -i rustls-webpki@0.102.8
cargo tree -i h2@0.3.27 --depth 4
```

```text
rustls-webpki v0.102.8
└── rustls v0.22.4
    ├── hyper-rustls v0.25.0
    │   └── libsql v0.10.0
    │       └── rustnote v0.1.0 (...\rustnote)
    └── tokio-rustls v0.25.0
        └── hyper-rustls v0.25.0 (*)
h2 v0.3.27
├── hyper v0.14.32
│   ├── axum v0.6.20
│   │   └── tonic v0.11.0
│   │       ├── libsql v0.10.0
│   │       ├── libsql_replication v0.10.0
│   │       └── tonic-web v0.11.0
│   ├── hyper-rustls v0.25.0
│   │   └── libsql v0.10.0 (*)
│   ├── hyper-timeout v0.4.1
│   │   └── tonic v0.11.0 (*)
│   ├── libsql v0.10.0 (*)
│   ├── tonic v0.11.0 (*)
│   └── tonic-web v0.11.0 (*)
└── tonic v0.11.0 (*)
```

- どれも、RustNote が直接使う `libsql 0.10.0` の中から来ています。`rustls-webpki` は Turso への接続の TLS（`hyper-rustls`）、`h2` は HTTP/2（`hyper 0.14`）です。`libsql` は、`hyper-rustls` の `0.25` を使うと決めているので、RustNote の側で `cargo update` をしても、`0.103` には上がりません。`libsql 0.10.0` は、2026 年 10 月の今、crates.io の最新です（`cargo search libsql`）。
- **影響を考える:** これらは、RustNote が **Turso に接続する**ときの部品です。ローカルの DB ファイル（開発・テスト）では使いません。証明書の確かめの弱点は、攻撃する人が、RustNote と Turso の通信の途中に入れる場合に効いてきます。軽く見ず、「すぐ直せないなら、どう手を打つか」を決めます（例10-3）。

#### 例10-3: 使わない機能を外す・受け入れて記録する

RustNote は、libsql の `Builder::new_local`（ローカルのファイル）と `Builder::new_remote`（Turso）しか使っていません。既定で入っている `replication`（埋め込みの複製）と `sync` は使わないので、外してみます（練習用のコピーで）。

```diff
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -20,7 +20,7 @@
 # .env ファイルを環境変数として読み込む（開発用）
 dotenvy = "=0.15.7"
 # Turso / libSQL のクライアント（ローカルファイルとリモートの両方）
-libsql = "=0.10.0"
+libsql = { version = "=0.10.0", default-features = false, features = ["core", "remote", "tls"] }
 # フォームや JSON を構造体に変換する
 serde = { version = "=1.0.229", features = ["derive"] }
 # エラー型を短く書く
```

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 23.32s
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.07s
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s
```

部品の数（`Cargo.lock` の `name = ` の行の数）は、次のように変わりました。

```text
外す前: 367
外した後: 330
```

```text
error: 8 vulnerabilities found!
warning: 1 allowed warning found
```

- テストは全部通り、部品は 367 → **330 個**に減りました。手入れの止まった `bincode`（replication が使っていた）も消え、知らせは 2 件 → 1 件になりました。ただし、**弱点の 8 件は残りました**。TLS と HTTP/2 は、Turso への接続（`remote`・`tls`）そのものに要るからです。
- 使わない機能を外すのは、**部品を減らす**（供給網を短くする）良い手です。弱点が全部消えなくても、これから見つかる弱点の当たる場所が減ります（改善点。教材の `Cargo.toml` は、まだ既定のままです）。

直せない弱点は、**理由と見直す日を書いて**、受け入れると決めます。cargo-audit は、`.cargo/audit.toml` の `ignore` に書いた番号を、失敗に数えません。

```toml
# cargo audit の設定。直せない弱点を、理由と見直す日を書いて、受け入れる
[advisories]
ignore = [
    # libsql 0.10.0（最新）の Turso への接続の部品。libsql の更新を待つ（2026-11-30 に見直す）
    "RUSTSEC-2026-0258", # h2 0.3.27
    "RUSTSEC-2026-0346", # rustls 0.22.4
    "RUSTSEC-2026-0349", # rustls 0.22.4
    "RUSTSEC-2026-0049", # rustls-webpki 0.102.8
    "RUSTSEC-2026-0098", # rustls-webpki 0.102.8
    "RUSTSEC-2026-0099", # rustls-webpki 0.102.8
    "RUSTSEC-2026-0104", # rustls-webpki 0.102.8
    "RUSTSEC-2026-0382", # rustls-webpki 0.102.8
]
```

```text
Crate:     bincode
Warning:   unmaintained
Crate:     rustls-pemfile
Warning:   unmaintained
warning: 2 allowed warnings found
```

- 8 件は失敗に数えられず、`exit=0` で通りました。手入れの止まった知らせの2件は、警告（`allowed warnings`）として出たままです。
- `ignore` は「見なかったことにする」ではありません。**なぜ受け入れたか**（直った版がまだ無い）と、**いつ見直すか**（2026-11-30）を書き、見直す日に、libsql に新しい版が出ていないかを確かめます。Turso は、新しい Rust の部品 `turso`（2026 年 10 月の今、`0.9.0-pre.2`。まだ試しの版）も出しているので、それも見張ります。

### よくあるエラーと読み方

#### エラー1: `Cargo.toml` に部品を足したのに、`--locked` で組み立てた

`Cargo.toml` に `itoa = "1"` を足し、`Cargo.lock` を更新しないまま、CI と同じ `--locked` で組み立てた場合です。

```powershell
cargo build --locked
```

```text
error: cannot update the lock file ...\rustnote\Cargo.lock because --locked was passed to prevent this
help: to generate the lock file without accessing the network, remove the --locked flag and use --offline instead.
```

- **何が起きたか:** 「`--locked` が付いているので、`Cargo.lock` を書き換えられません」というエラーです。`Cargo.toml` に足した `itoa` が `Cargo.lock` に無いので、cargo は記録を書き足そうとして、`--locked` に止められました。
- **なぜ止めるのか:** `--locked` が無いと、CI は、その日の crates.io の新しい版を勝手に選びます。手元と CI で違う版になり、「手元では通ったのに」が起きます。悪意のある新しい版が入る入口にもなります。
- **直し方:** 手元で `cargo build`（`--locked` なし）をして `Cargo.lock` を更新し、`Cargo.toml` と `Cargo.lock` を**一緒に**コミットします。

#### エラー2: 手元では警告だけだったのに、CI で止まった（`-D warnings`）

`src/routes/health.rs` に、使わない変数（`let started = std::time::Instant::now();`）を残した場合です。

```text
warning: unused variable: `started`
  --> src\routes\health.rs:22:9
warning: `rustnote` (lib) generated 1 warning (run `cargo fix --lib -p rustnote` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.75s
error: unused variable: `started`
  --> src\routes\health.rs:22:9
   = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustnote` (lib) due to 1 previous error
error: could not compile `rustnote` (lib test) due to 1 previous error
```

- **何が起きたか:** `cargo build` では**警告**（warning）で、組み立ては成功しました。CI と同じ `clippy … -- -D warnings` では、`-D warnings`（警告を全部エラーにする）のため、同じ所が**エラー**になって止まりました。
- **直し方:** 使わない変数は消します。わざと残すなら、`_started` のように `_` で始めます。手元でも、コミットの前に CI と同じコマンドを動かす習慣にします（例10-1）。

#### エラー3: 字下げを崩したまま、`fmt --check` を通そうとした

`src/routes/health.rs` の1行で、`,` の前に空白を入れた場合です。

```text
Diff in \\?\...\rustnote\src\routes\health.rs:20:
-        Ok(()) => Ok((StatusCode::OK, Json(Health { status: "ok" })))   ,
+        Ok(()) => Ok((StatusCode::OK, Json(Health { status: "ok" }))),
```

- **何が起きたか:** `cargo fmt --check` は、ファイルを書き換えずに、「整形すると、こう変わる」という差分（`-` が今、`+` が整形後）を表示し、`exit=1` で失敗しました。
- **直し方:** `cargo fmt`（`--check` なし）で整形してからコミットします。整形を機械に任せると、見た目の違いで差分が増えず、レビューで中身に集中できます。

### RustNote ではここで使う

- RustNote の `ci.yml` は、fmt・clippy・test・docker build の見本です。教材のリポジトリでは `.github/workflows/` に置いていないので、動いてはいません（公開するリポジトリに置く手順は、第12巻（準備中））。
- **改善点（この単元で見つかったこと）:**
  - `Cargo.lock` に、弱点の知らせが 8 件ある部品が入っています（`libsql 0.10.0` の Turso への接続の部品）。直った libsql はまだ無いので、`cargo audit` を CI で毎日動かし、libsql の更新を見張ります（問1）。
  - libsql の使わない機能（`replication`・`sync`）を外すと、部品が 37 個減り、テストも全部通りました。
- **事故の対応（部品の弱点が見つかった日）:** 第8巻 UNIT 10 の順番（気づく・止める・調べる・知らせる・直す・振り返る）を、部品に当てはめます（問3）。DB のトークンが漏れたときの手順は、第7巻 UNIT 10 です。
- この単元で、第11巻のテストと確かめは、ひととおりそろいました。ユニット（UNIT 01）・リポジトリ（UNIT 02）・HTTP（UNIT 03）・property と fuzz（UNIT 04）・E2E（UNIT 05）・守り（UNIT 06）・見える化（UNIT 07）・速さとメモリ（UNIT 08・09）、そして毎回の自動の確かめと部品の安全（UNIT 10）です。

### 練習問題

**問1:** `cargo audit` を、コードを変えていない日にも、毎日動かすには、`ci.yml` をどう変えればよいでしょうか。なぜ、変更の無い日にも動かす必要があるのでしょうか。

**問2:** 例10-3で、部品が 367 個から 330 個に減りました。弱点の 8 件は残ったのに、部品を減らすことに、どんな良いことがあるでしょうか。

**問3:** ある朝、`cargo audit` が、RustNote の部品に、新しい弱点（直った版がある）を見つけました。第8巻 UNIT 10 の事故の対応の順番に当てはめて、することを並べてください。

### 解答と解説

**問1の解答:**

```yaml
on:
  push:
    branches: [main]
  pull_request:
  # 毎日、日本の時刻の 9 時（UTC の 0 時）にも動かす
  schedule:
    - cron: "0 0 * * *"
```

```yaml
      - name: 部品の弱点の確かめ
        run: |
          cargo install cargo-audit --version 0.22.2 --locked
          cargo audit
```

- 弱点の知らせ（RustSec）は、コードを変えなくても、**毎日増えます**。例10-2の 8 件のうち 3 件は、この 1 か月の間に出た知らせでした（`RUSTSEC-2026-0346`・`0349` は 2026-10-14、`0382` は 2026-10-20。出力の `Date`）。9 月の終わりに同じ `Cargo.lock` を調べていたら、5 件だったはずです。変更のある日だけ確かめていると、変更の無い間に出た知らせに気づけません。
- `schedule` の `cron` は、UTC（世界の標準の時刻）で書きます。

**問2の解答:**

- **これから見つかる弱点が、当たりにくくなる:** 部品の数が減れば、その部品に、あとで弱点が見つかっても、RustNote には関係ありません。例10-3では、手入れの止まった `bincode` も一緒に消えました。
- **悪意のある変更の入口が減る:** 部品の作者のアカウントが乗っ取られ、新しい版に悪いコードが入る、という事故があります。使う部品が少ないほど、入口も少なくなります。
- **組み立てが速く、小さくなる:** 組み立てる物が減るので、CI の時間も、Docker のイメージも小さくなります。
- 使わない機能を外したら、**テストで確かめる**のを忘れないようにします（例10-3では、全部のテストが通りました）。

**問3の解答:**

| 順番 | 部品の弱点のときにすること |
|---|---|
| 気づく | `cargo audit` の知らせ（CI の毎日の確かめ）。勧告の番号・内容・`Solution` を読む |
| 止める | 攻撃されているおそれが高く、すぐ直せないなら、その機能を止める（例: 弱点のある入口を閉じる）。そうでなければ、急いで直す |
| 調べる | `cargo tree -i` で、どこから来ているか。RustNote で、その道すじを使っているか（例10-2の「影響を考える」） |
| 直す | `cargo update -p 部品の名前` で直った版に上げる（大きな版上げなら、使っている部品の更新を待つか、外す）。`cargo test --locked` を通し、`Cargo.lock` をコミットして、公開し直す |
| 知らせる | 利用者のデータに影響があったときは、知らせる（第8巻 UNIT 10） |
| 振り返る | 次に早く気づくために、毎日の `cargo audit` や、`audit.toml` の見直す日を決める |

### まとめ

- CI は、fmt・clippy（`-D warnings`）・test（`--locked`）を、変更のたびに、同じ順で自動に動かし、どれか1つでも失敗したら止まる。手元では警告だけでも CI では止まり、`--locked` は `Cargo.lock` の書き換えを止め、`fmt --check` は差分を見せて失敗する。
- cargo-audit で、RustNote の 367 個の部品から、弱点の知らせ 8 件（`libsql 0.10.0` の Turso への接続の `h2`・`rustls`・`rustls-webpki`）と、手入れの止まった知らせ 2 件が見つかった。どれも大きな版上げが要り、RustNote の側では直せない。`cargo tree -i` で、どこから来ているかをたどる。
- 使わない機能（`replication`・`sync`）を外すと、部品は 330 個に減ったが、弱点は残った。直せない弱点は、`audit.toml` に理由と見直す日を書いて受け入れ、`cargo audit` を毎日動かして、更新を見張る。
