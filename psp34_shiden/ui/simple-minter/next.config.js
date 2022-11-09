/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  assetPrefix: "/simple-minter",
  trailingSlash: true,
  async rewrites() {
    return [
      {
        source: "/simple-minter/api/:path*",
        destination: "/api/:path*",
      },
      {
        source: "/simple-minter/images/:query*",
        destination: '/_next/image/:query*'
      },
      {
        source: "/simple-minter/_next/:path*",
        destination: "/_next/:path*",
      },
    ]
  }
}

module.exports = nextConfig
