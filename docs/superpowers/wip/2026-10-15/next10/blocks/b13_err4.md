**▼ 実際の出力**（Next.js 16.4.1。`app/books/new/page.tsx` の1行目の `"use client";` を消した場合。抜粋）

```text
> Build error occurred
Error: Turbopack build failed with 2 errors:
./src/app/books/new/page.tsx:2:10
Error: You're importing a module that depends on `useRouter` into a React Server Component module. This API is only available in Client Components. To fix, mark the file (or its parent) with the `"use client"` directive.
    Learn more: https://nextjs.org/docs/app/api-reference/directives/use-client
  1 | import { useState } from "react";
> 2 | import { useRouter } from "next/navigation";
    |          ^^^^^^^^^
  3 | import { createClient } from "@/lib/supabase/client";
  4 |
  5 | export default function NewBookPage() {

Ecmascript file had an error
```

同じ種類のエラーが、`useState` についてももう1つ出ます。エラー文の最後に `To fix, mark the file (or its parent) with the "use client" directive.`（直すには、ファイルに "use client" を付けてください）と、直し方そのものが書かれています。
