<div style="page-break-after: always; text-align: center; margin: 0; padding: 0;">
<img src="assets/cover.jpg" alt="表紙" style="width: 100%; max-height: 175mm; object-fit: cover; border-radius: 10px;">
<div style="margin-top: 14px; font-size: 11px; letter-spacing: 0.18em; color: #c2410c; font-weight: 700;">VOLUME 13 / RUST BACKEND MASTERY</div>
<div style="font-size: 26px; font-weight: 800; color: #0f172a; margin-top: 6px; line-height: 1.35;">完全コード — 演習・解答・用語集・索引</div>
<div style="font-size: 13px; color: #475569; margin-top: 8px;">RustNote の全ファイルを順に読み、自分の PC で同じ結果を出して確かめる最終巻</div>
</div>

# 第13巻: 完全コード — 演習・解答・用語集・索引

## この巻で身に付けること

第12巻までで、RustNote を設計し（第9巻）、作り（第10巻）、確かめ（第11巻）、公開する（第12巻）までの流れを学びました。どの巻も、説明に必要な部分だけを抜き出して見せています。だから、「全部をつなげると、結局どのファイルに何が書いてあるのか」は、まだぼんやりしているかもしれません。

この巻は、教材のフォルダに入っている**完成した RustNote のソース**（`Rust/rustnote/`）を、**全部のファイルについて**、上から順に読んでいく巻です。ただ貼り付けて眺めるのではなく、ファイルごとに次の3つを確かめます。

- そのファイルは、**何を守るために**あるのか（どんな事故を防いでいるのか）
- **誰が読む**のか（cargo・rustup・rustc・ブラウザ・Docker・Fly.io など）
- 書き間違えたり消したりすると、**何が起きる**のか（実際のエラーで確かめる）

巻の最後には、全巻をまたぐ総合演習と解答、全巻の用語集、索引を置きます。

### 到達目標

この巻を終えると、次のことができるようになります。

- RustNote の 42 個のファイルについて、それぞれの役割と、読む道具（cargo・rustc・Docker など）を説明できる
- 教材と同じ版の道具と依存クレートで、RustNote を自分の PC でゼロからビルドし、テスト・整形・Clippy の確かめを通せる
- 1つのファイルを書き換えたときに、どのコマンドの、どんなエラーで気づけるかを予想し、実際に確かめられる
- 総合演習で、新しい機能を足すときの置き場所と、壊していないことの確かめ方を、自分で決められる

### 学習地図

