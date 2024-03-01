import {
  AptosSignAndSubmitTransactionOutput,
  AptosSignMessageOutput,
  AptosSignTransactionOutput
} from '@aptos-labs/wallet-standard'
import { HttpBaseClient, HttpBaseClientInitialize } from '@nightlylabs/nightly-connect-base'
import { HttpConnectSessionRequest } from '../../../bindings/HttpConnectSessionRequest'
import { HttpGetPendingRequestRequest } from '../../../bindings/HttpGetPendingRequestRequest'
import { HttpGetPendingRequestsRequest } from '../../../bindings/HttpGetPendingRequestsRequest'
import { HttpGetSessionInfoResponse } from '../../../bindings/HttpGetSessionInfoResponse'
import {
  parseRequest,
  serializeAccountAuthenticator,
  serializeObject,
  serializePendingTransactionResponse
} from './utils'

export class HttpClientAptos {
  baseClient: HttpBaseClient
  clientId: string | undefined = undefined
  public constructor({ clientId, timeout, url }: HttpBaseClientInitialize) {
    this.clientId = clientId
    this.baseClient = new HttpBaseClient({ clientId, timeout, url })
  }
  public getInfo = async (sessionId: string): Promise<HttpGetSessionInfoResponse> => {
    const response = await this.baseClient.getInfo(sessionId)
    return response
  }
  public connect = async (connect: Omit<HttpConnectSessionRequest, 'clientId'>) => {
    await this.baseClient.connect(connect)
  }
  public getPendingRequests = async (request: Omit<HttpGetPendingRequestsRequest, 'clientId'>) => {
    const requests = await this.baseClient.getPendingRequests(request)
    return requests.map((rq) => parseRequest(rq, request.sessionId))
  }
  public getPendingRequest = async (request: Omit<HttpGetPendingRequestRequest, 'clientId'>) => {
    const rq = await this.baseClient.getPendingRequest(request)
    return parseRequest(rq, request.sessionId)
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
    await this.baseClient.resolveSignTransactions({
      requestId: requestId,
      signedTransactions: serializedTxs,
      sessionId: sessionId
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
    await this.baseClient.resolveSignTransactions({
      requestId: requestId,
      signedTransactions: serializedTxs,
      sessionId: sessionId
    })
  }
  public resolveSignMessage = async ({
    requestId,
    signedMessages,
    sessionId
  }: ResolveSignAptosMessage) => {
    const serializedMsgs = signedMessages
      .map((tx) => serializeObject(tx))
      .map((tx) => {
        return { message: tx }
      })
    await this.baseClient.resolveSignMessages({
      requestId: requestId,
      sessionId: sessionId,
      signedMessages: serializedMsgs
    })
  }
  public rejectRequest = async ({ requestId, reason, sessionId }: RejectRequest) => {
    await this.baseClient.reject({ requestId: requestId, reason, sessionId: sessionId })
  }
}
export interface RejectRequest {
  requestId: string
  sessionId: string
  reason?: string
}
export interface ResolveSignAptosMessage {
  requestId: string
  signedMessages: Array<AptosSignMessageOutput>
  sessionId: string
}

export interface ResolveSignAptosTransactions {
  requestId: string
  signedTransactions: Array<AptosSignTransactionOutput>
  sessionId: string
}
export interface ResolveSignAndSubmitTransactions {
  requestId: string
  signedTransactions: Array<AptosSignAndSubmitTransactionOutput>
  sessionId: string
}
