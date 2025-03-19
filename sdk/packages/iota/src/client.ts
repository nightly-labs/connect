import type { IotaTransactionBlockResponse } from '@iota/iota-sdk/client'
import {
  BaseClient,
  ClientBaseInitialize,
  Connect,
  SignMessagesEvent,
  TransactionToSign
} from '@nightlylabs/nightly-connect-base'
import { EventEmitter } from 'eventemitter3'
import { AppDisconnectedEvent } from '../../../bindings/AppDisconnectedEvent'
import { GetInfoResponse } from '../../../bindings/GetInfoResponse'
import { IotaRequest } from './requestTypes'
import { SignedMessage, SignedTransaction, parseRequest } from './utils'
export interface SignIotaTransactionEvent {
  sessionId: string
  requestId: string
  transactions: Array<TransactionToSign>
}
export type SignIotaMessageEvent = SignMessagesEvent
export interface ClientIotaEvents {
  signTransactions: (e: SignIotaTransactionEvent) => void
  signMessages: (e: SignIotaMessageEvent) => void
  appDisconnected: (e: AppDisconnectedEvent) => void
}
export class ClientIota extends EventEmitter<ClientIotaEvents> {
  baseClient: BaseClient
  sessionId: string | undefined = undefined
  private constructor(baseClient: BaseClient) {
    super()
    baseClient.on('signTransactions', (e) => {
      const event: SignIotaTransactionEvent = {
        sessionId: e.sessionId,
        requestId: e.responseId,
        transactions: e.transactions
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
    client: ClientIota
    data: GetInfoResponse
  }> => {
    // Add prefix to client id
    const baseClient = await BaseClient.build({
      ...initData,
      clientId: 'iota-' + initData.clientId
    })
    const data = await baseClient.getInfo(sessionId)
    const client = new ClientIota(baseClient)
    return { client, data }
  }
  public static create = async (initData: ClientBaseInitialize) => {
    // Add prefix to client id
    const baseClient = await BaseClient.build({
      ...initData,
      clientId: 'iota-' + initData.clientId
    })
    const client = new ClientIota(baseClient)
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
  public getPendingRequests = async (sessionId?: string): Promise<IotaRequest[]> => {
    const sessionIdToUse = sessionId || this.sessionId

    if (sessionIdToUse === undefined) {
      throw new Error('Session id is undefined')
    }
    const requests = await this.baseClient.getPendingRequests(sessionIdToUse)
    return requests.map((request) => parseRequest(request, sessionIdToUse))
  }

  public resolveSignTransaction = async ({
    responseId,
    signedTransactions,
    sessionId
  }: ResolveSignIotaTransactions) => {
    const serializedTxs = signedTransactions
      .map((tx) => JSON.stringify(tx))
      .map((tx) => {
        return { transaction: tx }
      })
    const sessionIdToUse = sessionId || this.sessionId
    if (sessionIdToUse === undefined) {
      throw new Error('Session id is undefined')
    }
    await this.baseClient.resolveSignTransactions({
      requestId: responseId,
      signedTransactions: serializedTxs,
      sessionId: sessionIdToUse
    })
  }
  public resolveSignMessage = async ({
    responseId,
    signature,
    sessionId
  }: ResolveSignIotaMessage) => {
    const sessionIdToUse = sessionId || this.sessionId
    //Assert session id is defined
    if (sessionIdToUse === undefined) {
      throw new Error('Session id is undefined')
    }
    await this.baseClient.resolveSignMessages({
      requestId: responseId,
      sessionId: sessionIdToUse,
      signedMessages: [{ message: JSON.stringify(signature) }]
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
  public getSessions = async (): Promise<string[]> => {
    return await this.baseClient.getSessions()
  }
  public dropSessions = async (sessionsToDrop: string[]): Promise<string[]> => {
    return await this.baseClient.dropSessions(sessionsToDrop)
  }
}

export interface RejectRequest {
  requestId: string
  reason?: string
  sessionId?: string
}
export interface ResolveSignIotaTransactions {
  responseId: string
  signedTransactions: Array<SignedTransaction | IotaTransactionBlockResponse>
  sessionId?: string
}
export interface ResolveSignIotaMessage {
  responseId: string
  signature: SignedMessage
  sessionId?: string
}
