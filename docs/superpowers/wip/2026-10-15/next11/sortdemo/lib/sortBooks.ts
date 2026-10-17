// 並べ替えに使う項目だけを持つ「書籍の形」
export interface SortableBook {
  title: string;
  rating: number | null;
  created_at: string;
}

// 並べ替えの種類は3通りだけに限定する
export type SortKey = "title" | "rating" | "newest";

// 書籍の配列を受け取り、並べ替えた「新しい配列」を返す
export function sortBooks<T extends SortableBook>(books: T[], key: SortKey): T[] {
  // 元の配列を書き換えないよう、コピーしてから並べ替える
  const copied = [...books];

  if (key === "title") {
    // 日本語の読み順（あいうえお順）で比べる
    return copied.sort((a, b) => a.title.localeCompare(b.title, "ja"));
  }

  if (key === "rating") {
    // 評価なし（null）は 0 とみなし、評価の高い順に並べる
    return copied.sort((a, b) => (b.rating ?? 0) - (a.rating ?? 0));
  }

  // 新しい順: 登録日時を数値（ミリ秒）に直して大きい順に並べる
  return copied.sort(
    (a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime(),
  );
}
