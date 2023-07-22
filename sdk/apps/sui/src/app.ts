import { UserConnectedEvent } from '../../../bindings/UserConnectedEvent'
import { SignedMessage, SignedTransaction } from '@mysten/sui.js'
import {
  SuiSignAndExecuteTransactionBlockInput,
  SuiSignAndExecuteTransactionBlockOutput,
  SuiSignMessageInput,
  SuiSignTransactionBlockInput
} from '@mysten/wallet-standard'
import {
  AppBaseInitialize,
  BaseApp,
  DeeplinkConnect,
  getWalletsMetadata,
  MessageToSign,
  TransactionToSign
} from '@nightlylabs/nightly-connect-base'
import { EventEmitter } from 'eventemitter3'
import { SUI_NETWORK } from './utils'
import { UserDisconnectedEvent } from '../../../bindings/UserDisconnectedEvent'
import { WalletMetadata } from '../../../bindings/WalletMetadata'
export type AppSuiInitialize = Omit<AppBaseInitialize, 'network'>
interface SuiAppEvents {
  userConnected: (e: UserConnectedEvent) => void
  userDisconnected: (e: UserDisconnectedEvent) => void
  serverDisconnected: () => void
}
export class AppSui extends EventEmitter<SuiAppEvents> {
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
    return this.base.connectedPublicKeys
  }
  public static getWalletsMetadata = async (url?: string): Promise<WalletMetadata[]> => {
    return getWalletsMetadata(url)
  }
  public static build = async (initData: AppSuiInitialize): Promise<AppSui> => {
    const base = await BaseApp.build({ ...initData, network: SUI_NETWORK })
    return new AppSui(base)
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

  signMessage = async (input: SuiSignMessageInput, encoding?: string) => {
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
