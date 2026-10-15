// app/news/page.tsx（Cache Components 版の ISR）
import { cacheLife } from "next/cache";

async function getNews() {
  // 実際にはここで外部APIやDBから最新ニュースを取る
  // 「ページを作った時刻」を確認用に作る
  const now = new Date().toLocaleTimeString("ja-JP");
  return [{ title: "新刊が入荷しました", builtAt: now }];
}

export default async function NewsPage() {
  // このページの結果をキャッシュ（作り置き）する
  "use cache";
  // "minutes" は「1分たったら次のアクセスで裏で作り直す」という設定
  cacheLife("minutes");

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
