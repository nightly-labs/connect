import { Injected, InjectedExtension } from '@polkadot/extension-inject/types'
import { WalletIcon } from '@wallet-standard/core'
import { appToIcon } from './tempIcons'
import { WalletMetadata } from '@nightlylabs/wallet-selector-base'

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
    return Object.entries(window.injectedWeb3).map(([key, value]) => ({
      ...value,
      name: value.name ?? key, // value.name might be undefined
      slug: key,
      icon: value.icon ?? appToIcon[key] ?? 'https://registry.nightly.app/networks/polkadot.png' // TODO add default icon
    }))
  } else {
    return []
  }
}

export interface IPolkadotWalletListItem
  extends Pick<
    WalletMetadata,
    'name' | 'slug' | 'walletType' | 'mobile' | 'desktop' | 'image' | 'homepage'
  > {
  recent?: boolean
  detected?: boolean
  injectedWallet?: PolkadotWalletInjected
}

export const getPolkadotWalletsList = (presetList: WalletMetadata[], recentWalletName?: string) => {
  const windowWallets = getPolkadotWallets()

  const walletsData: Record<string, IPolkadotWalletListItem> = {}

  presetList.forEach((wallet) => {
    walletsData[wallet.slug.toLocaleLowerCase()] = {
      ...wallet,
      recent: recentWalletName === wallet.name
    }
  })
  for (const wallet of windowWallets) {
    // Check if wallet is already in the list
    // by namespace
    if (walletsData[wallet.slug.toLocaleLowerCase()]) {
      walletsData[wallet.slug.toLocaleLowerCase()] = {
        ...walletsData?.[wallet.slug.toLocaleLowerCase()],
        recent: recentWalletName === wallet.name,
        detected: true,
        injectedWallet: wallet,
        walletType: 'hybrid'
      }
    }

    // Check if wallet is already in the list
    // by name
    else if (walletsData[wallet.name.toLocaleLowerCase()]) {
      walletsData[wallet.name.toLocaleLowerCase()] = {
        ...walletsData[wallet.name.toLocaleLowerCase()],
        recent: recentWalletName === wallet.name,
        detected: true,
        injectedWallet: wallet,
        walletType: 'hybrid'
      }
    } else
      walletsData[wallet.name.toLocaleLowerCase()] = {
        slug: wallet.name,
        name: wallet.name,
        image: {
          default: wallet.icon as string,
          sm: wallet.icon as string,
          md: wallet.icon as string,
          lg: wallet.icon as string
        },
        desktop: null,
        mobile: null,
        recent: recentWalletName === wallet.name,
        detected: true,
        injectedWallet: wallet,
        walletType: 'hybrid',
        homepage: 'https://nightly.app/download'
      }
  }

  return Object.values(walletsData)
}
