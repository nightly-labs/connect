import { type Injected, type InjectedExtension } from '@polkadot/extension-inject/types'
import { type WalletIcon } from '@wallet-standard/core'
import { appToIcon } from './tempIcons'
import { IWalletListItem, MetadataWallet } from '@nightlylabs/wallet-selector-base'
import { WalletMetadata } from '@nightlylabs/nightly-connect-polkadot'
export interface PolkadotWalletInjected {
  // Default Polkadot standard
  connect?: (origin: string) => Promise<InjectedExtension> // Is this even used ?
  enable?: (origin: string) => Promise<Injected>
  version?: string
  // Custom should be provided by the wallet
  name: string
  slug: string
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
      value.slug = key
      value.icon =
        value.icon ?? appToIcon[key] ?? 'https://registry.nightly.app/networks/polkadot.png' // TODO add default icon
      return value
    })
  } else {
    return []
  }
}

export interface IPolkadotWalletListItem extends Omit<IWalletListItem, 'standardWallet'> {
  injectedWallet?: PolkadotWalletInjected
}

export const getPolkadotWalletsList = (presetList: WalletMetadata[], recentWalletName?: string) => {
  const windowWallets = getPolkadotWallets()

  const walletsData: Record<string, IPolkadotWalletListItem> = {}

  presetList.forEach((wallet) => {
    walletsData[wallet.slug.toLocaleLowerCase()] = {
      slug: wallet.slug,
      name: wallet.name,
      icon: wallet.image.default,
      deeplink: wallet.mobile,
      link: wallet.homepage,
      walletType: wallet.walletType,
      recent: recentWalletName === wallet.name
    }
  })
  console.log('windowWallets', windowWallets)
  console.log('walletsData', walletsData)
  for (const wallet of windowWallets) {
    // Check if wallet is already in the list
    // by name
    if (walletsData[wallet.name.toLocaleLowerCase()]) {
      console.log('a', walletsData[wallet.name.toLocaleLowerCase()])
      console.log(wallet)
      walletsData[wallet.name.toLocaleLowerCase()] = {
        ...(walletsData?.[wallet.name.toLocaleLowerCase()] ?? {
          name: wallet.name,
          icon: wallet.icon,
          link: '',
          deeplink: null,
          recent: recentWalletName === wallet.name,
          walletType: 'hybrid'
        }),
        detected: true,
        injectedWallet: wallet
      }
      continue
    }
    // Check if wallet is already in the list
    // by namespace
    if (walletsData[wallet.slug.toLocaleLowerCase()]) {
      console.log('b', walletsData[wallet.slug.toLocaleLowerCase()])
      console.log(wallet)
      walletsData[wallet.slug.toLocaleLowerCase()] = {
        ...(walletsData?.[wallet.slug.toLocaleLowerCase()] ?? {
          name: wallet.name,
          icon: wallet.icon,
          link: '',
          deeplink: null,
          recent: recentWalletName === wallet.name,
          walletType: 'hybrid'
        }),
        detected: true,
        injectedWallet: wallet
      }
      continue
    }
    console.log({ wallet })

    walletsData[wallet.name.toLocaleLowerCase()] = {
      slug: wallet.name,
      name: wallet.name,
      icon: wallet.icon as string,
      link: '',
      deeplink: null,
      recent: recentWalletName === wallet.name,
      detected: true,
      injectedWallet: wallet,
      walletType: 'hybrid'
    }
  }

  return Object.values(walletsData)
}
