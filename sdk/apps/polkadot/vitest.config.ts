import tsconfigPath from 'vite-tsconfig-paths'
import { defineConfig } from 'vitest/config'

export default defineConfig({
  plugins: [tsconfigPath()],
  test: {
    // singleThread: true
    testTimeout: 10000
  }
})
