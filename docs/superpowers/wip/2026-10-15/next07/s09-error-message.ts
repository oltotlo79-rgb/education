// supabase-js が返す error（PostgrestError）の4つの項目
type SupabaseError = {
  code: string;
  message: string;
  details: string | null;
  hint: string | null;
};

// エラーコード（code）を見て、画面に出す日本語メッセージを選ぶ
// message の文面はバージョンで変わることがあるので、分岐には code を使う
function toUserMessage(error: SupabaseError): string {
  switch (error.code) {
    case "23502":
      return "必須項目（タイトル・著者）が空です。";
    case "23514":
      return "評価やステータスの値が、許可された範囲の外です。";
    case "22P02":
    case "22007":
      return "数値や日付の形式が正しくありません。";
    case "42501":
      return "データを保存する権限がありません。";
    case "PGRST204":
    case "PGRST205":
      return "テーブル名または列名がデータベースと一致していません。";
    default:
      return "書籍の登録に失敗しました。時間をおいて再度お試しください。";
  }
}

// Supabase が実際に返すエラーの例
// （code と message は PostgreSQL / PostgREST の実物と同じ文面。details は長いので省略）
const samples: SupabaseError[] = [
  {
    code: "23502",
    message:
      'null value in column "title" of relation "books" violates not-null constraint',
    details: "Failing row contains (...)",
    hint: null,
  },
  {
    code: "23514",
    message:
      'new row for relation "books" violates check constraint "books_rating_check"',
    details: "Failing row contains (...)",
    hint: null,
  },
  {
    code: "22P02",
    message: 'invalid input syntax for type integer: ""',
    details: null,
    hint: null,
  },
  {
    code: "42501",
    message: 'new row violates row-level security policy for table "books"',
    details: null,
    hint: null,
  },
  {
    code: "PGRST204",
    message: "Could not find the 'memo' column of 'books' in the schema cache",
    details: null,
    hint: null,
  },
];

for (const error of samples) {
  console.log(`[${error.code}] ${error.message}`);
  console.log(`  → 画面に出す文: ${toUserMessage(error)}`);
}
