// import LocalStorage from 'isomorphic-localstorage'
import { ClientToServer } from '@bindings/ClientToServer'
import { ServerToClient } from '@bindings/ServerToClient'
import WebSocket from 'isomorphic-ws'
import { getRandomId } from './utils'
import { GetInfoRequest } from '@bindings/GetInfoRequest'
import { ConnectRequest } from '@bindings/ConnectRequest'
import { GetInfoResponse } from '@bindings/GetInfoResponse'
import { GetPendingRequestsResponse } from '@bindings/GetPendingRequestsResponse'
import { SignTransactionsEventReply } from '@bindings/SignTransactionsEventReply'
import { SignMessagesEventReply } from '@bindings/SignMessagesEventReply'
import { AppDisconnectedEvent } from '@bindings/AppDisconnectedEvent'
import { Reject } from '@bindings/Reject'
import { TypedEmitter } from 'tiny-typed-emitter'
import { Notification } from '@bindings/Notification'
import { SignMessagesRequest } from '@bindings/SignMessagesRequest'
import { SignTransactionsRequest } from '@bindings/SignTransactionsRequest'

export interface ClientBaseInitialize {
  wsUrl?: string
  timeout?: number
  notification?: Notification
}
interface BaseEvents {
  signTransactions: (e: SignTransactionsRequest) => void
  signMessages: (e: SignMessagesRequest) => void
  appDisconnected: (e: AppDisconnectedEvent) => void
}
export class BaseClient extends TypedEmitter<BaseEvents> {
  ws: WebSocket
  events: { [key: string]: { resolve: (data: any) => void; reject: (data: any) => void } } = {}
  sessionId = ''
  timeout: number

  private constructor(ws: WebSocket, timeout: number) {
    super()
    this.ws = ws
    this.timeout = timeout
  }

  public static build = async (baseInitialize: ClientBaseInitialize): Promise<BaseClient> => {
    return new Promise((resolve, _) => {
      const ws = baseInitialize.wsUrl
        ? new WebSocket(baseInitialize.wsUrl + '/client')
        : new WebSocket('wss://relay.nightly.app/client')
      const baseClient = new BaseClient(ws, baseInitialize.timeout ?? 40000)

      baseClient.ws.onopen = () => {
        baseClient.ws.onmessage = ({ data }: { data: any }) => {
          const response = JSON.parse(data) as ServerToClient
          switch (response.type) {
            case 'GetInfoResponse':
            case 'ConnectResponse':
            case 'GetPendingRequestsResponse':
            case 'AckMessage': {
              baseClient.events[response.responseId].resolve(response)
              break
            }
            case 'ErrorMessage': {
              baseClient.events[response.responseId].reject(response)
              break
            }
            case 'SignTransactionsEvent': {
              baseClient.emit('signTransactions', response.request)
              break
            }
            case 'SignMessagesEvent': {
              baseClient.emit('signMessages', response.request)
              break
            }
            case 'AppDisconnectedEvent': {
              baseClient.emit('appDisconnected', response)
              break
            }
          }
        }
        resolve(baseClient)
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
  getInfo = async (sessionId: string) => {
    console.log('getInfo2')

    const request: GetInfoRequest = {
      responseId: getRandomId(),
      sessionId
    }
    console.log(request)
    const response = (await this.send({
      ...request,
      type: 'GetInfoRequest'
    })) as GetInfoResponse
    return response
  }
  connect = async (request: Connect) => {
    await this.send({
      ...request,
      responseId: getRandomId(),
      type: 'ConnectRequest'
    })
  }
  getPendingRequests = async () => {
    const response = (await this.send({
      responseId: getRandomId(),
      type: 'GetPendingRequestsRequest'
    })) as GetPendingRequestsResponse
    return response
  }
  resolveSignTransactions = async (resolve: Omit<SignTransactionsEventReply, 'responseId'>) => {
    await this.send({
      responseId: getRandomId(),
      ...resolve,
      type: 'SignTransactionsEventReply'
    })
  }
  resolveSignMessages = async (resolve: Omit<SignMessagesEventReply, 'responseId'>) => {
    await this.send({
      responseId: getRandomId(),
      ...resolve,
      type: 'SignMessagesEventReply'
    })
  }
  reject = async (reject: Omit<Reject, 'responseId'>) => {
    await this.send({
      responseId: getRandomId(),
      ...reject,
      type: 'Reject'
    })
  }
}
export type Connect = Omit<ConnectRequest, 'type' | 'responseId'>
