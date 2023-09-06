import './components/nightly-selector/nightly-selector'
import { setStylesOverride } from './shared/tailwind.element'
import './shared/tailwind.global.css'
export { type NightlySelector } from './components/nightly-selector/nightly-selector'
export { type WalletSelectorItem } from './utils/types'

export const getNightlySelectorElement = (stylesOverride = '') => {
  const style = document.createElement('style')
  style.textContent = `@import url('https://fonts.googleapis.com/css2?family=Prompt:wght@300;600&display=swap');` // workaround because import inbundled styles in ignored for some reason
  document.head.appendChild(style)

  setStylesOverride(stylesOverride)

  const selectorElement = document.createElement('nightly-selector')

  return selectorElement
}
