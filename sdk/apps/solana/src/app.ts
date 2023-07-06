import { PublicKey, Transaction, VersionedTransaction } from '@solana/web3.js'
import {
  AppBaseInitialize,
  BaseApp,
  getWalletsMetadata,
  MessageToSign,
  TransactionToSign,
  DeeplinkConnect
} from '@nightlylabs/nightly-connect-base'
import { SOLANA_NETWORK } from './utils'
import { EventEmitter } from 'eventemitter3'
import { UserDisconnectedEvent } from '../../../bindings/UserDisconnectedEvent'
import { UserConnectedEvent } from '../../../bindings/UserConnectedEvent'
import { WalletMetadata } from '../../../bindings/WalletMetadata'

export type AppSolanaInitialize = Omit<AppBaseInitialize, 'network'>
interface SolanaAppEvents {
  userConnected: (e: UserConnectedEvent) => void
  userDisconnected: (e: UserDisconnectedEvent) => void
  serverDisconnected: () => void
}
export class AppSolana extends EventEmitter<SolanaAppEvents> {
  sessionId: string
  base: BaseApp

  constructor(base: BaseApp) {
    super()

    this.base = base
    this.sessionId = base.sessionId
    this.base.on('userConnected', (e) => {
      this.emit('userConnected', e)
    })
    this.base.on('userDisconnected', (e) => {
      this.emit('userDisconnected', e)
    })
    this.base.on('serverDisconnected', () => {
      this.emit('serverDisconnected')
    })
  }
  public hasBeenRestored = () => {
    return this.base.hasBeenRestored
  }
  public get connectedPublicKeys() {
    return this.base.connectedPublicKeys.map((pk) => new PublicKey(pk))
  }
  public static getWalletsMetadata = async (url?: string): Promise<WalletMetadata[]> => {
    return getWalletsMetadata(url)
  }
  public static build = async (initData: AppSolanaInitialize): Promise<AppSolana> => {
    const base = await BaseApp.build({ ...initData, network: SOLANA_NETWORK })
    base.connectDeeplink
    return new AppSolana(base)
  }
  connectDeeplink = async (data: DeeplinkConnect) => {
    this.base.connectDeeplink(data)
  }
  signTransaction = async (transaction: Transaction) => {
    return await this.signVersionedTransaction(
      new VersionedTransaction(transaction.compileMessage())
    )
  }

  signVersionedTransaction = async (transaction: VersionedTransaction) => {
    const transactionToSign: TransactionToSign = {
      transaction: Buffer.from(transaction.serialize()).toString('hex')
    }
    const signedTxs = await this.base.signTransactions([transactionToSign])

    return VersionedTransaction.deserialize(Buffer.from(signedTxs[0].transaction, 'hex'))
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
    const parsed = signedTx.map((tx) => Transaction.from(Buffer.from(tx.transaction, 'hex')))
    return parsed
  }

  signMessage = async (message: string, encoding?: string) => {
    const request: MessageToSign = {
      message,
      metadata: JSON.stringify({ encoding: encoding || 'hex' })
    }
    const signedTx = await this.base.signMessages([request])
    return Uint8Array.from(Buffer.from(signedTx[0].message, 'hex'))
  }
}
