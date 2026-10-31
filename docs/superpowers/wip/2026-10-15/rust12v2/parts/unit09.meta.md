title: 台数・地域・費用・Turso との距離
---checklist
- ☐ DB までの遅れを入れた実験で、1回の表示の時間が、往復の回数の分だけ増えることを示せる
- ☐ RustNote が DB に送ったリクエストを数えて、読む SQL 1つで describe と cursor の2回になることを確かめられる
- ☐ マシンと DB を、利用者の近くの同じ地域（nrt・aws-ap-northeast-1）に置く理由を説明できる
- ☐ 台数（止める／動かす・いつも1台・2台以上）と費用（Fly.io・Turso の公式の料金）の釣り合いを考えられる
---glossary
| レイテンシ | 1回の往復にかかる時間。距離が遠いほど長くなる |
| toxiproxy | 2つのプログラムの間にはさんで、通信をわざと遅くしたり切ったりできる道具 |
| Hrana | libSQL のサーバーと HTTP で話すときの決まり。`/v3/pipeline`・`/v3/cursor` に JSON を送る |
| リージョン | マシンや DB を置く地域。Fly.io の `nrt`、Turso の `aws-ap-northeast-1` など |
| `min_machines_running` | いつも動かしておくマシンの最小の数。0 なら、使われていない間は全部止まる |
| 読んだ行・書いた行 | Turso の費用を決める量。SQL が読んだ行の数と、書いた行の数 |
