title: アセットの束を本番用に作る
---checklist
- ☐ topcoat asset bundle --release が、中で cargo build をして、実行ファイルの asset! の印から束を作ることと、先に --locked でビルドしておく理由を言える
- ☐ manifest.toml の id・file・hash・content_type と、指紋付きの名前の決まり方を説明できる
- ☐ 束が 1 年間 immutable のキャッシュと gzip・br の圧縮で配られることを、本物の応答で確かめられる
- ☐ CSS を変えて束を作り直し忘れると、エラーも無く古い CSS のままになることを示せる
- ☐ topcoat CLI と topcoat クレートの版がずれたときの警告を読み、CLI をそろえられる
---glossary
| アセットの束 | CSS などを指紋付きの名前にして、目録と一緒にまとめたフォルダ。RustNote では実行ファイルの隣の assets/ |
| `manifest.toml` | 束の目録。asset! の id と、束の中のファイルの名前・指紋・種類の対応を書く |
| 指紋（ハッシュ） | ファイルの中身から計算した値。中身が1文字でも変われば、別の値になる |
| 圧縮 | 同じ情報のまま、送る中身を小さくすること。HTTP では gzip と br（Brotli）がよく使われる |
| `Accept-Encoding` | ブラウザが、読める圧縮の方式を伝える見出し |
| `vary` | 同じ URL でも、ある見出し（ここでは Accept-Encoding）によって中身が違う、という印 |
