title: 見た目
---checklist
- ☐ style.css の組み立て（色の変数・全体・ヘッダー・本文の枠・ボタンとフォーム・お知らせ・一覧・詳細）を、view! のクラスの名前と結びつけて説明できる
- ☐ 本物の Chrome で、幅ごとの枠の大きさ・横のはみ出し・CSS が効いた結果を測れ、375px と 320px のヘッダーのはみ出しを flex-wrap で直せる
- ☐ 色の組のコントラスト比を計算し、AA（4.5 : 1）にいちばん近い組を言える
- ☐ asset! と topcoat asset bundle の、印付きの URL と 1 年間の immutable なキャッシュのしくみを説明できる
- ☐ CSS のまちがい（ファイルの名前・閉じ括弧・変数の名前）が cargo build で見つからないことと、それぞれの見つけ方を言える
---glossary
| CSS の変数 | `--color-text: #0f172a;` のように決め、`var(--color-text)` で使う値。RustNote は色を `:root` にまとめる |
| `box-sizing: border-box` | 幅に、内側の余白と線を含めて数える指定 |
| flex | `display: flex` で、子の要素を横（または縦）に並べるしくみ。`flex-wrap: wrap` で折り返す |
| `white-space: pre-wrap` | 改行と空白を書いたとおりに表示し、端では折り返す指定 |
| `overflow-wrap: anywhere` | 区切りの無い長い文字の並びも、どこででも折り返す指定 |
| コントラスト比 | 文字と背景の明るさの比。WCAG の AA では、ふつうの文字は 4.5 : 1 以上 |
| `immutable` | `cache-control` の値。中身が変わらないので、期限まで確かめ直さなくてよい、という意味 |
| `getComputedStyle` | ブラウザで、CSS が効いた結果の値を読む JavaScript の関数 |
