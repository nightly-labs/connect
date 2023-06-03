import { VersionedTransaction } from '@solana/web3.js'
import { HttpBaseClient, HttpBaseClientInitialize } from 'base'
import { SOLANA_NETWORK } from './utils'
import { HttpConnectSessionRequest } from '@bindings/HttpConnectSessionRequest'
import { HttpGetPendingRequestsRequest } from '@bindings/HttpGetPendingRequestsRequest'

export class HttpClientSolana {
  baseClient: HttpBaseClient
  clientId: string | undefined = undefined
  public constructor({ clientId, timeout, url }: HttpBaseClientInitialize) {
    this.clientId = clientId
    this.baseClient = new HttpBaseClient({ clientId, timeout, url })
  }
  public getInfo = async (sessionId: string) => {
    const response = await this.baseClient.getInfo(sessionId)
    return response
  }
  public connect = async (connect: Omit<HttpConnectSessionRequest, 'clientId'>) => {
    await this.baseClient.connect(connect)
  }
  public getPendingRequests = async (request: Omit<HttpGetPendingRequestsRequest, 'clientId'>) => {
    return await this.baseClient.getPendingRequests(request)
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
  }: ResolveSignSolanaMessage) => {
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
export interface ResolveSignSolanaTransactions {
  requestId: string
  signedTransactions: VersionedTransaction[]
  sessionId: string
}
export interface ResolveSignSolanaMessage {
  requestId: string
  signature: Uint8Array
  sessionId: string
}
