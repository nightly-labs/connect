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

  sendEvent = async (
    request: HttpNightlyConnectCloudEvent,
    originHeader: string,
    method = 'POST'
  ) => {
    // We don't need response
    return await await fetch(this.endpoint, {
      body: JSON.stringify(request),
      method: method,
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
        Origin: originHeader
      }
    })
  }

  appConnected = async (event: AppConnectEvent, originHeader: string) => {
    return await this.sendEvent(
      {
        appId: this.appId,
        event: {
          type: 'AppConnect',
          ...event
        }
      },
      originHeader
    )
  }

  appDisconnected = async (event: AppDisconnectEvent, originHeader: string) => {
    return await this.sendEvent(
      {
        appId: this.appId,
        event: {
          type: 'AppDisconnect',
          ...event
        }
      },
      originHeader
    )
  }

  clientConnect = async (event: ClientConnectEvent, originHeader: string) => {
    return await this.sendEvent(
      {
        appId: this.appId,
        event: {
          type: 'ClientConnect',
          ...event
        }
      },
      originHeader
    )
  }

  clientConnectResolve = async (event: ClientConnectResolveEvent, originHeader: string) => {
    return await this.sendEvent(
      {
        appId: this.appId,
        event: {
          type: 'ClientConnectResolve',
          ...event
        }
      },
      originHeader
    )
  }

  clientDisconnect = async (event: ClientDisconnectEvent, originHeader: string) => {
    return await this.sendEvent(
      {
        appId: this.appId,
        event: {
          type: 'ClientDisconnect',
          ...event
        }
      },
      originHeader
    )
  }

  signMessage = async (event: SignMessageEvent, originHeader: string) => {
    return await this.sendEvent(
      {
        appId: this.appId,
        event: {
          type: 'SignMessage',
          ...event
        }
      },
      originHeader
    )
  }

  signMessageResolve = async (event: SignMessageResolveEvent, originHeader: string) => {
    return await this.sendEvent(
      {
        appId: this.appId,
        event: {
          type: 'SignMessageResolve',
          ...event
        }
      },
      originHeader
    )
  }

  signTransaction = async (event: SignTransactionEvent, originHeader: string) => {
    return await this.sendEvent(
      {
        appId: this.appId,
        event: {
          type: 'SignTransaction',
          ...event
        }
      },
      originHeader
    )
  }

  signTransactionResolve = async (event: SignTransactionResolveEvent, originHeader: string) => {
    return await this.sendEvent(
      {
        appId: this.appId,
        event: {
          type: 'SignTransactionResolve',
          ...event
        }
      },
      originHeader
    )
  }

  signAndSendTransaction = async (event: SignAndSendTransactionEvent, originHeader: string) => {
    return await this.sendEvent(
      {
        appId: this.appId,
        event: {
          type: 'SignAndSendTransaction',
          ...event
        }
      },
      originHeader
    )
  }

  signAndSendTransactionResolve = async (
    event: SignAndSendTransactionResolveEvent,
    originHeader: string
  ) => {
    return await this.sendEvent(
      {
        appId: this.appId,
        event: {
          type: 'SignAndSendTransactionResolve',
          ...event
        }
      },
      originHeader
    )
  }

  changeNetwork = async (event: ChangeNetworkEvent, originHeader: string) => {
    return await this.sendEvent(
      {
        appId: this.appId,
        event: {
          type: 'ChangeNetwork',
          ...event
        }
      },
      originHeader
    )
  }

  changeNetworkResolve = async (event: ChangeNetworkResolveEvent, originHeader: string) => {
    return await this.sendEvent(
      {
        appId: this.appId,
        event: {
          type: 'ChangeNetworkResolve',
          ...event
        }
      },
      originHeader
    )
  }

  changeWallet = async (event: ChangeWalletEvent, originHeader: string) => {
    return await this.sendEvent(
      {
        appId: this.appId,
        event: {
          type: 'ChangeWallet',
          ...event
        }
      },
      originHeader
    )
  }

  changeWalletResolve = async (event: ChangeWalletResolveEvent, originHeader: string) => {
    return await this.sendEvent(
      {
        appId: this.appId,
        event: {
          type: 'ChangeWalletResolve',
          ...event
        }
      },
      originHeader
    )
  }
}
