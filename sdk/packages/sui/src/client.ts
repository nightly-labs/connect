import { AppDisconnectedEvent } from '../../../bindings/AppDisconnectedEvent'
import type { SuiTransactionBlockResponse } from '@mysten/sui.js/client'
import type { SignedMessage, SignedTransaction } from '@mysten/sui.js/src/signers/types'
import {
  BaseClient,
  ClientBaseInitialize,
  Connect,
  SignMessagesEvent,
  TransactionToSign
} from '@nightlylabs/nightly-connect-base'
import { EventEmitter } from 'eventemitter3'
import { GetInfoResponse } from '../../../bindings/GetInfoResponse'
import { SuiRequest } from './requestTypes'
import { parseRequest } from './utils'
export interface SignSuiTransactionEvent {
  sessionId: string
  requestId: string
  transactions: Array<TransactionToSign>
}
export type SignSuiMessageEvent = SignMessagesEvent
export interface ClientSuiEvents {
  signTransactions: (e: SignSuiTransactionEvent) => void
  signMessages: (e: SignSuiMessageEvent) => void
  appDisconnected: (e: AppDisconnectedEvent) => void
}
export class ClientSui extends EventEmitter<ClientSuiEvents> {
  baseClient: BaseClient
  sessionId: string | undefined = undefined
  private constructor(baseClient: BaseClient) {
    super()
    baseClient.on('signTransactions', (e) => {
      const event: SignSuiTransactionEvent = {
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
    client: ClientSui
    data: GetInfoResponse
  }> => {
    // Add prefix to client id
    const baseClient = await BaseClient.build({ ...initData, clientId: 'sui-' + initData.clientId })
    const data = await baseClient.getInfo(sessionId)
    const client = new ClientSui(baseClient)
    return { client, data }
  }
  public static create = async (initData: ClientBaseInitialize) => {
    // Add prefix to client id
    const baseClient = await BaseClient.build({ ...initData, clientId: 'sui-' + initData.clientId })
    const client = new ClientSui(baseClient)
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
  public getPendingRequests = async (sessionId?: string): Promise<SuiRequest[]> => {
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
  }: ResolveSignSuiTransactions) => {
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
  }: ResolveSignSuiMessage) => {
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
export interface ResolveSignSuiTransactions {
  responseId: string
  signedTransactions: Array<SignedTransaction | SuiTransactionBlockResponse>
  sessionId?: string
}
export interface ResolveSignSuiMessage {
  responseId: string
  signature: SignedMessage
  sessionId?: string
}
