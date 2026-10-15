// React 本体から createElement（JSX の正体）と Children を取り込む
import { createElement, Children } from "react";
// React 要素を HTML の文字列に変換する関数を取り込む
import { renderToStaticMarkup } from "react-dom/server";

// (1) <h1 className="title">こんにちは</h1> と同じ意味の呼び出し
const heading = createElement("h1", { className: "title" }, "こんにちは");

// React 要素の中身をのぞいてみる
console.log("type :", heading.type);
console.log("props:", heading.props);

// (2) 入れ子の JSX は、createElement の入れ子になる
// <ul><li>りんご</li><li>バナナ</li></ul> と同じ意味
const list = createElement(
  "ul",
  null,
  createElement("li", { key: "a" }, "りんご"),
  createElement("li", { key: "b" }, "バナナ"),
);
console.log("ul の子要素の数:", Children.count(list.props.children));

// (3) React 要素（ただのオブジェクト）から HTML を作る
console.log(renderToStaticMarkup(heading));
console.log(renderToStaticMarkup(list));
