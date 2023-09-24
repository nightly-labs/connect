import { HttpBaseClient, HttpBaseClientInitialize } from '@nightlylabs/nightly-connect-base'
import { SUI_NETWORK, parseRequest } from './utils'
import { HttpConnectSessionRequest } from '../../../bindings/HttpConnectSessionRequest'
import { HttpGetPendingRequestsRequest } from '../../../bindings/HttpGetPendingRequestsRequest'
import { HttpGetPendingRequestRequest } from '../../../bindings/HttpGetPendingRequestRequest'
import { type SignedTransaction } from '@mysten/sui.js/src/signers/types'
import { HttpGetSessionInfoResponse } from '../../../bindings/HttpGetSessionInfoResponse'

export class HttpClientSui {
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
  }: ResolveSignSuiTransactions) => {
    const serializedTxs = signedTransactions
      .map((tx) => JSON.stringify(tx))
      .map((tx) => {
        return { network: SUI_NETWORK, transaction: tx }
      })

    await this.baseClient.resolveSignTransactions({
      requestId: requestId,
      signedTransactions: serializedTxs,
      sessionId: sessionId
    })
  }
  public resolveSignMessage = async ({
    requestId,
    signature,
    sessionId
  }: ResolveSignSuiMessage) => {
    await this.baseClient.resolveSignMessages({
      requestId: requestId,
      sessionId: sessionId,
      signedMessages: [{ message: Buffer.from(signature).toString('hex') }]
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
export interface ResolveSignSuiTransactions {
  requestId: string
  signedTransactions: SignedTransaction[]
  sessionId: string
}
export interface ResolveSignSuiMessage {
  requestId: string
  signature: Uint8Array
  sessionId: string
}
