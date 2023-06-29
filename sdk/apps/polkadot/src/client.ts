import { AppDisconnectedEvent } from '../../../bindings/AppDisconnectedEvent'
import {
  BaseClient,
  ClientBaseInitialize,
  Connect,
  SignMessagesEvent
} from '@nightlylabs/nightly-connect-base'
import { EventEmitter } from 'eventemitter3'
import { POLKADOT_NETWORK } from './utils'
import { GetInfoResponse } from '../../../bindings/GetInfoResponse'
import { GetPendingRequestsResponse } from '../../../bindings/GetPendingRequestsResponse'
import { SignerPayloadJSON, SignerPayloadRaw, SignerResult } from '@polkadot/types/types'

export interface SignPolkadotTransactionEvent {
  requestId: string
  transactions: Array<SignerPayloadRaw | SignerPayloadJSON>
  sessionId: string
}
export type SignPolkadotMessageEvent = SignMessagesEvent
export interface ClientPolkadotEvents {
  signTransactions: (e: SignPolkadotTransactionEvent) => void
  appDisconnected: (e: AppDisconnectedEvent) => void
}
export class ClientPolkadot extends EventEmitter<ClientPolkadotEvents> {
  baseClient: BaseClient
  sessionId: string | undefined = undefined
  private constructor(baseClient: BaseClient) {
    super()
    baseClient.on('signTransactions', (e) => {
      const event: SignPolkadotTransactionEvent = {
        sessionId: e.sessionId,
        requestId: e.responseId,
        transactions: e.transactions.map(
          (transaction) =>
            JSON.parse(transaction.transaction) as SignerPayloadRaw | SignerPayloadJSON
        )
      }
      this.emit('signTransactions', event)
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
    client: ClientPolkadot
    data: GetInfoResponse
  }> => {
    // Add prefix to client id
    const baseClient = await BaseClient.build({
      ...initData,
      clientId: 'polkadot-' + initData.clientId
    })
    const data = await baseClient.getInfo(sessionId)
    const client = new ClientPolkadot(baseClient)
    return { client, data }
  }
  public static create = async (initData: ClientBaseInitialize) => {
    // Add prefix to client id
    const baseClient = await BaseClient.build({
      ...initData,
      clientId: 'polkadot-' + initData.clientId
    })
    const client = new ClientPolkadot(baseClient)
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
  }: ResolveSignPolkadotTransactions) => {
    const serializedTxs = signedTransactions.map((tx) => {
      return { network: POLKADOT_NETWORK, transaction: tx }
    })
    const sessionIdToUse = sessionId || this.sessionId
    //Assert session id is defined
    if (sessionIdToUse === undefined) {
      throw new Error('Session id is undefined')
    }
    await this.baseClient.resolveSignTransactions({
      requestId: requestId,
      signedTransactions: serializedTxs.map((tx) => {
        return { ...tx, transaction: JSON.stringify(tx.transaction) }
      }),
      sessionId: sessionIdToUse
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
export interface ResolveSignPolkadotTransactions {
  requestId: string
  signedTransactions: SignerResult[]
  sessionId?: string
}