10 個の単元で、RustNote のファイルを「土台から上へ」の順に読みます。最初は版を決める設定ファイル、次に DB の表、起動、型、保存、ログイン、画面、見た目、最後にテストと公開です。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 400" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-001" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="6" y="30" width="708" height="90" rx="12" fill="#f8fafc"/>
<rect x="6" y="140" width="708" height="90" rx="12" fill="#eff6ff"/>
<rect x="6" y="250" width="708" height="90" rx="12" fill="#ecfdf5"/>
<text x="360" y="20" font-size="13" fill="#475569" text-anchor="middle">上の段から順に進みます（各段の中は左から右へ）</text>
<text x="65" y="70" font-size="14" font-weight="700" fill="#475569" text-anchor="middle">土台を</text>
<text x="65" y="90" font-size="14" font-weight="700" fill="#475569" text-anchor="middle">読む</text>
<text x="65" y="180" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">中身を</text>
<text x="65" y="200" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">読む</text>
<text x="65" y="290" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">画面と公開</text>
<text x="65" y="310" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">を読む</text>
<rect x="135" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="200" y="60" font-size="12" font-weight="700" fill="#475569" text-anchor="middle">UNIT 01</text>
<text x="200" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">版を固定する</text>
<text x="200" y="99" font-size="12" fill="#475569" text-anchor="middle">Cargo.toml など</text>
<rect x="282" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="347" y="60" font-size="12" font-weight="700" fill="#475569" text-anchor="middle">UNIT 02</text>
<text x="347" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">表と DB の入口</text>
<text x="347" y="99" font-size="12" fill="#475569" text-anchor="middle">migrations・db.rs</text>
<rect x="429" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="494" y="60" font-size="12" font-weight="700" fill="#475569" text-anchor="middle">UNIT 03</text>
<text x="494" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">設定と起動</text>
<text x="494" y="99" font-size="12" fill="#475569" text-anchor="middle">config・main・lib</text>
<rect x="576" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="641" y="60" font-size="12" font-weight="700" fill="#475569" text-anchor="middle">UNIT 04</text>
<text x="641" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">型とエラー</text>
<text x="641" y="99" font-size="12" fill="#475569" text-anchor="middle">domain・error</text>
<line x1="265" y1="75" x2="280" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-001)"/>
<line x1="412" y1="75" x2="427" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-001)"/>
<line x1="559" y1="75" x2="574" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-001)"/>
<path d="M641,110 V130 H200 V146" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-001)"/>
<rect x="135" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="200" y="170" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 05</text>
<text x="200" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">データの保存</text>
<text x="200" y="209" font-size="12" fill="#475569" text-anchor="middle">repository/</text>
<rect x="282" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="347" y="170" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 06</text>
<text x="347" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">ログインと状態</text>
<text x="347" y="209" font-size="12" fill="#475569" text-anchor="middle">auth・flash・state</text>
<rect x="429" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="494" y="170" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 07</text>
<text x="494" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">画面の枠と認証</text>
<text x="494" y="209" font-size="12" fill="#475569" text-anchor="middle">routes・views</text>
<rect x="576" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="641" y="170" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 08</text>
<text x="641" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">ノートの CRUD</text>
<text x="641" y="209" font-size="12" fill="#475569" text-anchor="middle">routes/notes/</text>
<line x1="265" y1="185" x2="280" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-001)"/>
<line x1="412" y1="185" x2="427" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-001)"/>
<line x1="559" y1="185" x2="574" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-001)"/>
<path d="M641,220 V240 H200 V256" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-001)"/>
<rect x="135" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="200" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 09</text>
<text x="200" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">見た目</text>
<text x="200" y="319" font-size="12" fill="#475569" text-anchor="middle">assets/style.css</text>
<rect x="282" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="347" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 10</text>
<text x="347" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">テストと公開</text>
<text x="347" y="319" font-size="12" fill="#475569" text-anchor="middle">tests・Docker・Fly</text>
<rect x="429" y="260" width="277" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="567" y="282" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">巻末</text>
<text x="567" y="304" font-size="12" fill="#0f172a" text-anchor="middle">総合演習と解答・全巻の用語集・索引</text>
<line x1="265" y1="295" x2="280" y2="295" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-001)"/>
<line x1="412" y1="295" x2="427" y2="295" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-001)"/>
<text x="360" y="372" font-size="13" fill="#475569" text-anchor="middle">各単元: ファイルの全文 → 役割の図 → 動かして確かめる → 壊して読むエラー → 練習問題</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図13-?: 第13巻の学習地図。土台の設定ファイルから読み始め、型・保存・ログイン・画面を経て、テストと公開で終わる。巻末に総合演習・用語集・索引を置く</div>
</div>

| 単元 | テーマ | 読むファイル |
|---|---|---|
| UNIT 01 | 版を固定する設定ファイル | `Cargo.toml`・`Cargo.lock`・`rust-toolchain.toml`・`rustfmt.toml`・`.gitignore`・`.env.example` |
| UNIT 02 | 表を作る SQL と DB の入口（準備中） | `migrations/0001〜0003`・`src/db.rs` |
| UNIT 03 | 設定と起動の順番（準備中） | `src/config.rs`・`src/main.rs`・`src/lib.rs` |
| UNIT 04 | ドメインの型とエラーの分類（準備中） | `src/domain.rs`・`src/error.rs` |
| UNIT 05 | リポジトリ（準備中） | `src/repository.rs`・`src/repository/` の3つ |
| UNIT 06 | ログイン・お知らせ・共有の状態（準備中） | `src/auth.rs`・`src/flash.rs`・`src/state.rs` |
| UNIT 07 | 画面の枠・部品・登録とログインの画面（準備中） | `src/routes.rs`・`src/views.rs`・`src/routes/` の signup・login・logout・health |
| UNIT 08 | ノートの CRUD（準備中） | `src/routes/notes.rs`・`src/routes/notes/` の4つ |
| UNIT 09 | 見た目（準備中） | `assets/style.css` |
| UNIT 10 | テスト・Docker・Fly.io・CI（準備中） | `tests/` の3つ・`Dockerfile`・`.dockerignore`・`fly.toml`・`ci.yml`・`README.md` |

> **お知らせ:** この版には UNIT 01 を収録しています。UNIT 02〜10 と、巻末の総合演習・解答・全巻の用語集・索引は準備中で、次の更新から順に追加します。

### RustNote の全ファイル

RustNote は、次の 42 個のファイルでできています（`git ls-files` で数えた、Git で管理しているファイルです）。行数は `Cargo.lock` を除くと約 4,700 行です。

