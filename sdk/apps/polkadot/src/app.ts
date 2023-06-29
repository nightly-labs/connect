import {
  AppBaseInitialize,
  BaseApp,
  getWalletsMetadata,
  TransactionToSign,
  DeeplinkConnect
} from '@nightlylabs/nightly-connect-base'
import { POLKADOT_NETWORK } from './utils'
import { EventEmitter } from 'eventemitter3'
import { UserDisconnectedEvent } from '../../../bindings/UserDisconnectedEvent'
import { UserConnectedEvent } from '../../../bindings/UserConnectedEvent'
import { WalletMetadata } from '../../../bindings/WalletMetadata'
import { SignerPayloadJSON, SignerPayloadRaw, SignerResult } from '@polkadot/types/types'

export type AppPolkadotInitialize = Omit<AppBaseInitialize, 'network'>
interface PolkadotAppEvents {
  userConnected: (e: UserConnectedEvent) => void
  userDisconnected: (e: UserDisconnectedEvent) => void
  serverDisconnected: () => void
}
export class AppPolkadot extends EventEmitter<PolkadotAppEvents> {
  sessionId: string
  base: BaseApp
  transactionId = 0

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
  public static getWalletsMetadata = async (url?: string): Promise<WalletMetadata[]> => {
    return getWalletsMetadata(url)
  }
  public static build = async (initData: AppPolkadotInitialize): Promise<AppPolkadot> => {
    const base = await BaseApp.build({ ...initData, network: POLKADOT_NETWORK })
    base.connectDeeplink
    return new AppPolkadot(base)
  }
  connectDeeplink = async (data: DeeplinkConnect) => {
    this.base.connectDeeplink(data)
  }

  signPayload = async (payload: SignerPayloadJSON): Promise<SignerResult> => {
    const id = ++this.transactionId
    const transactionToSign: TransactionToSign = {
      transaction: JSON.stringify(payload)
    }
    const signedTxs = await this.base.signTransactions([transactionToSign])
    const result = JSON.parse(signedTxs[0].transaction) as SignerResult
    return {
      ...result,
      id
    }
  }

  signRaw = async (payload: SignerPayloadRaw): Promise<SignerResult> => {
    const id = ++this.transactionId
    const transactionToSign: TransactionToSign = {
      transaction: JSON.stringify(payload)
    }
    const signedTxs = await this.base.signTransactions([transactionToSign])
    const result = JSON.parse(signedTxs[0].transaction) as SignerResult
    return {
      ...result,
      id
    }
  }
}
