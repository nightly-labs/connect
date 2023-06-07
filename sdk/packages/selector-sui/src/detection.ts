import { Wallet } from '@wallet-standard/core'
import { getWalletsList, IWalletListItem, isStandardWalletAdapterCompatibleWallet } from '@nightlylabs/wallet-selector-base'

export const suiWalletsFilter = (wallet: Wallet) => isStandardWalletAdapterCompatibleWallet(wallet, ['sui:signTransactionBlock'])

export const getSuiWalletsList = (
  presetList: Omit<IWalletListItem, 'recent' | 'detected'>[],
  recentWalletName?: string
) => getWalletsList(presetList, suiWalletsFilter, recentWalletName)
