// import LocalStorage from 'isomorphic-localstorage'
import { ClientToServer } from '../../../bindings/ClientToServer'
import { ServerToClient } from '../../../bindings/ServerToClient'
import WebSocket from 'isomorphic-ws'
import { getRandomId } from './utils'
import { GetInfoRequest } from '../../../bindings/GetInfoRequest'
import { ConnectRequest } from '../../../bindings/ConnectRequest'
import { GetInfoResponse } from '../../../bindings/GetInfoResponse'
import { GetPendingRequestsResponse } from '../../../bindings/GetPendingRequestsResponse'
import { AppDisconnectedEvent } from '../../../bindings/AppDisconnectedEvent'
import { EventEmitter } from 'eventemitter3'
import { MessageToSign, RequestContent, RequestInternal, TransactionToSign } from './content'
import {
  ResponseContent,
  ResponseContentType,
  SignedMessage,
  SignedTransaction
} from './responseContent'
import { ClientInitializeRequest } from '../../../bindings/ClientInitializeRequest'
import { GetSessionsRequest } from '../../../bindings/GetSessionsRequest'
import { GetSessionsResponse } from '../../../bindings/GetSessionsResponse'
import { DropSessionsRequest } from '../../../bindings/DropSessionsRequest'
import { DropSessionsResponse } from '../../../bindings/DropSessionsResponse'
import { ClientBaseInitialize } from './initializeTypes'

export interface SignTransactionsEvent {
  responseId: string
  sessionId: string
  transactions: TransactionToSign[]
}
export interface SignMessagesEvent {
  responseId: string
  sessionId: string
  messages: MessageToSign[]
}
export interface CustomEvent {
  responseId: string
  sessionId: string
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  content?: any
}
interface BaseEvents {
  signTransactions: (e: SignTransactionsEvent) => void
  signMessages: (e: SignMessagesEvent) => void
  customEvent: (e: CustomEvent) => void
  appDisconnected: (e: AppDisconnectedEvent) => void
}
export class BaseClient extends EventEmitter<BaseEvents> {
  url: string
  ws: WebSocket
  events: { [key: string]: { resolve: (data: any) => void; reject: (data: any) => void } } = {}
  timeout: number
  clientId: string
  private constructor(url: string, ws: WebSocket, timeout: number, clientId: string) {
    super()
    this.url = url
    this.ws = ws
    this.timeout = timeout
    this.clientId = clientId
  }

