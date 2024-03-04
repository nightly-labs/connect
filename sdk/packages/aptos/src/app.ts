import { UserConnectedEvent } from '../../../bindings/UserConnectedEvent'
import {
  AccountInfo,
  AptosSignAndSubmitTransactionMethod,
  AptosSignMessageMethod,
  AptosSignMessageOutput,
  AptosSignTransactionMethod,
  NetworkInfo,
  UserResponseStatus
} from '@aptos-labs/wallet-standard'
import {
  BaseApp,
  DeeplinkConnect,
  getWalletsMetadata,
  MessageToSign,
  TransactionToSign
} from '@nightlylabs/nightly-connect-base'
import { EventEmitter } from 'eventemitter3'
import {
  AppAptosInitialize,
  APTOS_NETWORK,
  deserializeAccountAuthenticator,
  deserializeConnectData,
  deserializeObject,
  deserializePendingTransactionResponse,
  serializeAptosTx,
  serializeObject
} from './utils'
import { UserDisconnectedEvent } from '../../../bindings/UserDisconnectedEvent'
import { WalletMetadata } from '../../../bindings/WalletMetadata'

interface AptosAppEvents {
  userConnected: (e: AccountInfo, networkInfo: NetworkInfo) => void
  userDisconnected: (e: UserDisconnectedEvent) => void
  serverDisconnected: () => void
}
export class AppAptos extends EventEmitter<AptosAppEvents> {
  sessionId: string
  base: BaseApp
  initData: AppAptosInitialize
  constructor(base: BaseApp, initData: AppAptosInitialize) {
    super()
    this.initData = initData
    this.base = base
    this.sessionId = base.sessionId
    this.base.on('userConnected', (e) => {
      if (e.metadata === undefined) {
        // Metadata has to be defined
        return
      }
      const { accountInfo, networkInfo } = deserializeConnectData(e.metadata)
      this.emit('userConnected', accountInfo, networkInfo)
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
      // If user was connected, emit userConnected
      if (base.connectedPublicKeys.length > 0) {
        if (base.clientMetadata === undefined) {
          // Metadata has to be defined
          this.emit('serverDisconnected')

          return
        }
        const { accountInfo, networkInfo } = deserializeConnectData(base.clientMetadata)
        this.emit('userConnected', accountInfo, networkInfo)
      }
      base.on('userConnected', (e) => {
        if (e.metadata === undefined) {
          // Metadata has to be defined
          return
        }
        const { accountInfo, networkInfo } = deserializeConnectData(e.metadata)
        this.emit('userConnected', accountInfo, networkInfo)
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
    return this.base.connectedPublicKeys
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
  signAndSubmitTransaction: AptosSignAndSubmitTransactionMethod = async (tx) => {
    const transactionToSign: TransactionToSign = {
      transaction: serializeAptosTx(tx),
      metadata: JSON.stringify({ execute: true })
    }
    const signedTx = await this.base.signTransactions([transactionToSign])
    return {
      status: UserResponseStatus.APPROVED,
      args: deserializePendingTransactionResponse(signedTx[0].transaction)
    }
  }

  signTransaction: AptosSignTransactionMethod = async (tx, asFeePayer) => {
    const transactionToSign: TransactionToSign = {
      transaction: serializeAptosTx(tx),
      metadata: JSON.stringify({ asFeePayer: asFeePayer, execute: false })
    }
    const signedTx = await this.base.signTransactions([transactionToSign])
    return {
      status: UserResponseStatus.APPROVED,
      args: deserializeAccountAuthenticator(signedTx[0].transaction)
    }
  }

  signMessage: AptosSignMessageMethod = async (input) => {
    const request: MessageToSign = {
      message: serializeObject(input)
    }
    const signedTx = await this.base.signMessages([request])
    return {
      status: UserResponseStatus.APPROVED,
      args: deserializeObject(signedTx[0].message) as AptosSignMessageOutput
    }
  }
}
