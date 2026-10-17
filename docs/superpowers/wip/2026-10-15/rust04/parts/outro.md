## この巻のまとめ

この巻では、「気をつける」ではなく「**仕組みで間違いを防ぐ**」ための道具を、型・trait・エラー・Cargo・テストの順にそろえました。次のチェックリストで、できるようになったことを確かめてください。自信がない項目は、括弧の中の単元に戻って、コードを動かし直しましょう。

**型で間違いを防ぐ**

- ☐ `UserId` と `NoteId` のような newtype で、引数の取り違えを `E0308` のコンパイルエラーにできる（UNIT 01）
- ☐ 中身を private にして `parse` だけを入口にし、「作れた＝正しい」型を作れる。文字数は `chars().count()` で数える（UNIT 01）
- ☐ trait で「約束」を決め、`impl トレイト for 型` で実装し、`&impl Trait` で受け取れる（UNIT 02）
- ☐ `NoteRepository` trait で、本物の保存先とテスト用の Fake を差し替えられる（UNIT 02）
- ☐ generics・trait 境界・`where` 句・条件付きの `impl` を書け、単相化（静的ディスパッチ）を説明できる（UNIT 03）
- ☐ `Box<dyn Trait>` / `Arc<dyn Trait + Send + Sync>` で実装を実行時に選べ、vtable と dyn compatible の条件（E0038）を説明できる（UNIT 04）

**失敗と部品を整理する**

- ☐ エラーを enum で分類し、thiserror の `#[error]`・`#[from]` と `?` で層をまたいで変換できる（UNIT 05）
- ☐ `AppError` を 400 / 404 / 409 / 500 に対応づけ、500 の詳細を画面に出さずログに書ける（UNIT 05）
- ☐ 種類で分岐するエラーは thiserror、人に報告するエラーは anyhow（`context`）と使い分けられる（UNIT 05）
- ☐ `mod`・ファイル分割・`pub` / `pub(crate)` / `pub(super)`・`pub use` で公開範囲を設計できる（UNIT 06）
- ☐ workspace で複数の package をまとめ、`[workspace.dependencies]` で版をそろえられる（UNIT 06）
- ☐ 版の指定（`"1.0.229"`・`"=0.10.0"`）を読め、`Cargo.lock` を保存して `--locked` でビルドする理由を説明できる（UNIT 07）
- ☐ features を定義して `#[cfg(feature = "...")]` で切り替え、依存の機能を必要な分だけ選べる。`rust-version = "1.98"` を書ける（UNIT 07）

**外との境界と検証**

- ☐ 入力 DTO（文字列のまま）→ `TryFrom` で検証 → ドメイン型 → 出力 DTO、の順に変換できる（UNIT 08）
- ☐ `deny_unknown_fields` で余計な項目を断り、パスワードのハッシュのような値を出力しない型を作れる（UNIT 08）
- ☐ serde_json のエラーメッセージ（`missing field`・`invalid type`・`unknown field`）を読める（UNIT 08）
- ☐ `Config::from_env()` で設定を起動時に1回だけ検証し、既定値 → ファイル → 環境変数の順に重ねられる（UNIT 09）
- ☐ 秘密を `Secret` 型に包み、`impl Debug` を手で書いて表示から隠せる。`set_var` が unsafe な理由を説明できる（UNIT 09）
- ☐ 単体テスト・結合テスト・doctest を書き、`cargo test` の成功と失敗の出力を読める（UNIT 10）
- ☐ 状態機械を enum と `match` で書き、全部の組み合わせを表駆動テストで確かめられる（UNIT 10）
- ☐ 公開する API を、名前・公開範囲・エラー・例・`#[must_use]` の観点でレビューできる（UNIT 10）

次の第5巻では、`async` と Tokio を使って、たくさんのリクエストを同時にさばくバックエンドの書き方を学びます。この巻の `NoteRepository` trait・`AppError`・`Config`・`AppState` は、そこでも形を変えながら使い続けます。

## 用語ミニ辞典

この巻で出てきた主な用語です。

