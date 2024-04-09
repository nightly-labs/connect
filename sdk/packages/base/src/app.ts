import { AppToServer } from '../../../bindings/AppToServer'
import { InitializeRequest } from '../../../bindings/InitializeRequest'
import { InitializeResponse } from '../../../bindings/InitializeResponse'
import { ServerToApp } from '../../../bindings/ServerToApp'
import { UserConnectedEvent } from '../../../bindings/UserConnectedEvent'
import WebSocket from 'isomorphic-ws'
import {
  getLocalStorage,
  getRandomId,
  getSessionIdLocalStorageKey,
  getWalletsMetadata
} from './utils'
import { UserDisconnectedEvent } from '../../../bindings/UserDisconnectedEvent'
import { ContentType, MessageToSign, RequestInternal, TransactionToSign } from './content'
import { ResponsePayload } from '../../../bindings/ResponsePayload'
import { WalletMetadata } from '../../../bindings/WalletMetadata'
import {
  CustomResponseContent,
  ResponseContent,
  ResponseContentType,
  SignMessagesResponseContent,
  SignTransactionsResponseContent,
  SignedMessage,
  SignedTransaction
} from './responseContent'
import { triggerDeeplink } from './deeplinks'
import { EventEmitter } from 'eventemitter3'
import { AppBaseInitialize } from './initializeTypes'

interface BaseEvents {
  userConnected: (e: UserConnectedEvent) => void
  userDisconnected: (e: UserDisconnectedEvent) => void
  serverDisconnected: () => void
}

export interface DeeplinkConnect {
  url: string
  walletName: string
}
export class BaseApp extends EventEmitter<BaseEvents> {
  url: string
  ws: WebSocket
  events: Map<string, { resolve: (data: any) => void; reject: (data: any) => void }> = new Map()
  sessionId = ''
  timeout: number
  deeplink: DeeplinkConnect | undefined
  connectedPublicKeys: string[] = []
  hasBeenRestored = false
  clientMetadata: string | undefined
  initializeData: AppBaseInitialize
  // TODO add info about the app
  private constructor(initializeData: AppBaseInitialize) {
    super()
    const url = initializeData.url ?? 'https://nc2.nightly.app'
    // get domain from url
    const path = url.replace('https://', 'wss://').replace('http://', 'ws://')
    const ws = new WebSocket(path + '/app')
    this.initializeData = initializeData
    this.url = url
    this.ws = ws
    this.timeout = initializeData.timeout ?? 40000
  }
  public static getWalletsMetadata = async (
    url?: string,
    network?: string
  ): Promise<WalletMetadata[]> => {
    return getWalletsMetadata(url, network)
  }
  public static build = async (baseInitialize: AppBaseInitialize): Promise<BaseApp> => {
    return new Promise((resolve, reject) => {
      const localStorage = getLocalStorage()
      const persistent = baseInitialize.persistent ?? true
      const persistentSessionId = persistent
        ? localStorage.getItem(getSessionIdLocalStorageKey(baseInitialize.network)) ?? undefined
        : undefined

      const baseApp = new BaseApp(baseInitialize)
      baseApp.ws.onclose = () => {
        baseApp.emit('serverDisconnected')
      }
      baseApp.ws.onopen = () => {
        baseApp.ws.onmessage = ({ data }: { data: any }) => {
          try {
            const response = JSON.parse(data) as ServerToApp
            switch (response.type) {
              case 'InitializeResponse':
              case 'AckMessage': {
                baseApp.events.get(response.responseId)?.resolve(response)
                break
              }
              case 'ErrorMessage': {
                baseApp.events.get(response.responseId)?.reject(response)
                break
              }
              case 'ResponsePayload': {
                baseApp.events.get(response.responseId)?.resolve(response)
                break
              }
              case 'UserConnectedEvent': {
                baseApp.connectedPublicKeys = response.publicKeys
                baseApp.emit('userConnected', response)
                break
              }
              case 'AlreadyConnected': {
                reject(new Error('Already connected'))
                break
              }
            }
          } catch (error) {
            console.warn('Error parsing message', error)
          }
        }
        baseApp.ws.onclose = () => {
          baseApp.emit('serverDisconnected')
        }
        const responseId = getRandomId()
        // Initialize the connection
        const initializeRequest: { type: 'InitializeRequest' } & InitializeRequest = {
          appMetadata: baseInitialize.appMetadata,
          network: baseInitialize.network,
          persistentSessionId: persistentSessionId,
          persistent: persistent,
          responseId: responseId,
          version: '#TODO version 0.0.0',
          type: 'InitializeRequest'
        }
        // Set up the timeout
        const timer = setTimeout(() => {
          reject(new Error(`Connection timed out after ${baseApp.timeout} ms`))
        }, baseApp.timeout)
        baseApp.events.set(initializeRequest.responseId, {
          reject: reject,
          resolve: (response: InitializeResponse) => {
            clearTimeout(timer)
            // TODO: Handle error
            baseApp.sessionId = response.sessionId
            if (!response.createdNew) {
              baseApp.hasBeenRestored = true
              baseApp.connectedPublicKeys = response.publicKeys
              baseApp.clientMetadata = response.metadata
            }
            // Save the session id
            if (persistent)
              localStorage.setItem(
                getSessionIdLocalStorageKey(baseInitialize.network),
                response.sessionId
              )
            resolve(baseApp)
          }
        })
        baseApp.ws.send(JSON.stringify(initializeRequest))
      }
    })
  }

