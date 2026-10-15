// プログラムの中で扱う「本」のデータ（オブジェクト）
const book = {
  title: "こころ",
  author: "夏目漱石",
  status: "finished",
};

// 通信で送れるように、オブジェクトを JSON という「ただの文字列」に変換する
const json = JSON.stringify(book);
console.log(json);
// typeof は「この値の種類は何か」を調べる仕組み
console.log(typeof json);

// 受け取った側で、JSON の文字列を元のオブジェクトに戻す
const received = JSON.parse(json);
console.log(received.author);
console.log(typeof received);
