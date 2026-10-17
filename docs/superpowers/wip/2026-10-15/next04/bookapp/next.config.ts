import type { NextConfig } from "next";

// CC=1 のときだけ Cache Components を有効にする（本書は No）
const cc = process.env.CC === "1";

const nextConfig: NextConfig = {
  cacheComponents: cc,
  partialPrefetching: cc,
};

export default nextConfig;
