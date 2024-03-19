import { AppDisconnectedEvent } from '../../../bindings/AppDisconnectedEvent'
import {
  BaseClient,
  ClientBaseInitialize,
  Connect as ConnectBase
} from '@nightlylabs/nightly-connect-base'
import { EventEmitter } from 'eventemitter3'
import { GetInfoResponse } from '../../../bindings/GetInfoResponse'
import { AptosRequest } from './requestTypes'
import {
  deserializeAptosTx,
  deserializeObject,
  parseRequest,
  serializeAccountAuthenticator,
  serializeConnectData,
  serializeObject,
  serializePendingTransactionResponse
} from './utils'
import { AnyRawTransaction } from '@aptos-labs/ts-sdk'
import {
  AccountInfo,
  AptosSignAndSubmitTransactionOutput,
  AptosSignMessageInput,
  AptosSignMessageOutput,
  AptosSignTransactionOutput,
  NetworkInfo
} from '@aptos-labs/wallet-standard'
export interface SignAndSubmitTransactionEvent {
  sessionId: string
  requestId: string
  transactions: Array<AnyRawTransaction>
}
export interface SignTransactionEvent {
  sessionId: string
  requestId: string
  transactions: Array<AnyRawTransaction>
}
export interface SignMessagesEvent {
  requestId: string
  sessionId: string
  messages: Array<AptosSignMessageInput>
}
export interface ClientAptosEvents {
  signAndSubmitTransaction: (e: SignAndSubmitTransactionEvent) => void
  signTransaction: (e: SignTransactionEvent) => void
  signMessage: (e: SignMessagesEvent) => void
  appDisconnected: (e: AppDisconnectedEvent) => void
}
export class ClientAptos extends EventEmitter<ClientAptosEvents> {
  baseClient: BaseClient
  sessionId: string | undefined = undefined
  private constructor(baseClient: BaseClient) {
    super()
    baseClient.on('signTransactions', (e) => {
      if (e.transactions.length === 0) {
        return
      }
      const execute = e.transactions[0].metadata
        ? JSON.parse(e.transactions[0].metadata).execute
        : true
      if (execute) {
        const event: SignAndSubmitTransactionEvent = {
          sessionId: e.sessionId,
          requestId: e.responseId,
          transactions: e.transactions.map((tx) => deserializeAptosTx(tx.transaction))
        }
        this.emit('signAndSubmitTransaction', event)
      } else {
        const event: SignTransactionEvent = {
          sessionId: e.sessionId,
          requestId: e.responseId,
          transactions: e.transactions.map((tx) => deserializeAptosTx(tx.transaction))
        }
        this.emit('signTransaction', event)
      }
    })
    baseClient.on('signMessages', (e) => {
      const event: SignMessagesEvent = {
        sessionId: e.sessionId,
        requestId: e.responseId,
        messages: e.messages.map((tx) => deserializeObject(tx.message))
      }
      this.emit('signMessage', event)
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
      clientId: 'aptos-' + initData.clientId
    })
    const data = await baseClient.getInfo(sessionId)
    const client = new ClientAptos(baseClient)
    return { client, data }
  }
  public static create = async (initData: ClientBaseInitialize) => {
    // Add prefix to client id
    const baseClient = await BaseClient.build({
      ...initData,
      clientId: 'aptos-' + initData.clientId
    })
    const client = new ClientAptos(baseClient)
    return client
  }
  public getInfo = async (sessionId: string): Promise<GetInfoResponse> => {
    const response = await this.baseClient.getInfo(sessionId)
    return response
  }
  public connect = async (connect: Connect) => {
    await this.baseClient.connect({
      sessionId: connect.sessionId,
      device: connect.device,
      metadata: serializeConnectData(connect.accountInfo, connect.networkInfo),
      notification: connect.notification,
      publicKeys: [connect.accountInfo.address.toString()]
    })
    this.sessionId = connect.sessionId
  }
  public getPendingRequests = async (sessionId?: string): Promise<AptosRequest[]> => {
    const sessionIdToUse = sessionId || this.sessionId

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
  }: ResolveSignAptosTransactions) => {
    const serializedTxs = signedTransactions
      .map((tx) => serializeAccountAuthenticator(tx))
      .map((tx) => {
        return { transaction: tx }
      })
    const sessionIdToUse = sessionId || this.sessionId
    if (sessionIdToUse === undefined) {
      throw new Error('Session id is undefined')
    }
    await this.baseClient.resolveSignTransactions({
      requestId,
      signedTransactions: serializedTxs,
      sessionId: sessionIdToUse
    })
  }
  public resolveSignAndSubmitTransaction = async ({
    requestId,
    signedTransactions,
    sessionId
  }: ResolveSignAndSubmitTransactions) => {
    const serializedTxs = signedTransactions
      .map((tx) => serializePendingTransactionResponse(tx))
      .map((tx) => {
        return { transaction: tx }
      })
    const sessionIdToUse = sessionId || this.sessionId
    if (sessionIdToUse === undefined) {
      throw new Error('Session id is undefined')
    }
    await this.baseClient.resolveSignTransactions({
      requestId,
      signedTransactions: serializedTxs,
      sessionId: sessionIdToUse
    })
  }
  public resolveSignMessage = async ({
    requestId,
    signedMessages,
    sessionId
  }: ResolveSignAptosMessage) => {
    const sessionIdToUse = sessionId || this.sessionId
    //Assert session id is defined
    if (sessionIdToUse === undefined) {
      throw new Error('Session id is undefined')
    }
    const serializedMsgs = signedMessages
      .map((tx) => serializeObject(tx))
      .map((tx) => {
        return { message: tx }
      })
    await this.baseClient.resolveSignMessages({
      requestId: requestId,
      sessionId: sessionIdToUse,
      signedMessages: serializedMsgs
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
export type Connect = Omit<ConnectBase, 'publicKeys' | 'metadata'> & {
  accountInfo: AccountInfo
  networkInfo: NetworkInfo
}
export interface RejectRequest {
  requestId: string
  reason?: string
  sessionId?: string
}
export interface ResolveSignAptosTransactions {
  requestId: string
  signedTransactions: Array<AptosSignTransactionOutput>
  sessionId?: string
}
export interface ResolveSignAndSubmitTransactions {
  requestId: string
  signedTransactions: Array<AptosSignAndSubmitTransactionOutput>
  sessionId?: string
}
export interface ResolveSignAptosMessage {
  requestId: string
  signedMessages: Array<AptosSignMessageOutput>
  sessionId?: string
}
