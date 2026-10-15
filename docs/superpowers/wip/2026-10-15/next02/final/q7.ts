function wait(ms: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

async function task(name: string, ms: number): Promise<string> {
  await wait(ms);
  console.log(`${name} 完了`);
  return name;
}

async function main(): Promise<void> {
  console.log("A");
  const first = task("1冊目", 200);
  const second = task("2冊目", 100);
  console.log("B");
  const results = await Promise.all([first, second]);
  console.log("C", results);
}

main();
console.log("D");
