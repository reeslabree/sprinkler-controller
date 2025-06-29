import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  reactStrictMode: true,
  env: {
    NEXT_PUBLIC_WEBSOCKET_IP: process.env.WEBSOCKET_IP,
    NEXT_PUBLIC_WEBSOCKET_PORT: process.env.WEBSOCKET_PORT,
    NEXT_PUBLIC_WEBSOCKET_PATH: process.env.WEBSOCKET_PATH,
  },
};

export default nextConfig;
