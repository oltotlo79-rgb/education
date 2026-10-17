// ===== 判別可能なユニオン型（Discriminated Union）=====
// 判別可能なユニオン型 = 「共通のキー（タグ）」を使って、ユニオン型の中の
// どのバリエーションかを区別できる型。TypeScript で複雑な状態を扱う鉄板パターン。

// ステータスに応じて異なる追加情報を持つ型
type BookWithDetails =
  // ユニオンの1つ目: status="want_to_read" のとき。
  | {
      // ↑ 「判別タグ」。これがあるとTSがバリエーションを区別できる。
      status: "want_to_read";
      title: string;
      // 読みたい理由
      // この種類だけが reason を持つ。
      reason: string;
    }
  // 2つ目: status="reading" のとき。
  | {
      status: "reading";
      title: string;
      // 読書中だけ「現在ページ」を持つ。
      currentPage: number;
      totalPages: number;
    }
  // 3つ目: status="completed" のとき。
  | {
      status: "completed";
      title: string;
      // 数値リテラル型のユニオンで 1〜5 に限定。
      rating: 1 | 2 | 3 | 4 | 5;
      // review はオプショナル。
      review?: string;
    };

function displayBookInfo(book: BookWithDetails): string {
  // book.status を見て分岐。
  switch (book.status) {
    case "want_to_read":
      // ここでは book.reason にアクセス可能
      // この case 内では TS が「book は『1つ目』のバリエーション」と絞り込む。
      return `「${book.title}」を読みたい（理由: ${book.reason}）`;

    case "reading":
      // ここでは book.currentPage, book.totalPages にアクセス可能
      // この case では「2つ目」に絞り込み。currentPage が読める。
      // Math.round は四捨五入。
      const progress = Math.round(
        // 進捗率 = 現在ページ / 総ページ × 100。
        (book.currentPage / book.totalPages) * 100
      );
      return `「${book.title}」を読書中（進捗: ${progress}%）`;

    case "completed":
      // ここでは book.rating, book.review にアクセス可能
      // この case では「3つ目」に絞り込み。
      // .repeat(n) は文字列を n 回繰り返すメソッド。「★★★★☆」のような星評価を作る。
      const stars = "★".repeat(book.rating) + "☆".repeat(5 - book.rating);
      return `「${book.title}」読了 ${stars}`;
  }
}

// 使用例
console.log(
  displayBookInfo({
    status: "reading",
    title: "TypeScript入門",
    currentPage: 150,
    totalPages: 400,
  })
);
// => 「TypeScript入門」を読書中（進捗: 38%）

console.log(
  displayBookInfo({
    status: "completed",
    title: "React実践ガイド",
    rating: 4,
    review: "とても実践的でした",
  })
);
// => 「React実践ガイド」読了 ★★★★☆
