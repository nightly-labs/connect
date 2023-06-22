import { Wallet } from '@wallet-standard/core'
import { getWalletsList, IWalletListItem } from '@nightlylabs/wallet-selector-base'
import { isStandardWalletAdapterCompatibleWallet } from '@mysten/wallet-standard'

export const suiWalletsFilter = (wallet: Wallet) =>
  isStandardWalletAdapterCompatibleWallet(wallet, [
    'sui:signAndExecuteTransactionBlock',
    'sui:signTransactionBlock',
    'sui:signMessage'
  ])

export const getSuiWalletsList = (
  presetList: Omit<IWalletListItem, 'recent' | 'detected'>[],
  recentWalletName?: string
) => getWalletsList(presetList, suiWalletsFilter, recentWalletName)
