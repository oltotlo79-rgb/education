const a = { title: "入門", tags: ["TS"] };
const b = a;
const c = { ...a };

b.title = "実践";
c.tags.push("React");

console.log(a.title);
console.log(a.tags);
console.log(a === b, a === c);
