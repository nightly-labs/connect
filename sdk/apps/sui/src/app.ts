import { UserConnectedEvent } from '../../../bindings/UserConnectedEvent'
import type { SignedMessage, SignedTransaction } from '@mysten/sui.js/src/signers/types'
import {
  SuiSignAndExecuteTransactionBlockInput,
  SuiSignAndExecuteTransactionBlockOutput,
  SuiSignPersonalMessageInput,
  SuiSignTransactionBlockInput
} from '@mysten/wallet-standard'
import {
  BaseApp,
  DeeplinkConnect,
  getWalletsMetadata,
  MessageToSign,
  TransactionToSign
} from '@nightlylabs/nightly-connect-base'
import { EventEmitter } from 'eventemitter3'
import { AppSuiInitialize, SUI_NETWORK } from './utils'
import { UserDisconnectedEvent } from '../../../bindings/UserDisconnectedEvent'
import { WalletMetadata } from '../../../bindings/WalletMetadata'

interface SuiAppEvents {
  userConnected: (e: UserConnectedEvent) => void
  userDisconnected: (e: UserDisconnectedEvent) => void
  serverDisconnected: () => void
}
export class AppSui extends EventEmitter<SuiAppEvents> {
  sessionId: string
  base: BaseApp
  initData: AppSuiInitialize
  constructor(base: BaseApp, initData: AppSuiInitialize) {
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
      const base = await BaseApp.build({ ...this.initData, network: SUI_NETWORK })
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
    return this.base.connectedPublicKeys
  }
  public static getWalletsMetadata = async (url?: string): Promise<WalletMetadata[]> => {
    return getWalletsMetadata(url, 'sui')
  }
  public static build = async (initData: AppSuiInitialize): Promise<AppSui> => {
    const base = await BaseApp.build({ ...initData, network: SUI_NETWORK })
    return new AppSui(base, initData)
  }
  connectDeeplink = async (data: DeeplinkConnect) => {
    this.base.connectDeeplink(data)
  }
  signTransactionBlock = async (
    input: SuiSignTransactionBlockInput
  ): Promise<SignedTransaction> => {
    const transactionToSign: TransactionToSign = {
      transaction: input.transactionBlock.serialize(),
      metadata: JSON.stringify({ account: input.account, chain: input.chain })
    }
    const signedTx = await this.base.signTransactions([transactionToSign])

    return JSON.parse(signedTx[0].transaction)
  }

  signMessage = async (input: SuiSignPersonalMessageInput, encoding?: string) => {
    const request: MessageToSign = {
      message: new TextDecoder().decode(input.message),
      metadata: JSON.stringify({ encoding: encoding || 'hex', account: input.account })
    }
    const signedTx = await this.base.signMessages([request])
    return JSON.parse(signedTx[0].message) as SignedMessage
  }

  signAndExecuteTransactionBlock = async (
    input: SuiSignAndExecuteTransactionBlockInput
  ): Promise<SuiSignAndExecuteTransactionBlockOutput> => {
    const transactionToSign: TransactionToSign = {
      transaction: input.transactionBlock.serialize(),
      metadata: JSON.stringify({ account: input.account, chain: input.chain, execute: true })
    }
    const signedTx = await this.base.signTransactions([transactionToSign])

    return JSON.parse(signedTx[0].transaction)
  }
}
