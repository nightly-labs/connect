import { Wallet } from '@wallet-standard/core'
import { getWalletsList, IWalletListItem } from '@nightlylabs/wallet-selector-base'

export const solanaWalletsFilter = (wallet: Wallet) => 'solana:signTransaction' in wallet.features

export const getSolanaWalletsList = (
  presetList: Omit<IWalletListItem, 'recent' | 'detected'>[],
  recentWalletName?: string
) => getWalletsList(presetList, solanaWalletsFilter, recentWalletName)
