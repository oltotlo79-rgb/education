title: アセット・CSS・束
---checklist
- ☐ `asset!("assets/site.css")` で CSS を宣言し、`view!` の `href` に入れて、束の中の URL にできる
- ☐ `topcoat asset bundle --bin …` で束（ハッシュ付きのファイルと `manifest.toml`）を作り、`AssetBundle::load()` と `.assets(…)` で配れる
- ☐ ファイル名のハッシュが中身から決まることと、`cache-control: …, immutable` で長く覚えてもらえる理由を説明できる
- ☐ `topcoat dev` で、保存するたびにビルド・束・起動をやり直す流れと、`topcoat::dev::script()` の役目を言える
- ☐ 束が無い・パスの打ち間違い・束が古いときのエラーが、それぞれいつ起きるかを説明できる
- ☐ 実行ファイルと束を、いつも同じビルド（同じ `--bin`・同じ `--release`）で作る理由を言える
---glossary
| アセット | CSS・画像・フォントなど、プログラムと一緒に配る、中身が決まったファイル |
| `asset!` | 配るファイルを宣言するマクロ。実行ファイルには番号（ID）だけが入る |
| 束（バンドル） | `topcoat asset bundle` が作る、ハッシュ付きの名前のファイルと目録のまとまり |
| `manifest.toml` | 束の目録。宣言の ID と、ファイル名・ハッシュ・種類の対応を書く |
| `AssetBundle::load()` | 実行ファイルの隣の `assets/` から、束を読み込む関数 |
| キャッシュ | 受け取ったファイルをブラウザが手元に取っておき、次は取りに行かずに使うこと |
| `cache-control` | どれだけの間、覚えてよいかを伝えるヘッダー。`immutable` は「中身は変わらない」 |
| `topcoat dev` | ファイルの変更を見張り、ビルド・束・起動を自動でやり直す開発用のサーバー |
| `topcoat asset clean` | 作った束を消すコマンド |
| `topcoat asset list` | 束に入る予定のファイルを表示するコマンド |
