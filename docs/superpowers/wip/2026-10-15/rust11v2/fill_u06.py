"""unit06.src.md の @@MUTANTS@@ と @@MUTANTS_Q3@@ に、cargo-mutants の結果の本文を入れる。"""
from pathlib import Path

p = Path(__file__).resolve().parent / "parts" / "unit06.src.md"
s = p.read_text(encoding="utf-8")

MUTANTS = """cargo-mutants 27.1.0 は、Windows（ARM64）では組み立てられなかったので（エラー2）、WSL の Ubuntu（aarch64）に入れて動かしました。壊した版ごとにテストを組み立て直すので、比べたいテスト（`tests/http.rs` と `tests/u06_attacks.rs`、あとで `tests/repository.rs`）だけを入れたコピーで動かしました。

```bash
# WSL（Ubuntu）の中で
cargo install cargo-mutants --version 27.1.0 --locked
cargo mutants --list --file src/auth.rs
```

```text
@@OUT:u06_mutants_list@@
```

- `src/auth.rs` の関数を、1つずつ「いつも決まった値を返すだけ」に書き換えた、13 個の版が作られます。たとえば `verify_password` を `Ok(true)`（どんなパスワードでも合っていることにする）にした版や、`log_out` を `Ok(())`（何もしないでログアウトしたことにする）にした版です。

RustNote の `tests/http.rs` だけで、13 個を確かめます（`--` のうしろは、`cargo test` に渡す引数です）。

```bash
cargo mutants --file src/auth.rs --timeout 300 -- --test http
```

```text
@@OUT:u06_mutants_http@@
```

```text
@@OUT:u06_mutants_http_unviable@@
```

- 9 個は倒し（caught）、4 個は **unviable**（組み立てられない版）でした。4 個とも「`Default::default()` の `User` を返す」版で、RustNote の `User` に `Default`（決まった初めの値）が無いので、コンパイルできません。組み立てられない版は、テストを動かすまでもなく、ありえない壊し方です。
- **見逃し（missed）は 0 個**でした。`tests/http.rs` は、`auth.rs` の関数を丸ごと何もしない版にすると、どれでも気づきます。`u06_attacks` を足しても同じでした。

```text
@@OUT:u06_mutants_both@@
```

検索や削除の SQL がある `src/repository/notes.rs` でも、HTTP のテスト（`tests/http.rs` と `u06_attacks`）で試しました。

```bash
cargo mutants --file src/repository/notes.rs --timeout 300 -- --test http --test u06_attacks
```

```text
@@OUT:u06_mut_notes_http@@
```

```text
@@OUT:u06_mut_notes_http_caught@@
```

- 12 個のうち、組み立てられる 6 個を全部倒しました（ほかの 6 個は、`Note` や `NotePage` に `Default` が無いので unviable）。削除で「消えた行の数が 0 か」を確かめる `==` を `!=` にした版や、`escape_like` を空の文字列を返す版も、倒せました。
- リポジトリの結合テストとユニットテスト（`--lib --test repository`）も足して動かすと、次のとおりでした。

```text
@@OUT:u06_mut_notes_all@@
```

- **大事な点:** cargo-mutants は、Rust のコード（関数の中身・`==` などの記号）を壊しますが、**SQL の文字列の中は壊しません**。例6-2で、`u06_attacks` も `tests/http.rs` も見逃した「検索の条件のかっこを外す」は、SQL の文字列の中の間違いなので、cargo-mutants の一覧に、もともと入っていません。「見逃し 0」は、「cargo-mutants が作った壊し方は、全部倒した」という意味で、「守りに抜けが無い」という意味ではありません。SQL の守りは、例6-2のように、人が壊し方を考えて確かめます。"""

MUTANTS_Q3 = """- 今回、cargo-mutants の見逃しは、`src/auth.rs` でも `src/repository/notes.rs` でも 0 個でした。なので、ここでは逆に、「例6-2で人が作った5つの壊し方のうち、cargo-mutants の一覧に入っているのはどれか」を考えます。
- `log_out` を `Ok(())` にする版は、「ログアウトで何もしない」ので、例6-2の「ログアウトで DB の記録を消さない」に近い壊し方です（cargo-mutants の版は、クッキーも消しません）。例6-2の版も、cargo-mutants の版も、`tests/http.rs` だけで倒せました。
- 残りの4つ（切り替え・削除の持ち主の条件、検索のかっこ、Origin の検査）は、SQL の文字列の中か、ルーターの組み立て（`src/routes.rs`）の1行で、cargo-mutants は作りません。だから、cargo-mutants で「見逃し 0」でも、例6-2のような、人が考えた壊し方の表は残しておきます。2つは、置き換えではなく、組み合わせて使います。"""

for marker, text in [("@@MUTANTS@@", MUTANTS), ("@@MUTANTS_Q3@@", MUTANTS_Q3)]:
    assert s.count(marker) == 1, marker
    s = s.replace(marker, text)
s = s.replace(
    "**問3:** 例6-3で cargo-mutants が見逃した壊し方について、それに気づくテストを考えてください。",
    "**問3:** 例6-2で人が作った5つの壊し方のうち、cargo-mutants が作る壊し方の一覧（例6-3）に入っているのは、どれでしょうか。",
)
p.write_text(s, encoding="utf-8", newline="\n")
print("ok")
