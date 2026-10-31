title: multi-stage の Dockerfile
---checklist
- ☐ RustNote の Dockerfile の builder の段と実行用の段が、それぞれ何を入れ、何を入れないかを、docker history の結果で説明できる
- ☐ 1段だけの Dockerfile と比べて、大きさ（3.98GB と 170MB）・中の道具・動かす利用者（root と app）の違いを言える
- ☐ Docker のキャッシュが段ごとに効くことと、ソースを1行変えると cargo build が全部やり直しになる理由を説明できる
- ☐ キャッシュのマウントで target/ を残し、やり直しを短くできる
- ☐ 実行用の段の Linux のちがい（alpine で no such file or directory）・COPY --from の場所・/data の持ち主のまちがいが、それぞれどこで、どう止まるかを言える
---glossary
| multi-stage ビルド | 1つの Dockerfile に FROM を2つ以上書き、前の段で作った物から要る物だけを COPY --from で運ぶ作り方 |
| レイヤー | イメージを作る1つ1つの手順の結果。同じ手順・同じ材料なら、前のビルドの結果（キャッシュ）を使い回す |
| `CACHED` | docker build の記録で、その手順の結果を前のビルドから使い回したという印 |
| glibc | Linux の C の基本のライブラリ。新しい glibc で作った実行ファイルは、古い glibc の上では動かないことがある |
| `docker history` | イメージのレイヤーと、それぞれの大きさ・作った手順を並べて見るコマンド |
| キャッシュのマウント | `RUN --mount=type=cache,…` で、ビルドをまたいでフォルダ（target/ など）を取っておくしくみ。イメージには残らない |
| 最小権限 | プログラムに、要る分だけの権限しか与えない考え方。RustNote は app で動き、書けるのは /data だけ |
