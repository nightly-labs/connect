import { type Wallet } from '@wallet-standard/core'
import { isWalletWithRequiredFeatureSet } from '@nightlylabs/aptos-wallet-standard'

export const aptosWalletsFilter = (wallet: Wallet) => isWalletWithRequiredFeatureSet(wallet, [])
