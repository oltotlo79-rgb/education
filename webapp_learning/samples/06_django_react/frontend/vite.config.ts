import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  // 成果物は Django の /static/ から配られる（onprem の 05 の 6.5）
  base: "/static/",
  build: {
    // 成果物を Django のプロジェクトの中（backend/frontend_dist）に書き出す
    outDir: "../backend/frontend_dist",
    emptyOutDir: true,
  },
  server: {
    // 開発中だけ: API と承認の画面は、Django の開発用のサーバーへ回す
    proxy: {
      "/api": "http://127.0.0.1:8000",
      "/approve": "http://127.0.0.1:8000",
    },
  },
});
