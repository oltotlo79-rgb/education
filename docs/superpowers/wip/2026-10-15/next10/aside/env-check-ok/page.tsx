export default function EnvCheckOkPage() {
  // Server Component の中なら、秘密の値を読んでもブラウザ用の JS には入らない
  const hasSecret = Boolean(process.env.SECRET_MESSAGE);

  return (
    <main className="p-6">
      {/* 値そのものではなく「設定されているか」だけを表示する */}
      <p>秘密の値は設定済み？ {hasSecret ? "はい" : "いいえ"}</p>
    </main>
  );
}
