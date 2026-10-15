import type { NextConfig } from "next";

// CC=0 で従来モデル、それ以外は create-next-app 推奨デフォルト（Cache Components 有効）
const cc = process.env.CC !== "0";

const nextConfig: NextConfig = {
  cacheComponents: cc,
  partialPrefetching: cc,
};

export default nextConfig;
