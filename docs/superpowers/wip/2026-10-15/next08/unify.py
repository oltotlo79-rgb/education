import io

TARGET = r"C:/Users/oltot/Documents/git-projects/def/next/08-crud-edit-delete.md"
s = io.open(TARGET, encoding="utf-8").read()


def rep(old, new, count=1):
    global s
    n = s.count(old)
    assert n == count, (n, count, old[:80])
    s = s.replace(old, new)


# ---------------------------------------------------------------
# 1. ステータス値の統一（want_to_read / reading / completed）
# ---------------------------------------------------------------
rep('''// データベースに保存されている英語のステータス値を
// 日本語の表示ラベルに変換します。
// 例: "unread" → "未読"
// -------------------------------------------------''',
'''// データベースに保存されている英語のステータス値を
// 日本語の表示ラベルに変換します。
// 例: "want_to_read" → "読みたい"
// 値とラベルは第7章の StatusBadge と同じにそろえています。
// -------------------------------------------------''')
rep('''    // DB の値 "unread" を「未読」に対応付け
    unread: "未読",
    // DB の値 "reading" を「読書中」に対応付け
    reading: "読書中",
    // DB の値 "finished" を「読了」に対応付け
    finished: "読了",''',
'''    // DB の値 "want_to_read" を「読みたい」に対応付け
    want_to_read: "読みたい",
    // DB の値 "reading" を「読書中」に対応付け
    reading: "読書中",
    // DB の値 "completed" を「読了」に対応付け
    completed: "読了",''')
rep('''    // 未読のとき
    case "unread":
      // 背景はグレー、文字は濃いグレー
      return "bg-gray-100 text-gray-800";''',
'''    // 読みたい（まだ読んでいない）とき
    case "want_to_read":
      // 背景は薄い黄、文字は濃い黄（第7章の StatusBadge と同じ色）
      return "bg-yellow-100 text-yellow-800";''')
rep('''    // 読了のとき
    case "finished":''', '''    // 読了のとき
    case "completed":''')
# 一覧ページ側のヘルパー
rep('''    unread: "未読",
    reading: "読書中",
    finished: "読了",''', '''    want_to_read: "読みたい",
    reading: "読書中",
    completed: "読了",''')
rep('''    case "unread":
      return "bg-gray-100 text-gray-800";
    case "reading":
      return "bg-blue-100 text-blue-800";
    case "finished":''', '''    case "want_to_read":
      return "bg-yellow-100 text-yellow-800";
    case "reading":
      return "bg-blue-100 text-blue-800";
    case "completed":''')

# BookForm の型: 第7章の BookStatus を使う
rep('''// ブラウザ向けの Supabase クライアント
import { createClient } from "@/lib/supabase/client";

// -------------------------------------------------
// 型定義
// -------------------------------------------------
// BookFormData: フォームで扱うデータの型''',
'''// ブラウザ向けの Supabase クライアント
import { createClient } from "@/lib/supabase/client";
// 第7章の StatusBadge で定義したステータスの型（型だけを読み込む）
import { type BookStatus } from "./StatusBadge";

// -------------------------------------------------
// 型定義
// -------------------------------------------------
// BookFormData: フォームで扱うデータの型''')
rep('''  // ステータス（"unread" / "reading" / "finished"）
  status: string;''', '''  // ステータス（"want_to_read" / "reading" / "completed" のどれか）
  status: BookStatus;''')
rep('''  // デフォルトは「未読」
  status: "unread",''', '''  // デフォルトは「読みたい」（第7章と同じ）
  status: "want_to_read",''')
rep('<option value="unread">未読</option>', '<option value="want_to_read">読みたい</option>', 2)
rep('<option value="finished">読了</option>', '<option value="completed">読了</option>', 2)

# 練習用コード form-state.ts
rep('const next = applyChange(initialData, "status", "finished");',
    'const next = applyChange(initialData, "status", "completed");')
rep('sameObject.status = "finished";', 'sameObject.status = "completed";', 2)
rep('変更後の status: finished', '変更後の status: completed')
rep('`name` が `"status"` なら `status: "finished"` と書いたのと同じです。',
    '`name` が `"status"` なら `status: "completed"` と書いたのと同じです。')

# 編集ページと convert.ts の既定値
rep('status: book.status ?? "unread",', 'status: book.status ?? "want_to_read",', 3)
rep('`status` は空だと困るので `?? "unread"`', '`status` は空だと困るので `?? "want_to_read"`')

