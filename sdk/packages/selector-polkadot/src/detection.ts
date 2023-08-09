import { type Injected, type InjectedExtension } from '@polkadot/extension-inject/types'
import { type WalletIcon } from '@wallet-standard/core'
import { appToIcon } from './tempIcons'
import { IWalletListItem, MetadataWallet } from '@nightlylabs/wallet-selector-base'
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
      value.icon = value.icon ?? appToIcon[key] ?? 'https://registry.connect.nightly.app/networks/polkadot.png' // TODO add default icon
      return value
    })
  } else {
    return []
  }
}

export interface IPolkadotWalletListItem extends Omit<IWalletListItem, 'standardWallet'> {
  injectedWallet?: PolkadotWalletInjected
}

export const getPolkadotWalletsList = (presetList: MetadataWallet[], recentWalletName?: string) => {
  const windowWallets = getPolkadotWallets()

  const walletsData: Record<string, IPolkadotWalletListItem> = {}

  presetList.forEach((wallet) => {
    walletsData[wallet.name] = {
      ...wallet,
      recent: recentWalletName === wallet.name
    }
  })

  windowWallets.forEach((wallet) => {
    walletsData[wallet.name] = {
      ...(walletsData?.[wallet.name] ?? {
        name: wallet.name,
        icon: wallet.icon,
        link: '',
        deeplink: null,
        recent: recentWalletName === wallet.name
      }),
      detected: true,
      injectedWallet: wallet
    }
  })

  return Object.values(walletsData)
}