| ファイル | 行数 | 単元 | ひとことで言うと |
|---|---|---|---|
| `Cargo.toml` | 35 | 01 | 使うクレートとその版の一覧 |
| `Cargo.lock` | 3,682 | 01 | 実際に選ばれた全クレートの版と、中身の指紋（自動で作られる） |
| `rust-toolchain.toml` | 5 | 01 | このフォルダで使う Rust の版（1.99.0） |
| `rustfmt.toml` | 2 | 01 | `cargo fmt` の設定 |
| `.gitignore` | 15 | 01 | Git に入れないファイル（ビルドの成果物・DB・`.env`） |
| `.env.example` | 15 | 01 | 環境変数の見本 |
| `migrations/0001_users.sql` ほか2つ | 9・18・12 | 02 | 表を作る SQL |
| `src/db.rs` | 185 | 02 | DB への接続とマイグレーション |
| `src/config.rs`・`src/main.rs`・`src/lib.rs` | 182・54・27 | 03 | 設定の読み込み・起動の入口・部品の一覧 |
| `src/domain.rs`・`src/error.rs` | 518・152 | 04 | 検査済みの型と、エラーの分類 |
| `src/repository.rs` と `src/repository/` の3つ | 39・243・85・112 | 05 | ノート・ユーザー・セッションの読み書き |
| `src/auth.rs`・`src/flash.rs`・`src/state.rs` | 179・117・42 | 06 | ログイン・一度だけのお知らせ・共有の状態 |
| `src/routes.rs`・`src/views.rs` | 142・249 | 07 | ルーターとレイアウト・画面の部品 |
| `src/routes/` の signup・login・logout・health | 162・89・22・34 | 07 | 登録・ログイン・ログアウト・ヘルスチェック |
| `src/routes/notes.rs` と `src/routes/notes/` の4つ | 145・80・113・29・27 | 08 | ノートの一覧・作成・詳細・編集・完了の切り替え・削除 |
| `assets/style.css` | 290 | 09 | 画面のスタイル |
| `tests/common/mod.rs`・`tests/http.rs`・`tests/repository.rs` | 159・398・248 | 10 | テストの道具と、41 件のテストのうち 22 件 |
| `Dockerfile`・`.dockerignore`・`fly.toml`・`ci.yml`・`README.md` | 50・7・38・48・192 | 10 | コンテナ・公開・CI の設定と説明書 |

- 残りの 19 件のテストは、`src/` の中のファイルの最後にある `#[cfg(test)]` のユニットテストです。それぞれのファイルの単元で読みます。
- 名前が `.` で始まるファイル（`.gitignore` など）は、Windows の PowerShell やエクスプローラーではそのまま表示されます。macOS や Linux の `ls` では隠れるので、`ls -a` で表示します。

### 前提知識（先に読んでおく巻）

この巻は、全巻のまとめです。どの単元も、元になった巻を「第9巻 UNIT 04」のように示します。

- **第1巻（環境構築）:** Rust・VS Code・Git が使えること。ARM64 版の Windows では、LLVM（clang）も入れておきます（第9巻の巻頭にも手順があります）。
- **第2巻〜第5巻:** Rust の文法、所有権、型と trait とエラー、Cargo と features（第4巻 UNIT 07）、async。
- **第6巻〜第8巻:** Topcoat、Turso・libSQL と SQL、ログインとセッション。
- **第9巻〜第12巻:** RustNote の設計・実装・品質の確かめ・公開。この巻は、その完成形を読む巻です。

> **注意:** 第6巻・第8巻・第10巻〜第12巻は、書き直しの途中の巻があります。この巻の本文は、教材のフォルダの `Rust/rustnote/` のコードと、この巻で実際に動かした結果だけをもとに書いているので、ほかの巻が未完成でも読み進められます。

### この巻の進め方

