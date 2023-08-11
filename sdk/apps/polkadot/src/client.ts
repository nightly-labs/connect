import {
  BaseClient,
  ClientBaseInitialize,
  Connect as ConnectBase,
  SignMessagesEvent
} from '@nightlylabs/nightly-connect-base'
import { InjectedAccount } from '@polkadot/extension-inject/types'
import { SignerPayloadJSON, SignerPayloadRaw, SignerResult } from '@polkadot/types/types'
import { EventEmitter } from 'eventemitter3'
import { AppDisconnectedEvent } from '../../../bindings/AppDisconnectedEvent'
import { GetInfoResponse } from '../../../bindings/GetInfoResponse'
import { Network } from '../../../bindings/Network'
import { parseRequest } from './utils'
import { PolkadotRequest } from './requestTypes'

export interface SignPolkadotTransactionEvent {
  requestId: string
  transactions: Array<SignerPayloadRaw | SignerPayloadJSON>
  sessionId: string
  network?: string
}

export type SignPolkadotMessageEvent = SignMessagesEvent
export interface ClientPolkadotEvents {
  signTransactions: (e: SignPolkadotTransactionEvent) => void
  appDisconnected: (e: AppDisconnectedEvent) => void
}
export class ClientPolkadot extends EventEmitter<ClientPolkadotEvents> {
  baseClient: BaseClient
  sessionId: string | undefined = undefined
  network: Network
  private constructor(baseClient: BaseClient, network: Network) {
    super()
    this.network = network
    baseClient.on('signTransactions', (e) => {
      const event: SignPolkadotTransactionEvent = {
        sessionId: e.sessionId,
        requestId: e.responseId,
        transactions: e.transactions.map(
          (transaction) =>
            JSON.parse(transaction.transaction) as SignerPayloadRaw | SignerPayloadJSON
        ),
        network: e.transactions[0]?.metadata
          ? JSON.parse(e.transactions[0]?.metadata).network
          : undefined
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
    initData: ClientBaseInitialize,
    network: Network
  ): Promise<{
    client: ClientPolkadot
    data: GetInfoResponse
  }> => {
    // Add prefix to client id
    const baseClient = await BaseClient.build({
      ...initData,
      clientId: network + '-' + initData.clientId
    })
    const data = await baseClient.getInfo(sessionId)
    const client = new ClientPolkadot(baseClient, network)
    return { client, data }
  }
  public static create = async (initData: ClientBaseInitialize, network: Network) => {
    // Add prefix to client id
    const baseClient = await BaseClient.build({
      ...initData,
      clientId: network + '-' + initData.clientId
    })
    const client = new ClientPolkadot(baseClient, network)
    return client
  }
  public getInfo = async (sessionId: string): Promise<GetInfoResponse> => {
    const response = await this.baseClient.getInfo(sessionId)
    return response
  }
  public connect = async (connect: Connect) => {
    const connectMsg: ConnectBase = {
      ...connect,
      publicKeys: connect.walletsMetadata.map((wallet) => wallet.address),
      metadata: JSON.stringify(connect.walletsMetadata)
    }
    await this.baseClient.connect(connectMsg)
    this.sessionId = connect.sessionId
  }
  public getPendingRequests = async (sessionId?: string): Promise<PolkadotRequest[]> => {
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
  }: ResolveSignPolkadotTransactions) => {
    const serializedTxs = signedTransactions.map((tx) => {
      return { network: this.network, transaction: tx }
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
export type Connect = ConnectBase & {
  walletsMetadata: InjectedAccount[]
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
