import './components/nightly-selector/nightly-selector'
import './shared/tailwind.global.css'
export { type NightlySelector } from './components/nightly-selector/nightly-selector'

export const getNightlySelectorElement = () => {
  const style = document.createElement('style')
  style.textContent = `@import url('https://fonts.googleapis.com/css2?family=Prompt:wght@300;600&display=swap');` // workaround because import inbundled styles in ignored for some reason
  document.head.appendChild(style)

  const selectorElement = document.createElement('nightly-selector')

  return selectorElement
}
