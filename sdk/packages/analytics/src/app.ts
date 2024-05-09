import { AppConnectEvent } from '../../../bindings/AppConnectEvent'
import { AppDisconnectEvent } from '../../../bindings/AppDisconnectEvent'
import { ChangeNetworkEvent } from '../../../bindings/ChangeNetworkEvent'
import { ChangeNetworkResolveEvent } from '../../../bindings/ChangeNetworkResolveEvent'
import { ChangeWalletEvent } from '../../../bindings/ChangeWalletEvent'
import { ChangeWalletResolveEvent } from '../../../bindings/ChangeWalletResolveEvent'
import { ClientConnectEvent } from '../../../bindings/ClientConnectEvent'
import { ClientConnectResolveEvent } from '../../../bindings/ClientConnectResolveEvent'
import { ClientDisconnectEvent } from '../../../bindings/ClientDisconnectEvent'
import { HttpNightlyConnectCloudEvent } from '../../../bindings/HttpNightlyConnectCloudEvent'
import { SignAndSendTransactionEvent } from '../../../bindings/SignAndSendTransactionEvent'
import { SignAndSendTransactionResolveEvent } from '../../../bindings/SignAndSendTransactionResolveEvent'
import { SignMessageEvent } from '../../../bindings/SignMessageEvent'
import { SignMessageResolveEvent } from '../../../bindings/SignMessageResolveEvent'
import { SignTransactionEvent } from '../../../bindings/SignTransactionEvent'
import { SignTransactionResolveEvent } from '../../../bindings/SignTransactionResolveEvent'
import { DEFAULT_ANALYTICS_URL } from './utils'
import { fetch } from 'cross-fetch'

export interface NightlyAnalyticsParams {
  sessionId: string
  network: string
  appId: string
  endpoint?: string
}
// SDK for sending analytics
export class NightlyAnalytics {
  sessionId: string
  network: string
  endpoint: string = DEFAULT_ANALYTICS_URL // Endpoint for sending analytics
  appId: string

  public constructor(params: NightlyAnalyticsParams) {
    this.sessionId = params.sessionId
    this.network = params.network
    this.endpoint = params.endpoint ?? DEFAULT_ANALYTICS_URL
    this.appId = params.appId
  }

  sendEvent = async (request: HttpNightlyConnectCloudEvent, method = 'POST') => {
    // We don't need response
    return await fetch(this.endpoint, {
      body: JSON.stringify(request),
      method: method,
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json'
      }
    })
  }

  appConnected = async (event: AppConnectEvent) => {
    return await this.sendEvent({
      appId: this.appId,
      event: {
        type: 'AppConnect',
        ...event
      }
    })
  }

  appDisconnected = async (event: AppDisconnectEvent) => {
    return await this.sendEvent({
      appId: this.appId,
      event: {
        type: 'AppDisconnect',
        ...event
      }
    })
  }

  clientConnect = async (event: ClientConnectEvent) => {
    return await this.sendEvent({
      appId: this.appId,
      event: {
        type: 'ClientConnect',
        ...event
      }
    })
  }

  clientConnectResolve = async (event: ClientConnectResolveEvent) => {
    return await this.sendEvent({
      appId: this.appId,
      event: {
        type: 'ClientConnectResolve',
        ...event
      }
    })
  }

  clientDisconnect = async (event: ClientDisconnectEvent) => {
    return await this.sendEvent({
      appId: this.appId,
      event: {
        type: 'ClientDisconnect',
        ...event
      }
    })
  }

  signMessage = async (event: SignMessageEvent) => {
    return await this.sendEvent({
      appId: this.appId,
      event: {
        type: 'SignMessage',
        ...event
      }
    })
  }

  signMessageResolve = async (event: SignMessageResolveEvent) => {
    return await this.sendEvent({
      appId: this.appId,
      event: {
        type: 'SignMessageResolve',
        ...event
      }
    })
  }

  signTransaction = async (event: SignTransactionEvent) => {
    return await this.sendEvent({
      appId: this.appId,
      event: {
        type: 'SignTransaction',
        ...event
      }
    })
  }

  signTransactionResolve = async (event: SignTransactionResolveEvent) => {
    return await this.sendEvent({
      appId: this.appId,
      event: {
        type: 'SignTransactionResolve',
        ...event
      }
    })
  }

  signAndSendTransaction = async (event: SignAndSendTransactionEvent) => {
    return await this.sendEvent({
      appId: this.appId,
      event: {
        type: 'SignAndSendTransaction',
        ...event
      }
    })
  }

  signAndSendTransactionResolve = async (
    event: SignAndSendTransactionResolveEvent,
    originHeader: string
  ) => {
    return await this.sendEvent({
      appId: this.appId,
      event: {
        type: 'SignAndSendTransactionResolve',
        ...event
      }
    })
  }

  changeNetwork = async (event: ChangeNetworkEvent) => {
    return await this.sendEvent({
      appId: this.appId,
      event: {
        type: 'ChangeNetwork',
        ...event
      }
    })
  }

  changeNetworkResolve = async (event: ChangeNetworkResolveEvent) => {
    return await this.sendEvent({
      appId: this.appId,
      event: {
        type: 'ChangeNetworkResolve',
        ...event
      }
    })
  }

  changeWallet = async (event: ChangeWalletEvent) => {
    return await this.sendEvent({
      appId: this.appId,
      event: {
        type: 'ChangeWallet',
        ...event
      }
    })
  }

  changeWalletResolve = async (event: ChangeWalletResolveEvent) => {
    return await this.sendEvent({
      appId: this.appId,
      event: {
        type: 'ChangeWalletResolve',
        ...event
      }
    })
  }
}
