import { IWalletListItem, WalletMetadata } from '@nightlylabs/wallet-selector-base'
import { isWalletAdapterCompatibleStandardWallet } from '@solana/wallet-adapter-base'
import { getWallets, Wallet } from '@wallet-standard/core'

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

  return Object.values(walletsData)
}
