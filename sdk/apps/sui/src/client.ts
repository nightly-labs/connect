import { AppDisconnectedEvent } from '@bindings/AppDisconnectedEvent'
import { SignedMessage, SignedTransaction, TransactionBlock } from '@mysten/sui.js'
import { BaseClient, ClientBaseInitialize, Connect } from 'base'
import { TypedEmitter } from 'tiny-typed-emitter'
import { SUI_NETWORK } from './utils'
import { SignMessagesEvent } from 'base/src/client'
export interface SignSuiTransactionEvent {
  requestId: string
  transactions: Array<TransactionBlock>
  metadata?: string
}
export type SignSuiMessageEvent = SignMessagesEvent
export interface ClientSuiEvents {
  signTransactions: (e: SignSuiTransactionEvent) => void
  signMessages: (e: SignSuiMessageEvent) => void
  appDisconnected: (e: AppDisconnectedEvent) => void
}
export class ClientSui extends TypedEmitter<ClientSuiEvents> {
  baseClient: BaseClient
  sessionId: string | undefined = undefined
  private constructor(baseClient: BaseClient) {
    super()
    baseClient.on('signTransactions', (e) => {
      const event: SignSuiTransactionEvent = {
        requestId: e.responseId,
        transactions: e.transactions.map((tx) => {
          return TransactionBlock.from(tx.transaction)
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
  public static build = async (sessionId: string, initData: ClientBaseInitialize) => {
    const baseClient = await BaseClient.build(initData)
    const data = await baseClient.getInfo(sessionId)
    const client = new ClientSui(baseClient)
    return { client, data }
  }
  public static create = async (initData: ClientBaseInitialize) => {
    const baseClient = await BaseClient.build(initData)
    const client = new ClientSui(baseClient)
    return client
  }
  public getInfo = async (sessionId: string) => {
    const response = await this.baseClient.getInfo(sessionId)
    return response
  }
  public connect = async (connect: Connect) => {
    await this.baseClient.connect(connect)
    this.sessionId = connect.sessionId
  }
  public getPendingRequests = async (sessionId?: string) => {
    const sessionIdToUse = sessionId || this.sessionId

    if (sessionIdToUse === undefined) {
      throw new Error('Session id is undefined')
    }
    return await this.baseClient.getPendingRequests(sessionIdToUse)
  }

  public resolveSignTransaction = async ({
    responseId,
    signedTransactions,
    sessionId
  }: ResolveSignSuiTransactions) => {
    const serializedTxs = signedTransactions
      .map((tx) => JSON.stringify(tx))
      .map((tx) => {
        return { network: SUI_NETWORK, transaction: tx }
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
}

export interface RejectRequest {
  requestId: string
  reason?: string
  sessionId?: string
}
export interface ResolveSignSuiTransactions {
  responseId: string
  signedTransactions: SignedTransaction[]
  sessionId?: string
}
export interface ResolveSignSuiMessage {
  responseId: string
  signature: SignedMessage
  sessionId?: string
}
