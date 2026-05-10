# 第2章: TypeScript の基礎

> この章では、TypeScript（タイプスクリプト）の基本を学びます。TypeScript は JavaScript に「型」という仕組みを追加した言語です。「型って何？」という方も安心してください。身近な例えを交えながら、一つずつ丁寧に解説します。

### この章で学ぶこと

- **型（Type）とは何か** — データの種類を指定する仕組み。「この箱には数字だけ入れていいよ」というラベルのようなもの
- **基本的な型** — `string`（文字列）、`number`（数値）、`boolean`（真偽値）など
- **インターフェース / 型エイリアス** — 自分だけのオリジナルの型を定義する方法。書籍管理アプリでは「Book型」を作ります
- **ジェネリクス**（Generics） — 型をパラメータ（引数）として受け取る柔軟な仕組み
- **よくあるエラーと対処法** — 初心者がつまづきやすいTypeScriptのエラーメッセージと、その読み方

> **なぜTypeScriptを学ぶの？** JavaScriptだけでもアプリは作れますが、TypeScriptを使うと「コードを書いている途中で間違いに気づける」「エディタが賢く補完してくれる」というメリットがあります。最初は少し面倒に感じるかもしれませんが、慣れると「TypeScriptなしでは開発できない！」と思うようになります。

## 目次

