import { AppDisconnectedEvent } from '@bindings/AppDisconnectedEvent'
import { VersionedTransaction } from '@solana/web3.js'
import { BaseClient, ClientBaseInitialize, Connect } from 'base'
import { TypedEmitter } from 'tiny-typed-emitter'
import { SOLANA_NETWORK } from './utils'
import { SignMessagesEvent } from 'base/src/client'
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
export class ClientSolana extends TypedEmitter<ClientSolanaEvents> {
  baseClient: BaseClient
  sessionId: string | undefined = undefined
  private constructor(baseClient: BaseClient) {
    super()
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
  public static build = async (sessionId: string, initData: ClientBaseInitialize) => {
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
