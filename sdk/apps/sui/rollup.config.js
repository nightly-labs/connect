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
        sourcemap: true,
        interop: 'compat'
      },
      {
        file: 'dist/index.mjs.js',
        format: 'esm',
        sourcemap: true
      }
    ],
    plugins: [typescript(), nodeResolve(), commonjs(), terser()],
    external: [
      '@mysten/sui.js',
      '@mysten/sui.js/dist/cjs/signers/types',
      '@mysten/sui.js/client',
      '@mysten/sui.js/transactions',
      '@mysten/sui.js/keypairs/ed25519',
      '@mysten/sui.js/dist/cjs/providers/json-rpc-provider',
      '@mysten/sui.js/dist/cjs/rpc/connection',
      '@mysten/sui.js/verify',
      '@mysten/sui.js/utils',
      '@mysten/sui.js/cryptography',
      '@mysten/wallet-standard',
      '@noble/hashes',
      '@nightlylabs/nightly-connect-base',
      'uuid',
      'eventemitter3',
      'isomorphic-ws',
      'ws'
    ]
  },
  {
    input: 'src/index.ts',
    output: [
      {
        file: 'dist/index.browser.cjs.js',
        format: 'cjs',
        sourcemap: true,
        interop: 'compat'
      },
      {
        file: 'dist/index.browser.mjs.js',
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
      '@mysten/sui.js/dist/cjs/signers/types',
      '@mysten/sui.js/client',
      '@mysten/sui.js/transactions',
      '@mysten/sui.js/keypairs/ed25519',
      '@mysten/sui.js/dist/cjs/providers/json-rpc-provider',
      '@mysten/sui.js/dist/cjs/rpc/connection',
      '@mysten/sui.js/verify',
      '@mysten/sui.js/utils',
      '@mysten/sui.js/cryptography',
      '@mysten/wallet-standard',
      '@noble/hashes',
      '@nightlylabs/nightly-connect-base',
      'uuid',
      'eventemitter3',
      'isomorphic-ws',
      'ws'
    ]
  },
  {
    input: 'dist/types/apps/sui/src/index.d.ts',
    output: [{ file: 'dist/index.d.ts', format: 'esm' }],
    plugins: [dts()]
  }
]