# 図8-10・図8-11・delete-where.ts
rep('<text x="60" y="208" font-size="13" fill="#475569">ステータス 未読</text>',
    '<text x="60" y="208" font-size="13" fill="#475569">ステータス 読みたい</text>')
rep('font-family="Consolas, monospace">unread</text>', 'font-family="Consolas, monospace">want_to_read</text>', 2)
rep('font-family="Consolas, monospace">finished</text>', 'font-family="Consolas, monospace">completed</text>', 2)
rep('{ id: "b2", title: "TypeScript入門", status: "unread" },', '{ id: "b2", title: "TypeScript入門", status: "want_to_read" },')
rep('{ id: "c3", title: "Next.js実践", status: "finished" },', '{ id: "c3", title: "Next.js実践", status: "completed" },')

# 本文中のラベル
rep('例えば「未読」フィルタをかけた状態で「評価（高い順）」にソートすると、「未読かつ評価の高い順」で表示されます。',
    '例えば「読みたい」フィルタをかけた状態で「評価（高い順）」にソートすると、「読みたい かつ 評価の高い順」で表示されます。')
rep('ステータス: `未読`, 評価: `3`', 'ステータス: `読みたい`, 評価: `3`')
rep('ステータスフィルタで「未読」を選択します', 'ステータスフィルタで「読みたい」を選択します')
rep('ステータスが「未読」の書籍のみが表示されること', 'ステータスが「読みたい」の書籍のみが表示されること')
rep('URL に `?status=unread` が含まれていること', 'URL に `?status=want_to_read` が含まれていること')
rep('`supabase.from("books").delete().eq("status", "finished")`', '`supabase.from("books").delete().eq("status", "completed")`')

# ---------------------------------------------------------------
# 2. 一覧ページは /（app/page.tsx）
# ---------------------------------------------------------------
rep('JavaScript のコードから別ページ（`/books`）へ自動で移動させる例です。',
    'JavaScript のコードから一覧ページ（`/`）へ自動で移動させる例です。')
rep('''    // /books に遷移する（履歴に積まれる＝戻るボタンで戻れる）
    router.push("/books");''', '''    // 一覧ページ（/）に遷移する（履歴に積まれる＝戻るボタンで戻れる）
    router.push("/");''')
rep('| `router.push("/books")` | 新しいURLに遷移 | 削除後に一覧へ戻る |',
    '| `router.push("/")` | 新しいURLに遷移 | 削除後に一覧（`/`）へ戻る |')
rep('''```
app/
  books/
    page.tsx          → /books（一覧ページ）
    new/
      page.tsx        → /books/new（新規登録ページ）''', '''```
app/
  page.tsx            → /（一覧ページ。第7章で作成）
  books/
    new/
      page.tsx        → /books/new（新規登録ページ）''')
rep('        href="/books"\n', '        href="/"\n', 2)
rep('''        // 登録成功: 一覧ページにリダイレクト
        // -------------------------------------------------
        router.push("/books");''', '''        // 登録成功: 一覧ページ（/）にリダイレクト
        // -------------------------------------------------
        router.push("/");''')
rep('新規登録後は一覧ページ（`/books`）へ', '新規登録後は一覧ページ（`/`）へ')
rep('`revalidatePath("/books")` を呼ぶと、その URL に紐づく', '`revalidatePath("/")` を呼ぶと、その URL に紐づく')
rep('revalidatePath("/books");', 'revalidatePath("/");', 2)
rep('削除完了 → router.push("/books")', '削除完了 → router.push("/")')
rep('''      // 一覧ページへ
      router.push("/books");''', '''      // 一覧ページ（/）へ
      router.push("/");''')
rep('- 削除後は `router.push("/books")` で一覧ページに遷移し、', '- 削除後は `router.push("/")` で一覧ページに遷移し、')
rep('''  // 一覧ページへ
  router.push("/books");''', '''  // 一覧ページ（/）へ
  router.push("/");''')
rep('もう存在しないので `router.push("/books")` で一覧へ戻します。', 'もう存在しないので `router.push("/")` で一覧へ戻します。')
rep('''}
router.push("/books");''', '''}
router.push("/");''', 2)
rep('// 現在のパス（例: /books）を取得', '// 現在のパス（一覧ページなら /）を取得')
rep('''      // 例: /books?q=React&status=reading
      //     検索クエリが空なら /books?status=reading''', '''      // 例: /?q=React&status=reading
      //     検索クエリが空なら /?status=reading''')
