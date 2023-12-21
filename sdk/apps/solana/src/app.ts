import { Keypair, PublicKey, Transaction, VersionedTransaction } from '@solana/web3.js'
import {
  BaseApp,
  getWalletsMetadata,
  MessageToSign,
  TransactionToSign,
  DeeplinkConnect
} from '@nightlylabs/nightly-connect-base'
import { AppSolanaInitialize, SOLANA_NETWORK } from './utils'
import { EventEmitter } from 'eventemitter3'
import { UserDisconnectedEvent } from '../../../bindings/UserDisconnectedEvent'
import { UserConnectedEvent } from '../../../bindings/UserConnectedEvent'
import { WalletMetadata } from '../../../bindings/WalletMetadata'

interface SolanaAppEvents {
  userConnected: (e: UserConnectedEvent) => void
  userDisconnected: (e: UserDisconnectedEvent) => void
  serverDisconnected: () => void
}
export class AppSolana extends EventEmitter<SolanaAppEvents> {
  sessionId: string
  base: BaseApp
  initData: AppSolanaInitialize
  constructor(base: BaseApp, initData: AppSolanaInitialize) {
    super()
    this.initData = initData
    this.base = base
    this.sessionId = base.sessionId
    this.base.on('userConnected', (e) => {
      this.emit('userConnected', e)
    })
    this.base.on('userDisconnected', (e) => {
      this.emit('userDisconnected', e)
    })
    this.base.on('serverDisconnected', async () => {
      // We need this because of power saving mode on mobile
      await this.tryReconnect()
    })
  }
  private tryReconnect = async () => {
    try {
      const base = await BaseApp.build({ ...this.initData, network: SOLANA_NETWORK })
      // On reconnect, if the base has not been restored, emit serverDisconnected
      if (!base.hasBeenRestored) {
        this.emit('serverDisconnected')
        return
      }
      // If user was connected, emit userConnected
      if (base.connectedPublicKeys.length > 0) {
        this.emit('userConnected', {
          publicKeys: base.connectedPublicKeys,
          metadata: base.clientMetadata
        })
      }
      base.on('userConnected', (e) => {
        this.emit('userConnected', e)
      })
      base.on('userDisconnected', (e) => {
        this.emit('userDisconnected', e)
      })
      base.on('serverDisconnected', async () => {
        await this.tryReconnect()
      })
      // If there is a deeplink, reconnect to it
      if (this.base.deeplink) {
        base.connectDeeplink(this.base.deeplink)
      }
      this.base = base
      return
    } catch (_) {
      this.emit('serverDisconnected')
    }
  }
  public hasBeenRestored = () => {
    return this.base.hasBeenRestored
  }
  public get connectedPublicKeys() {
    return this.base.connectedPublicKeys.map((pk) => new PublicKey(pk))
  }
  public static getWalletsMetadata = async (url?: string): Promise<WalletMetadata[]> => {
    return getWalletsMetadata(url, 'solana')
  }
  public static build = async (initData: AppSolanaInitialize): Promise<AppSolana> => {
    const base = await BaseApp.build({ ...initData, network: SOLANA_NETWORK })
    return new AppSolana(base, initData)
  }
  connectDeeplink = async (data: DeeplinkConnect) => {
    this.base.connectDeeplink(data)
  }
  signTransaction = async (transaction: Transaction) => {
    const serialized = Buffer.from(
      transaction.serialize({ requireAllSignatures: false, verifySignatures: false })
    ).toString('hex')
    return await this.signVersionedTransaction(
      VersionedTransaction.deserialize(Buffer.from(serialized, 'hex'))
    )
  }

  signVersionedTransaction = async (transaction: VersionedTransaction) => {
    const transactionToSign: TransactionToSign = {
      transaction: Buffer.from(transaction.serialize()).toString('hex')
    }
    const signedTxs = await this.base.signTransactions([transactionToSign])
    const signed = VersionedTransaction.deserialize(Buffer.from(signedTxs[0].transaction, 'hex'))
    VersionedTransaction.prototype['partialSign'] = function (this, keypair: Keypair) {
      return this.sign([keypair])
    }
    return signed
  }

  signAllTransactions = async (transactions: Transaction[]) => {
    return await this.signAllVersionedTransactions(
      transactions.map((tx) => {
        const serialized = Buffer.from(
          tx.serialize({ requireAllSignatures: false, verifySignatures: false })
        ).toString('hex')
        return VersionedTransaction.deserialize(Buffer.from(serialized, 'hex'))
      })
    )
  }

  signAllVersionedTransactions = async (transactions: VersionedTransaction[]) => {
    const transactionsToSign: TransactionToSign[] = transactions.map((tx) => ({
      network: SOLANA_NETWORK,
      transaction: Buffer.from(tx.serialize()).toString('hex')
    }))
    const signedTx = await this.base.signTransactions(transactionsToSign)
    const parsed = signedTx.map((tx) => {
      VersionedTransaction.prototype['partialSign'] = function (this, keypair: Keypair) {
        return this.sign([keypair])
      }
      return VersionedTransaction.deserialize(Uint8Array.from(Buffer.from(tx.transaction, 'hex')))
    })
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
