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
      '@mysten/sui.js',
      '@mysten/sui.js/verify',
      '@mysten/sui.js/dist/cjs/signers/types',
      '@mysten/sui.js/client',
      '@mysten/wallet-adapter-wallet-standard',
      '@mysten/wallet-adapter-wallet-standard/dist/StandardWalletAdapter',
      '@mysten/wallet-standard',
      '@nightlylabs/nightly-connect-sui',
      '@nightlylabs/wallet-selector-base',
      '@wallet-standard/core',
      'bs58',
      'events'
    ]
  },
  {
    input: 'dist/types/index.d.ts',
    output: [{ file: 'dist/index.d.ts', format: 'esm' }],
    plugins: [dts()]
  }
]