rep('これにより `/books?status=reading` のように', 'これにより `/?status=reading` のように')
rep('**ファイル: `app/books/page.tsx`（更新版）**', '**ファイル: `app/page.tsx`（更新版）**')
rep('1. ブラウザで `http://localhost:3000/books` にアクセスします', '1. ブラウザで `http://localhost:3000/` にアクセスします')
rep('''router.push("/books");
// サーバーコンポーネントのキャッシュを更新（最新データで再取得）''', '''router.push("/");
// サーバーコンポーネントのキャッシュを更新（最新データで再取得）''')
rep('`revalidatePath("/books")` を呼ぶと、指定した URL のキャッシュが破棄され', '`revalidatePath("/")` を呼ぶと、指定した URL のキャッシュが破棄され')
rep('''  // この行で関数全体が中断され、ブラウザは /books に飛ぶ
  redirect("/books");''', '''  // この行で関数全体が中断され、ブラウザは一覧ページ（/）に飛ぶ
  redirect("/");''')
rep('''  // エラーが発生するとここまで到達しない
  router.push("/books");''', '''  // エラーが発生するとここまで到達しない
  router.push("/");''')

# 8. ページ遷移図: トップページ = 一覧ページ
rep('''    <!-- HOME -->
    <div style="background:#e8f5e9;border:2px solid #4caf50;border-radius:10px;padding:12px 14px;box-shadow:0 2px 12px rgba(0,0,0,0.08);">
      <div style="font-weight:700;color:#2e7d32;font-size:14px;">/</div>
      <div style="font-size:12px;color:#1b5e20;">トップページ</div>
      <div style="margin-top:6px;font-size:11px;color:#4caf50;">↓ 「書籍一覧へ」</div>
    </div>
    <!-- LIST -->
    <div style="background:#e3f2fd;border:2px solid #2196f3;border-radius:10px;padding:12px 14px;box-shadow:0 2px 12px rgba(0,0,0,0.08);">
      <div style="font-weight:700;color:#1565c0;font-size:14px;">/books</div>
      <div style="font-size:12px;color:#0d47a1;">書籍一覧ページ</div>''', '''    <!-- LIST -->
    <div style="background:#e3f2fd;border:2px solid #2196f3;border-radius:10px;padding:12px 14px;box-shadow:0 2px 12px rgba(0,0,0,0.08);">
      <div style="font-weight:700;color:#1565c0;font-size:14px;">/</div>
      <div style="font-size:12px;color:#0d47a1;">書籍一覧ページ（トップページ）</div>''')
rep('<div style="grid-column:1/3;max-width:340px;margin:0 auto;background:#fce4ec;',
    '<div style="background:#fce4ec;')

# 図8-1 と前提
rep('<text x="105" y="86" text-anchor="middle" font-size="12" fill="#64748b" font-family="Consolas, monospace">/books</text>',
    '<text x="105" y="86" text-anchor="middle" font-size="12" fill="#64748b" font-family="Consolas, monospace">/</text>')
rep('fill="#64748b">一覧表示は第7章</text>', 'fill="#64748b">第7章で作成済み</text>')
rep('図8-1: この章の地図（点線の一覧ページは第7章で作った一覧表示。§ は節の番号）',
    '図8-1: この章の地図（点線の一覧ページ app/page.tsx は第7章で作成済み。§ は節の番号）')
rep('第7章までの書籍管理アプリ（書籍の一覧表示と、新規登録ページ `/books/new` が動いていること）',
    '第7章までの書籍管理アプリ（一覧ページ `/` と新規登録ページ `/books/new` が動いていること）')

