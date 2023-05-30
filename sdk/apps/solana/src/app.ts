import { MessageToSign } from '@bindings/MessageToSign'
import { TransactionToSign } from '@bindings/TransactionToSign'
import { Transaction, VersionedTransaction } from '@solana/web3.js'
import { AppBaseInitialize, BaseApp } from 'base'
import { SOLANA_NETWORK } from './utils'
export type AppSolanaInitialize = Omit<AppBaseInitialize, 'network'>
export class AppSolana {
  sessionId: string
  base: BaseApp

  constructor(base: BaseApp) {
    this.base = base
    this.sessionId = base.sessionId
  }

  public static build = async (initData: AppSolanaInitialize): Promise<AppSolana> => {
    const base = await BaseApp.build({ ...initData, network: SOLANA_NETWORK })
    return new AppSolana(base)
  }
  signTransaction = async (transaction: Transaction) => {
    return await this.signVersionedTransaction(
      new VersionedTransaction(transaction.compileMessage())
    )
  }

  signVersionedTransaction = async (transaction: VersionedTransaction) => {
    const transactionToSign: TransactionToSign = {
      network: SOLANA_NETWORK,
      transaction: Buffer.from(transaction.serialize()).toString('hex')
    }
    const signedTx = await this.base.signTransactions([transactionToSign])

    return VersionedTransaction.deserialize(
      Buffer.from(signedTx.signed_transactions[0].transaction, 'hex')
    )
  }

  signAllTransactions = async (transactions: Transaction[]) => {
    return await this.signAllVersionedTransactions(
      transactions.map((tx) => new VersionedTransaction(tx.compileMessage()))
    )
  }

  signAllVersionedTransactions = async (transactions: VersionedTransaction[]) => {
    const transactionsToSign: TransactionToSign[] = transactions.map((tx) => ({
      network: SOLANA_NETWORK,
      transaction: Buffer.from(tx.serialize()).toString('hex')
    }))
    const signedTx = await this.base.signTransactions(transactionsToSign)
    const parsed = signedTx.signed_transactions.map((tx) =>
      Transaction.from(Buffer.from(tx.transaction, 'hex'))
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
