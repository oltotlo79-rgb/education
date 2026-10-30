# 全巻を完成させる計画（2026-10-30 開始）

ユーザーの指示: 「全て完了まで進めて / バージョンはあげる / 全て最新に合わせる」。サブエージェントは使わない。仕上がった単位で main へ push（publish.sh）。Fly.io・Turso のアカウントは使わない。

## 版（2026-10-30 に確認した最新）
- Rust 1.99.1（rust-toolchain.toml）、rustup 1.29.2
- topcoat =0.11.0（rust-version 1.99、topcoat-cli 0.11.0 をインストール済み: ~/.cargo/bin/topcoat）
- tokio =1.54.1、libsql 0.10.0、argon2 0.6.0、serde 1.0.229、thiserror 2.0.21、anyhow 1.0.104、async-trait 0.1.92、dotenvy 0.15.7、tempfile 3.27.0、serde_json 1.0.154（いずれも最新）
- CI: ubuntu-26.04、actions/checkout@v7、Swatinem/rust-cache@v2（v2.9.3）
- Docker: rust:1.99.1-trixie / debian:trixie-slim がある（今は bookworm のまま。第12巻で docker build して確かめてから trixie へ）
- Docker Desktop: プロセスはあるが `docker version` が 40 秒で応答なし（第12巻の前に再起動が要る）

## 済み
- [x] RustNote を 0.11 へ更新して main へ（fbe63e4）。コード変更なし、41 テスト通過、asset bundle・起動確認

## 順番（各巻: まず既存 UNIT の版の表記と出力を新しい版で取り直す → 残りの UNIT を書く → 1単元ごとに push）
1. 第06巻（ファイル名 0_10 → 0_11 に変える。UNIT 03〜10）
2. 第09巻（UNIT 05〜10）
3. 第10巻（UNIT 03〜10）
4. 第08巻（UNIT 03〜10）
5. 第11巻（UNIT 03〜10）
6. 第13巻（UNIT 01〜02 の Cargo.toml/lock 一覧を更新。UNIT 03〜10）
7. 第12巻（UNIT 02〜10。Docker が要る。Fly.io は実行しない）
8. 第01巻（UNIT 04〜10。flyctl・Turso CLI は入れて版の確認まで。ログインはしない）
9. 完成済みの 00・02・03・04・05・07 の版の表記と、版に依存する出力を取り直す

## 道具
- $S/capx.sh（VOL=… WORKDIR=… で実行・マスク・保存）、$S/maskx.py
- 各巻の fill.py / assemble.py（範囲を広げる）
- check: docs/superpowers/wip/2026-10-15/ref/check_md.py、PDF: _build_pdfs.py、描画: $S/rust05/render.py
- 公開: docs/superpowers/wip/2026-10-15/ref/publish.sh "件名" "本文" パス...
- 注意: バックスラッシュを含む置換は Write で書いた .py か chr(92)。re.sub の置換文字列は lambda で。SVG 内の \ は Consolas の tspan に。
