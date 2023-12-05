import {
  AccountInfo,
  NetworkInfo,
  SignMessageResponse,
  Types
} from '@aptos-labs/wallet-adapter-core'
import { HttpBaseClient, HttpBaseClientInitialize } from '@nightlylabs/nightly-connect-base'
import { HttpConnectSessionRequest } from '../../../bindings/HttpConnectSessionRequest'
import { HttpGetPendingRequestRequest } from '../../../bindings/HttpGetPendingRequestRequest'
import { HttpGetPendingRequestsRequest } from '../../../bindings/HttpGetPendingRequestsRequest'
import { HttpGetSessionInfoResponse } from '../../../bindings/HttpGetSessionInfoResponse'
import { APTOS_NETWORK, parseRequest, serializeSignMessageResponse } from './utils'

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
  public connect = async (connect: AptosConnect) => {
    await this.baseClient.connect({
      ...connect,
      publicKeys: connect.publicKeys.map((pk) => JSON.stringify(pk)),
      metadata: JSON.stringify({ networkInfo: connect.networkInfo })
    })
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
    transactionHashes,
    sessionId
  }: ResolveSignAptosTransactions) => {
    const serializedTxs = transactionHashes.map((tx) => {
      return { network: APTOS_NETWORK, transaction: JSON.stringify({ hash: tx.hash }) }
    })

    await this.baseClient.resolveSignTransactions({
      requestId: requestId,
      signedTransactions: serializedTxs,
      sessionId: sessionId
    })
  }
  public resolveSignMessage = async ({
    requestId,
    response,
    sessionId
  }: ResolveSignAptosMessage) => {
    await this.baseClient.resolveSignMessages({
      requestId: requestId,
      sessionId: sessionId,
      signedMessages: [{ message: serializeSignMessageResponse(response) }]
    })
  }
  public rejectRequest = async ({ requestId, reason, sessionId }: RejectRequest) => {
    await this.baseClient.reject({ requestId: requestId, reason, sessionId: sessionId })
  }
}
export type AptosConnect = Omit<HttpConnectSessionRequest, 'publicKeys' | 'clientId'> & {
  publicKeys: AccountInfo[]
  networkInfo: NetworkInfo
}

export interface RejectRequest {
  requestId: string
  sessionId: string
  reason?: string
}
export interface ResolveSignAptosTransactions {
  requestId: string
  transactionHashes: Array<{ hash: Types.HexEncodedBytes }>
  sessionId: string
}
export interface ResolveSignAptosMessage {
  requestId: string
  response: SignMessageResponse
  sessionId: string
}
