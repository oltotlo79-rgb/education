// .delete().eq("id", id).select() の返り値 { data, error } から、
// 「本当に1件消えたのか」を判定する練習用コード
// （返り値の形だけを真似しています）

type Book = {
  id: string;
  title: string;
};

type MutationResult = {
  data: Book[] | null;
  error: { code: string } | null;
};

// 返り値を見て、画面に出すメッセージを決める
function judge(result: MutationResult): string {
  // ① Supabase がエラーを返した
  if (result.error) {
    return `失敗: エラーコード ${result.error.code}`;
  }
  // ② エラーは無いが 1 件も消えていない（id 違い・RLS で見えない行など）
  if (!result.data || result.data.length === 0) {
    return "失敗: 対象の行が 0 件でした（id や RLS を確認）";
  }
  // ③ ちゃんと消えた
  return `成功: 「${result.data[0].title}」を削除しました`;
}

// よくある 3 つの返り値
const cases: MutationResult[] = [
  { data: [{ id: "b2", title: "TypeScript入門" }], error: null },
  { data: [], error: null },
  { data: null, error: { code: "42501" } },
];

for (const result of cases) {
  console.log(judge(result));
}
