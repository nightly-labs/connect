// import LocalStorage from 'isomorphic-localstorage'
import { AppToServer } from '@bindings/AppToServer'
import { InitializeRequest } from '@bindings/InitializeRequest'
import { InitializeResponse } from '@bindings/InitializeResponse'
import { Network } from '@bindings/Network'
import { ServerToApp } from '@bindings/ServerToApp'
import { SignTransactionsResponse } from '@bindings/SignTransactionsResponse'
import { UserConnectedEvent } from '@bindings/UserConnectedEvent'
import { Version } from '@bindings/Version'
import WebSocket from 'isomorphic-ws'
import { TypedEmitter } from 'tiny-typed-emitter'
import { getRandomId } from './utils'
import { TransactionToSign } from '@bindings/TransactionToSign'
import { MessageToSign } from '@bindings/MessageToSign'
import { SignMessagesResponse } from '@bindings/SignMessagesResponse'
import { UserDisconnectedEvent } from '@bindings/UserDisconnectedEvent'
// const localStorage = LocalStorage('./.localstorage')
// const sessionIdKey = 'nightly-id-solana'

export interface AppBaseInitialize {
  appName: string
  network: Network
  version: Version
  wsUrl?: string
  timeout?: number
  appIcon?: string
  appDescription?: string
  additionalInfo?: string
  persistentSessionId?: string
  persistent?: boolean
}
interface BaseEvents {
  userConnected: (e: UserConnectedEvent) => void
  userDisconnected: (e: UserDisconnectedEvent) => void
}
export class BaseApp extends TypedEmitter<BaseEvents> {
  ws: WebSocket
  events: { [key: string]: (data: any) => void | undefined } = {}
  sessionId = ''
  timeout: number

  private constructor(ws: WebSocket, timeout: number) {
    super()
    this.ws = ws
    this.timeout = timeout
  }

  public static build = async (baseInitialize: AppBaseInitialize): Promise<BaseApp> => {
    return new Promise((resolve, reject) => {
      // const persistent =
      //   typeof appMetadata.persistent === 'undefined' ? true : appMetadata.persistent
      const ws = baseInitialize.wsUrl
        ? new WebSocket(baseInitialize.wsUrl + '/app')
        : new WebSocket('wss://relay.nightly.app/app')
      const baseApp = new BaseApp(ws, baseInitialize.timeout ?? 40000)
      // connection.events[ServerMessageTypes.UserConnected] = (data: UserConnectedMessage) => {
      //   const pk = new PublicKey(Buffer.from(data.publicKey, 'hex'))
      //   onUserConnect({ publicKey: pk })
      // }
      baseApp.ws.onopen = () => {
        baseApp.ws.onmessage = ({ data }: { data: any }) => {
          const response = JSON.parse(data) as ServerToApp
          switch (response.type) {
            case 'InitializeResponse':
            case 'SignTransactionsResponse':
            case 'SignMessagesResponse':
            case 'ErrorMessage': {
              baseApp.events[response.responseId](response)
              break
            }
            case 'UserConnectedEvent': {
              baseApp.emit('userConnected', response)
            }
          }
        }
        const reponseId = getRandomId()
        // Initialize the connection
        const initializeRequest: { type: 'InitializeRequest' } & InitializeRequest = {
          additionalInfo: baseInitialize.additionalInfo,
          appName: baseInitialize.appName,
          appDescription: baseInitialize.appDescription,
          appIcon: baseInitialize.appIcon,
          network: baseInitialize.network,
          persistentSessionId: baseInitialize.persistentSessionId,
          persistent: baseInitialize.persistent ?? true, // by default, persistent
          responseId: reponseId,
          version: baseInitialize.version,
          type: 'InitializeRequest'
        }
        // Set up the timeout
        const timer = setTimeout(() => {
          reject(new Error(`Connection timed out after ${baseApp.timeout} ms`))
        }, baseApp.timeout)

        baseApp.events[initializeRequest.responseId] = (response: InitializeResponse) => {
          clearTimeout(timer)
          // TODO: Handle error
          baseApp.sessionId = response.sessionId
          resolve(baseApp)
        }
        baseApp.ws.send(JSON.stringify(initializeRequest))
      }
    })
  }
  send = async (message: AppToServer): Promise<ServerToApp> => {
    return new Promise((resolve, reject) => {
      const request = JSON.stringify(message)
      // Set up the timeout
      const timer = setTimeout(() => {
        reject(new Error(`Request timed out after ${this.timeout} ms`))
      }, this.timeout)
      this.events[message.responseId] = (response: ServerToApp) => {
        clearTimeout(timer)
        resolve(response)
      }
      this.ws.send(request)
    })
  }
  signTransactions = async (transactions: Array<TransactionToSign>, metadata?: string) => {
    const response = (await this.send({
      responseId: getRandomId(),
      transactions,
      metadata,
      type: 'SignTransactionsRequest'
    })) as SignTransactionsResponse
    return response
  }
  signMessages = async (messages: Array<MessageToSign>, metadata?: string) => {
    const response = (await this.send({
      responseId: getRandomId(),
      messages,
      metadata,
      type: 'SignMessagesRequest'
    })) as SignMessagesResponse
    return response
  }
}
