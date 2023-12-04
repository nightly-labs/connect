import type {
  AccountInfo,
  NetworkInfo,
  SignMessagePayload,
  SignMessageResponse,
  Types
} from '@aptos-labs/wallet-adapter-core'
import {
  BaseApp,
  DeeplinkConnect,
  MessageToSign,
  TransactionToSign,
  getWalletsMetadata
} from '@nightlylabs/nightly-connect-base'
import EventEmitter from 'eventemitter3'
import { UserDisconnectedEvent } from '../../../bindings/UserDisconnectedEvent'
import { WalletMetadata } from '../../../bindings/WalletMetadata'
import { APTOS_NETWORK, AppAptosInitialize, deserializeSignMessageResponse } from './utils'
export interface AptosUserConnectedEvent {
  accounts: Array<AccountInfo>
  networkInfo: NetworkInfo
  metadata?: string
}

interface SuiAppEvents {
  userConnected: (e: AptosUserConnectedEvent) => void
  userDisconnected: (e: UserDisconnectedEvent) => void
  serverDisconnected: () => void
}
export class AppAptos extends EventEmitter<SuiAppEvents> {
  // Nightly Connect
  sessionId: string
  base: BaseApp
  initData: AppAptosInitialize

  constructor(base: BaseApp, initData: AppAptosInitialize) {
    super()
    this.initData = initData
    this.base = base
    this.sessionId = base.sessionId
    this.base.on('userConnected', (e) => {
      const keys = e.publicKeys.map((pk) => {
        const accountInfo: AccountInfo = JSON.parse(pk)
        return accountInfo
      })
      // metadata here should include networkInfo
      const networkInfo: NetworkInfo = JSON.parse(e.metadata!).networkInfo
      this.emit('userConnected', { accounts: keys, metadata: e.metadata, networkInfo })
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
      const base = await BaseApp.build({ ...this.initData, network: APTOS_NETWORK })
      // On reconnect, if the base has not been restored, emit serverDisconnected
      if (!base.hasBeenRestored) {
        this.emit('serverDisconnected')
        return
      }
      base.on('userConnected', (e) => {
        const keys = e.publicKeys.map((pk) => {
          const accountInfo: AccountInfo = JSON.parse(pk)
          return accountInfo
        })
        // metadata here should include networkInfo
        const networkInfo: NetworkInfo = JSON.parse(e.metadata!).networkInfo
        this.emit('userConnected', { accounts: keys, metadata: e.metadata, networkInfo })
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
      console.warn('Could not reconnect to nightly server')
    }
  }

  public hasBeenRestored = () => {
    return this.base.hasBeenRestored
  }
  public get accountInfo() {
    const accountInfo: AccountInfo = JSON.parse(this.base.connectedPublicKeys[0])
    return accountInfo
  }
  public get networkInfo() {
    const networkInfo: NetworkInfo = JSON.parse(this.base.connectedMetadata!).networkInfo
    return networkInfo
  }
  public get connectedPublicKeys() {
    return this.base.connectedPublicKeys
  }
  public get connectedMetadata() {
    return this.base.connectedMetadata
  }
  public static getWalletsMetadata = async (url?: string): Promise<WalletMetadata[]> => {
    return getWalletsMetadata(url, 'aptos')
  }
  public static build = async (initData: AppAptosInitialize): Promise<AppAptos> => {
    const base = await BaseApp.build({ ...initData, network: APTOS_NETWORK })
    return new AppAptos(base, initData)
  }
  connectDeeplink = async (data: DeeplinkConnect) => {
    this.base.connectDeeplink(data)
  }

  async signAndSubmitTransaction(
    transaction: Types.TransactionPayload,
    options?: any
  ): Promise<{ hash: Types.HexEncodedBytes }> {
    const transactionToSign: TransactionToSign = {
      transaction: JSON.stringify(transaction),
      metadata: JSON.stringify({ ...options, submit: true })
    }
    const signedTx = await this.base.signTransactions([transactionToSign])

    return JSON.parse(signedTx[0].transaction)
  }

  async signMessage(message: SignMessagePayload): Promise<SignMessageResponse> {
    if (typeof message !== 'object' || !message.nonce) {
      throw `Nightly Invalid signMessage Payload`
    }
    const request: MessageToSign = {
      message: JSON.stringify(message),
      metadata: undefined
    }
    const signedTx = await this.base.signMessages([request])
    return deserializeSignMessageResponse(signedTx[0].message)
  }
}
