### 7-6. 保存すると「violates ... constraint」というエラーが出る

**エラーメッセージの例:**

```text
null value in column "author" of relation "books" violates not-null constraint
```

```text
new row for relation "books" violates check constraint "books_status_check"
```

**原因:** テーブルに付けた **制約**（その列に入れてよい値のルール）に合わない値を送っています。エラーコード（`error.code`）で種類が分かります。

| エラーコード | 意味 | この章でよくある原因 |
|---|---|---|
| `23502` | not-null 制約違反（空を許さない列に `null` を送った） | `BookForm` は空欄を `null` に変換して送るため、著者を空のまま保存すると、`author` 列が `NOT NULL` のテーブルではこのエラーになる |
| `23514` | check 制約違反（決められた候補以外の値を送った） | `status` 列の `CHECK (status IN (...))` に書かれた値と、フォームの `<option value="...">` の綴りが違う |

**対処法:**

1. テーブルにどんな制約が付いているかを、Supabase の SQL Editor で確認します。

> **▼ このコードがやること（先に日本語で）:** PostgreSQL が内部に持っている「制約の一覧表」（`pg_constraint`）から、`books` テーブルの分だけを取り出し、制約の名前と中身を人が読める形で表示します。

```sql
-- books テーブルに付いている制約の名前と中身を一覧表示する
SELECT conname, pg_get_constraintdef(oid)
FROM pg_constraint
WHERE conrelid = 'books'::regclass;
```

- `conname` は制約の名前（例: `books_status_check`）、`pg_get_constraintdef(oid)` は制約の中身（例: `CHECK ((status = ANY (ARRAY['reading'::text, ...])))`）です。
- `NOT NULL` はこの一覧に出ないことがあります。その場合は Table Editor で、その列が null を許可しているかどうかを確認してください。

2. **check 制約違反（23514）** なら、フォームの `<option value="...">` の綴りを、制約に書かれている値とそろえます。第5章で作ったテーブル定義の `status` の候補と、この章で使っているステータスの値（`unread` / `reading` / `finished`）が一致しているかを確認してください。
3. **not-null 制約違反（23502）** なら、フォーム側でもその欄を必須にします（`<input required>` を付け、送信前に空ならエラーメッセージを出す）。データベースのルールとフォームのルールをそろえるのが根本的な解決です。
