import { Wallet, getWallets } from '@wallet-standard/core'

export interface IWalletListItem {
  name: string
  icon: string
  link?: string
  recent?: boolean
  detected?: boolean
  standardWallet?: Wallet
}

export const getWalletsList = (
  presetList: Omit<IWalletListItem, 'recent' | 'detected' | 'wallet'>[],
  walletsFilterCb: (wallet: Wallet) => boolean,
  recentWalletName?: string
) => {
  const { get } = getWallets()
  const windowWallets = get()

  const walletsData: Record<string, IWalletListItem> = {}

  presetList.forEach((wallet) => {
    walletsData[wallet.name] = wallet
  })

  windowWallets.filter(walletsFilterCb).forEach((wallet) => {
    walletsData[wallet.name] = {
      ...(walletsData?.[wallet.name] ?? {
        name: wallet.name,
        icon: wallet.icon
      }),
      detected: true,
      recent: recentWalletName === wallet.name,
      standardWallet: wallet
    }
  })

  return Object.values(walletsData)
}

export const getWallet = (name: string) => {
  const { get } = getWallets()
  const windowWallets = get()

  return windowWallets.find((w) => w.name === name)
}
