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

async function showTitle(): Promise<void> {
  // await で「中身が届くまで待って」から取り出す
  const book = await fetchBook("1");
  console.log(book);
  console.log(book.title);
}

showTitle();
