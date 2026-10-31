title: health check と起動の順番
---checklist
- ☐ RustNote の /health が DB に SELECT 1 をして 200・503 を返すことを、sqld を止めたり動かしたりして確かめられる
- ☐ Fly.io のヘルスチェックが失敗したマシンには振り分けないが、動かし直しはしないことを、公式のドキュメントにそって言える
- ☐ DB が無いまま起動すると、マイグレーションで止まって終了コード 1 になり、restart policy（on-fail）で動かし直されることを説明できる
- ☐ 起動から 200 までの時間を測って、grace_period を決められる
---glossary
| ヘルスチェック | 動いているアプリに決まった間をおいてリクエストを送り、正しく答えられるかを確かめること |
| sqld | libSQL のサーバー。Turso と同じ形（Hrana）で、ネットごしに SQL を受け付ける。手元の Docker で動かせる |
| Hrana | libSQL のクライアントとサーバーの間の、HTTP などでの話し方の決まり |
| restart policy | マシン（コンテナ）のプロセスが終わったとき、動かし直すかの決まり。Fly.io の既定は on-fail |
| `grace_period` | 起動してから、最初のヘルスチェックまで待つ時間 |
| 503 | Service Unavailable。今は仕事ができない、という状態コード |
