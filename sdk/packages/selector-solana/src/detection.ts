import { Wallet } from '@wallet-standard/core'
import { getWalletsList, IWalletListItem, isStandardWalletAdapterCompatibleWallet } from '@nightlylabs/wallet-selector-base'

export const solanaWalletsFilter = (wallet: Wallet) => isStandardWalletAdapterCompatibleWallet(wallet, ['solana:signTransaction'])

export const getSolanaWalletsList = (
  presetList: Omit<IWalletListItem, 'recent' | 'detected'>[],
  recentWalletName?: string
) => getWalletsList(presetList, solanaWalletsFilter, recentWalletName)
