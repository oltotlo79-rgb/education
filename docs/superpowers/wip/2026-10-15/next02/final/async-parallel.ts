function wait(ms: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

interface Book {
  id: string;
  title: string;
}

async function fetchBook(id: string): Promise<Book> {
  await wait(300);
  return { id, title: `本その${id}` };
}

// 経過ミリ秒を 100 単位に切り捨てて返す（実行のたびに数ミリ秒ずれるため）
function elapsed(start: number): number {
  return Math.floor((Date.now() - start) / 100) * 100;
}

async function main(): Promise<void> {
  // ① 1冊ずつ順番に待つ
  let start = Date.now();
  const a = await fetchBook("1");
  const b = await fetchBook("2");
  console.log(`順番に待つ: ${a.title}, ${b.title}（約${elapsed(start)}ミリ秒）`);

  // ② Promise.all で同時に待つ
  start = Date.now();
  const [c, d] = await Promise.all([fetchBook("3"), fetchBook("4")]);
  console.log(`同時に待つ: ${c.title}, ${d.title}（約${elapsed(start)}ミリ秒）`);
}

main();
