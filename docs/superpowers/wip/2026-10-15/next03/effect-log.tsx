import { useEffect, useState } from "react";

// count を受け取って表示するだけの子コンポーネント
function CountLogger({ count }: { count: number }) {
  // ① コンポーネント関数が呼ばれた（レンダー）
  console.log(`① レンダー（count = ${count}）`);

  useEffect(() => {
    // ② 画面に反映されたあとで実行される
    console.log(`② effect 実行（count = ${count}）`);

    // ③ 次の effect の前・画面から消えるときに実行される
    return () => {
      console.log(`③ クリーンアップ（count = ${count} の effect を片付け）`);
    };
  }, [count]);

  return <p>count: {count}</p>;
}

export default function EffectTimingDemo() {
  const [count, setCount] = useState<number>(0);
  const [isVisible, setIsVisible] = useState<boolean>(true);

  return (
    <div>
      {/* isVisible が false になると CountLogger は画面から消える */}
      {isVisible && <CountLogger count={count} />}
      <button onClick={() => setCount(count + 1)}>+1</button>
      <button onClick={() => setIsVisible(false)}>消す</button>
    </div>
  );
}
