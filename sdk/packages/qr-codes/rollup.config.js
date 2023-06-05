import dts from 'rollup-plugin-dts'
import esbuild from 'rollup-plugin-esbuild'

const bundle = config => ({
  ...config,
  input: 'src/index.ts',
  external: id => !/^[./]/.test(id),
})

export default [
  bundle({
    plugins: [esbuild()],
    output: [
      {
        file: "dist/qr.js",
        format: 'cjs',
        sourcemap: true,
      },
      {
        file: "dist/qr.mjs",
        format: 'es',
        sourcemap: true,
      },
    ],
  }),
  bundle({
    plugins: [dts()],
    output: {
      file: "dist/qr.d.ts",
      format: 'es',
    },
  }),
]
