import { isWalletWithRequiredFeatureSet, Wallet, getWallets } from '@iota/wallet-standard'
import { IWalletListItem, WalletMetadata } from '@nightlylabs/wallet-selector-base'

export const suiWalletsFilter = (wallet: Wallet) => {
  const is = isWalletWithRequiredFeatureSet(wallet, [
    'sui:signAndExecuteTransactionBlock',
    'sui:signTransactionBlock'
  ])
  return is
}

export const getSuiWalletsList = (presetList: WalletMetadata[], recentWalletName?: string) => {
  const { get } = getWallets()
  const windowWallets = get()

  const walletsData: Record<string, IWalletListItem> = {}

  presetList.forEach((wallet) => {
    walletsData[wallet.name] = {
      ...wallet,
      recent: recentWalletName === wallet.name
    }
  })

  windowWallets.filter(suiWalletsFilter).forEach((wallet) => {
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
