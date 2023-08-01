import { Wallet } from '@wallet-standard/core'

import { isStandardWalletAdapterCompatibleWallet } from '@mysten/wallet-standard'

export const suiWalletsFilter = (wallet: Wallet) =>
  isStandardWalletAdapterCompatibleWallet(wallet, [
    'sui:signAndExecuteTransactionBlock',
    'sui:signTransactionBlock',
    'sui:signMessage'
  ])
