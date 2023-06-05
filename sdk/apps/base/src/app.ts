// import LocalStorage from 'isomorphic-localstorage'
import { AppToServer } from '@bindings/AppToServer'
import { InitializeRequest } from '@bindings/InitializeRequest'
import { InitializeResponse } from '@bindings/InitializeResponse'
import { Network } from '@bindings/Network'
import { ServerToApp } from '@bindings/ServerToApp'
import { UserConnectedEvent } from '@bindings/UserConnectedEvent'
import { Version } from '@bindings/Version'
import WebSocket from 'isomorphic-ws'
import LocalStorage from 'isomorphic-localstorage'
import { getRandomId } from './utils'
import { UserDisconnectedEvent } from '@bindings/UserDisconnectedEvent'
import { TypedEmitter } from 'tiny-typed-emitter'
import { AppMetadata } from '@bindings/AppMetadata'
import { ContentType, MessageToSign, RequestContent, TransactionToSign } from './content'
import { ResponsePayload } from '@bindings/ResponsePayload'
import {
  CustomResponseContent,
  ResponseContent,
  ResponseContentType,
  SignMessagesResponseContent,
  SignTransactionsResponseContent,
  SignedMessage,
  SignedTransaction
} from './responseContent'

const localStorage = LocalStorage('./.nightly-connect-session')
export interface AppBaseInitialize {
  appMetadata: AppMetadata
  network: Network
  wsUrl?: string
  timeout?: number
  persistentSessionId?: string
  persistent?: boolean
}
interface BaseEvents {
  userConnected: (e: UserConnectedEvent) => void
  userDisconnected: (e: UserDisconnectedEvent) => void
  serverDisconnected: () => void
}
export class BaseApp extends TypedEmitter<BaseEvents> {
  ws: WebSocket
  events: { [key: string]: { resolve: (data: any) => void; reject: (data: any) => void } } = {}
  sessionId = ''
  timeout: number
  // TODO add info about the app
  private constructor(ws: WebSocket, timeout: number) {
    super()
    this.ws = ws
    this.timeout = timeout
  }

  public static build = async (baseInitialize: AppBaseInitialize): Promise<BaseApp> => {
    return new Promise((resolve, reject) => {
      const persistent = baseInitialize.persistent ?? true
      const persistentSessionId = persistent
        ? localStorage.getItem(baseInitialize.appMetadata.name) ?? undefined
        : undefined
      const ws = baseInitialize.wsUrl
        ? new WebSocket(baseInitialize.wsUrl + '/app')
        : new WebSocket('wss://relay.nightly.app/app')
      const baseApp = new BaseApp(ws, baseInitialize.timeout ?? 40000)
      baseApp.ws.onclose = () => {
        baseApp.emit('serverDisconnected')
      }
      baseApp.ws.onopen = () => {
        baseApp.ws.onmessage = ({ data }: { data: any }) => {
          const response = JSON.parse(data) as ServerToApp
          switch (response.type) {
            case 'InitializeResponse':
            case 'AckMessage': {
              baseApp.events[response.responseId].resolve(response)
              break
            }
            case 'ErrorMessage': {
              baseApp.events[response.responseId].reject(response)
              break
            }
            case 'ResponsePayload': {
              baseApp.events[response.responseId].resolve(response)
              break
            }
            case 'UserConnectedEvent': {
              baseApp.emit('userConnected', response)
            }
          }
        }
        baseApp.ws.onclose = () => {
          baseApp.emit('serverDisconnected')
        }
        const reponseId = getRandomId()
        // Initialize the connection
        const initializeRequest: { type: 'InitializeRequest' } & InitializeRequest = {
          appMetadata: baseInitialize.appMetadata,
          network: baseInitialize.network,
          persistentSessionId: persistentSessionId,
          persistent: persistent,
          responseId: reponseId,
          version: '#TODO version 0.0.0',
          type: 'InitializeRequest'
        }
        // Set up the timeout
        const timer = setTimeout(() => {
          reject(new Error(`Connection timed out after ${baseApp.timeout} ms`))
        }, baseApp.timeout)
        baseApp.events[initializeRequest.responseId] = {
          reject: reject,
          resolve: (response: InitializeResponse) => {
            clearTimeout(timer)
            // TODO: Handle error
            baseApp.sessionId = response.sessionId
            // Save the session id
            if (persistent)
              localStorage.setItem(initializeRequest.appMetadata.name, response.sessionId)
            resolve(baseApp)
          }
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
      this.events[message.responseId] = {
        reject: reject,
        resolve: (response: ServerToApp) => {
          clearTimeout(timer)
          resolve(response)
        }
      }
      this.ws.send(request)
    })
  }
  sendRequest = async (content: RequestContent) => {
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
