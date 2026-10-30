title: benchmark・profile・メモリの使い方
---checklist
- ☐ criterion で関数の速さを測り、time の3つの数と外れ値を読める（harness = false・black_box の理由も言える）
- ☐ 遅いと感じたら部分ごとに測り、ログインの大部分が Argon2 の照合であることを、数で示せる
- ☐ debug と release で速さが何倍も違うことを確かめ、速さは release で測る理由を言える
- ☐ 数える global allocator で、借りた回数とピークを測れる
- ☐ Argon2 が1回 19 MiB 使うことから、同時のログインの数とメモリの関係を計算できる
---glossary
| ベンチマーク | 同じ処理を何回もくり返して、1回あたりの時間を測ること |
| criterion | 統計を使って速さを測る、Rust のベンチマークのクレート |
| `black_box` | 値を、コンパイラーに「分からないもの」と思わせ、計算を省かせない印 |
| プロファイル | 時間やメモリが、どこで使われているかを調べること |
| release ビルド | 時間をかけて最適化した、速いプログラムを作る組み立て（`--release`） |
| global allocator | プログラム全体の、メモリを借りる・返す係。`#[global_allocator]` で替えられる |
| ピーク | 同時に借りていたメモリの量の、いちばん多い所 |
