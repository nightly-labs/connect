import { AppDisconnectedEvent } from '@bindings/AppDisconnectedEvent'
import { SignMessagesRequest } from '@bindings/SignMessagesRequest'
import {
  SerializedSignature,
  SignedMessage,
  SignedTransaction,
  TransactionBlock
} from '@mysten/sui.js'
import { BaseClient, ClientBaseInitialize, Connect } from 'base'
import { TypedEmitter } from 'tiny-typed-emitter'
import { SUI_NETWORK } from './utils'
export interface SignSuiTransactionEvent {
  requestId: string
  transactions: Array<TransactionBlock>
  metadata?: string
}
export type SignSuiMessageEvent = SignMessagesRequest
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
        }),
        metadata: e.metadata
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
  public getPendingRequests = async () => {
    //Assert session id is defined
    if (this.sessionId === undefined) {
      throw new Error('Session id is undefined')
    }
    return await this.baseClient.getPendingRequests()
  }

  public resolveSignTransaction = async ({
    responseId,
    signedTransactions
  }: ResolveSignSuiTransactions) => {
    const serializedTxs = signedTransactions
      .map((tx) => JSON.stringify(tx))
      .map((tx) => {
        return { network: SUI_NETWORK, transaction: tx }
      })
    await this.baseClient.resolveSignTransactions({
      requestId: responseId,
      signedTransactions: serializedTxs
    })
  }
  public resolveSignMessage = async ({ responseId, signature }: ResolveSignSuiMessage) => {
    await this.baseClient.resolveSignMessages({
      requestId: responseId,
      signedMessages: [{ signedMessage: JSON.stringify(signature) }]
    })
  }
  public rejectRequest = async (requestId: string, reason?: string) => {
    await this.baseClient.reject({ requestId, reason })
  }
}

export interface ResolveSignSuiTransactions {
  responseId: string
  signedTransactions: SignedTransaction[]
}
export interface ResolveSignSuiMessage {
  responseId: string
  signature: SignedMessage
}
