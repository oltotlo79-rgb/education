<div style="page-break-after: always; text-align: center; margin: 0; padding: 0;">
<img src="assets/cover.jpg" alt="表紙" style="width: 100%; max-height: 175mm; object-fit: cover; border-radius: 10px;">
<div style="margin-top: 14px; font-size: 11px; letter-spacing: 0.18em; color: #c2410c; font-weight: 700;">VOLUME 11 / RUST BACKEND MASTERY</div>
<div style="font-size: 26px; font-weight: 800; color: #0f172a; margin-top: 6px; line-height: 1.35;">品質保証 — テスト・セキュリティ・可観測性・性能</div>
<div style="font-size: 13px; color: #475569; margin-top: 8px;">何を、どの段のテストで確かめるかを決め、ユニットテストから負荷試験・CI までを手を動かして身に付ける</div>
</div>

# 第11巻: 品質保証 — テスト・セキュリティ・可観測性・性能

## この巻で身に付けること

第10巻までで、RustNote の画面と機能は、ひととおりそろいました。けれども、「今日、動いた」ことと、「明日も、誰かが直したあとも、ちゃんと動く」ことは別の話です。次のような事故は、動くアプリでも起きます。

- 小さな修正のつもりが、別の画面を壊していたのに、公開するまで誰も気づかなかった
- 英語のデータでは問題が無かったのに、日本語のタイトルを入れたら弾かれた
- 他人のノートが見えてしまう不具合を一度直したのに、半年後に同じ不具合がまた入った
- 公開したら遅くなったが、どこが遅いのか、測る方法が無かった
- 夜中に止まっていたのに、朝、利用者から言われるまで分からなかった

この巻では、こうした事故を、**テスト・見張り（可観測性）・測定**で、早く・自動で見つける方法を学びます。大事なのは、「とにかくテストをたくさん書く」ことではなく、**何を、どの段のテストで確かめるか**を決めることです。

### 到達目標

この巻を終えると、次のことができるようになります。

- 境界の値（空・1・最大・最大 + 1・日本語・空白）を表に並べた、ユニットテストを書ける。テストが本当に間違いを見つけられるかを、わざと壊した版で確かめられる
- 本物の DB を使った結合テスト、HTTP の返事（状態コード・ヘッダー・本文・クッキー）のテスト、ブラウザで操作する E2E テストを、役割を分けて書ける
- 脅威モデル（第8巻）を、「攻撃が止まること」を確かめる守りのテストに変えられる
- ログ・数の記録（メトリクス）・ヘルスチェックの違いを説明し、速さを release ビルドで測り、負荷をかけて限界を調べられる
- CI で、整形・Clippy・テスト・依存の監査を、毎回自動で回せる

### 学習地図

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 400" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-11-001" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="6" y="30" width="708" height="90" rx="12" fill="#eff6ff"/>
<rect x="6" y="140" width="708" height="90" rx="12" fill="#fff7ed"/>
<rect x="6" y="250" width="708" height="90" rx="12" fill="#ecfdf5"/>
<text x="360" y="20" font-size="13" fill="#475569" text-anchor="middle">上の段から順に進みます（各段の中は左から右へ）</text>
<text x="65" y="70" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">段ごとの</text>
<text x="65" y="90" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">テスト</text>
<text x="65" y="180" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">広く・</text>
<text x="65" y="200" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">守る</text>
<text x="65" y="290" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">測る・</text>
<text x="65" y="310" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">回し続ける</text>
<rect x="135" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="200" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 01</text>
<text x="200" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">ユニットテスト</text>
<text x="200" y="99" font-size="12" fill="#475569" text-anchor="middle">境界値・表の形</text>
<rect x="282" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="347" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 02</text>
<text x="347" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">DB の結合テスト</text>
<text x="347" y="99" font-size="12" fill="#475569" text-anchor="middle">リポジトリ</text>
<rect x="429" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="494" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 03</text>
<text x="494" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">HTTP のテスト</text>
<text x="494" y="99" font-size="12" fill="#475569" text-anchor="middle">状態・ヘッダー</text>
<rect x="576" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="641" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 04</text>
<text x="641" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">property・fuzz</text>
<text x="641" y="99" font-size="12" fill="#475569" text-anchor="middle">広い入力を試す</text>
<line x1="265" y1="75" x2="280" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-001)"/>
<line x1="412" y1="75" x2="427" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-001)"/>
<line x1="559" y1="75" x2="574" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-001)"/>
<path d="M641,110 V130 H200 V146" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-001)"/>
<rect x="135" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="200" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 05</text>
<text x="200" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">E2E</text>
<text x="200" y="209" font-size="12" fill="#475569" text-anchor="middle">ブラウザ・a11y</text>
<rect x="282" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="347" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 06</text>
<text x="347" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">守りのテスト</text>
<text x="347" y="209" font-size="12" fill="#475569" text-anchor="middle">攻撃が止まるか</text>
<rect x="429" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="494" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 07</text>
<text x="494" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">見張り</text>
<text x="494" y="209" font-size="12" fill="#475569" text-anchor="middle">ログ・数・SLO</text>
<rect x="576" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="641" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 08</text>
<text x="641" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">速さを測る</text>
<text x="641" y="209" font-size="12" fill="#475569" text-anchor="middle">benchmark</text>
<line x1="265" y1="185" x2="280" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-001)"/>
<line x1="412" y1="185" x2="427" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-001)"/>
<line x1="559" y1="185" x2="574" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-001)"/>
<path d="M641,220 V240 H200 V256" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-001)"/>
<rect x="135" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="200" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 09</text>
<text x="200" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">負荷試験</text>
<text x="200" y="319" font-size="12" fill="#475569" text-anchor="middle">限界と劣化</text>
<rect x="282" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="347" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 10</text>
<text x="347" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">CI・供給網</text>
<text x="347" y="319" font-size="12" fill="#475569" text-anchor="middle">毎回自動で</text>
<line x1="265" y1="295" x2="280" y2="295" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-001)"/>
<rect x="429" y="260" width="277" height="70" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-dasharray="5 4"/>
<text x="567" y="284" font-size="12" fill="#475569" text-anchor="middle">RustNote では</text>
<text x="567" y="304" font-size="12" fill="#475569" text-anchor="middle">src/ の #[cfg(test)]・tests/・ci.yml</text>
<text x="360" y="372" font-size="13" fill="#475569" text-anchor="middle">各単元の流れ: たとえ話 → 図 → 最小コード → 実務の形 → よくあるエラー → 練習問題</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図11-?: 第11巻の学習地図。小さな段のテストから順に積み上げ、広い入力と攻撃を試し、速さと動きを測って、CI で回し続ける</div>
</div>

