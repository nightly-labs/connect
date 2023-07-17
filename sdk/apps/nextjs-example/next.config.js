/** @type {import('next').NextConfig} */
const nextConfig = {
  webpack: (webpackConfig, { webpack }) => {
    webpackConfig.resolve.extensionAlias = {
      '.js': ['.ts', '.tsx', '.js', '.jsx'],
      '.mjs': ['.mts', '.mjs'],
      '.cjs': ['.cts', '.cjs']
    }
    return webpackConfig
  }
}

module.exports = nextConfig
