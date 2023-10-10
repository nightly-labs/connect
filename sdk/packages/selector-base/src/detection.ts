import { Wallet, getWallets } from '@wallet-standard/core'
import { type IWalletListItem, type MetadataWallet } from './types'

export const getWalletsList = (
  presetList: MetadataWallet[],
  walletsFilterCb: (wallet: Wallet) => boolean,
  recentWalletName?: string
) => {
  const { get } = getWallets()
  const windowWallets = get()

  const walletsData: Record<string, IWalletListItem> = {}

  presetList.forEach((wallet) => {
    walletsData[wallet.name] = {
      ...wallet,
      recent: recentWalletName === wallet.name
    }
  })

  windowWallets.filter(walletsFilterCb).forEach((wallet) => {
    walletsData[wallet.name] = {
      ...(walletsData?.[wallet.name] ?? {
        name: wallet.name,
        icon: wallet.icon,
        link: '',
        deeplink: null,
        recent: recentWalletName === wallet.name,
        walletType: 'hybrid'
      }),
      detected: true,
      standardWallet: wallet
    }
  })

  return Object.values(walletsData)
}
