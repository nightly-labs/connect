import { getWallets, Wallet } from '@wallet-standard/core'
import { isWalletAdapterCompatibleStandardWallet } from '@solana/wallet-adapter-base'
import { IWalletListItem, WalletMetadata } from '@nightlylabs/wallet-selector-base'

export const solanaWalletsFilter = (wallet: Wallet) =>
  isWalletAdapterCompatibleStandardWallet(wallet)

export const getSolanaWalletsList = (presetList: WalletMetadata[], recentWalletName?: string) => {
  const { get } = getWallets()
  const windowWallets = get()

  const walletsData: Record<string, IWalletListItem> = {}

  presetList.forEach((wallet) => {
    walletsData[wallet.name] = {
      ...wallet,
      recent: recentWalletName === wallet.name
    }
  })

  windowWallets.filter(solanaWalletsFilter).forEach((wallet) => {
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
