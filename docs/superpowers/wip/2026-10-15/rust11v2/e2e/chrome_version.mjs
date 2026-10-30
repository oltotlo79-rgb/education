// 第11巻 UNIT 05: Playwright が使う Chrome の版を表示する
import { chromium } from "@playwright/test";
const browser = await chromium.launch({ channel: "chrome" });
console.log(`Chrome ${browser.version()}`);
await browser.close();
