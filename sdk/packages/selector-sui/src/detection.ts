import { Wallet } from '@wallet-standard/core'
import { getWalletsList, IWalletListItem } from '@nightlylabs/wallet-selector-base'

export const suiWalletsFilter = (wallet: Wallet) => 'sui:signTransactionBlock' in wallet.features

export const getSuiWalletsList = (
  presetList: Omit<IWalletListItem, 'recent' | 'detected'>[],
  recentWalletName?: string
) => getWalletsList(presetList, suiWalletsFilter, recentWalletName)
