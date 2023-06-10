import { Wallet } from '@wallet-standard/core'
import { getWalletsList, IWalletListItem } from '@nightlylabs/wallet-selector-base'
import { isWalletAdapterCompatibleStandardWallet } from '@solana/wallet-adapter-base'

export const solanaWalletsFilter = (wallet: Wallet) => isWalletAdapterCompatibleStandardWallet(wallet)

export const getSolanaWalletsList = (
  presetList: Omit<IWalletListItem, 'recent' | 'detected'>[],
  recentWalletName?: string
) => getWalletsList(presetList, solanaWalletsFilter, recentWalletName)
