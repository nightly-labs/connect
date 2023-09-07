import { unsafeCSS } from 'lit'

import style from './tailwind.global.css'

let _stylesOverride = ''

export const setStylesOverride = (override: string) => {
  _stylesOverride = override
}

export const tailwindElement = (...customStyle: string[]) => [
  unsafeCSS(style),
  ...customStyle.map((s) => unsafeCSS(s)),
  unsafeCSS(_stylesOverride)
]
