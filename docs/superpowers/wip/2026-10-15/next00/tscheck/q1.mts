// 問題1の URL を部品に分ける
const url = new URL("http://localhost:3000/books?status=reading");
console.log(url.protocol);
console.log(url.hostname);
console.log(url.port);
console.log(url.pathname);
console.log(url.search);
console.log(url.searchParams.get("status"));
