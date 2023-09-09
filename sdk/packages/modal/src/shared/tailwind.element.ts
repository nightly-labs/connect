import { unsafeCSS } from 'lit'
import vector from '../static/svg/backButton.svg'
import search from '../static/svg/searchIcon.svg'
import copy from '../static/svg/copy.svg'
import scan from '../static/png/scan.png'
import Logo from '../static/svg/Logo.svg'
import Close from '../static/svg/Close.svg'
import Clouds from '../static/svg/Clouds.svg'
import Stars from '../static/svg/Stars.svg'
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

const imagesSrcCssVars = `
* {
  --nc-img-back: url("${vector}");
  --nc-img-search: url('${search}');
  --nc-img-scan: url("${scan}");
  --nc-img-copy: url("${copy}");
  --nc-img-logo: url("${Logo}");
  --nc-img-close: url("${Close}");
  --nc-img-header-bg: url("${Stars}");
  --nc-img-header-fg: url("${Clouds}");
}
`

export const tailwindElement = (customStyle: string) => [
  unsafeCSS(style),
  unsafeCSS(imagesSrcCssVars),
  unsafeCSS(customStyle),
  _overrides
]
