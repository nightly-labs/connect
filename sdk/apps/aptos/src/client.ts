import {
  AccountInfo,
  NetworkInfo,
  SignMessagePayload,
  SignMessageResponse,
  Types
} from '@aptos-labs/wallet-adapter-core'
import { BaseClient, ClientBaseInitialize, Connect } from '@nightlylabs/nightly-connect-base'
import { EventEmitter } from 'eventemitter3'
import { AppDisconnectedEvent } from '../../../bindings/AppDisconnectedEvent'
import { GetInfoResponse } from '../../../bindings/GetInfoResponse'
import { AptosRequest } from './requestTypes'
import { APTOS_NETWORK, parseRequest, serializeSignMessageResponse } from './utils'

export interface SignAptosTransactionEvent {
  requestId: string
  transactions: Array<{
    transaction: Types.TransactionPayload
    options: unknown & { submit: boolean }
  }>
  sessionId: string
}
export interface SignAptosMessageEvent {
  requestId: string
  sessionId: string
  messages: Array<{
    message: SignMessagePayload
    metadata?: string
  }>
}
export interface ClientAptosEvents {
  signTransactions: (e: SignAptosTransactionEvent) => void
  signMessages: (e: SignAptosMessageEvent) => void
  appDisconnected: (e: AppDisconnectedEvent) => void
}
export class ClientAptos extends EventEmitter<ClientAptosEvents> {
  baseClient: BaseClient
  sessionId: string | undefined = undefined
  private constructor(baseClient: BaseClient) {
    super()
    baseClient.on('signTransactions', (e) => {
      const event: SignAptosTransactionEvent = {
        sessionId: e.sessionId,
        requestId: e.responseId,
        transactions: e.transactions.map((tx) => {
          const transaction = JSON.parse(tx.transaction)
          const metadata = tx.metadata ? JSON.parse(tx.metadata) : { submit: true }
          return { options: metadata, transaction }
        })
      }
      this.emit('signTransactions', event)
    })
    baseClient.on('signMessages', (e) => {
      const event: SignAptosMessageEvent = {
        sessionId: e.sessionId,
        requestId: e.responseId,
        messages: e.messages.map((tx) => {
          const message = JSON.parse(tx.message)
          return { metadata: tx.metadata, message: message }
        })
      }
      this.emit('signMessages', event)
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
    client: ClientAptos
    data: GetInfoResponse
  }> => {
    // Add prefix to client id
    const baseClient = await BaseClient.build({
      ...initData,
      clientId: APTOS_NETWORK + '-' + initData.clientId
    })
    const data = await baseClient.getInfo(sessionId)
    const client = new ClientAptos(baseClient)
    return { client, data }
  }
  public static create = async (initData: ClientBaseInitialize) => {
    // Add prefix to client id
    const baseClient = await BaseClient.build({
      ...initData,
      clientId: APTOS_NETWORK + '-' + initData.clientId
    })
    const client = new ClientAptos(baseClient)
    return client
  }
  public getInfo = async (sessionId: string): Promise<GetInfoResponse> => {
    const response = await this.baseClient.getInfo(sessionId)
    return response
  }
  public connect = async (connect: AptosConnect) => {
    await this.baseClient.connect({
      ...connect,
      publicKeys: connect.publicKeys.map((pk) => JSON.stringify(pk)),
      metadata: JSON.stringify({ networkInfo: connect.networkInfo })
    })
    this.sessionId = connect.sessionId
  }
  public getPendingRequests = async (sessionId?: string): Promise<AptosRequest[]> => {
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
    transactionHashes,
    sessionId
  }: ResolveSignAptosTransactions) => {
    const serializedTxs = transactionHashes.map((tx) => {
      return { network: APTOS_NETWORK, transaction: JSON.stringify({ hash: tx.hash }) }
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
    response,
    sessionId
  }: ResolveSignAptosMessage) => {
    const sessionIdToUse = sessionId || this.sessionId
    //Assert session id is defined
    if (sessionIdToUse === undefined) {
      throw new Error('Session id is undefined')
    }
    await this.baseClient.resolveSignMessages({
      requestId: requestId,
      sessionId: sessionIdToUse,
      signedMessages: [{ message: serializeSignMessageResponse(response) }]
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
export interface ResolveSignAptosTransactions {
  requestId: string
  transactionHashes: Array<{ hash: Types.HexEncodedBytes }>
  sessionId?: string
}
export interface ResolveSignAptosMessage {
  requestId: string
  response: SignMessageResponse
  sessionId?: string
}

export type AptosConnect = Omit<Connect, 'publicKeys' | 'metadata'> & {
  publicKeys: AccountInfo[]
  networkInfo: NetworkInfo
}
