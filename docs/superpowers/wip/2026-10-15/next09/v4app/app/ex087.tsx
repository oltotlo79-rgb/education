// app/layout.tsx の <head> 内に追加

// dangerouslySetInnerHTML: React の機能。普通は危険なので使うべきではないが、
// React のハイドレーション前に実行するインラインスクリプトを書きたい時に使う
<script
  dangerouslySetInnerHTML={{
    __html: `
      /* 即時実行関数（IIFE）: 定義と同時に呼び出す関数。グローバル変数を汚さない */
      (function() {
        try {
          /* localStorage から保存済みテーマを取得 */
          var theme = localStorage.getItem('theme');
          /* isDark の判定:
             - 明示的に 'dark' が保存されている
             - または theme 未保存 かつ OS がダーク設定 */
          var isDark = theme === 'dark' ||
            (!theme && window.matchMedia('(prefers-color-scheme: dark)').matches);
          if (isDark) {
            /* <html> に dark クラスを付与（React の描画前に間に合う） */
            document.documentElement.classList.add('dark');
          }
        } catch (e) {
          /* localStorage が使えないブラウザ等のためのフォールバック（何もしない） */
        }
      })();
    `,
  }}
/>
