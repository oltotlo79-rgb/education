// Next.js が「URL から [id] を取り出して params に入れる」動きを
// 小さく真似した練習用コード（本物の Next.js の実装ではありません）

// ルートの形（例: "/books/[id]"）と実際の URL を見比べて、
// [ ] で囲まれた部分の値を集めたオブジェクトを返す。一致しなければ null
function matchRoute(
  pattern: string,
  url: string,
): Record<string, string> | null {
  // "/" で区切ってセグメント（区画）の配列にする。空文字は捨てる
  const patternParts = pattern.split("/").filter((s) => s !== "");
  const urlParts = url.split("/").filter((s) => s !== "");

  // セグメントの数が違えば、このルートとは一致しない
  if (patternParts.length !== urlParts.length) {
    return null;
  }

  // 取り出した値をためていく入れ物
  const params: Record<string, string> = {};

  for (let i = 0; i < patternParts.length; i++) {
    const p = patternParts[i];
    const u = urlParts[i];
    if (p.startsWith("[") && p.endsWith("]")) {
      // "[id]" の角括弧を外した "id" を名前にして、URL 側の値を入れる
      params[p.slice(1, -1)] = u;
    } else if (p !== u) {
      // 固定の部分（"books" など）が違えば不一致
      return null;
    }
  }
  return params;
}

// page.tsx と同じ形の Props 型（params は Promise）
type Props = {
  params: Promise<{ id: string }>;
};

// 正しい書き方: await で Promise の中身を取り出す
async function GoodPage({ params }: Props) {
  const { id } = await params;
  console.log(`[OK] id = "${id}" / 型は ${typeof id}`);
}

// まちがった書き方: await を忘れて Promise から直接 id を読む
async function BadPage({ params }: Props) {
  // Promise 自体には id というプロパティが無いので undefined になる
  const id = (params as unknown as { id?: string }).id;
  console.log(`[NG] id = ${id}`);
}

async function main() {
  // 3 つの URL を "/books/[id]" と見比べてみる
  const urls = ["/books/abc123", "/books/42", "/books/abc123/edit"];
  for (const url of urls) {
    const found = matchRoute("/books/[id]", url);
    console.log(`${url} → ${JSON.stringify(found)}`);
  }

  // Next.js と同じように、取り出した値を Promise に包んで渡す
  const params = Promise.resolve({ id: "42" });
  await GoodPage({ params });
  await BadPage({ params });
}

main();
