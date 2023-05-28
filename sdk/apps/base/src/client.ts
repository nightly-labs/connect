// import LocalStorage from 'isomorphic-localstorage'
import { ClientToServer } from '@bindings/ClientToServer'
import { ServerToClient } from '@bindings/ServerToClient'
import { Version } from '@bindings/Version'
import WebSocket from 'isomorphic-ws'
import { TypedEmitter } from 'tiny-typed-emitter'
import { getRandomId } from './utils'
import { SignTransactionsEvent } from '@bindings/SignTransactionsEvent'
import { GetInfoRequest } from '@bindings/GetInfoRequest'
import { ConnectRequest } from '@bindings/ConnectRequest'
import { GetInfoResponse } from '@bindings/GetInfoResponse'
import { ConnectResponse } from '@bindings/ConnectResponse'
import { GetPendingRequestsResponse } from '@bindings/GetPendingRequestsResponse'
import { SignedTransaction } from '@bindings/SignedTransaction'
import { SignTransactionsEventReply } from '@bindings/SignTransactionsEventReply'

export interface ClientBaseInitialize {
  version: Version
  wsUrl?: string
  timeout?: number
  persistent: boolean
}
interface BaseEvents {
  signTransaction: (e: SignTransactionsEvent) => void
}
export class BaseClient extends TypedEmitter<BaseEvents> {
  ws: WebSocket
  events: { [key: string]: (data: any) => void | undefined } = {}
  sessionId = ''
  timeout: number

  private constructor(ws: WebSocket, timeout: number) {
    super()
    this.ws = ws
    this.timeout = timeout
  }

  public static build = async (baseInitialize: ClientBaseInitialize): Promise<BaseClient> => {
    return new Promise((resolve, reject) => {
      const ws = baseInitialize.wsUrl
        ? new WebSocket(baseInitialize.wsUrl + '/client')
        : new WebSocket('wss://relay.nightly.app/client')
      console.log(baseInitialize.wsUrl + '/client')
      const baseClient = new BaseClient(ws, baseInitialize.timeout ?? 40000)

      baseClient.ws.onopen = () => {
        baseClient.ws.onmessage = ({ data }: { data: any }) => {
          console.log('data', data)
          const response = JSON.parse(data) as ServerToClient
          switch (response.type) {
            case 'GetInfoResponse':
            case 'ConnectResponse':
            case 'GetPendingRequestsResponse':
            case 'ErrorMessage': {
              baseClient.events[response.responseId](response)
              break
            }
            case 'SignTransactionsEvent': {
              baseClient.emit('signTransaction', response)
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
      this.events[message.responseId] = (response: ServerToClient) => {
        clearTimeout(timer)
        resolve(response)
      }
      this.ws.send(request)
    })
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
}
export type Connect = Omit<ConnectRequest, 'type' | 'responseId'>