  connectDeeplink = (deeplinkData: DeeplinkConnect) => {
    this.deeplink = deeplinkData
  }

  send = async (message: AppToServer): Promise<ServerToApp> => {
    return new Promise((resolve, reject) => {
      const request = JSON.stringify(message)
      // Set up the timeout
      const timer = setTimeout(() => {
        reject(new Error(`Request timed out after ${this.timeout} ms`))
      }, this.timeout)
      this.events.set(message.responseId, {
        reject: reject,
        resolve: (response: ServerToApp) => {
          clearTimeout(timer)
          resolve(response)
        }
      })
      this.ws.send(request)
      // If deeplink is set, send the deeplink
      if (this.deeplink && message.type === 'RequestPayload') {
        triggerDeeplink({
          path: this.deeplink.url,
          deeplinkParams: {
            relay: 'https://nc2.nightly.app',
            sessionId: this.sessionId,
            requestId: message.responseId
          }
        })
      }
    })
  }
  sendRequest = async (content: RequestInternal) => {
    const response = (await this.send({
      responseId: getRandomId(),
      content: JSON.stringify(content),
      type: 'RequestPayload'
    })) as ResponsePayload
    const payload = JSON.parse(response.content) as ResponseContent
    if (payload.type === ResponseContentType.Reject) {
      throw new Error(payload.reason)
    }
    switch (content.type) {
      case 'SignTransactions': {
        return payload as SignTransactionsResponseContent
      }
      case 'SignMessages': {
        return payload as SignMessagesResponseContent
      }
      case 'Custom': {
        return payload as CustomResponseContent
      }
    }
    throw new Error('Unknown response type')
  }
  signTransactions = async (transactions: TransactionToSign[]): Promise<SignedTransaction[]> => {
    const response = (await this.sendRequest({
      type: ContentType.SignTransactions,
      transactions: transactions
    })) as SignTransactionsResponseContent
    return response.transactions
  }
  signMessages = async (messages: MessageToSign[]): Promise<SignedMessage[]> => {
    const response = (await this.sendRequest({
      type: ContentType.SignMessages,
      messages: messages
    })) as SignMessagesResponseContent
    return response.messages
  }
  customRequest = async (content: string): Promise<CustomResponseContent> => {
    const response = (await this.sendRequest({
      type: ContentType.Custom,
      content: content
    })) as CustomResponseContent
    return response
  }
}
