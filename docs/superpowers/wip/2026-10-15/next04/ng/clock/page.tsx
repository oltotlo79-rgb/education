// app/clock/page.tsx
export default function ClockPage() {
  // このコードが「いつ」実行されたかを時刻で確かめる
  const now = new Date().toLocaleTimeString("ja-JP");
  return <p>このページを作った時刻: {now}</p>;
}
