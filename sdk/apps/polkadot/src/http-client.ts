import { HttpBaseClient, HttpBaseClientInitialize } from '@nightlylabs/nightly-connect-base'
import { HttpConnectSessionRequest } from '../../../bindings/HttpConnectSessionRequest'
import { HttpGetPendingRequestsRequest } from '../../../bindings/HttpGetPendingRequestsRequest'
import { HttpGetPendingRequestRequest } from '../../../bindings/HttpGetPendingRequestRequest'
import { HttpGetSessionInfoResponse } from '../../../bindings/HttpGetSessionInfoResponse'
import { SignerResult } from '@polkadot/types/types'
import { InjectedAccount } from '@polkadot/extension-inject/types'
import { Network } from '../../../bindings/Network'

export class HttpClientPolkadot {
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
  public connect = async (connect: HttpConnect) => {
    await this.baseClient.connect(connect)
  }
  public getPendingRequests = async (request: Omit<HttpGetPendingRequestsRequest, 'clientId'>) => {
    return await this.baseClient.getPendingRequests(request)
  }
  public getPendingRequest = async (request: Omit<HttpGetPendingRequestRequest, 'clientId'>) => {
    return await this.baseClient.getPendingRequest(request)
  }
  public resolveSignTransaction = async ({
    requestId,
    signedTransactions,
    sessionId,
    network
  }: ResolveSignPolkadotTransactions) => {
    const serializedTxs = signedTransactions.map((tx) => {
      return { network: network, transaction: tx }
    })

    await this.baseClient.resolveSignTransactions({
      requestId: requestId,
      signedTransactions: serializedTxs.map((tx) => {
        return { ...tx, transaction: JSON.stringify(tx.transaction) }
      }),
      sessionId: sessionId
    })
  }

  public rejectRequest = async ({ requestId, reason, sessionId }: RejectRequest) => {
    await this.baseClient.reject({ requestId: requestId, reason, sessionId: sessionId })
  }
}
export type HttpConnect = Omit<HttpConnectSessionRequest, 'clientId'> & {
  walletsMetadata: InjectedAccount[]
}
export interface RejectRequest {
  requestId: string
  sessionId: string
  reason?: string
}
export interface ResolveSignPolkadotTransactions {
  requestId: string
  signedTransactions: SignerResult[]
  sessionId: string
  network: Network
}
