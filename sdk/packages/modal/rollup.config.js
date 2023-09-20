import typescript from '@rollup/plugin-typescript'
import { nodeResolve } from '@rollup/plugin-node-resolve'
import commonjs from '@rollup/plugin-commonjs'
import terser from '@rollup/plugin-terser'
import dts from 'rollup-plugin-dts'
import litcss from 'rollup-plugin-lit-css'
import template from 'rollup-plugin-html-literals'
import styles from 'rollup-plugin-styles'
import image from '@rollup/plugin-image'

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
      image(),
      styles({
        mode: 'emit'
      }),
      litcss(),
      template(),
      typescript({
        exclude: ['.storybook', '**/*.stories.ts']
      }),
      nodeResolve(),
      commonjs(),
      terser()
    ],
    external: [
      '@nightlylabs/qr-code',
      'autoprefixer',
      'lit',
      'postcss',
      'postcss-lit',
      'tailwindcss',
      'lit/directives/style-map.js',
      'lit/directives/unsafe-html.js',
      'lit/decorators.js'
    ]
  },
  {
    input: 'dist/types/index.d.ts',
    output: [{ file: 'dist/index.d.ts', format: 'esm' }],
    plugins: [dts()],
    external: [/\.css$/u]
  }
]