0. [前提知識: JavaScriptの基礎の基礎](#0-前提知識-javascriptの基礎の基礎)
1. [TypeScript とは](#1-typescript-とは)
2. [基本的な型](#2-基本的な型)
3. [型注釈と型推論](#3-型注釈と型推論)
4. [インターフェースと型エイリアス](#4-インターフェースと型エイリアス)
5. [ジェネリクス](#5-ジェネリクス)
6. [ユニオン型とリテラル型](#6-ユニオン型とリテラル型)
7. [TypeScript の設定（tsconfig.json）](#7-typescript-の設定tsconfigjson)
8. [よくあるエラーと対処法](#8-よくあるエラーと対処法)

---

## 0. 前提知識: JavaScriptの基礎の基礎

TypeScript は JavaScript に「型」を追加した言語なので、JavaScript の文法をひと通り知っておくと話が早く進みます。ここでは TypeScript を読むうえで**最低限必要な JavaScript の文法**だけ、ぎゅっと圧縮して解説します。「もう知ってる！」という方は読み飛ばして 1 章へ。

> **動かしながら読みたい方へ:** 以下の例はどれも、ターミナルで `node` と入力して Enter を押すと開く対話モード（REPL）に貼り付けて実行できます。終了は `Ctrl+C` を2回押します。または、ブラウザの**開発者ツール → Console**（F12 で開ける）にそのまま貼り付けてもOKです。

### 0.1 コメント

コメントは「コンピュータには無視されるが、人間が読むためのメモ」です。

```javascript
// この行はコメント。実行されない。
let x = 1; // 行末にも書ける

/*
 * 複数行コメント。
 * /* と */ で囲む
 */
```

### 0.2 変数の宣言: `const` / `let`

値に名前を付けて保存する仕組みです。本書では原則 `const`（再代入できない）を使い、必要なときだけ `let`（再代入できる）を使います。古い書き方の `var` はバグの元なので使いません。

```javascript
const name = "太郎";    // const = 一度入れたら変えられない
// name = "次郎";       // ❌ エラー: Assignment to constant variable.

let count = 0;          // let = 後で書き換え可能
count = count + 1;      // ✅ OK
console.log(count);

// ▼ 実行結果
// 1
```

> **`console.log()` とは？**: ターミナル（Node.js の場合）またはブラウザのコンソール（ブラウザの場合）に値を表示する関数です。プログラムの動作確認に多用します。

### 0.3 基本のデータ型

| データ型 | 例 | 説明 |
|----------|-----|------|
| 文字列（string） | `"hello"` `'太郎'` `` `テンプレ${x}` `` | クォートで囲んだ文字 |
| 数値（number） | `42` `3.14` `-0.5` | 整数も小数も同じ型 |
| 真偽値（boolean） | `true` `false` | ハイorロー |
| `null` / `undefined` | `null` `undefined` | 値が「ない」ことを表す（null=明示的、undefined=未定義） |
| 配列（array） | `[1, 2, 3]` | 値の並び |
| オブジェクト（object） | `{ name: "太郎", age: 20 }` | キー：値の組 |

```javascript
const text = "hello";
const num = 42;
const isOk = true;
const list = [1, 2, 3];
const user = { name: "太郎", age: 20 };

console.log(text, num, isOk, list, user);

// ▼ 実行結果
// hello 42 true [ 1, 2, 3 ] { name: '太郎', age: 20 }
```

### 0.4 演算子（足し算・比較）

```javascript
console.log(1 + 2);        // 3 (足し算)
console.log(10 - 3);       // 7
console.log(4 * 5);        // 20
console.log(10 / 3);       // 3.3333333333333335
console.log(10 % 3);       // 1 (余り)
console.log("ab" + "cd");  // "abcd" (文字列の連結)

console.log(1 === 1);      // true (等しい。型もチェック)
console.log(1 === "1");    // false (型が違うと false)
console.log(1 !== 2);      // true (異なる)
console.log(3 > 2);        // true
console.log(true && false);// false (両方trueならtrue = AND)
console.log(true || false);// true  (片方trueならtrue = OR)
console.log(!true);        // false (反転)
```

> **`==` ではなく `===` を使う:** 1個少ない `==` は型変換しながら比較するため、`1 == "1"` が `true` になります。バグの温床なので、本書では一貫して `===` を使います。

### 0.5 文字列とテンプレートリテラル

```javascript
const name = "太郎";

// 連結（古い書き方）
const msg1 = "こんにちは、" + name + "さん！";

// テンプレートリテラル（新しい書き方。バッククォート ` で囲む）
const msg2 = `こんにちは、${name}さん！`;

console.log(msg1);
console.log(msg2);

// ▼ 実行結果
// こんにちは、太郎さん！
// こんにちは、太郎さん！
```

`${ ... }` の中には変数や式を入れられます。テンプレートリテラルは改行も含められて便利です。

### 0.6 if文（条件分岐）

```javascript
const score = 75;

if (score >= 80) {
  console.log("優");
} else if (score >= 60) {
  console.log("可");
} else {
  console.log("不可");
}

// ▼ 実行結果
// 可
```

`{ ... }` で囲んだ範囲を**ブロック**と呼びます。条件に当てはまったブロックだけ実行されます。

### 0.7 for ループ・配列のループ

```javascript
// 0, 1, 2 と3回繰り返す
for (let i = 0; i < 3; i++) {
  console.log(`i = ${i}`);
}

// ▼ 実行結果
// i = 0
// i = 1
// i = 2

// 配列を1つずつ取り出す（モダンな書き方）
const fruits = ["apple", "banana", "cherry"];
for (const fruit of fruits) {
  console.log(fruit);
}

// ▼ 実行結果
// apple
// banana
// cherry
```

### 0.8 関数の書き方

関数は「処理に名前を付けて何度でも呼び出せるようにしたもの」です。3 通りの書き方があります。本書では主に**3番のアロー関数**を使います。

```javascript
// 1. function 宣言
function add1(a, b) {
  return a + b;
}

// 2. function 式
const add2 = function (a, b) {
  return a + b;
};

// 3. アロー関数（=> を使う、Reactでよく使う）
const add3 = (a, b) => {
  return a + b;
};

// 3'. アロー関数の省略形（return 1行のとき）
const add4 = (a, b) => a + b;

console.log(add1(1, 2)); // 3
console.log(add2(1, 2)); // 3
console.log(add3(1, 2)); // 3
console.log(add4(1, 2)); // 3
```

> **アロー関数の魅力:** `function` の文字を書かなくて済むので短く、コールバック関数（あとで呼んでもらう関数）として渡すときに見やすくなります。React/Next.jsのコードはほぼアロー関数で書かれています。

### 0.9 配列の便利メソッド: map / filter / forEach

これは超重要です。後の章で頻繁に登場します。

```javascript
const numbers = [1, 2, 3, 4, 5];

// forEach: 1つずつ処理する
numbers.forEach((n) => {
  console.log(n);
});
// ▼ 実行結果
// 1
// 2
// 3
// 4
// 5

// map: 1つずつ加工して新しい配列を作る
const doubled = numbers.map((n) => n * 2);
console.log(doubled);
// ▼ 実行結果
// [ 2, 4, 6, 8, 10 ]

// filter: 条件を満たす要素だけ残した新しい配列を作る
const evens = numbers.filter((n) => n % 2 === 0);
console.log(evens);
// ▼ 実行結果
// [ 2, 4 ]
```

### 0.10 オブジェクトの読み書き

```javascript
const user = { name: "太郎", age: 20 };

// 値の取り出し（2つの書き方）
console.log(user.name);     // "太郎" (ドット記法)
console.log(user["name"]);  // "太郎" (ブラケット記法)

// 値の書き換え
user.age = 21;
console.log(user.age);      // 21

// プロパティの追加
user.email = "taro@example.com";
console.log(user);
// ▼ 実行結果
// { name: '太郎', age: 21, email: 'taro@example.com' }

// 分割代入（オブジェクトから一気に取り出す）
const { name, age } = user;
console.log(name, age);     // "太郎" 21
```

### 0.11 セミコロン `;` と改行

JavaScript/TypeScript では文末にセミコロン `;` を付ける習慣があります。実は省略しても大体動きますが、本書では**付ける派**です（VS Code が自動で付けてくれる場合も多い）。

```javascript
const a = 1;
const b = 2;
console.log(a + b);
```

これで JavaScript の超基礎は OK。次の節からいよいよ TypeScript 本編に入ります。

---

## 1. TypeScript とは

### 1.1 JavaScript との関係

TypeScript は、Microsoft が開発した **JavaScript のスーパーセット（上位互換：JavaScriptの機能を全て含み、さらに追加機能がある言語）** です。すべての JavaScript コードは有効な TypeScript コードですが、TypeScript には「型システム」（Type System：変数や関数に入るデータの種類を事前に定義し、チェックする仕組み）という強力な機能が追加されています。

TypeScript のコードは直接ブラウザや Node.js で実行できません。必ず **コンパイル**（Compile：人間が書いたプログラムを、コンピュータが実行できる形に変換すること。TypeScriptの場合はJavaScriptに変換する処理を指す。「トランスパイル」とも呼ばれる）という変換処理を経て、JavaScript に変換されてから実行されます。

> **身近な例え：** TypeScriptとJavaScriptの関係は、「下書き用紙」と「清書用紙」に似ています。TypeScript（下書き）には赤ペンで注意書き（型情報）が書いてあり、間違いがあればその場で気づけます。最終的にコンパイルすると、注意書きを消した綺麗なJavaScript（清書）ができあがります。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; display: flex; align-items: center; justify-content: center; gap: 12px;">
  <div style="background: #3178c6; color: white; border-radius: 10px; padding: 14px 20px; text-align: center; min-width: 120px; box-shadow: 0 2px 8px rgba(49,120,198,0.3);">
    <div style="font-size: 24px; font-weight: 700;">TS</div>
    <div style="font-size: 11px; margin-top: 4px;">.ts / .tsx ファイル</div>
  </div>
  <div style="color: #3b82f6; font-size: 24px;">→</div>
  <div style="background: #f59e0b; color: white; border-radius: 10px; padding: 14px 20px; text-align: center; min-width: 120px; box-shadow: 0 2px 8px rgba(245,158,11,0.3);">
    <div style="font-size: 14px; font-weight: 700;">tsc</div>
    <div style="font-size: 11px; margin-top: 4px;">コンパイラ</div>
  </div>
  <div style="color: #3b82f6; font-size: 24px;">→</div>
  <div style="background: #f7df1e; color: #1e293b; border-radius: 10px; padding: 14px 20px; text-align: center; min-width: 120px; box-shadow: 0 2px 8px rgba(247,223,30,0.3);">
    <div style="font-size: 24px; font-weight: 700;">JS</div>
    <div style="font-size: 11px; margin-top: 4px;">.js ファイル</div>
  </div>
  <div style="color: #3b82f6; font-size: 24px;">→</div>
  <div style="background: #16a34a; color: white; border-radius: 10px; padding: 14px 20px; text-align: center; min-width: 120px; box-shadow: 0 2px 8px rgba(22,163,74,0.3);">
    <div style="font-size: 14px; font-weight: 700;">実行</div>
    <div style="font-size: 11px; margin-top: 4px;">ブラウザ / Node.js</div>
  </div>
</div>

この流れをもう少し詳しく見てみましょう。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif;">
  <!-- 開発時 group -->
  <div style="border: 2px solid #3b82f6; border-radius: 12px; padding: 18px; margin-bottom: 12px; background: #f8faff;">
    <div style="font-size: 12px; font-weight: 700; color: #1e40af; margin-bottom: 12px; text-transform: uppercase; letter-spacing: 0.5px;">開発時</div>
    <!-- Step A -->
    <div style="background: #3178c6; color: white; border-radius: 10px; padding: 12px 18px; text-align: center; font-size: 13px; font-weight: 600; box-shadow: 0 2px 8px rgba(49,120,198,0.25);">開発者が TypeScript でコードを書く</div>
    <div style="text-align: center; color: #64748b; font-size: 20px; line-height: 1.4;">▼</div>
    <!-- Step B -->
    <div style="background: #f59e0b; color: #1e293b; border-radius: 10px; padding: 12px 18px; text-align: center; font-size: 13px; font-weight: 600; box-shadow: 0 2px 8px rgba(245,158,11,0.25);">TypeScript コンパイラ (tsc) が型チェックを実行</div>
    <!-- Branches -->
    <div style="display: flex; justify-content: center; gap: 40px; margin-top: 8px;">
      <div style="text-align: center;">
        <div style="font-size: 11px; color: #dc2626; font-weight: 600;">型エラーがある場合 ▼</div>
        <div style="background: #dc2626; color: white; border-radius: 10px; padding: 12px 16px; text-align: center; font-size: 12px; font-weight: 600; margin-top: 6px;">コンパイルエラー<br>開発者に通知</div>
        <div style="font-size: 11px; color: #3b82f6; margin-top: 6px; font-weight: 600;">↩ 修正して再度書く</div>
      </div>
      <div style="text-align: center;">
        <div style="font-size: 11px; color: #16a34a; font-weight: 600;">型エラーがない場合 ▼</div>
        <div style="background: #f7df1e; color: #1e293b; border-radius: 10px; padding: 12px 16px; text-align: center; font-size: 12px; font-weight: 600; margin-top: 6px;">JavaScript に変換</div>
      </div>
    </div>
  </div>
  <!-- 実行時 group -->
  <div style="border: 2px solid #16a34a; border-radius: 12px; padding: 18px; background: #f0fdf4;">
    <div style="font-size: 12px; font-weight: 700; color: #16a34a; margin-bottom: 12px; text-transform: uppercase; letter-spacing: 0.5px;">実行時</div>
    <div style="text-align: center; color: #64748b; font-size: 20px; line-height: 1;">▼</div>
    <div style="background: #16a34a; color: white; border-radius: 10px; padding: 12px 18px; text-align: center; font-size: 13px; font-weight: 600; box-shadow: 0 2px 8px rgba(22,163,74,0.25);">ブラウザまたは Node.js で実行</div>
  </div>
</div>

> **ポイント**: TypeScript の型情報は、コンパイル後の JavaScript には一切残りません。型はあくまで「開発時の安全ネット」です。これを **型消去（Type Erasure）** と呼びます。

### 1.2 なぜ TypeScript を使うのか

TypeScript を使う最大の理由は **型安全性** です。以下の表で、JavaScript と TypeScript の違いを比較してみましょう。

| 観点 | JavaScript | TypeScript |
|------|-----------|------------|
| 型チェック | 実行時にエラー発覚 | コンパイル時にエラー発覚 |
| エディタ補完 | 限定的 | 強力な自動補完 |
| リファクタリング | 手動で確認が必要 | 型の変更で影響範囲を自動検出 |
| ドキュメント性 | コメントに頼る | 型そのものがドキュメント |
| バグ発見のタイミング | ユーザーが使用中に発覚 | 開発中に発覚 |
| 学習コスト | 低い | やや高い（型の学習が必要） |
| 大規模開発 | 困難 | 適している |

### 1.3 JavaScript vs TypeScript の比較コード例

#### 例1: 関数の引数に誤った型を渡すケース

```javascript
// ---- JavaScript ----
function greet(name) {
  return "こんにちは、" + name + "さん！";
}

// 数値を渡してもエラーにならない（実行時まで気づけない）
console.log(greet(42));
// => "こんにちは、42さん！" ← 意図しない動作
```

```typescript
// ---- TypeScript ----
function greet(name: string): string {
  return "こんにちは、" + name + "さん！";
}

// コンパイル時にエラーが発生！
console.log(greet(42));
// エラー: Argument of type 'number' is not assignable to parameter of type 'string'

// 正しい使い方
console.log(greet("太郎"));
// => "こんにちは、太郎さん！"
```

#### 例2: オブジェクトのプロパティアクセス

```javascript
// ---- JavaScript ----
const user = {
  name: "田中",
  age: 25,
};

// タイプミスしてもエラーにならない（実行時に undefined になる）
console.log(user.nmae); // => undefined ← バグ！
```

```typescript
// ---- TypeScript ----
const user = {
  name: "田中",
  age: 25,
};

// タイプミスするとコンパイル時にエラー！
console.log(user.nmae);
// エラー: Property 'nmae' does not exist on type '{ name: string; age: number; }'.
// Did you mean 'name'?

// 正しいアクセス
console.log(user.name); // => "田中"
```

#### 例3: 配列操作での型安全性

```javascript
// ---- JavaScript ----
const numbers = [1, 2, 3, 4, 5];

// 文字列を追加してもエラーにならない
numbers.push("six"); // ← バグの原因になる

// 後で計算しようとすると予期しない結果に
const sum = numbers.reduce((a, b) => a + b, 0);
console.log(sum); // => "15six" ← 文字列連結になってしまう！
```

```typescript
// ---- TypeScript ----
const numbers: number[] = [1, 2, 3, 4, 5];

// 文字列を追加しようとするとコンパイルエラー！
numbers.push("six");
// エラー: Argument of type 'string' is not assignable to parameter of type 'number'

// 正しい使い方
numbers.push(6);
const sum = numbers.reduce((a, b) => a + b, 0);
console.log(sum); // => 21
```

---

## 2. 基本的な型

TypeScript には豊富な型が用意されています。ここでは、すべての基本型を詳しく見ていきます。

### TypeScript の型階層

まず、TypeScript の型システム全体を俯瞰してみましょう。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif;">
  <!-- Top: unknown -->
  <div style="text-align: center;">
    <div style="display: inline-block; background: #dc2626; color: white; border-radius: 10px; padding: 12px 28px; font-size: 14px; font-weight: 700; box-shadow: 0 2px 10px rgba(220,38,38,0.3);">unknown<br><span style="font-size: 11px; font-weight: 400;">（すべての型のトップ）</span></div>
  </div>
  <div style="text-align: center; color: #94a3b8; font-size: 18px; line-height: 1.2;">│</div>
  <!-- Primitive types row -->
  <div style="display: flex; flex-wrap: wrap; justify-content: center; gap: 6px; margin: 4px 0;">
    <div style="background: #2563eb; color: white; border-radius: 8px; padding: 8px 12px; font-size: 12px; font-weight: 600; text-align: center; min-width: 62px;">string</div>
    <div style="background: #16a34a; color: white; border-radius: 8px; padding: 8px 12px; font-size: 12px; font-weight: 600; text-align: center; min-width: 62px;">number</div>
    <div style="background: #d97706; color: #1e293b; border-radius: 8px; padding: 8px 12px; font-size: 12px; font-weight: 600; text-align: center; min-width: 62px;">boolean</div>
    <div style="background: #7c3aed; color: white; border-radius: 8px; padding: 8px 12px; font-size: 12px; font-weight: 600; text-align: center; min-width: 62px;">object</div>
    <div style="background: #64748b; color: white; border-radius: 8px; padding: 8px 12px; font-size: 12px; font-weight: 600; text-align: center; min-width: 62px;">symbol</div>
    <div style="background: #64748b; color: white; border-radius: 8px; padding: 8px 12px; font-size: 12px; font-weight: 600; text-align: center; min-width: 62px;">bigint</div>
    <div style="background: #64748b; color: white; border-radius: 8px; padding: 8px 12px; font-size: 12px; font-weight: 600; text-align: center; min-width: 62px;">null</div>
    <div style="background: #64748b; color: white; border-radius: 8px; padding: 8px 12px; font-size: 12px; font-weight: 600; text-align: center; min-width: 62px;">undefined</div>
    <div style="background: #64748b; color: white; border-radius: 8px; padding: 8px 12px; font-size: 12px; font-weight: 600; text-align: center; min-width: 62px;">void</div>
  </div>
  <div style="text-align: center; color: #94a3b8; font-size: 18px; line-height: 1.2;">│</div>
  <!-- Sub-types row -->
  <div style="display: flex; justify-content: center; gap: 24px; margin: 4px 0;">
    <!-- object subtypes -->
    <div style="text-align: center;">
      <div style="font-size: 10px; color: #7c3aed; font-weight: 600; margin-bottom: 4px;">object の派生型</div>
      <div style="display: flex; gap: 4px;">
        <div style="background: #a78bfa; color: white; border-radius: 6px; padding: 6px 10px; font-size: 11px; font-weight: 600;">Array</div>
        <div style="background: #a78bfa; color: white; border-radius: 6px; padding: 6px 10px; font-size: 11px; font-weight: 600;">Tuple</div>
        <div style="background: #a78bfa; color: white; border-radius: 6px; padding: 6px 10px; font-size: 11px; font-weight: 600;">Function</div>
        <div style="background: #a78bfa; color: white; border-radius: 6px; padding: 6px 10px; font-size: 11px; font-weight: 600;">interface / class</div>
      </div>
    </div>
    <!-- literal subtypes -->
    <div style="text-align: center;">
      <div style="font-size: 10px; color: #1e40af; font-weight: 600; margin-bottom: 4px;">リテラル型</div>
      <div style="display: flex; gap: 4px;">
        <div style="background: #93c5fd; color: #1e3a5f; border-radius: 6px; padding: 6px 8px; font-size: 10px; font-weight: 600;">'hello' | 'world'</div>
        <div style="background: #86efac; color: #14532d; border-radius: 6px; padding: 6px 8px; font-size: 10px; font-weight: 600;">1 | 2 | 3</div>
        <div style="background: #fde68a; color: #78350f; border-radius: 6px; padding: 6px 8px; font-size: 10px; font-weight: 600;">true | false</div>
      </div>
    </div>
  </div>
  <div style="text-align: center; color: #94a3b8; font-size: 18px; line-height: 1.2;">│</div>
  <!-- Bottom: never -->
  <div style="text-align: center;">
    <div style="display: inline-block; background: #1e293b; color: white; border-radius: 10px; padding: 12px 28px; font-size: 14px; font-weight: 700; box-shadow: 0 2px 10px rgba(30,41,59,0.3);">never<br><span style="font-size: 11px; font-weight: 400;">（すべての型のボトム）</span></div>
  </div>
</div>

> **ポイント**: `unknown` はすべての型の「親」で、`never` はすべての型の「子」です。`unknown` にはどんな値も代入できますが、`never` にはどんな値も代入できません。

### 2.1 string（文字列型）

文字列を表す型です。シングルクォート、ダブルクォート、テンプレートリテラルのいずれでも使えます。

```typescript
// ==========================================================================
// string 型のサンプル — 「文字列」を意味する型
// ==========================================================================
// const は「再代入禁止の変数宣言」。一度値を入れたら別の値で上書きできない。
// 変数名の右の : string が「型注釈」。「この変数は string 型ですよ」という宣言。
// = の右側が「初期値」。

// (1) ダブルクォート " で囲んだ文字列
//     greeting という名前の string 型変数に "こんにちは" を入れる。
const greeting: string = "こんにちは";

// (2) シングルクォート ' でも書ける（JavaScript/TypeScriptでは両者同じ意味）
//     チームのコーディング規約で「" に統一」「' に統一」を決めるのが普通。
const name: string = '太郎';

// (3) テンプレートリテラル: バッククォート ` で囲み、${変数名} で値を埋め込める
//     greeting と name の中身が連結されて入る。
//     文字列連結を + で書くより読みやすい（"こんにちは" + "、" + name + "さん！" よりずっと短い）
const message: string = `${greeting}、${name}さん！`;

// (4) console.log は「ターミナルやブラウザのコンソールに値を表示する」関数
//     第1引数で渡された文字列を1行表示する。
console.log(message);
// ▼ 実行結果（ターミナルにこう出る）
// こんにちは、太郎さん！


// (5) テンプレートリテラルは改行もそのまま含められる
//     これが文字列の中に改行コード(\n)として保持される。
const multiLine: string = `
  1行目
  2行目
  3行目
`;
console.log(multiLine);
// ▼ 実行結果
// (空行)
//   1行目
//   2行目
//   3行目
// (空行)
```

```typescript
// ==========================================================================
// string 型でエラーになる例 — VS Code が赤線で警告してくれる
// ==========================================================================

// (1) 数値を string 型変数に入れようとしている → ❌
//     42 は数値リテラル。string が期待される場所には置けない。
const title: string = 42;
// ▼ エラーメッセージ
// Type 'number' is not assignable to type 'string'.
//   （number 型は string 型に代入できません）

// (2) 真偽値を string 型変数に入れようとしている → ❌
const flag: string = true;
// Type 'boolean' is not assignable to type 'string'.

// (3) null を string 型変数に入れようとしている → ❌
//     ※ tsconfig.json で "strictNullChecks": true のとき
//        null や undefined はそれぞれ独立した型として扱われる。
const nothing: string = null;
// Type 'null' is not assignable to type 'string'.
```

### 2.2 number（数値型）

整数・浮動小数点数の両方を含む数値型です。JavaScript と同様に、TypeScript の number はすべて浮動小数点数として扱われます。

```typescript
// ==========================================================================
// number 型のサンプル — 「数値（整数も小数も）」を意味する型
// ==========================================================================

// (1) 整数
const age: number = 25;

// (2) 小数（浮動小数点数）
const price: number = 1980.5;

// (3) 負の数
const negative: number = -10;

// (4) 16進数（0x で始まる）。Web開発で色コードを書くときに使う
//     0xff の f は 15 を意味する1桁の16進文字。ff = 15*16 + 15 = 255
const hex: number = 0xff;        // 256 - 1 = 255

// (5) 2進数（0b で始まる）。フラグ操作などで使う
//     0b1010 = 1*8 + 0*4 + 1*2 + 0*1 = 10
const binary: number = 0b1010;   // = 10

// (6) 8進数（0o で始まる）。Linuxのファイル権限で見る形
//     0o744 = 7*64 + 4*8 + 4*1 = 484
const octal: number = 0o744;     // = 484

// (7) 数値リテラル区切り（_ で読みやすくする）
//     1_000_000 と書いても 1000000 と書いても結果は同じ。実行時の値に _ は含まれない。
const big: number = 1_000_000;   // = 1000000（百万）

// (8) 特殊な数値
//     Infinity = 無限大、NaN = Not a Number（不正な数値計算の結果）
const inf: number = Infinity;
const notANumber: number = NaN;

console.log(age, price, negative, hex, binary, octal, big);
// ▼ 実行結果
// 25 1980.5 -10 255 10 484 1000000

console.log(1 / 0);     // → Infinity（0で割ると無限大）
console.log(0 / 0);     // → NaN（0÷0 は数学的に未定義）
console.log("a" * 2);   // → NaN（文字列を数値として掛けると NaN）
```

```typescript
// ==========================================================================
// number 型でエラーになる例
// ==========================================================================

// (1) 数字に見えるが文字列なのでダメ → ❌
const count: number = "5";
// Type 'string' is not assignable to type 'number'.

// (2) "100" + 50 は JavaScript的には string になる（"10050"になる）→ TS は型エラー扱い
const total: number = "100" + 50;
// Type 'string' is not assignable to type 'number'.
//
// ※ TypeScript はこれを書く前にエラーで止めてくれる。
//   JavaScript だけだと、後の計算で予想外のバグになる。

// (3) 数値型変数 a と文字列型変数 b を + するとどうなる?
//     → JavaScript では "10" + "20" のように文字列連結に化ける。
//     TypeScript は「これは number として使えませんよ」と止めてくれる。
const a: number = 10;
const b: string = "20";
const result: number = a + b;
// Type 'string' is not assignable to type 'number'.
```

### 2.3 boolean（真偽値型）

`true` または `false` のどちらかを持つ型です。

```typescript
// ==========================================================================
// boolean 型のサンプル — 「true か false」しか入らない型
// ==========================================================================

// (1) 直接 true / false を入れる
const isActive: boolean = true;       // true = 「アクティブ」
const isCompleted: boolean = false;   // false = 「未完了」

// (2) 比較式の結果も boolean になる
//     >= は「以上」、=== は「等しい（型もチェック）」
//     式が成り立てば true、成り立たなければ false が返る。
const isAdult: boolean = age >= 18;       // age が 25 なので true
const isEmpty: boolean = name === "";     // name が "太郎" なので false

// (3) 論理演算子（&& と ||）
//     && (AND) : 両方 true なら true、片方でも false なら false
//     || (OR)  : 片方でも true なら true、両方 false なら false
//     ! (NOT)  : true ↔ false を反転
const canAccess: boolean = isActive && isAdult;  // true && true = true

console.log(isActive, isCompleted, isAdult, isEmpty, canAccess);
// ▼ 実行結果
// true false true false true
```

```typescript
// ==========================================================================
// boolean 型でエラーになる例
// ==========================================================================

// (1) 数字 1 は「真っぽい値」だが boolean 型ではない → ❌
const flag: boolean = 1;
// Type 'number' is not assignable to type 'boolean'.

// (2) "true" は文字列であって、真偽値ではない → ❌
const active: boolean = "true";
// Type 'string' is not assignable to type 'boolean'.

// (3) 0 も「偽っぽい値」だが boolean 型ではない → ❌
//     JavaScript では if (0) {...} は実行されない（falsy扱い）が、
//     boolean 型としては number の 0 と boolean の false は別物。
const truthy: boolean = 0;
// Type 'number' is not assignable to type 'boolean'.
```

### 2.4 array（配列型）

同じ型の要素を複数格納するコレクションです。書き方は2通りあります。

```typescript
// ==========================================================================
// 配列型のサンプル — 「同じ型の値を順番に並べたリスト」
// ==========================================================================

// ──────────────────────────── 書き方1: 型名 + [] ───────────────────────────
// string[] は「string が並んだ配列」の意味。一番一般的な書き方。

// (1) 文字列の配列
const fruits: string[] = ["りんご", "みかん", "バナナ"];
//                          [0]       [1]       [2]   ← 添え字（0から始まる）

// (2) 数値の配列
const scores: number[] = [85, 90, 78, 92];

// (3) 真偽値の配列
const flags: boolean[] = [true, false, true];


// ──────────────────────────── 書き方2: Array<型名> ─────────────────────────
// 結果は [] 記法と同じ。読み手の好みで選ぶ。

const colors: Array<string> = ["赤", "青", "緑"];
const ages: Array<number> = [20, 25, 30];


// ──────────────────────────── 配列の主な操作 ───────────────────────────────

// (4) push: 配列の末尾に要素を追加（破壊的に変更される）
//     型が一致する値だけ受け付ける（string配列に number は入らない）
fruits.push("ぶどう");          // fruits = ["りんご", "みかん", "バナナ", "ぶどう"]
scores.push(100);               // scores = [85, 90, 78, 92, 100]

// (5) [添え字] でアクセス。存在しない添え字は undefined（厳密モードでは型エラー）
const first: string = fruits[0];  // = "りんご"
console.log(first);
// ▼ 実行結果
// りんご

// (6) 配列の長さを取る
console.log(fruits.length);
// ▼ 実行結果
// 4

// (7) 空配列を作るときも型注釈を書いておくのが安全
//     こうしないと never[] と推論されてしまい、後で push できない。
const emptyStrings: string[] = [];
emptyStrings.push("hello");      // OK
console.log(emptyStrings);
// ▼ 実行結果
// [ 'hello' ]
```

```typescript
// --- エラーになる例 ---
const numbers: number[] = [1, 2, "three"];
// エラー: Type 'string' is not assignable to type 'number'

const items: string[] = ["a", "b", "c"];
items.push(42);
// エラー: Argument of type 'number' is not assignable to parameter of type 'string'

// 異なる型が混在する配列を number[] として定義
const mixed: number[] = [1, true, "hello"];
// エラー: Type 'boolean' is not assignable to type 'number'
// エラー: Type 'string' is not assignable to type 'number'
```

### 2.5 tuple（タプル型）

**固定長で、各位置の型が決まっている配列** です。通常の配列と異なり、要素ごとに異なる型を持てます。

```typescript
// --- 正しい例 ---

// [名前, 年齢] のタプル
const person: [string, number] = ["田中", 30];

// [ID, 名前, アクティブフラグ] のタプル
const record: [number, string, boolean] = [1, "太郎", true];

// 要素へのアクセス（型が正しく推論される）
const personName: string = person[0]; // "田中" ← string 型
const personAge: number = person[1];  // 30 ← number 型

// 分割代入も可能
const [name, age] = person;
// name は string 型、age は number 型

// オプショナルなタプル要素
const optionalTuple: [string, number?] = ["田中"];
// 2番目の要素はあってもなくてもよい
```

```typescript
// --- エラーになる例 ---
const pair: [string, number] = [42, "田中"];
// エラー: Type 'number' is not assignable to type 'string'（1番目）
// エラー: Type 'string' is not assignable to type 'number'（2番目）

const triple: [string, number, boolean] = ["太郎", 25];
// エラー: Source has 2 element(s) but target requires 3

// 型が異なる位置へのアクセス
const data: [string, number] = ["hello", 42];
const wrongType: number = data[0];
// エラー: Type 'string' is not assignable to type 'number'
```

### 2.6 object（オブジェクト型）

キーと値のペアを持つ構造化されたデータ型です。

```typescript
// --- 正しい例 ---

// オブジェクトリテラル型
const user: { name: string; age: number; email: string } = {
  name: "田中太郎",
  age: 30,
  email: "tanaka@example.com",
};

// オプショナルプロパティ（? をつける）
const product: { name: string; price: number; description?: string } = {
  name: "TypeScript入門書",
  price: 2980,
  // description は省略可能
};

// 読み取り専用プロパティ
const config: { readonly apiUrl: string; readonly timeout: number } = {
  apiUrl: "https://api.example.com",
  timeout: 5000,
};

// ネストしたオブジェクト
const company: {
  name: string;
  address: {
    city: string;
    zipCode: string;
  };
} = {
  name: "サンプル株式会社",
  address: {
    city: "東京",
    zipCode: "100-0001",
  },
};
```

```typescript
// --- エラーになる例 ---
const user: { name: string; age: number } = {
  name: "田中",
  // age がない！
};
// エラー: Property 'age' is missing in type '{ name: string; }'

const item: { name: string; price: number } = {
  name: "ペン",
  price: 100,
  color: "赤", // 余分なプロパティ
};
// エラー: Object literal may only specify known properties,
// and 'color' does not exist in type '{ name: string; price: number; }'

// readonly プロパティへの再代入
const settings: { readonly theme: string } = { theme: "dark" };
settings.theme = "light";
// エラー: Cannot assign to 'theme' because it is a read-only property
```

### 2.7 any 型

**すべての型チェックを無効化する** 型です。どんな値でも受け入れ、どんな操作も許可します。

```typescript
// --- any の使用例（非推奨だが動く）---
let anything: any = "文字列";
anything = 42;           // OK（number を代入）
anything = true;         // OK（boolean を代入）
anything = [1, 2, 3];   // OK（配列を代入）
anything.foo();          // OK（存在しないメソッドも呼べる → 実行時エラー！）
anything.bar.baz;        // OK（存在しないプロパティもアクセス可能 → 実行時エラー！）
```

> **警告**: `any` を使うと TypeScript を使う意味がほぼなくなります。**原則として `any` は使わないでください。** やむを得ない場合（外部ライブラリの型定義がない場合など）のみ、限定的に使います。

```typescript
// --- なぜ any が危険か ---
function processData(data: any) {
  // 型チェックが一切行われない
  return data.toUpperCase(); // data が string でなければ実行時エラー
}

processData("hello");   // OK: "HELLO"
processData(42);         // 実行時エラー: data.toUpperCase is not a function
processData(null);       // 実行時エラー: Cannot read properties of null
```

### 2.8 unknown 型

`any` の安全な代替です。どんな値も代入できますが、**使う前に型チェックが必要** です。

```typescript
// --- unknown の正しい使い方 ---
let value: unknown = "こんにちは";
value = 42;       // OK（代入は自由）
value = true;     // OK

// ただし、そのまま使うことはできない
// const upper: string = value.toUpperCase();
// エラー: 'value' is of type 'unknown'

// 型チェック（型ガード）を行ってから使う
if (typeof value === "string") {
  // このブロック内では value は string 型として扱える
  console.log(value.toUpperCase()); // OK
}

if (typeof value === "number") {
  // このブロック内では value は number 型として扱える
  console.log(value.toFixed(2)); // OK
}
```

```typescript
// --- any と unknown の比較 ---

// any: 危険（型チェックなし）
function unsafeProcess(data: any): string {
  return data.toUpperCase(); // 実行時に壊れる可能性あり
}

// unknown: 安全（型チェック必須）
function safeProcess(data: unknown): string {
  if (typeof data === "string") {
    return data.toUpperCase(); // 型チェック済みなので安全
  }
  return "不明なデータ";
}
```

### 2.9 never 型

**決して発生しない値** の型です。主に以下の場面で使われます。

```typescript
// --- 用途1: 絶対に値を返さない関数 ---
function throwError(message: string): never {
  throw new Error(message);
  // この関数は必ず例外を投げるので、正常に値を返すことがない
}

// --- 用途2: 無限ループ ---
function infiniteLoop(): never {
  while (true) {
    // 永遠に終わらない
  }
}

// --- 用途3: 到達不可能なコードの検出（網羅性チェック）---
type Shape = "circle" | "square" | "triangle";

function getArea(shape: Shape): number {
  switch (shape) {
    case "circle":
      return Math.PI * 10 * 10;
    case "square":
      return 10 * 10;
    case "triangle":
      return (10 * 10) / 2;
    default:
      // すべてのケースを処理済みなら、shape は never 型になる
      const _exhaustiveCheck: never = shape;
      return _exhaustiveCheck;
  }
}
```

```typescript
// --- never 型の重要性: 網羅性チェック ---

// Shape に新しい種類を追加した場合
type Shape = "circle" | "square" | "triangle" | "pentagon"; // pentagon を追加

function getArea(shape: Shape): number {
  switch (shape) {
    case "circle":
      return Math.PI * 10 * 10;
    case "square":
      return 10 * 10;
    case "triangle":
      return (10 * 10) / 2;
    default:
      // "pentagon" のケースが未処理なのでエラーになる！
      const _exhaustiveCheck: never = shape;
      // エラー: Type 'string' is not assignable to type 'never'
      // → "pentagon" の処理を追加し忘れたことに気づける
      return _exhaustiveCheck;
  }
}
```

### 2.10 void 型

**値を返さない関数** の戻り値の型です。`never` と違い、関数自体は正常に終了します。

```typescript
// --- 正しい例 ---
function logMessage(message: string): void {
  console.log(message);
  // return 文がない、または return; のみ
}

function showAlert(text: string): void {
  alert(text);
  return; // return; は OK（値を返さない）
}

// void 型の変数には undefined のみ代入可能
const result: void = undefined;
```

```typescript
// --- エラーになる例 ---
function greet(name: string): void {
  return `こんにちは、${name}さん`;
  // エラー: Type 'string' is not assignable to type 'void'
  // void なのに値を返している
}

const value: void = "hello";
// エラー: Type 'string' is not assignable to type 'void'

const num: void = 42;
// エラー: Type 'number' is not assignable to type 'void'
```

### 2.11 null と undefined

`null` は「値が意図的に空」であることを表し、`undefined` は「値が未定義」であることを表します。

```typescript
// --- 正しい例 ---

// strictNullChecks が有効な場合（推奨）
let nullableString: string | null = null;   // 明示的に null を許可
let optionalValue: number | undefined = undefined;

// null チェック
function findUser(id: number): string | null {
  if (id === 1) {
    return "田中太郎";
  }
  return null; // ユーザーが見つからない場合
}

const user = findUser(999);
if (user !== null) {
  console.log(user.toUpperCase()); // null チェック後なので安全
}

// オプショナルチェイニング（?. ）
const length = user?.length; // user が null なら undefined を返す

// null 合体演算子（??）
const displayName = user ?? "ゲスト"; // user が null/undefined なら "ゲスト"
```

```typescript
// --- エラーになる例（strictNullChecks 有効時）---
let name: string = null;
// エラー: Type 'null' is not assignable to type 'string'

let age: number = undefined;
// エラー: Type 'undefined' is not assignable to type 'number'

// null チェックなしでのメソッド呼び出し
function findItem(id: number): string | null {
  return id > 0 ? "アイテム" : null;
}

const item = findItem(-1);
console.log(item.toUpperCase());
// エラー: 'item' is possibly 'null'
// → item が null の可能性があるので、チェックなしでは使えない
```

### 基本型のまとめ表

| 型 | 説明 | 例 | 使用場面 |
|---|------|---|---------|
| `string` | 文字列 | `"hello"`, `'world'` | テキストデータ全般 |
| `number` | 数値 | `42`, `3.14` | 数値計算、ID |
| `boolean` | 真偽値 | `true`, `false` | フラグ、条件 |
| `string[]` | 文字列配列 | `["a", "b"]` | リストデータ |
| `[string, number]` | タプル | `["太郎", 25]` | 固定構造のペア |
| `object` | オブジェクト | `{ key: "value" }` | 構造化データ |
| `any` | なんでも | すべて | **使用非推奨** |
| `unknown` | 不明な型 | すべて | 安全な any の代替 |
| `never` | 発生しない | なし | 網羅性チェック |
| `void` | 返り値なし | `undefined` | 関数の戻り値 |
| `null` | 空 | `null` | 意図的な空値 |
| `undefined` | 未定義 | `undefined` | 未初期化の値 |

---

## 3. 型注釈と型推論

### 3.1 明示的な型注釈

**型注釈（Type Annotation）** とは、変数や関数のパラメータ・戻り値に明示的に型を記述することです。コロン（`:`）の後に型を書きます。

```typescript
// 変数の型注釈
const bookTitle: string = "TypeScript入門";
const pageCount: number = 350;
const isPublished: boolean = true;

// 関数のパラメータと戻り値の型注釈
function calculateTotal(price: number, quantity: number): number {
  return price * quantity;
}

// アロー関数の型注釈
const add = (a: number, b: number): number => a + b;

// オブジェクトの型注釈
const book: { title: string; author: string; pages: number } = {
  title: "TypeScript入門",
  author: "山田太郎",
  pages: 350,
};

// 配列の型注釈
const tags: string[] = ["プログラミング", "TypeScript", "入門"];
```

### 3.2 型推論の仕組み

TypeScript は非常に賢い **型推論（Type Inference）** エンジンを持っています。多くの場合、型注釈を書かなくても TypeScript が自動的に型を判断してくれます。

```typescript
// --- TypeScript が自動で型を推論する例 ---

// 変数の初期化から推論
const message = "こんにちは";    // string と推論
const count = 42;                // number と推論
const isValid = true;            // boolean と推論
const items = [1, 2, 3];        // number[] と推論

// 関数の戻り値も推論される
function multiply(a: number, b: number) {
  return a * b; // 戻り値は number と推論される
}

// オブジェクトの構造も推論される
const user = {
  name: "田中",     // name: string
  age: 25,          // age: number
  isActive: true,   // isActive: boolean
};
// user の型は { name: string; age: number; isActive: boolean } と推論

// 配列のメソッドから推論
const doubled = [1, 2, 3].map((n) => n * 2);
// doubled は number[] と推論

// 条件式から推論
const status = count > 10 ? "many" : "few";
// status は string と推論
```

```typescript
// --- 型推論の注意点 ---

// let で宣言すると広い型に推論される
let color = "red";    // string と推論（"red" リテラル型ではない）
color = "blue";       // OK（string なので他の文字列も代入可能）

// const で宣言するとリテラル型に推論される
const direction = "north"; // "north" と推論（リテラル型）
// direction = "south";    // エラー: const なので再代入不可

// 空配列は any[] に推論される（要注意）
const emptyList = []; // any[] と推論
// → 型注釈をつけるべき
const emptyStrings: string[] = []; // 明示的に型を指定
```

### 3.3 いつ型注釈を書くべきか

<div style="max-width: 420px; margin: 20px auto; font-family: 'Segoe UI', sans-serif;">
  <!-- Step 1 -->
  <div style="background: #f1f5f9; border: 2px solid #94a3b8; border-radius: 10px; padding: 12px 18px; text-align: center; font-size: 13px; font-weight: 600; color: #334155;">変数・関数を書く</div>
  <div style="text-align: center; color: #64748b; font-size: 20px; line-height: 1.4;">▼</div>
  <!-- Decision 1 -->
  <div style="background: #eff6ff; border: 2px solid #3b82f6; border-radius: 10px; padding: 12px 18px; text-align: center; font-size: 13px; font-weight: 600; color: #1e40af;">TypeScript が正しく推論できる？</div>
  <div style="display: flex; justify-content: center; gap: 80px; margin-top: 4px;">
    <div style="text-align: center;">
      <div style="font-size: 11px; color: #16a34a; font-weight: 700;">はい ▼</div>
    </div>
    <div style="text-align: center;">
      <div style="font-size: 11px; color: #dc2626; font-weight: 700;">いいえ ▼</div>
    </div>
  </div>
  <div style="display: flex; justify-content: center; gap: 20px; margin-top: 6px;">
    <!-- Left branch -->
    <div style="flex: 1; max-width: 180px;">
      <div style="background: #eff6ff; border: 2px solid #3b82f6; border-radius: 10px; padding: 12px 14px; text-align: center; font-size: 12px; font-weight: 600; color: #1e40af;">コードの意図を<br>明確にしたい？</div>
      <div style="display: flex; justify-content: center; gap: 30px; margin-top: 4px;">
        <div style="font-size: 11px; color: #16a34a; font-weight: 700;">はい ▼</div>
        <div style="font-size: 11px; color: #dc2626; font-weight: 700;">いいえ ▼</div>
      </div>
      <div style="display: flex; gap: 6px; margin-top: 6px;">
        <div style="flex: 1; background: #3178c6; color: white; border-radius: 8px; padding: 10px 6px; text-align: center; font-size: 11px; font-weight: 600;">型注釈を書く</div>
        <div style="flex: 1; background: #16a34a; color: white; border-radius: 8px; padding: 10px 6px; text-align: center; font-size: 11px; font-weight: 600;">型推論に任せる</div>
      </div>
    </div>
    <!-- Right branch -->
    <div style="flex: 1; max-width: 180px; display: flex; align-items: flex-start; justify-content: center;">
      <div style="background: #3178c6; color: white; border-radius: 8px; padding: 12px 16px; text-align: center; font-size: 12px; font-weight: 600; box-shadow: 0 2px 8px rgba(49,120,198,0.3);">型注釈を書く</div>
    </div>
  </div>
</div>

#### 型注釈を書くべき場面

```typescript
// 1. 関数のパラメータ（必須！推論できない）
function greet(name: string, age: number): string {
  return `${name}さんは${age}歳です`;
}

// 2. 空配列を初期化する場合
const users: string[] = []; // 型注釈がないと any[] になる

// 3. 関数の戻り値を明確にしたい場合（公開APIなど）
function fetchUser(id: number): Promise<User> {
  // 戻り値の型が明確になり、実装ミスを防げる
  return fetch(`/api/users/${id}`).then((res) => res.json());
}

// 4. 複数の型を受け入れる場合
let result: string | number;
result = "成功";
result = 404;

// 5. オブジェクトの構造を明確にしたい場合
interface Config {
  apiUrl: string;
  timeout: number;
  retries: number;
}

const config: Config = {
  apiUrl: "https://api.example.com",
  timeout: 5000,
  retries: 3,
};
```

#### 型推論に任せてよい場面

```typescript
// 1. 初期値から型が明らかな場合
const name = "田中太郎";     // 明らかに string
const age = 30;              // 明らかに number
const isActive = true;       // 明らかに boolean

// 2. 配列リテラルで初期化する場合
const fruits = ["りんご", "みかん"]; // 明らかに string[]

// 3. 関数の戻り値が単純な場合
function add(a: number, b: number) {
  return a + b; // 明らかに number を返す
}

// 4. 変数の型が変わらない場合
const total = price * quantity; // 明らかに number
```

---

## 4. インターフェースと型エイリアス

### 4.1 interface の定義方法

`interface` はオブジェクトの「形（シェイプ）」を定義する方法です。

```typescript
// ==========================================================================
// interface の基本 — 「オブジェクトの設計図」を作る
// ==========================================================================
// interface は「このオブジェクトは こういう形をしてないとダメだよ」という
// ルールを宣言する仕組み。クラスの設計図のようなものを軽量に書ける。
// 大文字始まりにするのが慣習（クラス名と同じ）。

// (1) User という名前の interface を定義
//     「User 型のオブジェクトには id, name, email, age の4つが必須」と宣言
interface User {
  id: number;       // 数値型のID（必須）
  name: string;     // 文字列型の名前（必須）
  email: string;    // 文字列型のメールアドレス（必須）
  age: number;      // 数値型の年齢（必須）
}

// (2) User 型の変数を作る
//     プロパティが1つでも欠けるとエラー、余計なプロパティもエラー（厳格チェック）
const user: User = {
  id: 1,
  name: "田中太郎",
  email: "tanaka@example.com",
  age: 30,
};

console.log(user);
// ▼ 実行結果
// { id: 1, name: '田中太郎', email: 'tanaka@example.com', age: 30 }


// ==========================================================================
// オプショナルプロパティ「?」を使うと、あってもなくても良くなる
// ==========================================================================

interface Product {
  id: number;             // 必須
  name: string;           // 必須
  price: number;          // 必須
  description?: string;   // ←「?」付きなので省略可能。値が無いときは undefined
}

// description を省略しても OK
const pen: Product = {
  id: 1,
  name: "ボールペン",
  price: 150,
  // description は書いていないが、? なのでエラーにならない
};

console.log(pen.description);
// ▼ 実行結果
// undefined


// ==========================================================================
// readonly: 「あとから変えられないプロパティ」
// ==========================================================================

interface Config {
  readonly apiUrl: string;    // readonly = 後で代入禁止
  readonly timeout: number;
}

const config: Config = {
  apiUrl: "https://api.example.com",
  timeout: 5000,
};

// config.apiUrl = "https://other.com";   // ← これを書くとエラー
// ▼ エラー
// Cannot assign to 'apiUrl' because it is a read-only property.
//   （'apiUrl' は読み取り専用プロパティのため代入できません）
```

```typescript
// ==========================================================================
// interface の継承（extends）— 既存の型に項目を追加した新しい型を作る
// ==========================================================================

// (1) ベースとなる Animal 型
interface Animal {
  name: string;     // 動物の名前
  age: number;      // 年齢
}

// (2) Dog は Animal を「extends」している
//     → Animal の name, age を全て持ったうえで、breed と isVaccinated が追加される。
//     継承元の項目は書かなくてもOK（自動で引き継がれる）。
interface Dog extends Animal {
  breed: string;          // 犬種（Dog で追加）
  isVaccinated: boolean;  // ワクチン接種済みか（Dog で追加）
}

// (3) Dog 型のオブジェクトには合計4つのプロパティが必須
const myDog: Dog = {
  name: "ポチ",            // Animal 由来
  age: 3,                  // Animal 由来
  breed: "柴犬",           // Dog 固有
  isVaccinated: true,      // Dog 固有
};
console.log(myDog.name, myDog.breed);
// ▼ 実行結果
// ポチ 柴犬


// ==========================================================================
// 複数の interface を同時に継承（カンマ区切り）
// ==========================================================================
// 「ID を持つ」「タイムスタンプを持つ」のような共通の特徴を細かく分けて、
// 必要な組み合わせを extends で混ぜるのが現代的な設計。

// (1) 「ID を持つ」だけを表現する interface
interface HasId {
  id: number;
}

// (2) 「作成日時・更新日時を持つ」だけを表現する interface
interface HasTimestamp {
  createdAt: Date;     // Date は JavaScript 標準の日時オブジェクト
  updatedAt: Date;
}

// (3) BlogPost は HasId と HasTimestamp の両方を継承し、独自フィールドを追加
interface BlogPost extends HasId, HasTimestamp {
  title: string;       // タイトル
  content: string;     // 本文
  author: string;      // 著者
}

// (4) BlogPost 型のオブジェクトには6つのプロパティが必須
//     new Date("2025-01-01") は「2025年1月1日 0:00 UTC」を表す Date オブジェクト
const post: BlogPost = {
  id: 1,
  createdAt: new Date("2025-01-01"),
  updatedAt: new Date("2025-06-15"),
  title: "TypeScript入門",
  content: "TypeScriptの基礎を学びましょう",
  author: "田中太郎",
};

console.log(post.title, "投稿日:", post.createdAt.toISOString());
// ▼ 実行結果
// TypeScript入門 投稿日: 2025-01-01T00:00:00.000Z
```

```typescript
// interface でメソッドを定義
interface Calculator {
  add(a: number, b: number): number;
  subtract(a: number, b: number): number;
  multiply(a: number, b: number): number;
  divide(a: number, b: number): number;
}

const calc: Calculator = {
  add: (a, b) => a + b,
  subtract: (a, b) => a - b,
  multiply: (a, b) => a * b,
  divide: (a, b) => {
    if (b === 0) throw new Error("0で割ることはできません");
    return a / b;
  },
};
```

### 4.2 type の定義方法

`type`（型エイリアス）はあらゆる型に名前をつける方法です。

```typescript
// 基本的な type エイリアス
type UserName = string;
type Age = number;
type IsActive = boolean;

// オブジェクト型
type User = {
  id: number;
  name: string;
  email: string;
  age: number;
};

const user: User = {
  id: 1,
  name: "田中太郎",
  email: "tanaka@example.com",
  age: 30,
};

// ユニオン型（interface ではできない）
type Status = "active" | "inactive" | "pending";
type Id = string | number;

const userStatus: Status = "active";
const userId: Id = "user_123";

// タプル型
type Coordinate = [number, number];
type NameAge = [string, number];

const point: Coordinate = [35.6762, 139.6503]; // 東京の座標

// 関数型
type MathOperation = (a: number, b: number) => number;

const add: MathOperation = (a, b) => a + b;
const subtract: MathOperation = (a, b) => a - b;
```

```typescript
// 交差型（Intersection Type）
type HasName = {
  name: string;
};

type HasAge = {
  age: number;
};

type Person = HasName & HasAge;
// Person は { name: string; age: number; } と同じ

const person: Person = {
  name: "田中",
  age: 30,
};

// 条件付き型（Conditional Type）
type IsString<T> = T extends string ? "yes" : "no";

type A = IsString<string>;  // "yes"
type B = IsString<number>;  // "no"

// マップ型（Mapped Type）
type Readonly<T> = {
  readonly [P in keyof T]: T[P];
};

type ReadonlyUser = Readonly<User>;
// すべてのプロパティが readonly になる
```

### 4.3 interface vs type の使い分け

| 機能 | `interface` | `type` |
|-----|:-----------:|:------:|
| オブジェクトの形を定義 | OK | OK |
| 継承（extends） | OK | 交差型（`&`）で代替 |
| 実装（implements） | OK | OK |
| 宣言のマージ（Declaration Merging） | OK | 不可 |
| ユニオン型 | 不可 | OK |
| タプル型 | 不可 | OK |
| プリミティブ型のエイリアス | 不可 | OK |
| マップ型 | 不可 | OK |
| 条件付き型 | 不可 | OK |
| 計算されたプロパティ | 不可 | OK |

```typescript
// --- interface でしかできないこと: 宣言のマージ ---
interface Window {
  title: string;
}

interface Window {
  appVersion: number;
}

// 2つの宣言がマージされる
// Window = { title: string; appVersion: number; }

// type では同じ名前で再定義するとエラーになる
type Animal = { name: string };
type Animal = { age: number }; // エラー: Duplicate identifier 'Animal'
```

```typescript
// --- type でしかできないこと ---

// ユニオン型
type Result = "success" | "error" | "loading";

// プリミティブのエイリアス
type ID = string | number;

// タプル型
type Point = [number, number];

// 関数型
type Formatter = (input: string) => string;

// 条件付き型
type NonNullable<T> = T extends null | undefined ? never : T;
```

**使い分けの指針**:

- **interface を使う場面**: オブジェクトの形状を定義する場合（特にクラスが実装する場合や、ライブラリの型を拡張したい場合）
- **type を使う場面**: ユニオン型、タプル型、関数型、プリミティブ型エイリアスなど、interface では表現できない型を定義する場合

> **実務でのコツ**: チーム内で統一するのが最も重要です。迷ったら **interface をデフォルトで使い、interface で表現できない場合のみ type を使う** という方針が一般的です。

### 4.4 書籍管理アプリで使う型の定義例

実際のアプリケーション開発を想定して、書籍管理アプリに必要な型を定義してみましょう。**初心者でもわかるように、ほぼ全行に解説コメントを付けています。**

```typescript
// ============================================================================
// 書籍管理アプリの型定義（詳細コメント版）
// ============================================================================
// このファイルはアプリ全体で使う「データの形」を集めた辞書のようなもの。
// ここで型を1度作っておくと、関数の引数や useState の中身など
// あらゆる場所で同じ型を再利用でき、間違いを TypeScript が指摘してくれる。
// ============================================================================


// ----------------------------------------------------------------------------
// (1) 「読書ステータス」の型
// ----------------------------------------------------------------------------
// 「ユニオン型」（| で区切って列挙）を使い、値を3つの文字列のどれかに限定する。
// こうすると status = "yomu" のような typo が即座にエラーになる。
type ReadingStatus = "want-to-read" | "reading" | "completed";
//                       ↑読みたい     ↑読書中    ↑読了


// ----------------------------------------------------------------------------
// (2) 「評価」の型 — 1〜5 の数値リテラルだけを許す
// ----------------------------------------------------------------------------
// number だけだと 0 や 100 も入ってしまう。1〜5 のいずれかに限定したいので、
// 数値リテラル型のユニオンで宣言する。
type Rating = 1 | 2 | 3 | 4 | 5;


// ----------------------------------------------------------------------------
// (3) カテゴリの型 — 列挙したいときの定石
// ----------------------------------------------------------------------------
// 各値の右側のコメントは「人間用のメモ」。実行時には消える。
type BookCategory =
  | "fiction"         // 小説
  | "non-fiction"     // ノンフィクション
  | "technology"      // 技術書
  | "business"        // ビジネス
  | "self-help"       // 自己啓発
  | "other";          // その他


// ----------------------------------------------------------------------------
// (4) 著者情報の型 — interface で「オブジェクトの形」を定義
// ----------------------------------------------------------------------------
// `?` を付けたプロパティは「あってもなくてもいい」（オプショナル）
interface Author {
  id: string;            // 著者のユニークID（必須）
  name: string;          // 著者名（必須）
  nationality?: string;  // 国籍（省略可能。? が付く）
}


// ----------------------------------------------------------------------------
// (5) 書籍の基本情報
// ----------------------------------------------------------------------------
// `author` プロパティは前で定義した Author 型を再利用している。
// このように型を入れ子にできるのが TypeScript の便利なところ。
interface Book {
  id: string;                 // 書籍のユニークID
  title: string;              // タイトル
  author: Author;             // 著者（Author 型のオブジェクト）
  isbn: string;               // ISBN番号（書籍の世界共通ID）
  category: BookCategory;     // カテゴリ（前述の6種類のどれか）
  pages: number;              // 総ページ数
  publishedDate: string;      // 出版日。ISO 8601 形式 "2025-01-15"
  coverImageUrl?: string;     // 表紙画像URL（省略可能）
  description?: string;       // 説明文（省略可能）
}


// ----------------------------------------------------------------------------
// (6) 読書記録の型 — Book を「継承」して項目を追加
// ----------------------------------------------------------------------------
// `extends Book` は「Book のすべての項目をそのまま受け継いだうえで、追加項目を持つ」
// という意味。継承を使うと「Book にこれだけ足したやつ」という意図が明確になる。
interface ReadingRecord extends Book {
  status: ReadingStatus;      // 読書ステータス（必須）
  rating?: Rating;            // 評価（読了後にのみ設定するためオプショナル）
  startDate?: string;         // 読み始めた日
  endDate?: string;           // 読み終わった日
  notes?: string;             // メモ
  currentPage?: number;       // 現在のページ（読書中のみ意味を持つ）
}


// ----------------------------------------------------------------------------
// (7) ユーティリティ型 Omit / Partial の活用
// ----------------------------------------------------------------------------
// 同じことを毎回手書きすると面倒なので、TypeScript の組み込みユーティリティを使う。

// `Omit<T, K>` = T から K プロパティを抜いた型を作る。
// 新規作成時は id がまだ存在しないので、Book から id を抜く。
type CreateBookInput = Omit<Book, "id">;
// 結果として「id 以外の全フィールドが必須」の型ができる。

// `Partial<T>` = T のすべてのプロパティを「オプショナル」にした型。
// 更新時は「変えたい項目だけ」送れば良いので、全項目を ? 付きにしたい。
type UpdateBookInput = Partial<Omit<Book, "id">>;
// 結果として「id 以外の全フィールドが省略可」の型ができる。


// ----------------------------------------------------------------------------
// (8) フィルター条件の型 — 検索・並び替えのオプション
// ----------------------------------------------------------------------------
// すべて省略可（?）にして、ユーザーが指定したものだけ使うパターン。
interface BookFilter {
  status?: ReadingStatus;     // ステータスで絞る
  category?: BookCategory;    // カテゴリで絞る
  searchQuery?: string;       // タイトルまたは著者名で検索
  sortBy?: "title" | "publishedDate" | "rating";   // 並び替え基準
  sortOrder?: "asc" | "desc"; // asc=昇順 (1→9, A→Z), desc=降順 (9→1, Z→A)
}


// ----------------------------------------------------------------------------
// (9) ジェネリクスを使った API レスポンス型
// ----------------------------------------------------------------------------
// `<T>` は「あとで決まる型のプレースホルダー」。
// 例えば `ApiResponse<Book>` と書けば T = Book になり、
// `ApiResponse<Author>` と書けば T = Author になる。
// 「成功・失敗の枠組みは同じだが、データの中身は呼び出し側で決まる」を表現するのに最適。
interface ApiResponse<T> {
  data: T;                    // 取得したデータ本体（型は呼び出し側次第）
  success: boolean;           // 成否
  message?: string;           // メッセージ（省略可）
}


// ----------------------------------------------------------------------------
// (10) ページネーション付きレスポンス
// ----------------------------------------------------------------------------
// 一覧取得APIは「データ配列＋ページ情報」をセットで返すのが定番。
// data: T[] の T[] は「T の配列」を意味する記法。
interface PaginatedResponse<T> {
  data: T[];                  // 1ページ分のデータ配列
  total: number;              // 全件数
  page: number;               // 現在のページ番号（1始まり）
  perPage: number;            // 1ページあたりの件数
  totalPages: number;         // 総ページ数
}

// ===== 使用例 =====

// 書籍の作成
const newBook: CreateBookInput = {
  title: "TypeScript実践ガイド",
  author: {
    id: "author_001",
    name: "山田太郎",
    nationality: "日本",
  },
  isbn: "978-4-XXXX-XXXX-X",
  category: "technology",
  pages: 400,
  publishedDate: "2025-06-01",
  description: "TypeScriptの基礎から実践まで学べる一冊",
};

// 読書記録
const myReading: ReadingRecord = {
  id: "book_001",
  title: "TypeScript実践ガイド",
  author: {
    id: "author_001",
    name: "山田太郎",
  },
  isbn: "978-4-XXXX-XXXX-X",
  category: "technology",
  pages: 400,
  publishedDate: "2025-06-01",
  status: "reading",
  startDate: "2025-07-01",
  currentPage: 150,
  notes: "第5章まで読了。ジェネリクスの説明がわかりやすい。",
};

// フィルターを使った検索
const filter: BookFilter = {
  status: "reading",
  category: "technology",
  sortBy: "title",
  sortOrder: "asc",
};

// API レスポンスの型適用
const response: ApiResponse<ReadingRecord> = {
  data: myReading,
  success: true,
  message: "書籍情報を取得しました",
};

// ページネーション付きレスポンス
const listResponse: PaginatedResponse<ReadingRecord> = {
  data: [myReading],
  total: 1,
  page: 1,
  perPage: 10,
  totalPages: 1,
};
```

---

## 5. ジェネリクス

### 5.1 基本概念

**ジェネリクス（Generics）** は、型を「パラメータ化」する仕組みです。関数やクラス、インターフェースを定義する時点では型を決めず、使う時点で具体的な型を指定できます。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif;">
  <div style="display: flex; align-items: center; gap: 16px;">
    <!-- Generic function box -->
    <div style="background: #7c3aed; color: white; border-radius: 12px; padding: 18px 20px; text-align: center; min-width: 180px; box-shadow: 0 2px 10px rgba(124,58,237,0.3);">
      <div style="font-size: 13px; font-weight: 700;">ジェネリック関数</div>
      <div style="font-size: 12px; margin-top: 6px; font-family: 'Consolas', monospace; background: rgba(255,255,255,0.15); border-radius: 6px; padding: 6px 8px;">identity&lt;T&gt;(arg: T): T</div>
    </div>
    <!-- Arrows + results -->
    <div style="display: flex; flex-direction: column; gap: 8px;">
      <div style="display: flex; align-items: center; gap: 10px;">
        <div style="color: #7c3aed; font-size: 20px; font-weight: 700;">→</div>
        <div style="background: #2563eb; color: white; border-radius: 10px; padding: 10px 16px; font-size: 12px; box-shadow: 0 2px 6px rgba(37,99,235,0.25);">
          <span style="font-family: 'Consolas', monospace;">identity&lt;string&gt;('hello')</span><br>
          <span style="font-size: 11px; opacity: 0.85;">→ string を返す</span>
        </div>
      </div>
      <div style="display: flex; align-items: center; gap: 10px;">
        <div style="color: #7c3aed; font-size: 20px; font-weight: 700;">→</div>
        <div style="background: #16a34a; color: white; border-radius: 10px; padding: 10px 16px; font-size: 12px; box-shadow: 0 2px 6px rgba(22,163,74,0.25);">
          <span style="font-family: 'Consolas', monospace;">identity&lt;number&gt;(42)</span><br>
          <span style="font-size: 11px; opacity: 0.85;">→ number を返す</span>
        </div>
      </div>
      <div style="display: flex; align-items: center; gap: 10px;">
        <div style="color: #7c3aed; font-size: 20px; font-weight: 700;">→</div>
        <div style="background: #d97706; color: white; border-radius: 10px; padding: 10px 16px; font-size: 12px; box-shadow: 0 2px 6px rgba(217,119,6,0.25);">
          <span style="font-family: 'Consolas', monospace;">identity&lt;boolean&gt;(true)</span><br>
          <span style="font-size: 11px; opacity: 0.85;">→ boolean を返す</span>
        </div>
      </div>
    </div>
  </div>
</div>

```typescript
// --- ジェネリクスなしの場合（問題あり）---

// 方法1: any を使う → 型安全性が失われる
function identityAny(arg: any): any {
  return arg;
}
const result1 = identityAny("hello"); // result1 は any 型（string 情報が失われる）

// 方法2: 型ごとに関数を作る → コードの重複
function identityString(arg: string): string {
  return arg;
}
function identityNumber(arg: number): number {
  return arg;
}
```

```typescript
// ============================================================================
// ジェネリクスを使った場合 — 「型を変数化」して1つの関数で全てに対応
// ============================================================================

// `<T>` の T は「Type」の頭文字。慣習的によく使われる。
// 「この関数は何かの型 T を受け取り、同じ T を返すよ」という宣言。
//
// arg: T  → 引数の型は T
// : T     → 戻り値の型も T（引数と同じ型）
function identity<T>(arg: T): T {
  return arg;  // 受け取ったものをそのまま返す
}

// ─────────── 使用例(1): 型を明示的に指定する ───────────
// `<string>` のように山カッコで型を渡すことを「型引数を渡す」と呼ぶ。
const str = identity<string>("hello");
//          ↑ T = string と決まるので、引数は string 限定、戻り値も string
console.log(str);
// ▼ 実行結果
// hello
// ▼ str の型: string
// （number を渡そうとするとコンパイルエラー: identity<string>(42) は ❌）

const num = identity<number>(42);
console.log(num);
// ▼ 実行結果
// 42
// ▼ num の型: number

const bool = identity<boolean>(true);
console.log(bool);
// ▼ 実行結果
// true
// ▼ bool の型: boolean


// ─────────── 使用例(2): 型を省略する（型推論にお任せ） ───────────
// 引数の値 "world" を見て、TypeScript が「あ、T = string ですね」と自動推論する。
// 多くの場合、型引数を書かなくてOK。
const inferred = identity("world");
console.log(inferred);
// ▼ 実行結果
// world
// ▼ inferred の型: string（自動推論された）
```

> **ジェネリクスのありがたみ:** 上の `identity` 関数は、もし型を毎回手書きだったら `identityString` `identityNumber` `identityBoolean`... と無限に増えてしまう。`<T>` 1つで全てに対応できる。配列メソッドの `Array.prototype.map<U>(callback)` などはまさにこの仕組みで動いている。

```typescript
// --- ジェネリクスの慣習的な型パラメータ名 ---
// T: Type（一般的な型）
// U: 2番目の型パラメータ
// K: Key（オブジェクトのキー）
// V: Value（オブジェクトの値）
// E: Element（要素）
// R: Return（戻り値）

// 複数の型パラメータ
function pair<T, U>(first: T, second: U): [T, U] {
  return [first, second];
}

const p1 = pair<string, number>("hello", 42);   // [string, number]
const p2 = pair("田中", true);                   // [string, boolean]（型推論）
```

### 5.2 実用的な例

#### 例1: 配列操作のユーティリティ関数

```typescript
// 配列の最初の要素を取得する関数
function getFirst<T>(arr: T[]): T | undefined {
  return arr[0];
}

const firstFruit = getFirst(["りんご", "みかん"]);  // string | undefined
const firstNum = getFirst([10, 20, 30]);             // number | undefined

// 配列の最後の要素を取得する関数
function getLast<T>(arr: T[]): T | undefined {
  return arr.length > 0 ? arr[arr.length - 1] : undefined;
}

// 配列をシャッフルする関数
function shuffle<T>(arr: T[]): T[] {
  const result = [...arr];
  for (let i = result.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [result[i], result[j]] = [result[j], result[i]];
  }
  return result;
}

const shuffledFruits = shuffle(["りんご", "みかん", "バナナ"]); // string[]
const shuffledNums = shuffle([1, 2, 3, 4, 5]);                 // number[]
```

#### 例2: ジェネリックなインターフェース

```typescript
// API レスポンスの汎用型
interface ApiResponse<T> {
  data: T;
  status: number;
  message: string;
  timestamp: Date;
}

// ユーザーデータ用のレスポンス
interface User {
  id: number;
  name: string;
  email: string;
}

const userResponse: ApiResponse<User> = {
  data: {
    id: 1,
    name: "田中太郎",
    email: "tanaka@example.com",
  },
  status: 200,
  message: "成功",
  timestamp: new Date(),
};

// 書籍データ用のレスポンス
interface Book {
  id: string;
  title: string;
}

const bookResponse: ApiResponse<Book[]> = {
  data: [
    { id: "1", title: "TypeScript入門" },
    { id: "2", title: "React実践ガイド" },
  ],
  status: 200,
  message: "成功",
  timestamp: new Date(),
};
```

#### 例3: 制約付きジェネリクス（extends）

```typescript
// T は必ず length プロパティを持つ型に制限
function getLength<T extends { length: number }>(arg: T): number {
  return arg.length;
}

getLength("hello");         // OK: string は length を持つ → 5
getLength([1, 2, 3]);       // OK: 配列は length を持つ → 3
getLength({ length: 10 });  // OK: length プロパティがある → 10

// getLength(42);
// エラー: Argument of type 'number' is not assignable to
// parameter of type '{ length: number; }'
// → number には length プロパティがない

// オブジェクトのキーに制限をかける
function getProperty<T, K extends keyof T>(obj: T, key: K): T[K] {
  return obj[key];
}

const user = { name: "田中", age: 30, email: "tanaka@example.com" };

const name = getProperty(user, "name");   // string
const age = getProperty(user, "age");     // number

// getProperty(user, "address");
// エラー: Argument of type '"address"' is not assignable to
// parameter of type '"name" | "age" | "email"'
```

#### 例4: ジェネリックなクラス

```typescript
// スタック（Last In, First Out）のデータ構造
class Stack<T> {
  private items: T[] = [];

  push(item: T): void {
    this.items.push(item);
  }

  pop(): T | undefined {
    return this.items.pop();
  }

  peek(): T | undefined {
    return this.items[this.items.length - 1];
  }

  isEmpty(): boolean {
    return this.items.length === 0;
  }

  size(): number {
    return this.items.length;
  }
}

// 数値スタック
const numberStack = new Stack<number>();
numberStack.push(10);
numberStack.push(20);
numberStack.push(30);
console.log(numberStack.pop()); // 30
console.log(numberStack.peek()); // 20

// 文字列スタック
const stringStack = new Stack<string>();
stringStack.push("TypeScript");
stringStack.push("React");
console.log(stringStack.pop()); // "React"
```

---

## 6. ユニオン型とリテラル型

### 6.1 ユニオン型

**ユニオン型（Union Type）** は、複数の型のいずれかを受け入れる型です。`|`（パイプ）で型を区切ります。

```typescript
// ==========================================================================
// ユニオン型の基本 — 「A型 または B型」を受け入れる型
// ==========================================================================

// (1) string 型 または number 型を入れられる変数を宣言する
//     let は const と違って後から再代入できる。
//     | は「または」の意味（AND ではなく OR）。
let value: string | number;

value = "hello";  // OK（string なので許可）
value = 42;       // OK（number なので許可）
// value = true;
// ▼ エラー
// Type 'boolean' is not assignable to type 'string | number'.
//   （boolean は string|number に代入不可）


// ==========================================================================
// 関数の引数にユニオン型を使う場合の「型の絞り込み」
// ==========================================================================
// ユニオン型のままでは「両方の型に共通するメソッドしか」呼べない。
// typeof 演算子などで「いま中身が何の型か」を確かめてから使う。
// この技法を「型ガード（Type Guard）」と呼ぶ。

function formatId(id: string | number): string {
  // (1) typeof 演算子は「値の型を文字列で返す」演算子
  //     文字列なら "string"、数値なら "number" を返す。
  if (typeof id === "string") {
    // (2) この if ブロックの中だけ、TS は id を string として扱ってくれる。
    //     なので string 専用メソッド .toUpperCase() が使える。
    return id.toUpperCase();   // 例: "abc" → "ABC"
  } else {
    // (3) else 側では「string ではない」=「number しか残らない」と推論される。
    //     .toString() は数値を文字列に変換するメソッド。
    //     .padStart(5, "0") は「5文字に満たないなら左を 0 で埋める」メソッド。
    return id.toString().padStart(5, "0");  // 例: 42 → "00042"
  }
}

console.log(formatId("abc"));
// ▼ 実行結果
// ABC

console.log(formatId(42));
// ▼ 実行結果
// 00042


// ==========================================================================
// 配列のユニオン型 — 配列の中に複数の型が混ざってもOKにする
// ==========================================================================
// 注意: () で囲まないと意味が変わる！
//   string | number[]   → 「string 1個」または「number の配列」
//   (string | number)[] → 「string と number が混在した配列」（こちらが正しい）
const mixed: (string | number)[] = [1, "two", 3, "four"];
console.log(mixed);
// ▼ 実行結果
// [ 1, 'two', 3, 'four' ]
```

### 6.2 リテラル型

**リテラル型（Literal Type）** は、特定の値のみを許可する型です。

```typescript
// ==========================================================================
// 文字列リテラル型 — 「決められた文字列のどれか」だけを許す型
// ==========================================================================
// type で名前を付け、許可する値を | で並べる。Enum の代わりによく使う。

type Direction = "north" | "south" | "east" | "west";
//                 ↑ この4つの文字列以外は受け付けない

let dir: Direction = "north";   // OK（許可されている値）
// dir = "up";
// ▼ エラー
// Type '"up"' is not assignable to type 'Direction'.

console.log(dir);
// ▼ 実行結果
// north


// ==========================================================================
// 数値リテラル型 — 「決められた数値のどれか」だけを許す型
// ==========================================================================
// サイコロの目（1〜6）のように値が固定の場面で使う。
type DiceRoll = 1 | 2 | 3 | 4 | 5 | 6;

let roll: DiceRoll = 3;   // OK
// roll = 7;
// ▼ エラー
// Type '7' is not assignable to type 'DiceRoll'.


// ==========================================================================
// 真偽値リテラル型 — true だけ、または false だけを許す
// ==========================================================================
// 滅多に使わないが、特定の値しか取らないフラグを表現したいとき便利。
type True = true;
let flag: True = true;    // OK
// flag = false;          // ▼ エラー: Type 'false' is not assignable to type 'true'.
```

### 6.3 書籍管理アプリでの実践例

```typescript
// ===== 読書ステータスの定義 =====

type ReadingStatus = "want-to-read" | "reading" | "completed";

interface Book {
  id: string;
  title: string;
  author: string;
  status: ReadingStatus;
}

// --- 正しい使い方 ---
const book1: Book = {
  id: "1",
  title: "TypeScript入門",
  author: "山田太郎",
  status: "reading",
};

const book2: Book = {
  id: "2",
  title: "React実践ガイド",
  author: "佐藤花子",
  status: "completed",
};

const book3: Book = {
  id: "3",
  title: "Next.js入門",
  author: "鈴木一郎",
  status: "want-to-read",
};

// --- エラーになる例 ---
const invalidBook: Book = {
  id: "4",
  title: "間違った書籍",
  author: "テスト太郎",
  status: "finished", // エラー！
  // Type '"finished"' is not assignable to type 'ReadingStatus'
  // → "want-to-read" | "reading" | "completed" のどれかにしてください
};
```

```typescript
// ===== ステータスに応じた処理 =====

function getStatusLabel(status: ReadingStatus): string {
  switch (status) {
    case "want-to-read":
      return "読みたい";
    case "reading":
      return "読書中";
    case "completed":
      return "読了";
  }
  // すべてのケースを処理しているため、default 不要
  // TypeScript がすべてのケースが網羅されていることを保証
}

function getStatusColor(status: ReadingStatus): string {
  switch (status) {
    case "want-to-read":
      return "#f39c12"; // オレンジ
    case "reading":
      return "#3498db"; // 青
    case "completed":
      return "#27ae60"; // 緑
  }
}

function getStatusIcon(status: ReadingStatus): string {
  const icons: Record<ReadingStatus, string> = {
    "want-to-read": "bookmark",
    reading: "book-open",
    completed: "check-circle",
  };
  return icons[status];
}

// ===== フィルタリング =====

function filterBooksByStatus(
  books: Book[],
  status: ReadingStatus
): Book[] {
  return books.filter((book) => book.status === status);
}

const allBooks: Book[] = [book1, book2, book3];
const readingBooks = filterBooksByStatus(allBooks, "reading");
const completedBooks = filterBooksByStatus(allBooks, "completed");

// ===== ステータスの変更 =====

function updateBookStatus(
  book: Book,
  newStatus: ReadingStatus
): Book {
  return { ...book, status: newStatus };
}

const updatedBook = updateBookStatus(book1, "completed");
console.log(updatedBook.status); // "completed"

// 無効なステータスへの変更はコンパイルエラー
// updateBookStatus(book1, "abandoned");
// エラー: Argument of type '"abandoned"' is not assignable to type 'ReadingStatus'
```

```typescript
// ===== 判別可能なユニオン型（Discriminated Union）=====

// ステータスに応じて異なる追加情報を持つ型
type BookWithDetails =
  | {
      status: "want-to-read";
      title: string;
      reason: string;  // 読みたい理由
    }
  | {
      status: "reading";
      title: string;
      currentPage: number;
      totalPages: number;
    }
  | {
      status: "completed";
      title: string;
      rating: 1 | 2 | 3 | 4 | 5;
      review?: string;
    };

function displayBookInfo(book: BookWithDetails): string {
  switch (book.status) {
    case "want-to-read":
      // ここでは book.reason にアクセス可能
      return `「${book.title}」を読みたい（理由: ${book.reason}）`;

    case "reading":
      // ここでは book.currentPage, book.totalPages にアクセス可能
      const progress = Math.round(
        (book.currentPage / book.totalPages) * 100
      );
      return `「${book.title}」を読書中（進捗: ${progress}%）`;

    case "completed":
      // ここでは book.rating, book.review にアクセス可能
      const stars = "★".repeat(book.rating) + "☆".repeat(5 - book.rating);
      return `「${book.title}」読了 ${stars}`;
  }
}

// 使用例
console.log(
  displayBookInfo({
    status: "reading",
    title: "TypeScript入門",
    currentPage: 150,
    totalPages: 400,
  })
);
// => 「TypeScript入門」を読書中（進捗: 38%）

console.log(
  displayBookInfo({
    status: "completed",
    title: "React実践ガイド",
    rating: 4,
    review: "とても実践的でした",
  })
);
// => 「React実践ガイド」読了 ★★★★☆
```

---

## 7. TypeScript の設定（tsconfig.json）

### 7.1 tsconfig.json とは

`tsconfig.json` は TypeScript プロジェクトの設定ファイルです。コンパイラのオプション、対象ファイル、出力先などを指定します。

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "lib": ["ES2022", "DOM", "DOM.Iterable"],
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist"]
}
```

### 7.2 主要な設定項目

| 設定項目 | 説明 | 推奨値 | 備考 |
|---------|------|--------|------|
| `target` | コンパイル先の JavaScript バージョン | `"ES2022"` | モダンブラウザ対象なら ES2022 以降 |
| `module` | モジュールシステム | `"ESNext"` | ES Modules を使用 |
| `lib` | 使用する型定義ライブラリ | `["ES2022", "DOM"]` | ブラウザ用に DOM を含める |
| `strict` | すべての厳格チェックを有効化 | `true` | **必ず true にする** |
| `noEmit` | JavaScript ファイルを出力しない | `true` | Next.js 等のバンドラーを使う場合 |
| `jsx` | JSX の処理方法 | `"react-jsx"` | React 17+ の新しい JSX Transform |
| `esModuleInterop` | CommonJS/ESModule の相互運用 | `true` | import 構文の互換性向上 |
| `skipLibCheck` | 型定義ファイルのチェックを省略 | `true` | コンパイル速度の向上 |
| `forceConsistentCasingInFileNames` | ファイル名の大文字小文字を区別 | `true` | OS 間の差異を防止 |
| `resolveJsonModule` | JSON ファイルの import を許可 | `true` | JSON をモジュールとして読み込める |
| `isolatedModules` | ファイル単位のトランスパイルを保証 | `true` | Babel 等との互換性 |
| `baseUrl` | モジュール解決のベースパス | `"."` | パスエイリアスの基準 |
| `paths` | パスエイリアスの定義 | `{"@/*": ["./src/*"]}` | `@/components` のような短縮パス |

#### strict モードに含まれる個別オプション

`"strict": true` を設定すると、以下のオプションがすべて有効になります。

| オプション | 説明 |
|-----------|------|
| `strictNullChecks` | `null` / `undefined` を他の型と区別する |
| `strictFunctionTypes` | 関数の引数の型チェックを厳密にする |
| `strictBindCallApply` | `bind`, `call`, `apply` の引数を厳密にチェック |
| `strictPropertyInitialization` | クラスプロパティの初期化を必須にする |
| `noImplicitAny` | 暗黙の `any` 型を禁止する |
| `noImplicitThis` | 暗黙の `this` 型を禁止する |
| `alwaysStrict` | JavaScript の strict mode を有効にする |
| `useUnknownInCatchVariables` | catch の変数を `unknown` 型にする |

### 7.3 Next.js での TypeScript 設定

Next.js プロジェクトでは、`next.config.ts` と `tsconfig.json` の両方で TypeScript の設定を行います。Next.js が自動的に最適な `tsconfig.json` を生成してくれます。

```json
// Next.js プロジェクトの tsconfig.json（自動生成される）
{
  "compilerOptions": {
    "target": "ES2017",
    "lib": ["dom", "dom.iterable", "esnext"],
    "allowJs": true,
    "skipLibCheck": true,
    "strict": true,
    "noEmit": true,
    "esModuleInterop": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "jsx": "preserve",
    "incremental": true,
    "plugins": [
      {
        "name": "next"
      }
    ],
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": [
    "next-env.d.ts",
    "**/*.ts",
    "**/*.tsx",
    ".next/types/**/*.ts"
  ],
  "exclude": ["node_modules"]
}
```

> **ポイント**: Next.js では `"jsx": "preserve"` が使われます。これは JSX の変換を Next.js（SWC）に任せるためです。`"noEmit": true` なので TypeScript コンパイラ自身は JavaScript を出力しません。

```typescript
// Next.js 特有の型の例

// ページコンポーネントの型
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "書籍管理アプリ",
  description: "あなたの読書を管理するアプリです",
};

export default function HomePage() {
  return <h1>書籍管理アプリ</h1>;
}

// Server Component のデータ取得
interface PageProps {
  params: Promise<{ id: string }>;
  searchParams: Promise<{ [key: string]: string | string[] | undefined }>;
}

export default async function BookPage({ params }: PageProps) {
  const { id } = await params;
  // サーバーサイドでデータを取得
  const book = await fetchBook(id);
  return <div>{book.title}</div>;
}
```

---

## 8. よくあるエラーと対処法

TypeScript を使い始めると、必ず遭遇するエラーがあります。ここでは代表的なエラーとその解決方法を詳しく解説します。

### 8.1 Type 'X' is not assignable to type 'Y'

**最も頻出するエラー** です。ある型の値を、互換性のない別の型に代入しようとした際に発生します。

```typescript
// ===== エラーパターン1: プリミティブ型の不一致 =====

// 悪い例
const age: number = "25";
// エラー: Type 'string' is not assignable to type 'number'

// 良い例
const age: number = 25;
// または、文字列から変換する場合
const age: number = parseInt("25", 10);
const age: number = Number("25");
```

```typescript
// ===== エラーパターン2: オブジェクト型の不一致 =====

interface User {
  name: string;
  age: number;
}

// 悪い例
const user: User = {
  name: "田中",
  age: "30", // string を number に代入
};
// エラー: Type 'string' is not assignable to type 'number'

// 良い例
const user: User = {
  name: "田中",
  age: 30,
};
```

```typescript
// ===== エラーパターン3: ユニオン型のリテラルが一致しない =====

type Status = "active" | "inactive";

// 悪い例
const status: Status = "enabled";
// エラー: Type '"enabled"' is not assignable to type 'Status'

// 良い例
const status: Status = "active";
```

```typescript
// ===== エラーパターン4: 変数の型が広すぎる =====

type Color = "red" | "blue" | "green";

// 悪い例
let colorName = "red"; // string と推論される
const color: Color = colorName;
// エラー: Type 'string' is not assignable to type 'Color'

// 良い例（方法1: as const を使う）
const colorName = "red" as const; // "red" リテラル型と推論
const color: Color = colorName;   // OK

// 良い例（方法2: 型注釈を使う）
const colorName: Color = "red";

// 良い例（方法3: satisfies を使う）
const colorName = "red" satisfies Color; // "red" リテラル型かつ Color として検証
```

### 8.2 Object is possibly 'undefined'

`strictNullChecks` が有効な場合に発生する、**null や undefined の可能性がある値にアクセスしようとしたとき**のエラーです。

```typescript
// ===== エラーパターン1: 配列の要素アクセス =====

const fruits = ["りんご", "みかん", "バナナ"];

// 悪い例
const first: string = fruits[0];
// エラー（strict 設定次第）: Type 'string | undefined' is not assignable to type 'string'
// 配列のインデックスアクセスは undefined を返す可能性がある

// 良い例（方法1: undefined チェック）
const first = fruits[0];
if (first !== undefined) {
  console.log(first.toUpperCase()); // OK
}

// 良い例（方法2: デフォルト値）
const first = fruits[0] ?? "デフォルト";
console.log(first.toUpperCase()); // OK
```

```typescript
// ===== エラーパターン2: オプショナルプロパティ =====

interface User {
  name: string;
  email?: string; // オプショナル（string | undefined）
}

const user: User = { name: "田中" };

// 悪い例
console.log(user.email.toUpperCase());
// エラー: Object is possibly 'undefined'
// email は設定されていないかもしれない

// 良い例（方法1: if チェック）
if (user.email) {
  console.log(user.email.toUpperCase()); // OK
}

// 良い例（方法2: オプショナルチェイニング）
console.log(user.email?.toUpperCase()); // undefined なら undefined を返す

// 良い例（方法3: null 合体演算子と組み合わせ）
console.log(user.email?.toUpperCase() ?? "メール未設定");
```

```typescript
// ===== エラーパターン3: Map.get() の戻り値 =====

const userMap = new Map<string, string>();
userMap.set("user1", "田中");

// 悪い例
const name: string = userMap.get("user1");
// エラー: Type 'string | undefined' is not assignable to type 'string'
// Map.get() は undefined を返す可能性がある

// 良い例
const name = userMap.get("user1");
if (name !== undefined) {
  console.log(name); // OK
}

// または
const name = userMap.get("user1") ?? "不明";
```

```typescript
// ===== エラーパターン4: document.querySelector =====

// 悪い例
const button = document.querySelector("#submit-btn");
button.addEventListener("click", handleClick);
// エラー: Object is possibly 'null'
// querySelector は要素が見つからない場合 null を返す

// 良い例（方法1: null チェック）
const button = document.querySelector("#submit-btn");
if (button) {
  button.addEventListener("click", handleClick);
}

// 良い例（方法2: 存在が確実な場合は Non-null assertion）
const button = document.querySelector("#submit-btn")!;
// ただし、要素が存在しない場合は実行時エラーになるので注意
button.addEventListener("click", handleClick);

// 良い例（方法3: 型を絞り込む）
const button = document.querySelector<HTMLButtonElement>("#submit-btn");
if (button instanceof HTMLButtonElement) {
  button.disabled = true; // HTMLButtonElement のプロパティにアクセス可能
}
```

### 8.3 Property does not exist on type

**オブジェクトに存在しないプロパティにアクセスしようとした際** に発生するエラーです。

```typescript
// ===== エラーパターン1: タイプミス =====

interface User {
  name: string;
  email: string;
}

const user: User = { name: "田中", email: "tanaka@example.com" };

// 悪い例
console.log(user.emial);
// エラー: Property 'emial' does not exist on type 'User'.
// Did you mean 'email'?

// 良い例
console.log(user.email);
```

```typescript
// ===== エラーパターン2: 型定義にプロパティが不足 =====

interface Product {
  name: string;
  price: number;
}

const product: Product = { name: "ペン", price: 150 };

// 悪い例
console.log(product.description);
// エラー: Property 'description' does not exist on type 'Product'

// 良い例（方法1: 型定義を修正）
interface Product {
  name: string;
  price: number;
  description?: string; // プロパティを追加
}

// 良い例（方法2: オブジェクトリテラルにプロパティを追加する場合）
const productWithDesc = { ...product, description: "赤いボールペン" };
```

```typescript
// ===== エラーパターン3: ユニオン型での共通でないプロパティ =====

interface Dog {
  kind: "dog";
  bark(): void;
}

interface Cat {
  kind: "cat";
  meow(): void;
}

type Animal = Dog | Cat;

// 悪い例
function makeSound(animal: Animal) {
  animal.bark();
  // エラー: Property 'bark' does not exist on type 'Animal'
  // Property 'bark' does not exist on type 'Cat'
  // → Animal が Cat の場合、bark() は存在しない
}

// 良い例（型の絞り込み）
function makeSound(animal: Animal) {
  if (animal.kind === "dog") {
    animal.bark();   // OK: Dog 型として認識
  } else {
    animal.meow();   // OK: Cat 型として認識
  }
}
```

```typescript
// ===== エラーパターン4: API レスポンスの型が不足 =====

// 悪い例: JSON.parse の結果は any ではなく unknown として扱うべき
async function fetchData() {
  const response = await fetch("/api/data");
  const data = await response.json(); // any 型

  // この時点では data の構造が不明
  console.log(data.items.length);
  // ← 実行時エラーの可能性あり
}

// 良い例: 型を明示的に定義
interface ApiData {
  items: string[];
  total: number;
}

async function fetchData(): Promise<ApiData> {
  const response = await fetch("/api/data");
  const data: ApiData = await response.json();

  // 型安全にアクセスできる
  console.log(data.items.length);
  console.log(data.total);

  return data;
}
```

### 8.4 エラー対処のまとめフローチャート

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif;">
  <!-- Error Start -->
  <div style="background: #dc2626; color: white; border-radius: 10px; padding: 14px 20px; text-align: center; font-size: 14px; font-weight: 700; box-shadow: 0 2px 8px rgba(220,38,38,0.3);">TypeScript エラー発生</div>
  <div style="text-align: center; color: #64748b; font-size: 20px; line-height: 1.4;">▼</div>
  <!-- Decision -->
  <div style="background: #eff6ff; border: 2px solid #3b82f6; border-radius: 10px; padding: 12px 18px; text-align: center; font-size: 13px; font-weight: 600; color: #1e40af;">エラーの種類は？</div>
  <div style="text-align: center; color: #64748b; font-size: 20px; line-height: 1.4;">▼</div>
  <!-- Three branches -->
  <div style="display: flex; gap: 12px;">
    <!-- Branch 1: Type mismatch -->
    <div style="flex: 1; display: flex; flex-direction: column; align-items: center; gap: 6px;">
      <div style="background: #f59e0b; color: #1e293b; border-radius: 10px; padding: 10px 8px; text-align: center; font-size: 11px; font-weight: 600; width: 100%; box-shadow: 0 2px 6px rgba(245,158,11,0.25);">Type 'X' is not assignable to type 'Y'<br><span style="font-weight: 400;">型の不一致</span></div>
      <div style="color: #64748b; font-size: 16px;">▼</div>
      <div style="background: #f1f5f9; border-radius: 8px; padding: 8px; text-align: center; font-size: 11px; color: #475569; width: 100%;">値の型を確認</div>
      <div style="color: #64748b; font-size: 16px;">▼</div>
      <div style="background: #16a34a; color: white; border-radius: 8px; padding: 10px 8px; text-align: center; font-size: 11px; font-weight: 600; width: 100%;">型変換または<br>型定義を修正</div>
    </div>
    <!-- Branch 2: null/undefined -->
    <div style="flex: 1; display: flex; flex-direction: column; align-items: center; gap: 6px;">
      <div style="background: #f59e0b; color: #1e293b; border-radius: 10px; padding: 10px 8px; text-align: center; font-size: 11px; font-weight: 600; width: 100%; box-shadow: 0 2px 6px rgba(245,158,11,0.25);">Object is possibly 'undefined'<br><span style="font-weight: 400;">null/undefined の可能性</span></div>
      <div style="color: #64748b; font-size: 16px;">▼</div>
      <div style="background: #16a34a; color: white; border-radius: 8px; padding: 8px 6px; text-align: center; font-size: 10px; font-weight: 600; width: 100%;">null チェックを追加<br><code style="font-size: 9px; color: #bbf7d0;">if (value) { ... }</code></div>
      <div style="background: #16a34a; color: white; border-radius: 8px; padding: 8px 6px; text-align: center; font-size: 10px; font-weight: 600; width: 100%; margin-top: 4px;">オプショナルチェイニング<br><code style="font-size: 9px; color: #bbf7d0;">value?.property</code></div>
      <div style="background: #16a34a; color: white; border-radius: 8px; padding: 8px 6px; text-align: center; font-size: 10px; font-weight: 600; width: 100%; margin-top: 4px;">null 合体演算子<br><code style="font-size: 9px; color: #bbf7d0;">value ?? default</code></div>
    </div>
    <!-- Branch 3: Property not found -->
    <div style="flex: 1; display: flex; flex-direction: column; align-items: center; gap: 6px;">
      <div style="background: #f59e0b; color: #1e293b; border-radius: 10px; padding: 10px 8px; text-align: center; font-size: 11px; font-weight: 600; width: 100%; box-shadow: 0 2px 6px rgba(245,158,11,0.25);">Property does not exist on type<br><span style="font-weight: 400;">存在しないプロパティ</span></div>
      <div style="color: #64748b; font-size: 16px;">▼</div>
      <div style="background: #16a34a; color: white; border-radius: 8px; padding: 8px 6px; text-align: center; font-size: 10px; font-weight: 600; width: 100%;">タイプミスを確認</div>
      <div style="background: #16a34a; color: white; border-radius: 8px; padding: 8px 6px; text-align: center; font-size: 10px; font-weight: 600; width: 100%; margin-top: 4px;">型定義にプロパティを追加</div>
      <div style="background: #16a34a; color: white; border-radius: 8px; padding: 8px 6px; text-align: center; font-size: 10px; font-weight: 600; width: 100%; margin-top: 4px;">型の絞り込み<br><code style="font-size: 9px; color: #bbf7d0;">(type guard)</code></div>
    </div>
  </div>
</div>

---

## まとめ

この章では、TypeScript の基礎として以下の内容を学びました。

| トピック | 学んだこと |
|---------|-----------|
| TypeScript とは | JavaScript のスーパーセットであり、型安全性を提供する |
| 基本的な型 | `string`, `number`, `boolean`, `array`, `tuple`, `object`, `any`, `unknown`, `never`, `void`, `null`, `undefined` |
| 型注釈と型推論 | 明示的に型を書く方法と、TypeScript が自動で型を判断する仕組み |
| interface と type | オブジェクトの形を定義する2つの方法とその使い分け |
| ジェネリクス | 型をパラメータ化して再利用性を高める仕組み |
| ユニオン型とリテラル型 | 複数の型や特定の値のみを許可する型定義 |
| tsconfig.json | TypeScript プロジェクトの設定ファイル |
| よくあるエラー | 代表的な型エラーの原因と対処法 |

### 次の章へ

次の章では、**React の基礎** を学び、TypeScript と React を組み合わせた開発方法を習得していきます。この章で学んだ型の知識は、React コンポーネントの Props や State の定義で活用されます。
