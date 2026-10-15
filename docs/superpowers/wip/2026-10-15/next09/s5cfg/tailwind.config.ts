import type { Config } from "tailwindcss";

const config: Config = {
  content: ["./app/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        primary: { 600: "#4f46e5" },
      },
    },
  },
};

export default config;
