## この巻のまとめ

この版に収録した UNIT 01 で、次のことができるようになったかを確かめましょう。できたものに印を付けてください。

**UNIT 01: release ビルドと再現性**

- ☐ debug と release の違いを、大きさ（約 21.7 MB と 11.8 MB）・速さ（ハッシュ化 約 400 ms と 20 ms）・整数のあふれ（panic と折り返し）で説明できる
- ☐ `cargo build --release --locked` → `topcoat asset bundle --release` の順で本番用の実行ファイルを作り、起動して `GET /health` で確かめられる
- ☐ 実行ファイルは「自分の隣の `assets\`」を、DB は「今いるフォルダ」（または `DATABASE_URL`）を使うことを説明できる
- ☐ `no asset bundle at ...` と `os error 10048` のエラーを読み、直し方を言える
- ☐ 同じソースから作った実行ファイルの中身が違う理由（リンクした時刻・識別番号）と、作る場所が中身に入り込むこと（ユーザーフォルダの場所・`asset!` のクレートの場所）を、実際の比べ方で説明できる
- ☐ release でも整数のあふれに気づけるよう、大きい型・`checked_*`・`overflow-checks = true` のどれかを選べる
- ☐ コミット・Rust の版・`Cargo.lock` と実行ファイルの指紋を、`release-record.txt` に記録できる

> **お知らせ:** この版には UNIT 01 を収録しています。UNIT 02（multi-stage の Dockerfile）・UNIT 03（アセットの束を本番用に作る）・UNIT 04（fly launch・fly.toml・ポート）・UNIT 05（secrets・設定・入れ替え）・UNIT 06（health check と起動の順番）・UNIT 07（migration を出す順番と release command）・UNIT 08（deploy・ログ・動作の確かめ・独自ドメイン）・UNIT 09（台数・地域・費用・Turso との距離）・UNIT 10（rollback・backup・障害からの復旧・更新）は準備中で、次の更新から順に追加します。それまでの間、公開の全体の流れは、RustNote の `README.md` の「公開する（Fly.io + Turso）の流れ」と、`Dockerfile`・`fly.toml` の先頭のコメントで確かめられます。

## 用語ミニ辞典

この巻（UNIT 01）に出てきた用語です。

### UNIT 01 の用語

| 用語 | 意味 |
|---|---|
| debug ビルド | `cargo build` で作る試し用の実行ファイル。最適化なし、整数のあふれを確かめる。`target\debug` にできる |
| release ビルド | `cargo build --release` で作る本番用の実行ファイル。最適化あり、既定では整数のあふれを確かめない。`target\release` にできる |
| 最適化 | 同じ動きのまま、速く小さくなるように、コンパイラーがプログラムを作り直すこと |
| `cfg!(debug_assertions)` | debug ビルドなら `true`、release ビルドなら `false` になる式 |
| 整数のあふれ（オーバーフロー） | 計算の答えが、型で表せる範囲を超えること。debug は panic、release は既定で折り返す |
| `checked_add` / `checked_mul` | あふれたら `None` を返す計算。あふれたことをプログラムで扱える |
| `overflow-checks` | `[profile.release]` に書くと、release でもあふれを確かめる設定 |
| アセットの束 | `topcoat asset bundle` が作る、CSS などのファイルと目録（`manifest.toml`）。実行ファイルの隣の `assets\` に置く |
| 再現性 | 同じソース・同じ道具・同じ部品から、いつ誰が作っても同じ物ができること |
| SHA-256 の指紋 | ファイルの中身から計算した長い値。1 バイトでも違えば、まったく別の値になる。`Get-FileHash` で表示できる |
| `/Brepro` | Windows のリンカーに、時刻の代わりに中身から計算した値を使わせる指定 |
| `--remap-path-prefix` | 実行ファイルに入れるフォルダの場所の文字を、別の文字に置き換える rustc の指定 |
| `RUSTFLAGS` | cargo が rustc に渡す追加の指定を書く環境変数。変えると全部のクレートが作り直しになる |
| os error 10048 | Windows の「そのアドレスとポートは、もう使われている」エラー |