| 単元 | テーマ | ひとことで言うと |
|---|---|---|
| UNIT 01 | ユニットテスト・境界値・表の形のテスト | 端の値を表に並べ、関数1つを速く確かめる |
| UNIT 02 | リポジトリの結合テスト | 本物の DB と SQL で、保存と読み出しを確かめる |
| UNIT 03 | HTTP のテスト | 状態コード・Location・クッキー・本文を確かめる |
| UNIT 04 | property test・fuzz・入力の読み取り | 思いつかない入力を、たくさん自動で試す |
| UNIT 05 | E2E・ブラウザ・アクセシビリティ（準備中） | 大事な流れだけを、本物のブラウザで確かめる |
| UNIT 06 | 守りの回帰テスト（準備中） | 攻撃が止まることを、毎回確かめる |
| UNIT 07 | tracing・メトリクス・health・SLO（準備中） | 動いている様子を外から見えるようにする |
| UNIT 08 | benchmark・profile・メモリの使い方（準備中） | 遅いところを、測ってから直す |
| UNIT 09 | 負荷試験・背圧・容量（準備中） | たくさん来たときの限界と、壊れ方を決める |
| UNIT 10 | CI・依存・供給網・事故の対応（準備中） | 毎回同じ確かめを自動で回し、部品の安全も見る |

> **お知らせ:** この版には UNIT 01〜04 を収録しています。UNIT 05〜10 は準備中で、次の更新から順に追加します。

### 前提知識（先に読んでおく巻）

- **第2巻（Rust 基礎）:** 日本語の文字数を `chars().count()` で数えること（第2巻 UNIT 05）。UNIT 01 の表に、日本語と絵文字の行が出てきます。
- **第4巻（設計）:** 検査済みの型（第4巻 UNIT 01）と、テストの書き方の基本（`#[test]`・`assert!`・`assert_eq!`。第4巻 UNIT 10）。
- **第9巻（RustNote 設計）:** RustNote のドメインの型（`NoteTitle`・`Email`・`Password`・`NoteId`）と、その決まり（第9巻 UNIT 03）。この巻のテストも、第9巻と同じ形（`tests/` に足して `cargo test --test 名前`）で書きます。

### この巻の進め方

テストは、「通った」だけでは、本当に役に立っているかが分かりません。各単元では、次の順番で進めてください。

1. **決まりを書き出す:** テストを書く前に、確かめる決まり（「タイトルは 1〜80 文字」など）を、日本語で書き出します。
2. **端の値を並べる:** 決まりの両側（80 と 81、0 と 1）と、間違えやすい入力（空白・日本語・絵文字）を表にします。
3. **テストを動かす:** 全部の行が通ることを確かめます。
4. **わざと壊して確かめる:** 関数をわざと間違えた版に変えても、テストがちゃんと失敗するかを確かめます。失敗しないテストは、何も守っていません。

### 使うツールと版

この巻のコードは、すべて次の環境で実際にコンパイル・実行し、実行結果とエラーメッセージは実際の出力をそのまま載せています（パソコンのユーザー名などを含む部分は `...` に置き換えています）。

| 項目 | 版・内容 |
|---|---|
| OS | Windows 11（ARM64）。コマンドは PowerShell を主に、WSL / macOS / Linux は補足で示します |
| Rust | **1.99 以上**（本書シリーズの基準。Topcoat 0.11 が 1.99 以上を求めます）。動作確認は rustc 1.99.1 / cargo 1.99.1（`aarch64-pc-windows-msvc`） |
| RustNote | 教材の `Rust/rustnote/`（`rustnote` 0.1.0）。この巻では `src/` を変えずに、`tests/` と `examples/` に確かめ用のファイルを足す（UNIT 03 からは、練習用のコピーの `Cargo.toml` の dev-dependencies にも足す） |
| scraper | 0.29.0（UNIT 03。HTML を読んで、CSS セレクターで要素を探す） |
| proptest | 1.12.0（UNIT 04。property test） |
| cargo-fuzz | 0.13.2（UNIT 04。WSL の Ubuntu 24.04（aarch64）で、rustc 1.101.0-nightly とともに使う） |
| テストの道具 | Rust の標準の `#[test]`・`assert!`・`assert_eq!`（UNIT 01 では、外のクレートを足しません） |

### 練習用のフォルダを用意する

第9巻と同じように、教材の `Rust/rustnote/` を**作業用のフォルダにコピー**して使います。

```powershell
# 教材の rustnote フォルダを、中身ごと（-Recurse）作業用のフォルダへコピーする
Copy-Item -Recurse "教材のフォルダ\Rust\rustnote" .\rustnote
cd rustnote
# このフォルダで使われる Rust の版を確かめる（1.99.1 になる）
rustup show active-toolchain
```