  public static build = async (baseInitialize: ClientBaseInitialize): Promise<BaseClient> => {
    return new Promise((resolve, reject) => {
      const url = baseInitialize.url ?? 'https://nc2.nightly.app'
      // get domain from url
      const path = url.replace('https://', 'wss://').replace('http://', 'ws://')
      const ws = new WebSocket(path + '/client')
      const clientId = baseInitialize.clientId ?? getRandomId()
      const baseClient = new BaseClient(url, ws, baseInitialize.timeout ?? 40000, clientId)
      baseClient.ws.onopen = () => {
        baseClient.ws.onmessage = ({ data }: { data: any }) => {
          const response = JSON.parse(data) as ServerToClient
          switch (response.type) {
            case 'GetInfoResponse':
            case 'ConnectResponse':
            case 'GetPendingRequestsResponse':
            case 'ClientInitializeResponse':
            case 'DropSessionsResponse':
            case 'GetSessionsResponse':
            case 'AckMessage': {
              baseClient.events[response.responseId].resolve(response)
              break
            }
            case 'ErrorMessage': {
              baseClient.events[response.responseId].reject(response)
              break
            }
            case 'NewPayloadEvent': {
              const payload = JSON.parse(response.payload) as RequestInternal
              switch (payload.type) {
                case 'SignTransactions': {
                  baseClient.emit('signTransactions', {
                    responseId: response.requestId,
                    sessionId: response.sessionId,
                    transactions: payload.transactions
                  })
                  break
                }
                case 'SignMessages': {
                  baseClient.emit('signMessages', {
                    responseId: response.requestId,
                    sessionId: response.sessionId,
                    messages: payload.messages
                  })
                  break
                }
                case 'Custom': {
                  baseClient.emit('customEvent', {
                    responseId: response.requestId,
                    sessionId: response.sessionId,
                    content: payload.content
                  })
                  break
                }
              }

              break
            }

            case 'AppDisconnectedEvent': {
              baseClient.emit('appDisconnected', response)
              break
            }
          }
        }
        const reponseId = getRandomId()
        // Initialize the connection
        const initializeRequest: { type: 'ClientInitializeRequest' } & ClientInitializeRequest = {
          clientId: clientId,
          responseId: reponseId,
          type: 'ClientInitializeRequest'
        }
        // Set up the timeout
        const timer = setTimeout(() => {
          reject(new Error(`Connection timed out after ${baseClient.timeout} ms`))
        }, baseClient.timeout)
        baseClient.events[initializeRequest.responseId] = {
          reject: reject,
          resolve: () => {
            clearTimeout(timer)
            resolve(baseClient)
          }
        }
        baseClient.ws.send(JSON.stringify(initializeRequest))
      }
    })
  }
  send = async (message: ClientToServer): Promise<ServerToClient> => {
    return new Promise((resolve, reject) => {
      const request = JSON.stringify(message)
      // Set up the timeout
      const timer = setTimeout(() => {
        reject(new Error(`Request timed out after ${this.timeout} ms`))
      }, this.timeout)
      this.events[message.responseId] = {
        reject: reject,
        resolve: (response: ServerToClient) => {
          clearTimeout(timer)
          resolve(response)
        }
      }
      this.ws.send(request)
    })
  }
  getSessions = async () => {
    const request: GetSessionsRequest = {
      responseId: getRandomId()
    }
    const response = (await this.send({
      ...request,
      type: 'GetSessionsRequest'
    })) as GetSessionsResponse
    return response.sessions
  }
  dropSessions = async (sessions: string[]) => {
    const request: DropSessionsRequest = {
      responseId: getRandomId(),
      sessions
    }
    const response = (await this.send({
      ...request,
      type: 'DropSessionsRequest'
    })) as DropSessionsResponse
    return response.droppedSessions
  }
  getInfo = async (sessionId: string) => {
    const request: GetInfoRequest = {
      responseId: getRandomId(),
      sessionId
    }
    const response = (await this.send({
      ...request,
      type: 'GetInfoRequest'
    })) as GetInfoResponse
    return response
  }
  connect = async (request: Connect) => {
    await this.send({
      ...request,
      clientId: this.clientId,
      responseId: getRandomId(),
      type: 'ConnectRequest'
    })
  }
  getPendingRequests = async (sessionId: string) => {
    const response = (await this.send({
      responseId: getRandomId(),
      sessionId: sessionId,
      type: 'GetPendingRequestsRequest'
    })) as GetPendingRequestsResponse
    const requests = response.requests.map((request) => {
      return {
        requestId: request.requestId,
        content: JSON.parse(request.content)
      } as unknown as RequestContent
    })
    return requests
  }
  resolveRequest = async (resolve: ResolveRequest) => {
    await this.send({
      responseId: getRandomId(),
      ...resolve,
      content: JSON.stringify(resolve.content),
      type: 'NewPayloadEventReply'
    })
  }
  resolveSignTransactions = async ({
    requestId,
    sessionId,
    signedTransactions
  }: ResolveSignTransactions) => {
    await this.resolveRequest({
      requestId,
      content: {
        type: ResponseContentType.SignTransactions,
        transactions: signedTransactions
      },
      sessionId
    })
  }
  resolveSignMessages = async ({ requestId, sessionId, signedMessages }: ResolveSignMessages) => {
    await this.resolveRequest({
      requestId,
      content: {
        type: ResponseContentType.SignMessages,
        messages: signedMessages
      },
      sessionId
    })
  }
  resolveCustom = async ({ requestId, sessionId, content }: ResolveCustom) => {
    await this.resolveRequest({
      requestId,
      content: {
        type: ResponseContentType.Custom,
        content: content
      },
      sessionId
    })
  }
  reject = async ({ requestId, sessionId, reason }: Reject) => {
    await this.resolveRequest({
      requestId,
      content: {
        type: ResponseContentType.Reject,
        reason: reason
      },
      sessionId
    })
  }
}
export interface ResolveSignTransactions {
  requestId: string
  sessionId: string
  signedTransactions: SignedTransaction[]
}
export interface ResolveSignMessages {
  requestId: string
  sessionId: string
  signedMessages: SignedMessage[]
}
export interface ResolveCustom {
  requestId: string
  sessionId: string
  content?: string
}
export interface Reject {
  requestId: string
  sessionId: string
  reason?: string
}
export interface ResolveRequest {
  requestId: string
  sessionId: string
  content: ResponseContent
}

export type Connect = Omit<ConnectRequest, 'type' | 'responseId' | 'clientId'>
