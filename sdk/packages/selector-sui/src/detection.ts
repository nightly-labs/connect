import { Wallet } from '@wallet-standard/core'

import { isStandardWalletAdapterCompatibleWallet } from '@mysten/wallet-standard'

export const suiWalletsFilter = (wallet: Wallet) => {
  const is = isStandardWalletAdapterCompatibleWallet(wallet, [
    'sui:signAndExecuteTransactionBlock',
    'sui:signTransactionBlock'
  ])
  console.log(wallet)
  console.log(is)
  return is
}
