import { type Wallet } from '@wallet-standard/core'
import { isWalletAdapterCompatibleStandardWallet } from '@solana/wallet-adapter-base'

export const solanaWalletsFilter = (wallet: Wallet) =>
  isWalletAdapterCompatibleStandardWallet(wallet)
