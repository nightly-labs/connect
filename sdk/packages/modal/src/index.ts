import { XMLOptions } from '@nightlylabs/qr-code'
import './components/nightly-selector/nightly-selector'
import { setStylesOverride, setVariablesOverride } from './shared/tailwind.element'
import './shared/tailwind.global.css'
export { type NightlySelector } from './components/nightly-selector/nightly-selector'
export { type WalletSelectorItem } from './utils/types'

export const getNightlySelectorElement = (
  variablesOverride?: object, // simple changes like changing global colors
  stylesOverride?: string, // more advanced changes
  qrConfigOverride?: Partial<XMLOptions> // customization of qr codes
) => {
  setVariablesOverride(variablesOverride ?? {})
  setStylesOverride(stylesOverride ?? '')

  const selectorElement = document.createElement('nightly-selector')
  selectorElement.qrConfigOverride = qrConfigOverride ?? {}

  return selectorElement
}

export { type XMLOptions }
