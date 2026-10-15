// 自分の名前と、本棚の冊数を変数に入れる
const myName: string = "花子";
const bookCount: number = 3;

// テンプレートリテラル（バッククォート）で文章に埋め込んで表示する
console.log(`${myName}さんの本棚には ${bookCount} 冊の本があります`);

// 1冊増えたことにして、もう一度表示する
console.log(`1冊買ったので ${bookCount + 1} 冊になりました`);
