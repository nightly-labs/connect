import { SignedTransaction, TransactionBlock } from '@mysten/sui.js'
import { MessageToSign } from '@bindings/MessageToSign'
import { TransactionToSign } from '@bindings/TransactionToSign'
import { AppBaseInitialize, BaseApp } from 'base'
import { SUI_NETWORK } from './utils'
export type AppSuiInitialize = Omit<AppBaseInitialize, 'network'>
export class AppSui {
  sessionId: string
  base: BaseApp

  constructor(base: BaseApp) {
    this.base = base
    this.sessionId = base.sessionId
  }

  public static build = async (initData: AppSuiInitialize): Promise<AppSui> => {
    const base = await BaseApp.build({ ...initData, network: SUI_NETWORK })
    return new AppSui(base)
  }
  signTransaction = async (transaction: TransactionBlock) => {
    return await this.signTransactionBlock(transaction)
  }

  signTransactionBlock = async (transaction: TransactionBlock): Promise<SignedTransaction> => {
    const transactionToSign: TransactionToSign = {
      network: SUI_NETWORK,
      transaction: transaction.serialize()
    }
    const signedTx = await this.base.signTransactions([transactionToSign])

    return JSON.parse(signedTx.signed_transactions[0].transaction)
  }

  signAllTransactions = async (transactions: TransactionBlock[]) => {
    return await this.signAllTransactionBlocks(transactions)
  }

  signAllTransactionBlocks = async (
    transactions: TransactionBlock[]
  ): Promise<SignedTransaction[]> => {
    const transactionsToSign: TransactionToSign[] = transactions.map((tx) => ({
      network: SUI_NETWORK,
      transaction: tx.serialize()
    }))
    const signedTx = await this.base.signTransactions(transactionsToSign)
    const parsed = signedTx.signed_transactions.map(
      (tx) => JSON.parse(tx.transaction) as SignedTransaction
    )
    return parsed
  }

  signMessage = async (message: string, encoding?: string) => {
    const request: MessageToSign = {
      message,
      metadata: JSON.stringify({ encoding: encoding || 'hex' })
    }
    const signedTx = await this.base.signMessages([request])
    return Uint8Array.from(Buffer.from(signedTx.signedMessages[0].signedMessage, 'hex'))
  }
}
