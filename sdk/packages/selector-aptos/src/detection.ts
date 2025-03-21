import { getWallets, Wallet } from '@wallet-standard/core'

import { isWalletWithRequiredFeatureSet } from '@aptos-labs/wallet-standard'
import { IWalletListItem, WalletMetadata } from '@nightlylabs/wallet-selector-base'
import { getNightlySnapAdapter } from './moveSnap'

export const aptosWalletsFilter = (wallet: Wallet) => {
  const is = isWalletWithRequiredFeatureSet(wallet, []) // We don't filter for now
  return is
}

export const getAptosWalletsList = (presetList: WalletMetadata[], recentWalletName?: string) => {
  const { get } = getWallets()
  const windowWallets = get()

  const walletsData: Record<string, IWalletListItem> = {}

  presetList.forEach((wallet) => {
    walletsData[wallet.name] = {
      ...wallet,
      recent: recentWalletName === wallet.name
    }
  })

  windowWallets.filter(aptosWalletsFilter).forEach((wallet) => {
    if (walletsData[wallet.name]) {
      walletsData[wallet.name] = {
        ...walletsData[wallet.name],
        recent: recentWalletName === wallet.name,
        detected: true,
        standardWallet: wallet,
        walletType: 'hybrid'
      }
    } else {
      walletsData[wallet.name] = {
        name: wallet.name,
        image: {
          default: wallet.icon as string,
          lg: wallet.icon as string,
          md: wallet.icon as string,
          sm: wallet.icon as string
        },
        desktop: null,
        homepage: 'https://nightly.app/download', // Fall back to nightly.app
        mobile: null,
        slug: wallet.name,
        recent: recentWalletName === wallet.name,
        walletType: 'hybrid',
        detected: true,
        standardWallet: wallet
      }
    }
  })

  const nightlySnap = getNightlySnapAdapter()
  if (nightlySnap && nightlySnap.isMetamaskReady) {
    walletsData[nightlySnap.name] = {
      name: nightlySnap.name,
      image: {
        default: nightlySnap.icon as string,
        lg: nightlySnap.icon as string,
        md: nightlySnap.icon as string,
        sm: nightlySnap.icon as string
      },
      desktop: null,
      homepage: nightlySnap.url,
      mobile: null,
      slug: nightlySnap.name,
      recent: recentWalletName === nightlySnap.name,
      walletType: 'hybrid',
      detected: nightlySnap.isMetamaskReady,
      standardWallet: nightlySnap
    }
  }

  return Object.values(walletsData)
}
