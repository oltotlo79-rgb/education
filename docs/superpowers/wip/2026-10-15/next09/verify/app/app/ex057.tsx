// tailwind.config.ts
// ↑ Tailwind CSS のすべての設定をまとめたファイル。
//   Tailwind はビルド時にこのファイルを読み、どのクラスを生成するかを決める。

// "tailwindcss" パッケージから Config 型を「型としてのみ」インポート
// （import type なら実行時には消える → ビルドサイズを節約）
import type { Config } from "tailwindcss";

// config: Config と書くことで、TypeScript に「これは Tailwind の設定」と伝えられ
// 補完やタイプチェックが効くようになる
const config: Config = {
  // ダークモードの切り替え方法
  // "class"  : <html class="dark"> のように手動でクラスを付けて切り替える方式
  // "media"  : OSのカラーモード設定（prefers-color-scheme）に自動連動する方式
  // 本アプリでは「ユーザーが切り替えボタンで選べる」必要があるので "class" を選択
  darkMode: "class",

  // Tailwind がクラスを抽出する対象ファイルのパス
  // ここに書かれたファイル内で実際に使われているクラスだけを最終CSSに含めることで
  // バンドルサイズを最小化する（PurgeCSS / JIT エンジンの仕組み）
  content: [
    // Pages Router 用（使っていなくてもOK）
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    // 自作コンポーネント
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    // App Router 用
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
    // ※ ** はワイルドカード（任意の深さのフォルダ）、{ } は複数拡張子の指定
  ],

  // theme: Tailwind のデザインシステム本体。
  // ここに書いたものが Tailwind のクラスとして利用可能になる。
  theme: {
    // ========================================
    // コンテナの設定
    // ========================================
    // .container クラスの挙動をカスタマイズ。
    // <div class="container"> と書いたときの最大幅・余白・中央寄せの設定。
    container: {
      // コンテナを左右中央寄せ（margin: 0 auto 相当）
      center: true,
      // 画面サイズごとの左右パディング
      padding: {
        // 全画面共通の最小余白 = 16px
        DEFAULT: "1rem",
        // 640px 以上 = 32px
        sm: "2rem",
        // 1024px 以上 = 64px
        lg: "4rem",
        // 1280px 以上 = 80px
        xl: "5rem",
        // 1536px 以上 = 96px
        "2xl": "6rem",
      },
      // 各ブレークポイントでのコンテナ最大幅
      screens: {
        sm: "640px",
        md: "768px",
        lg: "1024px",
        xl: "1280px",
        // デフォルトの1536pxより少し狭く（読みやすさのため）
        "2xl": "1400px",
      },
    },

    // extend: 既存のテーマを「上書き」せず「追加」する。
    // theme: { colors: {...} } と書くと標準色が全部消えるので、extend を使うのが基本。
    extend: {
      // ========================================
      // カスタムカラーパレット
      // ========================================
      colors: {
        // プライマリカラー（インディゴ系）
        // CSS変数（globals.css の :root で定義した --color-primary-XX）を経由することで
        // ダークモード時に変数の値だけ差し替えれば自動で色が変わる
        primary: {
          // bg-primary-50 のように使える
          50: "var(--color-primary-50)",
          100: "var(--color-primary-100)",
          200: "var(--color-primary-200)",
          300: "var(--color-primary-300)",
          400: "var(--color-primary-400)",
          500: "var(--color-primary-500)",
          600: "var(--color-primary-600)",
          700: "var(--color-primary-700)",
          800: "var(--color-primary-800)",
          900: "var(--color-primary-900)",
          950: "var(--color-primary-950)",
        },
        // セカンダリカラー（エメラルド系）
        secondary: {
          50: "var(--color-secondary-50)",
          100: "var(--color-secondary-100)",
          200: "var(--color-secondary-200)",
          300: "var(--color-secondary-300)",
          400: "var(--color-secondary-400)",
          500: "var(--color-secondary-500)",
          600: "var(--color-secondary-600)",
          700: "var(--color-secondary-700)",
          800: "var(--color-secondary-800)",
          900: "var(--color-secondary-900)",
          950: "var(--color-secondary-950)",
        },
        // セマンティックカラー（用途ベースの命名）
        // bg-background で使える
        background: "var(--color-background)",
        // text-foreground で使える
        foreground: "var(--color-foreground)",
        muted: {
          // bg-muted（DEFAULT は階層なしの指定で参照される）
          DEFAULT: "var(--color-muted)",
          // text-muted-foreground
          foreground: "var(--color-muted-foreground)",
        },
        border: "var(--color-border)",
        card: {
          DEFAULT: "var(--color-card)",
          foreground: "var(--color-card-foreground)",
        },
        success: "var(--color-success)",
        warning: "var(--color-warning)",
        error: "var(--color-error)",
        info: "var(--color-info)",
      },

      // ========================================
      // カスタムフォント
      // ========================================
      // font-sans / font-mono クラスで使われるフォントの「フォールバック順」
      // 先頭から探して、利用可能な最初のフォントが使われる
      fontFamily: {
        sans: [
          // 第一候補: 欧文用の美しいフォント
          "Inter",
          // 日本語フォント（Inter は日本語が含まれないので必須）
          "Noto Sans JP",
          // OS標準のサンセリフ（macOS / Windows）
          "ui-sans-serif",
          // システム標準
          "system-ui",
          // iOS / macOS の San Francisco
          "-apple-system",
          // 最終フォールバック（必ず使える総称名）
          "sans-serif",
        ],
        // 等幅フォント（コード表示用）
        mono: [
          // プログラミング用の美しい等幅フォント
          "JetBrains Mono",
          // リガチャ（合字）対応の等幅フォント
          "Fira Code",
          "ui-monospace",
          "monospace",
        ],
      },

      // ========================================
      // カスタムフォントサイズ
      // ========================================
      // text-2xs クラスを新規追加（標準には無い超小サイズ）
      // 値の形式: [フォントサイズ, { lineHeight: 行間 }]
      fontSize: {
        // 10px / 行間14px
        "2xs": ["0.625rem", { lineHeight: "0.875rem" }],
      },

      // ========================================
      // カスタムシャドウ
      // ========================================
      boxShadow: {
        // shadow-sm が CSS 変数を参照するように
        sm: "var(--shadow-sm)",
        md: "var(--shadow-md)",
        lg: "var(--shadow-lg)",
        // 独自の影を新規追加
        // カード用
        card: "0 2px 8px -2px rgb(0 0 0 / 0.08), 0 4px 12px -4px rgb(0 0 0 / 0.04)",
        "card-hover":
          // ホバー時用
          "0 8px 24px -4px rgb(0 0 0 / 0.12), 0 4px 8px -4px rgb(0 0 0 / 0.08)",
      },

      // ========================================
      // カスタムボーダー半径
      // ========================================
      // rounded-sm / rounded-md などの値をCSS変数経由に置き換え
      borderRadius: {
        sm: "var(--radius-sm)",
        md: "var(--radius-md)",
        lg: "var(--radius-lg)",
        xl: "var(--radius-xl)",
      },

      // ========================================
      // カスタムアニメーション
      // ========================================
      // keyframes: 動きの定義（CSS の @keyframes 相当）
      keyframes: {
        "fade-in": {
          // 透明＋8px下
          from: { opacity: "0", transform: "translateY(8px)" },
          // 不透明＋元位置
          to: { opacity: "1", transform: "translateY(0)" },
        },
        "fade-out": {
          from: { opacity: "1", transform: "translateY(0)" },
          to: { opacity: "0", transform: "translateY(8px)" },
        },
        "slide-in-right": {
          // 16px 右からスライド
          from: { opacity: "0", transform: "translateX(16px)" },
          to: { opacity: "1", transform: "translateX(0)" },
        },
        "slide-in-up": {
          // 16px 下からスライド
          from: { opacity: "0", transform: "translateY(16px)" },
          to: { opacity: "1", transform: "translateY(0)" },
        },
        "scale-in": {
          // 95% から
          from: { opacity: "0", transform: "scale(0.95)" },
          // 100% へ拡大
          to: { opacity: "1", transform: "scale(1)" },
        },
        shimmer: {
          "0%": { backgroundPosition: "-200% 0" },
          "100%": { backgroundPosition: "200% 0" },
        },
        "toast-slide-in": {
          from: { opacity: "0", transform: "translateX(100%)" },
          to: { opacity: "1", transform: "translateX(0)" },
        },
        "toast-slide-out": {
          from: { opacity: "1", transform: "translateX(0)" },
          to: { opacity: "0", transform: "translateX(100%)" },
        },
      },
      // animation: keyframes を「いつ・どんな速度で」再生するかを定義
      // 形式: "<keyframes名> <時間> <イージング> <繰り返し> <塗り潰し>"
      animation: {
        // animate-fade-in クラスとして使える
        "fade-in": "fade-in 0.3s ease-out",
        "fade-out": "fade-out 0.3s ease-out",
        "slide-in-right": "slide-in-right 0.3s ease-out",
        "slide-in-up": "slide-in-up 0.4s ease-out",
        "scale-in": "scale-in 0.2s ease-out",
        // infinite: 無限ループ、linear: 等速
        shimmer: "shimmer 2s infinite linear",
        "toast-in": "toast-slide-in 0.3s ease-out",
        // forwards: 終了状態をキープ
        "toast-out": "toast-slide-out 0.3s ease-in forwards",
        // animate-spin（標準）の上書き
        spin: "spin 1s linear infinite",
      },

      // ========================================
      // カスタムスペーシング
      // ========================================
      // 標準にない余白/サイズの値を追加。w-18, h-88, p-128 のように使える
      spacing: {
        // 72px
        "18": "4.5rem",
        // 352px
        "88": "22rem",
        // 512px
        "128": "32rem",
      },

      // ========================================
      // カスタムトランジション
      // ========================================
      // duration-250 / duration-350 を使えるようにする
      transitionDuration: {
        "250": "250ms",
        "350": "350ms",
      },
    },
  },

  // plugins: Tailwind の機能を拡張するプラグインを並べる配列
  // 例: @tailwindcss/forms、@tailwindcss/typography など
  // 今回は使わないので空配列
  plugins: [],
};

// ESM 形式でデフォルトエクスポート（Next.js が require できるようにする）
export default config;