完成したコードを読むときは、つい「読んで分かったつもり」になりがちです。各単元では、次のサイクルで進めてください。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 200" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-002" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="40" width="120" height="64" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="80" y="66" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">① 読む</text>
<text x="80" y="88" font-size="11" fill="#0f172a" text-anchor="middle">全文と解説</text>
<rect x="160" y="40" width="120" height="64" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="220" y="66" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">② 予想する</text>
<text x="220" y="88" font-size="11" fill="#0f172a" text-anchor="middle">消したらどうなる？</text>
<rect x="300" y="40" width="120" height="64" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="66" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">③ 動かす</text>
<text x="360" y="88" font-size="11" fill="#0f172a" text-anchor="middle">教材と同じ結果か</text>
<rect x="440" y="40" width="120" height="64" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="500" y="66" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">④ 1か所壊す</text>
<text x="500" y="88" font-size="11" fill="#0f172a" text-anchor="middle">エラーを読む</text>
<rect x="580" y="40" width="120" height="64" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="640" y="66" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">⑤ 元に戻す</text>
<text x="640" y="88" font-size="11" fill="#0f172a" text-anchor="middle">3つの確かめ</text>
<line x1="140" y1="72" x2="156" y2="72" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-002)"/>
<line x1="280" y1="72" x2="296" y2="72" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-002)"/>
<line x1="420" y1="72" x2="436" y2="72" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-002)"/>
<line x1="560" y1="72" x2="576" y2="72" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-002)"/>
<path d="M640,104 V150 H80 V108" fill="none" stroke="#475569" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-13-002)"/>
<text x="360" y="172" font-size="12" fill="#475569" text-anchor="middle">次のファイルへ（戻したあとは、教材と同じ状態から始める）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図13-?: この巻の進め方。読んで、予想して、動かして、1か所だけ壊してエラーを読み、元に戻してから次のファイルへ進む</div>
</div>

1. **読む:** ファイルの全文と解説を読みます。
2. **予想する:** 「この行を消したら、どのコマンドが、どんなエラーを出すか」を、先に予想します。
3. **動かす:** 教材のコマンドをそのまま実行し、結果が教材と同じか比べます。
4. **1か所だけ壊す:** 「よくあるエラーと読み方」の節のとおりに、1か所だけ書き換えて、エラーを自分の目で見ます。
5. **元に戻す:** 書き換えたファイルを元に戻し、次の3つのコマンドが全部通ることを確かめてから、次のファイルへ進みます。

```powershell
# 見た目が Rust の標準の形か（直すときは --check を外す）
cargo fmt --check
# よくある書き間違いを、警告ではなくエラーとして止める。--locked は Cargo.lock を書き換えさせない
cargo clippy --all-targets --locked -- -D warnings
# テストを全部実行する
cargo test --locked
```

この3つは、RustNote の `ci.yml`（GitHub Actions の設定の例）で、コードを送るたびに自動で実行されるのと同じコマンドです（UNIT 10（準備中））。

> **ヒント:** 元に戻すのがいちばん確実なのは、教材のフォルダから、そのファイルだけをもう一度コピーすることです。練習用のフォルダを Git で管理しておく（`git init` して最初に全部をコミットしておく）と、`git diff` で変えた所を確かめ、`git checkout -- ファイル名` で元に戻せます（UNIT 01 の問3）。

### 使うツールと版

この巻のコードは、すべて次の環境で実際にビルド・実行し、実行結果とエラーメッセージは実際の出力をそのまま載せています（パソコンのユーザー名などを含む部分は `...` に置き換えています）。

| 項目 | 版・内容 |
|---|---|
| OS | Windows 11（ARM64）。コマンドは PowerShell を主に、WSL / macOS / Linux は補足で示します |
| Rust | **1.98 以上**（本書シリーズの基準）。動作確認は rustc 1.99.0 / cargo 1.99.0（`aarch64-pc-windows-msvc`） |
| edition | 2024 |
| RustNote | 教材の `Rust/rustnote/`（`rustnote` 0.1.0）。この巻では中身を変えずに読む |
| 直接使うクレート | topcoat 0.10.0、libsql 0.10.0、tokio 1.53.3、argon2 0.6.0、async-trait 0.1.92、serde 1.0.229、thiserror 2.0.21、anyhow 1.0.104、dotenvy 0.15.7、tempfile 3.27.0（テスト用） |
| topcoat-cli | 0.10.0（UNIT 09・10 で使います。UNIT 01 では使いません） |

### 練習用のフォルダを用意する

第9巻と同じように、教材の `Rust/rustnote/` を**作業用のフォルダにコピー**して使います。この巻では、わざとファイルを書き換えてエラーを見るので、お手本そのものには触らないことが特に大切です。

```powershell
# 教材の rustnote フォルダを、中身ごと（-Recurse）作業用のフォルダへコピーする
Copy-Item -Recurse "教材のフォルダ\Rust\rustnote" .\rustnote
cd rustnote
# このフォルダで使われる Rust の版を確かめる
rustup show active-toolchain
```

```text
1.99.0-aarch64-pc-windows-msvc (overridden by '...\rustnote\rust-toolchain.toml')
```

- `"教材のフォルダ\Rust\rustnote"` の部分は、自分の PC で教材を置いた場所に読み替えてください。
- 第9巻で使ったコピーがあれば、それとは別の、新しいコピーを作ってください。第9巻のコピーには、確かめ用のテストを足してあるからです。
