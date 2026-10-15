function App() {
  return (
    // 開始タグ: 名前のないタグ（Fragment）
    <>
      <h1>タイトル</h1>
      <p>本文</p>
      {/* 終了タグ */}
    </>
  );
}
function NG() {
  return (
    // ← 1つ目のルート要素
    <h1>タイトル</h1>
    // ← 2つ目のルート要素 → ❌ エラー
    <p>本文</p>
  );
}
type Status = "loading" | "success" | "error";
function DataDisplay() {
  const status: Status = "success";
  const renderContent = (): JSX.Element => {
    switch (status) {
      case "loading":
        return <p>読み込み中...</p>;
      case "error":
        return <p className="error">エラー</p>;
      case "success":
        return <p>ok</p>;
    }
  };
  return <div>{renderContent()}</div>;
}
export { App, NG, DataDisplay };