| 用語 | 読み・意味 | 出てくる単元 |
|---|---|---|
| newtype | ニュータイプ。既存の型を1つだけ包んだ、意味の違う新しい型（`struct UserId(i64);`） | UNIT 01 |
| primitive obsession | 何でも `i64` や `String` のまま扱ってしまう癖。取り違えや未検証の値の原因 | UNIT 01 |
| 不正な状態を表現できない型 | 作れた時点で必ず正しい値になるよう、作り方を制限した型 | UNIT 01・10 |
| 正規化 | 同じ意味のものを同じ形にそろえること（メールの小文字化など） | UNIT 01 |
| `FromStr` | 文字列から作れる型を表す標準の trait。実装すると `.parse()` が使える | UNIT 01・09 |
| trait | トレイト。型が守るべきメソッドの一覧を決めた「約束」 | UNIT 02 |
| 既定の実装 | trait の中で中身まで書いたメソッド。実装側で上書きもできる | UNIT 02 |
| trait 境界 | trait bound。型パラメータに「この trait を実装していること」を求める条件 | UNIT 02・03 |
| ユースケース | アプリが利用者のためにする1つの仕事（ノートを作る、など） | UNIT 02 |
| Repository | リポジトリ。データのしまい方・取り出し方を引き受ける部品 | UNIT 02・04 |
| Fake | フェイク。テスト用の、本物と同じ約束を守る簡単な実装 | UNIT 02 |
| 依存性の逆転 | 上の層が具体的な実装ではなく、間に置いた約束（trait）に頼る設計 | UNIT 02 |
| 孤児ルール | orphan rule。trait か型の少なくとも一方が自分の crate のものでないと `impl` できない決まり | UNIT 02 |
| generics | ジェネリクス。型をパラメータとして受け取る仕組み（`fn f<T>(x: T)`） | UNIT 03 |
| `where` 句 | trait 境界を関数名の後ろにまとめて書く書き方 | UNIT 03 |
| 単相化 | monomorphization。generics を使われた型ごとの専用の関数にコンパイルすること | UNIT 03 |
| 静的ディスパッチ | 呼び出す関数をコンパイル時に決めること | UNIT 03・04 |
| ターボフィッシュ | `parse::<u32>()` の `::<>` の書き方 | UNIT 03 |
| trait オブジェクト | `dyn Trait`。その trait を実装した何かの型の値。型は実行時まで分からない | UNIT 04 |
| 動的ディスパッチ | 呼び出す関数を、実行時に vtable を引いて決めること | UNIT 04 |
| vtable | ブイテーブル。型ごとの「メソッドの実体の一覧表」 | UNIT 04 |
| ファットポインタ | 場所に加えて追加の情報（vtable の場所や長さ）を持つポインタ | UNIT 04 |
| dyn compatible | `dyn` で使える条件を満たす trait。旧称「オブジェクト安全」 | UNIT 04 |
| `Send` / `Sync` | スレッドをまたいで送ってよい／同時に共有してよい、を表す trait | UNIT 04 |
| `std::error::Error` | エラーを表す型の標準の trait。`source()` で原因をたどれる | UNIT 05 |
| thiserror | エラー型の `Display` と `Error` を derive で作るクレート（2.0.21） | UNIT 05 |
| anyhow | どんなエラーでも入れられ、文脈を積み重ねて報告するクレート（1.0.104） | UNIT 05 |
| 文脈（context） | 「何をしようとして失敗したか」をエラーに足した説明 | UNIT 05 |
| ステータスコード | HTTP の返事に付ける3桁の番号（400・404・409・500 など） | UNIT 05 |
| 一意制約 | UNIQUE。DB の列に同じ値を2つ入れてはいけない決まり | UNIT 05 |
| `#[must_use]` | 戻り値を捨てたら警告する印。`Result` にも付いている | UNIT 05・10 |
| package | `Cargo.toml` 1つで管理される単位。lib と bin の crate を持てる | UNIT 06 |
| crate | クレート。コンパイルの単位（ライブラリ・実行ファイル） | UNIT 02・06 |
| module | モジュール。crate の中の名前の区画。公開範囲の境界 | UNIT 06 |
| `pub(crate)` / `pub(super)` | crate の中まで／親の module まで、に公開範囲を限る印 | UNIT 06 |
| 再公開 | `pub use`。別の場所の項目を、ここにもあるように見せること | UNIT 06 |
| workspace | 複数の package をまとめ、`Cargo.lock` と `target` を共有する仕組み | UNIT 06 |
| SemVer | セムバー。版を「大.中.小」で表し、互換性の約束を決める方式 | UNIT 07 |
| `Cargo.lock` | 実際に使う依存の版とチェックサムの記録。アプリでは Git に保存する | UNIT 07 |
| `--locked` | `Cargo.lock` の書き換えが必要ならエラーで止めるオプション | UNIT 07 |
| features | クレートの機能の ON/OFF。`#[cfg(feature = "...")]` でコードを出し入れする | UNIT 07 |
| 任意の依存 | `optional = true`。feature で ON にしたときだけ使う依存 | UNIT 07 |
| `rust-version` | その package に必要な Rust の最小の版（本書は `"1.98"`） | UNIT 07 |
| Serde | サーデ。値と JSON などの形式を相互に変換する仕組み（serde 1.0.229） | UNIT 08 |
| シリアライズ / デシリアライズ | 値 → 文字列（JSON など）／文字列 → 値 の変換 | UNIT 08 |
| DTO | Data Transfer Object。外とのやり取りのためだけの入れ物の型 | UNIT 08 |
| 一括代入の脆弱性 | mass assignment。送られた項目をそのまま使い、`owner_id` などを書き換えられる弱点 | UNIT 08 |
| 環境変数 | OS がプログラムに渡す「名前＝値」の組。値はいつも文字列 | UNIT 09 |
| 秘密（シークレット） | パスワード・トークンなど、知られると悪用される値 | UNIT 09 |
| `.env` / `.env.example` | 開発用の設定ファイル（Git に入れない）／その見本（Git に入れる） | UNIT 09 |
| 単体テスト / 結合テスト | 部品ごとのテスト／ライブラリを外から使う `tests/` のテスト | UNIT 10 |
| doctest | ドキュメントの `# Examples` のコードを、テストとして実行する仕組み | UNIT 10 |
| rustdoc | `///`・`//!` のコメントからドキュメントを作る仕組み（`cargo doc`） | UNIT 10 |
| 境界値テスト | 80 文字と 81 文字のように、境目の値を確かめるテスト | UNIT 10 |
| 表駆動テスト | 入力と期待する結果を表にして、ループで確かめるテスト | UNIT 10 |
| 状態機械 | 「今の状態」と「操作」で「次の状態」が決まる仕組み | UNIT 10 |
| テストピラミッド | 速い単体テストを多く、遅いテストを少なくする考え方 | UNIT 10 |