# ---------------------------------------------------------------
# 3. 著者は必須
# ---------------------------------------------------------------
rep('''    // -------------------------------------------------
    // バリデーション
    // -------------------------------------------------
    // タイトルは必須項目です。空の場合はエラーを
    // 表示して処理を中断します。
    // trim() で前後の空白を除去してからチェックする
    // ことで「スペースだけ」の入力もエラー扱いに。
    // -------------------------------------------------
    // タイトルが空白のみ または 空文字の場合
    if (!formData.title.trim()) {
      setError("タイトルは必須です。");
      // 送信中フラグを戻す
      setIsSubmitting(false);
      // 処理を中断
      return;
    }
''', '''    // -------------------------------------------------
    // バリデーション
    // -------------------------------------------------
    // タイトルと著者は必須項目です（第7章の BookForm と
    // 同じルール。DB の列も NOT NULL）。空の場合は
    // エラーを表示して処理を中断します。
    // trim() で前後の空白を除去してからチェックする
    // ことで「スペースだけ」の入力もエラー扱いに。
    // -------------------------------------------------
    // タイトルが空白のみ または 空文字の場合
    if (!formData.title.trim()) {
      setError("タイトルは必須です。");
      // 送信中フラグを戻す
      setIsSubmitting(false);
      // 処理を中断
      return;
    }
    // 著者が空白のみ または 空文字の場合
    if (!formData.author.trim()) {
      setError("著者は必須です。");
      setIsSubmitting(false);
      return;
    }
''')
rep('''            // 前後の空白を除去
            title: formData.title.trim(),
            // 空文字なら null（DB をクリーンに保つ）
            author: formData.author.trim() || null,
            publisher: formData.publisher.trim() || null,''', '''            // 前後の空白を除去
            title: formData.title.trim(),
            // 著者は必須なので null にしない（空白を除くだけ）
            author: formData.author.trim(),
            // 空文字なら null（DB をクリーンに保つ）
            publisher: formData.publisher.trim() || null,''')
rep('''            title: formData.title.trim(),
            author: formData.author.trim() || null,
            publisher: formData.publisher.trim() || null,''', '''            title: formData.title.trim(),
            author: formData.author.trim(),
            publisher: formData.publisher.trim() || null,''')
rep('''      // 前後の空白を除去
      title: formData.title.trim(),
      // 空文字なら null（DB をクリーンに保つ）
      author: formData.author.trim() || null,
      publisher: formData.publisher.trim() || null,''', '''      // 前後の空白を除去
      title: formData.title.trim(),
      // 著者は必須なので null にしない（空白を除くだけ）
      author: formData.author.trim(),
      // 空文字なら null（DB をクリーンに保つ）
      publisher: formData.publisher.trim() || null,''')
rep('''        <label
          htmlFor="author"
          className="block text-sm font-medium text-gray-700"
        >
          著者
        </label>
        <input
          type="text"
          id="author"
          // name="author" → handleChange で formData.author が更新される
          name="author"''', '''        <label
          htmlFor="author"
          className="block text-sm font-medium text-gray-700"
        >
          {/* 著者もタイトルと同じく必須 */}
          著者 <span className="text-red-500">*</span>
        </label>
        <input
          type="text"
          id="author"
          // name="author" → handleChange で formData.author が更新される
          name="author"
          // ブラウザ側の必須チェック
          required''')
rep('- `formData.author.trim() || null` は「前後の空白を除いた結果が空文字なら `null` を入れる」という意味です（`||` は左が空っぽのとき右を使う）。空文字ではなく `null` を入れることで、データベースを「未入力＝null」というきれいな状態に保てます。',
    '- `formData.publisher.trim() || null` は「前後の空白を除いた結果が空文字なら `null` を入れる」という意味です（`||` は左が空っぽのとき右を使う）。空文字ではなく `null` を入れることで、データベースを「未入力＝null」というきれいな状態に保てます。\n- ただし **著者は必須**（第7章と同じルールで、DB の `author` 列も NOT NULL）なので、`formData.author.trim()` と空白を除くだけにしています。空のまま送られないよう、送信前のバリデーションで止めています。')
rep('逆に `BookForm` の `formData.author.trim() || null` で `||` を使うのは',
    '逆に `BookForm` の `formData.publisher.trim() || null` で `||` を使うのは')
# convert.ts
rep('''type BookRow = {
  id: string;
  title: string;
  author: string | null;''', '''type BookRow = {
  id: string;
  title: string;
  author: string;''')
rep('    author: form.author.trim() || null,\n', '    author: form.author.trim(),\n')
rep('// BookForm が .update() に渡す中身と同じ変換（"" → null、空白を除去）',
    '// BookForm が .update() に渡す中身と同じ変換（"" → null、空白を除去）\n// 著者は必須なので null にはしない')

io.open(TARGET, "w", encoding="utf-8", newline="\n").write(s)
print("done")
