import {
  IotaSignAndExecuteTransactionInput,
  IotaSignAndExecuteTransactionOutput,
  IotaSignPersonalMessageInput,
  IotaSignTransactionInput
} from '@iota/wallet-standard'
import {
  BaseApp,
  DeeplinkConnect,
  MessageToSign,
  TransactionToSign,
  getWalletsMetadata
} from '@nightlylabs/nightly-connect-base'
import { EventEmitter } from 'eventemitter3'
import { UserConnectedEvent } from '../../../bindings/UserConnectedEvent'
import { UserDisconnectedEvent } from '../../../bindings/UserDisconnectedEvent'
import { WalletMetadata } from '../../../bindings/WalletMetadata'
import { AppIotaInitialize, IOTA_NETWORK, SignedMessage, SignedTransaction } from './utils'

interface IotaAppEvents {
  userConnected: (e: UserConnectedEvent) => void
  userDisconnected: (e: UserDisconnectedEvent) => void
  serverDisconnected: () => void
}
export class AppIota extends EventEmitter<IotaAppEvents> {
  sessionId: string
  base: BaseApp
  initData: AppIotaInitialize
  constructor(base: BaseApp, initData: AppIotaInitialize) {
    super()
    this.initData = initData
    this.base = base
    this.sessionId = base.sessionId
    this.base.on('userConnected', (e) => {
      this.emit('userConnected', e)
    })
    this.base.on('userDisconnected', (e) => {
      this.emit('userDisconnected', { message: 'User disconnected' })
    })
    this.base.on('serverDisconnected', async () => {
      // We need this because of power saving mode on mobile
      await this.tryReconnect()
    })
  }
  private tryReconnect = async () => {
    try {
      const base = await BaseApp.build({ ...this.initData, network: IOTA_NETWORK })
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
        this.emit('userDisconnected', { message: 'User disconnected' })
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
    return this.base.connectedPublicKeys
  }
  public static getWalletsMetadata = async (url?: string): Promise<WalletMetadata[]> => {
    return getWalletsMetadata(url, 'sui')
  }
  public static build = async (initData: AppIotaInitialize): Promise<AppIota> => {
    const base = await BaseApp.build({ ...initData, network: IOTA_NETWORK })
    return new AppIota(base, initData)
  }
  connectDeeplink = async (data: DeeplinkConnect) => {
    this.base.connectDeeplink(data)
  }
  signTransactionBlock = async (input: IotaSignTransactionInput): Promise<SignedTransaction> => {
    const transactionToSign: TransactionToSign = {
      transaction: await input.transaction.toJSON(),
      metadata: JSON.stringify({ account: input.account, chain: input.chain })
    }
    const signedTx = await this.base.signTransactions([transactionToSign])

    return JSON.parse(signedTx[0].transaction)
  }

  signMessage = async (input: IotaSignPersonalMessageInput, encoding?: string) => {
    const request: MessageToSign = {
      message: new TextDecoder().decode(input.message),
      metadata: JSON.stringify({ encoding: encoding || 'hex', account: input.account })
    }
    const signedTx = await this.base.signMessages([request])
    return JSON.parse(signedTx[0].message) as SignedMessage
  }

  signAndExecuteTransactionBlock = async (
    input: IotaSignAndExecuteTransactionInput
  ): Promise<IotaSignAndExecuteTransactionOutput> => {
    const transactionToSign: TransactionToSign = {
      transaction: await input.transaction.toJSON(),
      metadata: JSON.stringify({ account: input.account, chain: input.chain, execute: true })
    }
    const signedTx = await this.base.signTransactions([transactionToSign])

    return JSON.parse(signedTx[0].transaction)
  }
}
