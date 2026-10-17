**▼ 実際の出力**（Next.js 16.4.1。`app/page.tsx` で `@/components/BookCard` を `@/components/bookCard` と小文字で書いた場合）

```text
> Build error occurred
Error: Turbopack build failed with 1 error:
./src/app/page.tsx:2:1
Error: Module not found: Can't resolve '@/components/bookCard'
  1 | import { createClient } from "@/lib/supabase/server";
> 2 | import BookCard from "@/components/bookCard";
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  3 | import type { Book } from "@/types/book";
  4 |
  5 | export default async function HomePage() {

Import map: aliased to relative './src/components/bookCard' inside of [project]/


https://nextjs.org/docs/messages/module-not-found
```

Next.js 16 の Turbopack は Windows でも大文字・小文字を区別して探すので、この打ち間違いは手元のビルドで見つかります。やっかいなのは、次の「Git が気づかない」ケースです。

> **つまずきポイント:** ファイル名の**大文字・小文字だけ**を変えたとき（例: `bookCard.tsx` → `BookCard.tsx`）、Windows や Mac の Git は、その変更に気づきません。手元では新しい名前で動くのに、GitHub には古い名前のまま残るため、**Vercel のビルドだけが `Module not found` で失敗します**。

**NG: エクスプローラーやエディタで名前だけ変えて、そのままコミットする**

```powershell
# 名前を変えたあとで、Git が変更に気づいているか確かめる
git status --short
# Git が覚えているファイル名を表示する
git ls-files src/components
```

```text
PS> git status --short
PS> git ls-files src/components
src/components/bookCard.tsx
```

`git status` には何も出ず（＝変更なしと思われている）、Git はまだ古い `bookCard.tsx` という名前を覚えています。

**OK: 名前の変更は `git mv` で Git に伝える**（すでに名前を変えたあとでも、このコマンドで伝えられます）

```powershell
# 古い名前 → 新しい名前 の順に書く
git mv src/components/bookCard.tsx src/components/BookCard.tsx
git status --short
```

```text
PS> git mv src/components/bookCard.tsx src/components/BookCard.tsx
PS> git status --short
R  src/components/bookCard.tsx -> src/components/BookCard.tsx
```

先頭の `R` は Renamed（名前の変更）の意味です。これをコミットして push すれば、GitHub と Vercel にも新しい名前が届きます。
