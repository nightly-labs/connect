import { SignedMessage, SignedTransaction, TransactionBlock } from '@mysten/sui.js'
import { AppBaseInitialize, BaseApp } from 'base'
import { MessageToSign, TransactionToSign } from 'base/src/content'
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
      transaction: transaction.serialize()
    }
    const signedTx = await this.base.signTransactions([transactionToSign])

    return JSON.parse(signedTx[0].transaction)
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
    const parsed = signedTx.map((tx) => JSON.parse(tx.transaction) as SignedTransaction)
    return parsed
  }

  signMessage = async (message: string, encoding?: string) => {
    const request: MessageToSign = {
      message,
      metadata: JSON.stringify({ encoding: encoding || 'hex' })
    }
    const signedTx = await this.base.signMessages([request])
    return JSON.parse(signedTx[0].message) as SignedMessage
  }
}
