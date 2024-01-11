import { XMLOptions } from '@nightlylabs/qr-code'
import './components/nightly-selector/nightly-selector'
import { setStylesOverride, setVariablesOverride } from './shared/tailwind.element'
import './shared/tailwind.global.css'
export { type NightlySelector } from './components/nightly-selector/nightly-selector'
export { type WalletSelectorItem } from './utils/types'

export const getNightlySelectorElement = (
  variablesOverride?: object, // simple changes like changing global colors
  stylesOverride?: string, // more advanced changes
  qrConfigOverride?: Partial<XMLOptions>, // customization of qr codes
  optionalParams?: object
) => {
  const style = document.createElement('style')
  style.textContent = `@import url('https://fonts.googleapis.com/css2?family=Prompt:wght@400;600&display=swap');` // workaround because import in bundled styles in ignored for some reason
  document.head.appendChild(style)

  // test console.log for alephCustom route optional parameters
  console.log('%c MOCKED_OPTIONAL_DATA_TEST', 'background: green', optionalParams)

  setVariablesOverride(variablesOverride ?? {})
  setStylesOverride(stylesOverride ?? '')

  const selectorElement = document.createElement('nightly-selector')
  selectorElement.qrConfigOverride = qrConfigOverride ?? {}
  selectorElement.optionalParams = optionalParams ?? {}

  return selectorElement
}

export { type XMLOptions }
