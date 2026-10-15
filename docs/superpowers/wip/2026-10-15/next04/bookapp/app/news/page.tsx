// app/news/page.tsx
// 60秒ごとに作り直す（予約名の export）
export const revalidate = 60;

async function getNews() {
  const now = new Date().toLocaleTimeString("ja-JP");
  return [{ title: "新刊が入荷しました", builtAt: now }];
}

export default async function NewsPage() {
  const news = await getNews();
  return (
    <div>
      <h1>お知らせ</h1>
      <ul>
        {news.map((item, i) => (
          <li key={i}>
            {item.title}（生成時刻: {item.builtAt}）
          </li>
        ))}
      </ul>
    </div>
  );
}
