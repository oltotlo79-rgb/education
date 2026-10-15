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
  // await を書き忘れた
  const book = fetchBook("1");
  console.log(book);
  console.log(book.title);
}

showTitle();
