// scripts/check-env.ts
// .env.local に必要な2つの値が書かれているかを、アプリを起動せずに確かめるスクリプト

// 確かめたい環境変数の名前を配列にまとめる
const names = ["NEXT_PUBLIC_SUPABASE_URL", "NEXT_PUBLIC_SUPABASE_ANON_KEY"];

// 1つずつ取り出して、値があるかどうかを表示する
for (const name of names) {
  // process.env[name] で、その名前の環境変数の値を読む（無ければ undefined）
  const value = process.env[name];
  if (value) {
    // 秘密を画面に出しすぎないよう、先頭20文字だけ見せる
    console.log(`OK  ${name} = ${value.slice(0, 20)}...`);
  } else {
    console.log(`NG  ${name} が見つかりません`);
  }
}
