// supabase-js の戻り値と同じ「{ data, error } の形」を表す型
type Result = {
  data: { title: string }[] | null;
  error: { code: string; message: string } | null;
};

// にせものの取得関数。テーブル名が "books" 以外なら error を返す（throw はしない）
async function fakeFrom(table: string): Promise<Result> {
  if (table !== "books") {
    return {
      data: null,
      error: {
        code: "PGRST205",
        message: `Could not find the table 'public.${table}' in the schema cache`,
      },
    };
  }
  return { data: [{ title: "リーダブルコード" }], error: null };
}

// ❌ NG: try / catch だけで安心してしまう書き方
async function ngVersion(table: string) {
  try {
    const { data } = await fakeFrom(table);
    console.log("NG版: data =", data);
    console.log("NG版: 成功したつもりで先へ進んでしまう");
  } catch (err) {
    console.log("NG版: catch に来た", err);
  }
}

// ✅ OK: error を自分でチェックする書き方
async function okVersion(table: string) {
  const { data, error } = await fakeFrom(table);
  if (error) {
    console.log("OK版: エラーを検出 →", error.code, error.message);
    return;
  }
  const books = data ?? [];
  console.log("OK版: 取得件数 =", books.length);
}

async function main() {
  // テーブル名をわざと間違える（books → bookz）
  await ngVersion("bookz");
  await okVersion("bookz");
  // 正しいテーブル名
  await okVersion("books");
}

main();
