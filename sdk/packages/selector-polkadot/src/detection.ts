import { type Injected, type InjectedExtension } from '@polkadot/extension-inject/types'
import { type WalletIcon } from '@wallet-standard/core'

export interface PolkadotWalletInjected {
  // Default Polkadot standard
  connect?: (origin: string) => Promise<InjectedExtension> // Is this even used ?
  enable?: (origin: string) => Promise<Injected>
  version?: string
  // Custom should be provided by the wallet
  name: string
  icon?: WalletIcon
}
declare global {
  interface Window {
    injectedWeb3?: { [key in string]: PolkadotWalletInjected }
  }
}
export const getPolkadotWallets = (): PolkadotWalletInjected[] => {
  if (window && window.injectedWeb3) {
    return Object.entries(window.injectedWeb3).map(([key, value]) => {
      // value.name might be undefined
      value.name = value.name ?? key
      return value
    })
  } else {
    return []
  }
}
