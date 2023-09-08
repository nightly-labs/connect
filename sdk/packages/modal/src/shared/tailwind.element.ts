import { unsafeCSS } from 'lit'

import style from './tailwind.global.css'

let _variablesOverride = ''
let _stylesOverride = ''

export const setVariablesOverride = (override: object) => {
  let overrideString = '* {'
  Object.entries(override).forEach(([key, value]) => {
    if (/^--/.test(key)) {
      overrideString += `${key}: ${value};`
    }
  })
  overrideString += '}'
  _variablesOverride = overrideString
}

export const setStylesOverride = (override: string) => {
  _stylesOverride = override
}

export const tailwindElement = (...customStyle: string[]) => [
  unsafeCSS(style),
  ...customStyle.map((s) => unsafeCSS(s)),
  unsafeCSS(_variablesOverride),
  unsafeCSS(_stylesOverride)
]
