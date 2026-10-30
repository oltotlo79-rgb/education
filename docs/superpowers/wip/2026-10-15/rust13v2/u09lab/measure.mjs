// 第13巻 UNIT 09: 本物の Chrome で RustNote の画面を開き、CSS が効いた結果（寸法・はみ出し・色）を測る。
// 使い方: node measure.mjs <見るもの: all | broken | typo>（サーバーは 127.0.0.1:3014 で動かしておく）
import { chromium } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

const B = 'http://127.0.0.1:3014';
const mode = process.argv[2] ?? 'all';
const browser = await chromium.launch({ channel: 'chrome' });
const context = await browser.newContext({ viewport: { width: 1280, height: 800 } });
const page = await context.newPage();

// 登録して、ノートを3つ作る（1つは完了・1つは長い URL の本文）
await page.goto(`${B}/signup`);
await page.fill('#email', 'alice.long.address@example.com');
await page.fill('#password', 'correct horse battery');
await page.fill('#password_confirm', 'correct horse battery');
await page.click('form.auth-form button[type=submit]');
async function create(title, body) {
  await page.goto(`${B}/notes`);
  await page.click('details.new-note summary');
  await page.fill('#title', title);
  await page.fill('#body', body);
  await page.click('form.note-form button[type=submit]');
}
await create('買い物', '牛乳\n卵');
await create('リンク集', 'https://example.com/' + 'a'.repeat(120));
await create('宿題', '');
await page.click('form[action$="/toggle"] button');

const style = (selector, props) =>
  page.$eval(selector, (e, props) => {
    const s = getComputedStyle(e);
    return props.map((p) => `${p}: ${s.getPropertyValue(p)}`).join('・');
  }, props);
const overflow = () =>
  page.evaluate(() => {
    const d = document.documentElement;
    return d.scrollWidth > d.clientWidth ? `あり（${d.scrollWidth}px / ${d.clientWidth}px）` : 'なし';
  });
const width = (selector) => page.$eval(selector, (e) => Math.round(e.getBoundingClientRect().width));

if (mode === 'all') {
  for (const w of [1280, 375, 320]) {
    await page.setViewportSize({ width: w, height: 800 });
    await page.goto(`${B}/notes`);
    const header = await page.$eval('.site-header', (e) => `${e.scrollWidth}px / ${e.clientWidth}px`);
    console.log(`--- 画面の幅 ${w}px（一覧）`);
    console.log(`.container の幅 ${await width('.container')}px・横のはみ出し ${await overflow()}・ヘッダーの中身/枠 ${header}`);
    await page.goto(`${B}/notes/2`);
    console.log(`詳細（長い URL）: .note-body の幅 ${await width('.note-body')}px・横のはみ出し ${await overflow()}`);
  }
  await page.setViewportSize({ width: 1280, height: 800 });
  await page.goto(`${B}/notes`);
  console.log('--- 一覧の、CSS が効いた結果');
  console.log(`完了のリンク: ${await style('.note-list li.done a', ['color', 'text-decoration-line'])}`);
  console.log(`完了でないリンク: ${await style('.note-list li:not(.done) a', ['color', 'text-decoration-line'])}`);
  console.log(`ページ送り: ${await style('.pagination', ['display', 'justify-content'])}`);
  console.log(`作成のボタン: ${await style('form.note-form button', ['background-color', 'color'])}`);
  await page.goto(`${B}/notes/1`);
  console.log(`本文: ${await style('.note-body', ['white-space', 'overflow-wrap'])}・高さ ${await page.$eval('.note-body', (e) => Math.round(e.getBoundingClientRect().height))}px`);
  console.log('--- axe（アクセシビリティの検査）');
  for (const path of ['/notes', '/notes/1']) {
    await page.goto(`${B}${path}`);
    const result = await new AxeBuilder({ page }).analyze();
    const found = result.violations.map((v) => `${v.id}（${v.nodes.length} か所）`);
    console.log(`${path}: ${found.length === 0 ? '問題なし' : found.join('・')}`);
  }
} else if (mode === 'broken') {
  await page.goto(`${B}/notes`);
  console.log(`ページ送り: ${await style('.pagination', ['display', 'justify-content', 'margin-top'])}`);
  console.log(`ノートの本文（詳細）: ${await (async () => { await page.goto(`${B}/notes/1`); return style('.note-body', ['white-space']); })()}`);
  await page.goto(`${B}/notes`);
  console.log(`検索のフォーム（.empty より前の規則）: ${await style('.search-form', ['display'])}`);
} else if (mode === 'csp') {
  // 総合演習8: 守りのヘッダー（CSP）があっても、画面が壊れないか（CSP の違反がコンソールに出ないか）
  const violations = [];
  page.on('console', (message) => {
    if (message.text().includes('Content Security Policy')) violations.push(message.text());
  });
  for (const path of ['/', '/notes', '/notes/1', '/notes/1/edit']) {
    await page.goto(`${B}${path}`);
  }
  await page.goto(`${B}/notes`);
  console.log(`ヘッダーの display: ${await style('.site-header', ['display'])}（CSS が読めている）`);
  console.log(`CSP の違反の知らせ: ${violations.length} 件`);
  const result = await new AxeBuilder({ page }).analyze();
  console.log(`axe: ${result.violations.length === 0 ? '問題なし' : result.violations.map((v) => v.id).join('・')}`);
} else if (mode === 'typo') {
  await page.goto(`${B}/notes`);
  console.log(`一覧のリンク: ${await style('.note-list li:not(.done) a', ['color'])}`);
  console.log(`本文の文字: ${await style('body', ['color'])}`);
}
await browser.close();
