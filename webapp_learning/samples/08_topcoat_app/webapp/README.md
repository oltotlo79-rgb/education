# 依頼・承認アプリ（Rust＋Topcoat 版の練習）

06 の Django＋React 版と同じことをする、Rust＋Topcoat 版です。説明は `webapp_learning/08_…` の資料にあります。

## 動かし方（開発のパソコン）

1. PostgreSQL に、練習用のユーザーとデータベースを作る（資料の 2 章）
2. `.env.example` を `.env` という名前でコピーし、`DATABASE_URL` のパスワードを直す（`.env` は Git に入れない）
3. 次を順に動かす

```powershell
topcoat asset bundle          # CSS と JavaScript を束ねる（target\debug\assets）
cargo run -- seed-demo        # 練習用の社員 5 人（パスワードは全員 practice-pass-2026）
cargo run                     # http://localhost:3000 で動く
cargo test                    # テスト（DATABASE_URL のユーザーに、データベースを作る権限が要る）
```

メールは送らずに `sent_mail\` フォルダへ `.eml` のファイルとして書きます。添付ファイルは `files\` に置きます。

## サーバーへ持ち込む物

```powershell
cargo build --release
topcoat asset bundle --release
# target\release\webapp.exe と target\release\assets\ を、必ず一緒に持ち込む（onprem の 04）
```
