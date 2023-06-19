import { AppDisconnectedEvent } from '../../../bindings/AppDisconnectedEvent'
import { VersionedTransaction } from '@solana/web3.js'
import {
  BaseClient,
  ClientBaseInitialize,
  Connect,
  SignMessagesEvent
} from '@nightlylabs/nightly-connect-base'
import { SOLANA_NETWORK } from './utils'
import { GetInfoResponse } from '../../../bindings/GetInfoResponse'
import { GetPendingRequestsResponse } from '../../../bindings/GetPendingRequestsResponse'
export interface SignSolanaTransactionEvent {
  requestId: string
  transactions: Array<VersionedTransaction>
}
export type SignSolanaMessageEvent = SignMessagesEvent
export interface ClientSolanaEvents {
  signTransactions: (e: SignSolanaTransactionEvent) => void
  signMessages: (e: SignSolanaMessageEvent) => void
  appDisconnected: (e: AppDisconnectedEvent) => void
}
export class ClientSolana {
  baseClient: BaseClient
  sessionId: string | undefined = undefined

  listeners: Map<string, Array<(data: any) => void>> = new Map()
  on = (eventName: string, cb: (data: any) => void) => {
    if (typeof this.listeners.get(eventName) === 'undefined') {
      this.listeners.set(eventName, [])
    }

    this.listeners.get(eventName)!.push(cb)
  }

  emit = (eventName: string, data: any) => {
    this.listeners.get(eventName)?.forEach((cb) => {
      cb(data)
    })
  }

  removeAllListeners = () => {
    this.listeners = new Map()
  }

  public constructor(baseClient: BaseClient) {
    baseClient.on('signTransactions', (e) => {
      const event: SignSolanaTransactionEvent = {
        requestId: e.responseId,
        transactions: e.transactions.map((tx) => {
          return VersionedTransaction.deserialize(Buffer.from(tx.transaction, 'hex'))
        })
      }
      this.emit('signTransactions', event)
    })
    baseClient.on('signMessages', (e) => {
      this.emit('signMessages', e)
    })
    baseClient.on('appDisconnected', (e) => {
      this.emit('appDisconnected', e)
    })
    this.baseClient = baseClient
  }
  public static build = async (
    sessionId: string,
    initData: ClientBaseInitialize
  ): Promise<{
    client: ClientSolana
    data: GetInfoResponse
  }> => {
    const baseClient = await BaseClient.build(initData)
    const data = await baseClient.getInfo(sessionId)
    const client = new ClientSolana(baseClient)
    return { client, data }
  }
  public static create = async (initData: ClientBaseInitialize) => {
    const baseClient = await BaseClient.build(initData)
    const client = new ClientSolana(baseClient)
    return client
  }
  public getInfo = async (sessionId: string): Promise<GetInfoResponse> => {
    const response = await this.baseClient.getInfo(sessionId)
    return response
  }
  public connect = async (connect: Connect) => {
    await this.baseClient.connect(connect)
    this.sessionId = connect.sessionId
  }
  public getPendingRequests = async (sessionId?: string): Promise<GetPendingRequestsResponse> => {
    const sessionIdToUse = sessionId || this.sessionId
    //Assert session id is defined
    if (sessionIdToUse === undefined) {
      throw new Error('Session id is undefined')
    }
    return await this.baseClient.getPendingRequests(sessionIdToUse)
  }

  public resolveSignTransaction = async ({
    requestId,
    signedTransactions,
    sessionId
  }: ResolveSignSolanaTransactions) => {
    const serializedTxs = signedTransactions
      .map((tx) => Buffer.from(tx.serialize()).toString('hex'))
      .map((tx) => {
        return { network: SOLANA_NETWORK, transaction: tx }
      })
    const sessionIdToUse = sessionId || this.sessionId
    //Assert session id is defined
    if (sessionIdToUse === undefined) {
      throw new Error('Session id is undefined')
    }
    await this.baseClient.resolveSignTransactions({
      requestId: requestId,
      signedTransactions: serializedTxs,
      sessionId: sessionIdToUse
    })
  }
  public resolveSignMessage = async ({
    requestId,
    signature,
    sessionId
  }: ResolveSignSolanaMessage) => {
    const sessionIdToUse = sessionId || this.sessionId
    //Assert session id is defined
    if (sessionIdToUse === undefined) {
      throw new Error('Session id is undefined')
    }
    await this.baseClient.resolveSignMessages({
      requestId: requestId,
      sessionId: sessionIdToUse,
      signedMessages: [{ message: Buffer.from(signature).toString('hex') }]
    })
  }
  public rejectRequest = async ({ requestId, reason, sessionId }: RejectRequest) => {
    const sessionIdToUse = sessionId || this.sessionId
    //Assert session id is defined
    if (sessionIdToUse === undefined) {
      throw new Error('Session id is undefined')
    }
    await this.baseClient.reject({ requestId: requestId, reason, sessionId: sessionIdToUse })
  }
}

export interface RejectRequest {
  requestId: string
  reason?: string
  sessionId?: string
}
export interface ResolveSignSolanaTransactions {
  requestId: string
  signedTransactions: VersionedTransaction[]
  sessionId?: string
}
export interface ResolveSignSolanaMessage {
  requestId: string
  signature: Uint8Array
  sessionId?: string
}
