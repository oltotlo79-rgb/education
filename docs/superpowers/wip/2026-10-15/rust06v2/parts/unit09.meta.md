title: runtime・signal・procedure・shard
---checklist
- ☐ `topcoat::runtime::script()`・`.runtime()`・アセットの束で runtime を有効にし、`signal`・`@click`・`:hidden` で、読み直さずに画面を変えられる
- ☐ `$( … )` の式が、サーバーで最初の値を作り、同じ式が JavaScript になってブラウザで計算し直されることを説明できる
- ☐ `#[procedure]` の呼び出しが `POST /api/…` の通信であることを、ブラウザの通信の記録と curl で確かめられる
- ☐ ページの確かめが procedure（と shard）の入口を守らない理由と、中で値と権限を確かめる書き方を言える
- ☐ OriginPolicy が止める要求（ほかのサイトのページから）と、止めない要求（直接の要求）を区別できる
- ☐ 構造体を `signal` に入れたときの `E0277`（`#[record]` で直す）・`.discover()` 忘れの 404・あふれる値の 500 を読める
---glossary
| runtime | Topcoat がページに入れる、ブラウザの中で動くプログラム。`$( … )` の式をブラウザでも動かす（0.11 でも実験的） |
| signal | ブラウザが持つ状態。値が変わると、使っている式がブラウザで計算し直される |
| `$( … )` | runtime の式。サーバーで最初の値を作り、ブラウザでも同じ計算をする |
| `@click` | クリックされたときに動く、ブラウザの中の処理（イベントハンドラー） |
| `:hidden` などの結び付け属性 | signal が変わるたびに、ブラウザが属性を付け直す属性 |
| procedure | ブラウザから `POST` で呼ばれる、サーバーの async 関数。`#[procedure("/api/…")]` で URL を決められる |
| shard | 引数や signal が変わると、サーバーで描き直される部品。自分の入口（URL）を持つ |
| `#[record]` | 自分の構造体を、signal や procedure で使えるようにする印 |
| OriginPolicy | ほかのサイトのページから送られた、状態を変える要求を 403 で止める Topcoat の仕組み |
| `checked_mul` | 掛け算があふれるときに `None` を返すメソッド。あふれる値を断るのに使う |
