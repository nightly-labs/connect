import { unsafeCSS } from 'lit'

import style from './tailwind.global.css'

const _overrides = new CSSStyleSheet()

export const setVariablesOverride = (override: object) => {
  let overrideString = '* {'
  Object.entries(override).forEach(([key, value]) => {
    if (/^--/.test(key)) {
      overrideString += `${key}: ${value};`
    }
  })
  overrideString += '}'
  _overrides.insertRule(overrideString)
}

export const setStylesOverride = (override: string) => {
  override.split('}').forEach((rule) => {
    try {
      if (!rule.trim().length) {
        return
      }
  
      _overrides.insertRule(rule + '}')
    } catch (error) {
      console.log('[custom rule error]:', error)
    }
  })
}

export const tailwindElement = (...customStyle: string[]) => [
  unsafeCSS(style),
  ...customStyle.map((s) => unsafeCSS(s)),
  _overrides
]
