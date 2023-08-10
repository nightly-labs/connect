import typescript from '@rollup/plugin-typescript'
import { nodeResolve } from '@rollup/plugin-node-resolve'
import commonjs from '@rollup/plugin-commonjs'
import terser from '@rollup/plugin-terser'
import dts from 'rollup-plugin-dts'

export default [
  {
    input: 'src/index.ts',
    output: [
      {
        file: 'dist/index.cjs.js',
        format: 'cjs',
        sourcemap: true
      },
      {
        file: 'dist/index.mjs.js',
        format: 'esm',
        sourcemap: true
      }
    ],
    plugins: [
      typescript(),
      nodeResolve({ browser: true, preferBuiltins: false }),
      commonjs(),
      terser()
    ],
    external: [
      '@nightlylabs/nightly-connect-polkadot',
      '@nightlylabs/wallet-selector-base',
      '@polkadot/extension-inject',
      '@polkadot/api',
      '@wallet-standard/core'
    ]
  },
  {
    input: 'dist/types/index.d.ts',
    output: [{ file: 'dist/index.d.ts', format: 'esm' }],
    plugins: [dts()]
  }
]
