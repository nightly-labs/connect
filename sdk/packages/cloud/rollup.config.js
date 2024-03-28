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
    external: ['uuid', 'cross-fetch']
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
    external: ['uuid']
  },
  {
    input: 'dist/types/packages/cloud/src/index.d.ts',
    output: [{ file: 'dist/index.d.ts', format: 'esm' }],
    plugins: [dts()]
  }
]
