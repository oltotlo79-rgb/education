// 第11巻 UNIT 05: Playwright の設定。
// - テストの前に、RustNote のサーバーを、新しい DB ファイルで起動する（webServer）
// - ブラウザは、パソコンに入っている Google Chrome を使う（channel: "chrome"。ブラウザをダウンロードしない）
import { defineConfig } from "@playwright/test";
import { tmpdir } from "node:os";
import { join } from "node:path";

// RustNote の実行ファイル（topcoat asset bundle と cargo build を済ませておく）
const server = process.env.RUSTNOTE_EXE ?? "../rustnote/target/debug/rustnote";
// 実行のたびに、別の DB ファイル（前の実行のデータが残らないように）
const database = join(tmpdir(), `rustnote-e2e-${Date.now()}.db`);

export default defineConfig({
  testDir: "./tests",
  // 1つずつ順に動かす（同じサーバーと DB を使うため）
  workers: 1,
  reporter: "list",
  use: {
    baseURL: "http://127.0.0.1:3011",
    channel: "chrome",
  },
  webServer: {
    command: server,
    url: "http://127.0.0.1:3011/health",
    env: { HOST: "127.0.0.1", PORT: "3011", DATABASE_URL: database },
    reuseExistingServer: false,
  },
});
