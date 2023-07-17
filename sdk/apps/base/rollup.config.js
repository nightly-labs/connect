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
        file: 'dist/cjs/index.cjs',
        format: 'cjs',
        sourcemap: true,
        interop: 'compat'
      },
      {
        file: 'dist/esm/index.mjs',
        format: 'esm',
        sourcemap: true
      }
    ],
    plugins: [typescript(), nodeResolve(), commonjs(), terser()],
    external: [
      'cross-fetch',
      'isomorphic-localstorage',
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
        file: 'dist/browser/cjs/index.cjs',
        format: 'cjs',
        sourcemap: true,
        interop: 'compat'
      },
      {
        file: 'dist/browser/esm/index.mjs',
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
      'cross-fetch',
      'isomorphic-localstorage',
      'uuid',
      'eventemitter3',
      'isomorphic-ws',
      'ws'
    ]
  },
  {
    input: 'dist/esm/types/apps/base/src/index.d.ts',
    output: [{ file: 'dist/index.d.ts', format: 'esm' }],
    plugins: [dts()]
  }
]
