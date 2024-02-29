import { AppDisconnectedEvent } from '../../../bindings/AppDisconnectedEvent'
import { VersionedTransaction } from '@solana/web3.js'
import {
  BaseClient,
  ClientBaseInitialize,
  Connect,
  SignMessagesEvent
} from '@nightlylabs/nightly-connect-base'
import { EventEmitter } from 'eventemitter3'
import { SOLANA_NETWORK } from './utils'
import { GetInfoResponse } from '../../../bindings/GetInfoResponse'
import { parseRequest } from './utils'
import { SolanaRequest } from './requestTypes'

export interface SignSolanaTransactionEvent {
  requestId: string
  transactions: Array<VersionedTransaction>
  sessionId: string
}
export type SignSolanaMessageEvent = SignMessagesEvent
export interface ClientSolanaEvents {
  signTransactions: (e: SignSolanaTransactionEvent) => void
  signMessages: (e: SignSolanaMessageEvent) => void
  appDisconnected: (e: AppDisconnectedEvent) => void
}
export class ClientSolana extends EventEmitter<ClientSolanaEvents> {
  baseClient: BaseClient
  sessionId: string | undefined = undefined
  private constructor(baseClient: BaseClient) {
    super()
    baseClient.on('signTransactions', (e) => {
      const event: SignSolanaTransactionEvent = {
        sessionId: e.sessionId,
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
    // Add prefix to client id
    const baseClient = await BaseClient.build({
      ...initData,
      clientId: 'solana-' + initData.clientId
    })
    const data = await baseClient.getInfo(sessionId)
    const client = new ClientSolana(baseClient)
    return { client, data }
  }
  public static create = async (initData: ClientBaseInitialize) => {
    // Add prefix to client id
    const baseClient = await BaseClient.build({
      ...initData,
      clientId: 'solana-' + initData.clientId
    })
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
  public getPendingRequests = async (sessionId?: string): Promise<SolanaRequest[]> => {
    const sessionIdToUse = sessionId || this.sessionId
    //Assert session id is defined
    if (sessionIdToUse === undefined) {
      throw new Error('Session id is undefined')
    }
    const requests = await this.baseClient.getPendingRequests(sessionIdToUse)
    return requests.map((request) => parseRequest(request, sessionIdToUse))
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
