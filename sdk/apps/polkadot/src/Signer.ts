import { BaseApp, TransactionToSign } from '@nightlylabs/nightly-connect-base'
import type { Signer as SignerInterface, SignerResult } from '@polkadot/api/types'
import type { SignerPayloadRaw } from '@polkadot/types/types'

export class Signer implements SignerInterface {
  base: BaseApp
  constructor(base: BaseApp) {
    this.base = base
  }

  signRaw = async (payload: SignerPayloadRaw): Promise<SignerResult> => {
    const transactionToSign: TransactionToSign = {
      transaction: JSON.stringify(payload),
      metadata: JSON.stringify({
        network: this.base.initializeData.network
      })
    }
    const signedTxs = await this.base.signTransactions([transactionToSign])
    const result = JSON.parse(signedTxs[0].transaction) as SignerResult
    return result
  }
  // signPayload = async (payload: SignerPayloadJSON): Promise<SignerResult> => { // TODO: commented until we find a method to turn recreate tx from SignerPayloadJSON
  //   const transactionToSign: TransactionToSign = {
  //     transaction: JSON.stringify(payload)
  //   }
  //   const signedTxs = await this.base.signTransactions([transactionToSign])
  //   const result = JSON.parse(signedTxs[0].transaction) as SignerResult
  //   return result
  // }
  // Ignore update
}
