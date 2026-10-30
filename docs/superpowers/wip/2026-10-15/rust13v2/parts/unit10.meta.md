title: テスト・Docker・Fly.io・CI
---checklist
- ☐ tests/common/mod.rs の道具（一時フォルダの DB・テスト用のアセット・Router::handle）と、repository.rs・http.rs が確かめていることを説明できる
- ☐ CI と同じ3つの手順（fmt --check・clippy -D warnings・test --locked）を手元で動かし、41 個のテストの結果と、--locked の止め方を読める
- ☐ Dockerfile の2つの段と .dockerignore を説明し、本物の Docker でビルド・起動・/health を確かめられる（キャッシュの効き方も言える）
- ☐ assets のコピー忘れ・ポートの食い違い・HOST=127.0.0.1 が、それぞれどう見えるかを言える
- ☐ fly.toml の internal_port・force_https・/health のチェックと、コンテナの中の DB が作り直しで消えることを説明できる
---glossary
| CI | コードを送るたびに、整形・静的な検査・テスト・ビルドを自動で動かすしくみ |
| `tempfile::tempdir()` | 一時フォルダを作る。返す `TempDir` が捨てられると、フォルダごと消える |
| `Router::handle` | ポートを開かずに、リクエストをルーターに直接渡して、応答を受け取る |
| multi-stage ビルド | Dockerfile の中で段を分け、大きな builder の段で作った物を、小さな実行用の段にだけコピーする作り方 |
| `.dockerignore` | docker build のときに、Docker へ送らないファイルの一覧 |
| `CACHED` | docker build の段の結果を、前のビルドから使い回したという印 |
| volume | コンテナの外に置く、データの置き場。コンテナを作り直しても残る |
| `0.0.0.0` | 「どのネットワークの入口からでも受け付ける」という待ち受けのアドレス。コンテナの外から届くために要る |
| `--locked` | `Cargo.lock` のとおりにビルドし、勝手に版を変えない、という cargo の指定 |
