// usage: node pw.mjs <scenario> [url]
import { chromium } from "playwright-core";
const scenario = process.argv[2];
const base = process.argv[3] ?? "http://localhost:3000";
const browser = await chromium.launch({
  executablePath: "C:/Program Files/Google/Chrome/Application/chrome.exe",
  headless: true,
});
const page = await browser.newPage({ viewport: { width: 900, height: 600 } });
const logs = [];
page.on("console", (m) => logs.push(`[console.${m.type()}] ${m.text()}`));
page.on("pageerror", (e) => logs.push(`[pageerror] ${e.message}`));
const mainHtml = async () =>
  (await page.locator("main").innerHTML()).replace(/<!--.*?-->/g, "");

if (scenario === "shelf") {
  await page.goto(base + "/shelf");
  console.log("before:", await mainHtml());
  await page.fill('input[name="title"]', process.argv[4] ?? "達人プログラマー");
  await page.click('button[type="submit"]');
  await page.waitForTimeout(1500);
  console.log("after :", await mainHtml());
  console.log("url   :", page.url());
} else if (scenario === "goto") {
  await page.goto(base + (process.argv[4] ?? "/"));
  await page.waitForTimeout(1500);
  console.log(await mainHtml());
} else if (scenario === "click") {
  await page.goto(base + process.argv[4]);
  await page.waitForTimeout(1000);
  console.log("before:", await mainHtml());
  await page.click(process.argv[5]);
  await page.waitForTimeout(Number(process.argv[6] ?? 1500));
  console.log("after :", await mainHtml());
  console.log("url   :", page.url());
} else if (scenario === "shot") {
  await page.goto(base + process.argv[4]);
  await page.waitForTimeout(Number(process.argv[6] ?? 1500));
  await page.screenshot({ path: process.argv[5] });
}
console.log("---- browser console ----");
console.log(logs.join("\n"));
await browser.close();
